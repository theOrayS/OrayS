use core::cmp;
use core::mem::size_of;
use core::sync::atomic::Ordering;

use axalloc::global_allocator;
use axerrno::LinuxError;
use axfs::fops::{FileAttr, FileType, OpenOptions};
use linux_raw_sys::general;
use orays_linux::user::{UserAddr, UserPtr, Write};
use std::string::{String, ToString};
use std::vec::Vec;

use super::credentials::{access_allowed, apply_chown_metadata, chown_ids};
use super::fd_table::{
    FdEntry, check_parent_write_search_permission, lstat_absolute_path, resolve_dirfd_path,
};
use super::linux_abi::{
    ACCESS_MODE_MASK, ACCESS_W_OK, DEVFS_MAGIC, EXT4_SUPER_MAGIC, FILE_MODE_GROUP_EXECUTE,
    FILE_MODE_PERMISSION_MASK, FILE_MODE_SET_GID, FILE_MODE_SET_UID, LINUX_EACCES,
    MAX_IN_MEMORY_FILE_SIZE, PIPEFS_MAGIC, PROC_SUPER_MAGIC, RAMFS_MAGIC, RLIMIT_FSIZE_RESOURCE,
    ST_MODE_BLK, ST_MODE_CHR, ST_MODE_DIR, ST_MODE_FIFO, ST_MODE_FILE, ST_MODE_LNK, ST_MODE_SOCKET,
    ST_MODE_TYPE_MASK, STATFS_BLOCK_SIZE, STATFS_NAME_MAX, SYNTHETIC_BLOCK_DEVICE_SIZE,
    SYSFS_MAGIC, TMPFS_MAGIC, neg_errno, neg_errno_code,
};
use super::runtime_paths::normalize_path;
use super::synthetic_fs::{dev_shm_host_path, proc_exe_link_target};
use super::time_abi::clock_gettime_timespec;
use super::user_memory::{
    read_cstr, read_user_bytes, read_user_value, write_user_bytes, write_user_slice,
    write_user_value,
};
use super::{PathTimes, UserProcess};

pub(super) const DEV_NULL_RDEV: u64 = 259; // Linux makedev(1, 3).
pub(super) const DEV_ZERO_RDEV: u64 = 261; // Linux makedev(1, 5).
pub(super) const DEV_CPU_DMA_LATENCY_RDEV: u64 = 2_684; // Linux misc makedev(10, 124).
const DEV_VDA_RDEV: u64 = 65_024; // Linux makedev(254, 0), virtio block.
const LINUX_PATH_MAX: usize = 4096;
const XATTR_CREATE: usize = 0x1;
const XATTR_REPLACE: usize = 0x2;
const XATTR_NAME_MAX: usize = 255;
const XATTR_SIZE_MAX: usize = 65_536;
const NSEC_PER_SEC: i64 = 1_000_000_000;
// Reclaimed-space credits are stored as zero-length sparse-data sentinels.
// Real userspace paths in this shell are bounded far below this offset, while
// keeping the credit in path_sparse_data avoids widening UserProcess state for
// a bookkeeping value that only matters to sparse regular-file emulation.
const PATH_FREE_BLOCK_CREDIT_BASE: u64 = 1 << 63;
// Keep sparse byte extents bounded like the regular-file physical prefix.  This
// preserves the no-large-contiguous-allocation invariant while avoiding one
// metadata/data extent per tiny sequential write (iozone commonly writes 1 KiB
// records into tmpfs files).
const SPARSE_BYTE_EXTENT_MERGE_LIMIT: usize = 64 * 1024;

#[derive(Clone, Copy)]
enum UtimeSelection {
    Set(general::timespec),
    Now,
    Omit,
}

#[derive(Clone, Copy)]
struct UtimeRequest {
    atime: UtimeSelection,
    mtime: UtimeSelection,
}

impl UtimeRequest {
    fn both_now(self) -> bool {
        matches!(self.atime, UtimeSelection::Now) && matches!(self.mtime, UtimeSelection::Now)
    }

    fn both_omit(self) -> bool {
        matches!(self.atime, UtimeSelection::Omit) && matches!(self.mtime, UtimeSelection::Omit)
    }
}

impl PathTimes {
    fn from_stat(st: &general::stat) -> Self {
        Self {
            atime: stat_timespec(st.st_atime, st.st_atime_nsec),
            mtime: stat_timespec(st.st_mtime, st.st_mtime_nsec),
            ctime: stat_timespec(st.st_ctime, st.st_ctime_nsec),
        }
    }

    fn apply_to_stat(self, st: &mut general::stat) {
        st.st_atime = self.atime.tv_sec as _;
        st.st_atime_nsec = self.atime.tv_nsec.max(0) as _;
        st.st_mtime = self.mtime.tv_sec as _;
        st.st_mtime_nsec = self.mtime.tv_nsec.max(0) as _;
        st.st_ctime = self.ctime.tv_sec as _;
        st.st_ctime_nsec = self.ctime.tv_nsec.max(0) as _;
    }
}

fn stat_timespec(sec: i64, nsec: u64) -> general::timespec {
    general::timespec {
        tv_sec: sec as _,
        tv_nsec: cmp::min(nsec, (NSEC_PER_SEC - 1) as u64) as _,
    }
}

fn realtime_timespec() -> Result<general::timespec, LinuxError> {
    clock_gettime_timespec(general::CLOCK_REALTIME)
}

pub(super) fn file_attr_to_stat(attr: &FileAttr, path: Option<&str>) -> general::stat {
    let st_mode = file_type_mode(attr.file_type()) | attr.perm().bits() as u32;
    let mut st: general::stat = unsafe { core::mem::zeroed() };
    st.st_dev = 1;
    st.st_ino = path_inode(path);
    st.st_mode = st_mode;
    st.st_nlink = 1;
    st.st_size = attr.size() as _;
    st.st_blksize = 512;
    st.st_blocks = attr.blocks() as _;
    st
}

pub(super) fn normalize_file_mode(mode: u32) -> u32 {
    mode & FILE_MODE_PERMISSION_MASK
}

impl UserProcess {
    pub(super) fn set_path_mode(&self, path: String, mode: u32) {
        self.path_modes
            .lock()
            .insert(path, normalize_file_mode(mode));
    }

    pub(super) fn path_mode(&self, path: &str) -> Option<u32> {
        self.path_modes.lock().get(path).copied()
    }

    pub(super) fn set_path_times(&self, path: String, times: PathTimes) {
        self.path_times.lock().insert(path, times);
    }

    pub(super) fn remove_path_times(&self, path: &str) {
        self.path_times.lock().remove(path);
    }

    pub(super) fn path_times(&self, path: &str) -> Option<PathTimes> {
        self.path_times.lock().get(path).copied()
    }

    pub(super) fn set_path_inode(&self, path: String, ino: u64) {
        self.path_inodes.lock().insert(path, ino.max(1));
    }

    pub(super) fn remove_path_inode(&self, path: &str) {
        self.path_inodes.lock().remove(path);
    }

    pub(super) fn path_inode_override(&self, path: &str) -> Option<u64> {
        self.path_inodes.lock().get(path).copied()
    }

    pub(super) fn move_path_metadata(&self, old_path: &str, new_path: String) {
        let fallback_ino = path_inode(Some(old_path));
        let ino = self
            .path_inodes
            .lock()
            .remove(old_path)
            .unwrap_or(fallback_ino);
        self.set_path_inode(new_path.clone(), ino);

        let mut modes = self.path_modes.lock();
        if let Some(mode) = modes.remove(old_path) {
            modes.insert(new_path.clone(), mode);
        } else {
            modes.remove(new_path.as_str());
        }
        drop(modes);

        let mut special_modes = self.path_special_modes.lock();
        if let Some(ty) = special_modes.remove(old_path) {
            special_modes.insert(new_path.clone(), ty);
        } else {
            special_modes.remove(new_path.as_str());
        }
        drop(special_modes);

        let mut rdevs = self.path_rdevs.lock();
        if let Some(rdev) = rdevs.remove(old_path) {
            rdevs.insert(new_path.clone(), rdev);
        } else {
            rdevs.remove(new_path.as_str());
        }
        drop(rdevs);

        let mut owners = self.path_owners.lock();
        if let Some(owner) = owners.remove(old_path) {
            owners.insert(new_path.clone(), owner);
        } else {
            owners.remove(new_path.as_str());
        }
        drop(owners);

        let mut inode_flags = self.path_inode_flags.lock();
        if let Some(flags) = inode_flags.remove(old_path) {
            inode_flags.insert(new_path.clone(), flags);
        } else {
            inode_flags.remove(new_path.as_str());
        }
        drop(inode_flags);

        let mut symlinks = self.path_symlinks.lock();
        if let Some(target) = symlinks.remove(old_path) {
            symlinks.insert(new_path.clone(), target);
        } else {
            symlinks.remove(new_path.as_str());
        }
        drop(symlinks);

        let mut xattrs = self.path_xattrs.lock();
        if let Some(attrs) = xattrs.remove(old_path) {
            xattrs.insert(new_path.clone(), attrs);
        } else {
            xattrs.remove(new_path.as_str());
        }
        drop(xattrs);

        let mut times = self.path_times.lock();
        if let Some(value) = times.remove(old_path) {
            times.insert(new_path.clone(), value);
        } else {
            times.remove(new_path.as_str());
        }
        drop(times);

        self.move_path_sparse_file(old_path, new_path);
    }

    pub(super) fn set_path_special_mode(&self, path: String, ty: u32) {
        self.path_special_modes
            .lock()
            .insert(path, ty & ST_MODE_TYPE_MASK);
    }

    pub(super) fn remove_path_special_mode(&self, path: &str) {
        self.path_special_modes.lock().remove(path);
    }

    pub(super) fn path_special_mode(&self, path: &str) -> Option<u32> {
        self.path_special_modes.lock().get(path).copied()
    }

    pub(super) fn set_path_rdev(&self, path: String, rdev: u64) {
        self.path_rdevs.lock().insert(path, rdev);
    }

    pub(super) fn remove_path_rdev(&self, path: &str) {
        self.path_rdevs.lock().remove(path);
    }

    pub(super) fn path_rdev(&self, path: &str) -> Option<u64> {
        self.path_rdevs.lock().get(path).copied()
    }

    pub(super) fn apply_umask(&self, mode: u32) -> u32 {
        mode & !self.umask.load(Ordering::Acquire)
    }

    pub(super) fn replace_umask(&self, mask: u32) -> u32 {
        self.umask
            .swap(mask & FILE_MODE_PERMISSION_MASK, Ordering::AcqRel)
            & FILE_MODE_PERMISSION_MASK
    }

    pub(super) fn set_path_owner(&self, path: String, owner: Option<u32>, group: Option<u32>) {
        let mut path_owners = self.path_owners.lock();
        let (current_owner, current_group) =
            path_owners.get(path.as_str()).copied().unwrap_or((0, 0));
        path_owners.insert(
            path,
            (
                owner.unwrap_or(current_owner),
                group.unwrap_or(current_group),
            ),
        );
    }

    pub(super) fn path_owner(&self, path: &str) -> Option<(u32, u32)> {
        self.path_owners.lock().get(path).copied()
    }

    pub(super) fn set_path_inode_flags(&self, path: String, flags: u32) {
        if flags == 0 {
            self.path_inode_flags.lock().remove(path.as_str());
        } else {
            self.path_inode_flags.lock().insert(path, flags);
        }
    }

    pub(super) fn remove_path_inode_flags(&self, path: &str) {
        self.path_inode_flags.lock().remove(path);
    }

    pub(super) fn path_inode_flags(&self, path: &str) -> u32 {
        self.path_inode_flags.lock().get(path).copied().unwrap_or(0)
    }

    pub(super) fn set_path_symlink(&self, path: String, target: String) {
        self.path_symlinks.lock().insert(path, target);
    }

    pub(super) fn remove_path_symlink(&self, path: &str) -> bool {
        self.path_symlinks.lock().remove(path).is_some()
    }

    pub(super) fn path_symlink(&self, path: &str) -> Option<String> {
        self.path_symlinks.lock().get(path).cloned()
    }

    pub(super) fn path_hardlink_backing(&self, path: &str) -> Option<String> {
        self.path_hardlinks.lock().get(path).cloned()
    }

    pub(super) fn path_hardlink_exists(&self, path: &str) -> bool {
        self.path_hardlinks.lock().contains_key(path)
    }

    pub(super) fn path_hardlink_count(&self, path: &str) -> Option<u64> {
        let canonical = self
            .path_hardlinks
            .lock()
            .get(path)
            .cloned()
            .unwrap_or_else(|| path.to_string());
        self.path_hardlink_counts
            .lock()
            .get(canonical.as_str())
            .copied()
            .filter(|count| *count > 1)
    }

    pub(super) fn set_path_hardlink(&self, existing_path: &str, new_path: String, ino: u64) {
        let canonical = self
            .path_hardlink_backing(existing_path)
            .unwrap_or_else(|| existing_path.to_string());
        let count = {
            let mut links = self.path_hardlinks.lock();
            links.entry(canonical.clone()).or_insert(canonical.clone());
            links.insert(new_path.clone(), canonical.clone());
            links
                .values()
                .filter(|target| *target == &canonical)
                .count() as u64
        };
        self.path_hardlink_counts
            .lock()
            .insert(canonical.clone(), count);
        self.set_path_inode(canonical, ino);
        self.set_path_inode(new_path, ino);
    }

    pub(super) fn remove_path_hardlink(&self, path: &str) -> Option<(String, u64)> {
        let (canonical, remaining) = {
            let mut links = self.path_hardlinks.lock();
            let canonical = links.remove(path)?;
            let remaining = links
                .values()
                .filter(|target| *target == &canonical)
                .count() as u64;
            (canonical, remaining)
        };
        let mut counts = self.path_hardlink_counts.lock();
        if remaining > 1 {
            counts.insert(canonical.clone(), remaining);
        } else {
            counts.remove(canonical.as_str());
        }
        Some((canonical, remaining))
    }

    pub(super) fn path_symlink_names_in_dir(&self, dir: &str) -> Vec<String> {
        let prefix = if dir == "/" {
            String::from("/")
        } else {
            let mut prefix = dir.trim_end_matches('/').to_string();
            prefix.push('/');
            prefix
        };
        self.path_symlinks
            .lock()
            .keys()
            .filter_map(|path| {
                let name = path.strip_prefix(prefix.as_str())?;
                if name.is_empty() || name.contains('/') {
                    None
                } else {
                    Some(name.to_string())
                }
            })
            .collect()
    }

    pub(super) fn resolve_path_symlink(&self, path: &str) -> Result<Option<String>, LinuxError> {
        let current = self.resolve_path_symlinks(path, true)?;
        Ok((current != path).then_some(current))
    }

    pub(super) fn resolve_parent_symlinks(&self, path: &str) -> Result<String, LinuxError> {
        self.resolve_path_symlinks(path, false)
    }

    fn resolve_path_symlinks(&self, path: &str, follow_final: bool) -> Result<String, LinuxError> {
        let mut current = normalize_path("/", path).ok_or(LinuxError::EINVAL)?;
        let mut traversals = 0;

        loop {
            let components: Vec<&str> =
                current.split('/').filter(|part| !part.is_empty()).collect();
            if components.is_empty() {
                return Ok(current);
            }

            let mut prefix = String::new();
            let mut changed = false;
            for (idx, component) in components.iter().enumerate() {
                prefix.push('/');
                prefix.push_str(component);

                let is_final = idx + 1 == components.len();
                if is_final && !follow_final {
                    continue;
                }
                let Some(target) = self.path_symlink(prefix.as_str()) else {
                    continue;
                };
                traversals += 1;
                if traversals > 40 {
                    return Err(LinuxError::ELOOP);
                }
                let target = normalize_symlink_target(prefix.as_str(), target.as_str())
                    .ok_or(LinuxError::EINVAL)?;
                let suffix = components[idx + 1..].join("/");
                current = if suffix.is_empty() {
                    target
                } else {
                    normalize_path(target.as_str(), suffix.as_str()).ok_or(LinuxError::EINVAL)?
                };
                changed = true;
                break;
            }
            if !changed {
                return Ok(current);
            }
        }
    }

    pub(super) fn path_contains_followed_symlink(
        &self,
        path: &str,
        follow_final: bool,
    ) -> Result<bool, LinuxError> {
        let current = normalize_path("/", path).ok_or(LinuxError::EINVAL)?;
        let components: Vec<&str> = current.split('/').filter(|part| !part.is_empty()).collect();
        let mut prefix = String::new();
        for (idx, component) in components.iter().enumerate() {
            prefix.push('/');
            prefix.push_str(component);
            let is_final = idx + 1 == components.len();
            if is_final && !follow_final {
                continue;
            }
            if self.path_symlink(prefix.as_str()).is_some() {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(super) fn path_symlink_stat(&self, path: &str) -> Option<general::stat> {
        let target = self.path_symlink(path)?;
        let mut st: general::stat = unsafe { core::mem::zeroed() };
        st.st_dev = 1;
        st.st_ino = path_inode(Some(path));
        st.st_mode = ST_MODE_LNK | 0o777;
        st.st_nlink = 1;
        st.st_size = target.len() as _;
        st.st_blksize = 512;
        st.st_blocks = 0;
        if let Some((uid, gid)) = self.path_owner(path) {
            st.st_uid = uid;
            st.st_gid = gid;
        }
        if let Some(times) = self.path_times(path) {
            times.apply_to_stat(&mut st);
        }
        Some(st)
    }

    fn set_path_xattr(
        &self,
        path: String,
        name: String,
        value: Vec<u8>,
        flags: usize,
    ) -> Result<(), LinuxError> {
        if flags & !(XATTR_CREATE | XATTR_REPLACE) != 0 {
            return Err(LinuxError::EINVAL);
        }
        let mut all_xattrs = self.path_xattrs.lock();
        let attrs = all_xattrs.entry(path).or_default();
        let exists = attrs.contains_key(name.as_str());
        if flags & XATTR_CREATE != 0 && exists {
            return Err(LinuxError::EEXIST);
        }
        if flags & XATTR_REPLACE != 0 && !exists {
            return Err(LinuxError::ENODATA);
        }
        attrs.insert(name, value);
        Ok(())
    }

    fn get_path_xattr(&self, path: &str, name: &str) -> Result<Vec<u8>, LinuxError> {
        self.path_xattrs
            .lock()
            .get(path)
            .and_then(|attrs| attrs.get(name).cloned())
            .ok_or(LinuxError::ENODATA)
    }

    fn list_path_xattrs(&self, path: &str) -> Vec<u8> {
        let mut out = Vec::new();
        if let Some(attrs) = self.path_xattrs.lock().get(path) {
            for name in attrs.keys() {
                out.extend_from_slice(name.as_bytes());
                out.push(0);
            }
        }
        out
    }

    fn remove_path_xattr(&self, path: &str, name: &str) -> Result<(), LinuxError> {
        let mut all_xattrs = self.path_xattrs.lock();
        let attrs = all_xattrs.get_mut(path).ok_or(LinuxError::ENODATA)?;
        if attrs.remove(name).is_none() {
            return Err(LinuxError::ENODATA);
        }
        if attrs.is_empty() {
            all_xattrs.remove(path);
        }
        Ok(())
    }

    pub(super) fn set_path_sparse_size(&self, path: String, size: u64) {
        self.path_sparse_sizes.lock().insert(path, size);
    }

    pub(super) fn path_sparse_size(&self, path: &str) -> Option<u64> {
        self.path_sparse_sizes.lock().get(path).copied()
    }

    fn sparse_data_block_size() -> u64 {
        512
    }

    fn data_range_block_floor(offset: u64) -> u64 {
        let block_size = Self::sparse_data_block_size();
        offset / block_size * block_size
    }

    fn data_range_block_ceil(offset: u64) -> u64 {
        let block_size = Self::sparse_data_block_size();
        offset.saturating_add(block_size - 1) / block_size * block_size
    }

    pub(super) fn ensure_path_data_ranges(&self, path: String, physical_size: u64) {
        let mut all_ranges = self.path_data_ranges.lock();
        if all_ranges.contains_key(path.as_str()) {
            return;
        }
        let ranges = if physical_size == 0 {
            Vec::new()
        } else {
            vec![(0, Self::data_range_block_ceil(physical_size))]
        };
        all_ranges.insert(path, ranges);
    }

    pub(super) fn mark_path_data_range(&self, path: String, offset: u64, len: u64) {
        if len == 0 {
            return;
        }
        let start = Self::data_range_block_floor(offset);
        let end = Self::data_range_block_ceil(offset.saturating_add(len));
        if start >= end {
            return;
        }
        let mut all_ranges = self.path_data_ranges.lock();
        let path_key = path.clone();
        let ranges = all_ranges.entry(path).or_default();
        let missing_blocks = Self::missing_data_range_512_blocks(ranges, start, end);
        let mut merged = Vec::new();
        let mut pending_start = start;
        let mut pending_end = end;
        for (range_start, range_end) in ranges.drain(..) {
            if range_end < pending_start {
                merged.push((range_start, range_end));
            } else if range_start > pending_end {
                merged.push((pending_start, pending_end));
                pending_start = range_start;
                pending_end = range_end;
            } else {
                pending_start = pending_start.min(range_start);
                pending_end = pending_end.max(range_end);
            }
        }
        merged.push((pending_start, pending_end));
        merged.sort_by_key(|(range_start, _)| *range_start);
        *ranges = merged;
        drop(all_ranges);

        let _ = self.consume_path_free_512_blocks(path_key.as_str(), missing_blocks);
    }

    pub(super) fn path_data_ranges(&self, path: &str) -> Option<Vec<(u64, u64)>> {
        self.path_data_ranges.lock().get(path).cloned()
    }

    pub(super) fn missing_path_data_512_blocks(
        &self,
        path: &str,
        start: u64,
        end: u64,
    ) -> Option<u64> {
        self.path_data_ranges
            .lock()
            .get(path)
            .map(|ranges| Self::missing_data_range_512_blocks(ranges, start, end))
    }

    pub(super) fn path_data_ranges_cover(&self, path: &str, offset: u64, len: u64) -> bool {
        if len == 0 {
            return true;
        }
        let end = offset.saturating_add(len);
        let ranges = self.path_data_ranges.lock();
        let Some(ranges) = ranges.get(path) else {
            return false;
        };
        let mut cursor = offset;
        for &(range_start, range_end) in ranges {
            if range_end <= cursor {
                continue;
            }
            if range_start > cursor {
                return false;
            }
            cursor = cursor.max(range_end);
            if cursor >= end {
                return true;
            }
        }
        false
    }

    fn take_path_free_512_block_credit(extents: &mut Vec<(u64, Vec<u8>)>) -> u64 {
        let mut blocks = 0u64;
        extents.retain(|(offset, data)| {
            if data.is_empty() && *offset >= PATH_FREE_BLOCK_CREDIT_BASE {
                blocks = blocks.saturating_add(offset.saturating_sub(PATH_FREE_BLOCK_CREDIT_BASE));
                false
            } else {
                true
            }
        });
        blocks
    }

    fn store_path_free_512_block_credit(extents: &mut Vec<(u64, Vec<u8>)>, blocks: u64) {
        if blocks == 0 {
            return;
        }
        let encoded = PATH_FREE_BLOCK_CREDIT_BASE.saturating_add(blocks);
        extents.push((encoded, Vec::new()));
        extents.sort_by_key(|(offset, _)| *offset);
    }

    fn repeated_sparse_byte(data: &[u8]) -> Option<u8> {
        let (&first, rest) = data.split_first()?;
        if rest.iter().all(|byte| *byte == first) {
            Some(first)
        } else {
            None
        }
    }

    fn copy_sparse_bytes(data: &[u8]) -> Result<Vec<u8>, LinuxError> {
        let mut out = Vec::new();
        out.try_reserve_exact(data.len())
            .map_err(|_| LinuxError::ENOMEM)?;
        out.extend_from_slice(data);
        Ok(out)
    }

    fn insert_sparse_byte_extent(
        extents: &mut Vec<(u64, Vec<u8>)>,
        offset: u64,
        data: &[u8],
    ) -> Result<(), LinuxError> {
        if data.is_empty() {
            return Ok(());
        }

        // `clear_sparse_byte_extents()` removes any overlap before insertion, so
        // the common write path only needs an ordered insert and local neighbor
        // coalescing.  Avoid resorting and re-merging the full extent vector for
        // every small random write: ftest/iozone create many tiny sparse writes
        // and full-vector rebuilds turn those honest file operations quadratic.
        let insert_at = extents.partition_point(|(extent_offset, extent_data)| {
            if extent_data.is_empty() {
                false
            } else {
                *extent_offset < offset
            }
        });
        extents.insert(insert_at, (offset, Self::copy_sparse_bytes(data)?));
        Self::merge_sparse_byte_neighbors(extents, insert_at);
        Ok(())
    }

    fn clear_sparse_byte_extents(extents: &mut Vec<(u64, Vec<u8>)>, offset: u64, end: u64) {
        if offset >= end || extents.is_empty() {
            return;
        }

        let mut idx = extents.partition_point(|(extent_offset, data)| {
            !data.is_empty() && extent_offset.saturating_add(data.len() as u64) <= offset
        });
        while idx < extents.len() {
            if extents[idx].1.is_empty() {
                break;
            }
            let extent_offset = extents[idx].0;
            let extent_end = extent_offset.saturating_add(extents[idx].1.len() as u64);
            if extent_end <= offset {
                idx += 1;
                continue;
            }
            if extent_offset >= end {
                break;
            }

            if extent_offset < offset {
                let keep = offset.saturating_sub(extent_offset) as usize;
                if extent_end > end {
                    let skip = end.saturating_sub(extent_offset) as usize;
                    let right = extents[idx].1.split_off(skip);
                    extents[idx].1.truncate(keep);
                    extents.insert(idx + 1, (end, right));
                    break;
                } else {
                    extents[idx].1.truncate(keep);
                    idx += 1;
                }
            } else if extent_end > end {
                let skip = end.saturating_sub(extent_offset) as usize;
                extents[idx].1.drain(..skip);
                extents[idx].0 = end;
                break;
            } else {
                extents.remove(idx);
            }
        }
    }

    fn merge_sparse_byte_neighbors(extents: &mut Vec<(u64, Vec<u8>)>, mut idx: usize) {
        if extents.is_empty() || idx >= extents.len() {
            return;
        }

        while idx > 0 && Self::try_merge_sparse_byte_pair(extents, idx - 1) {
            idx -= 1;
        }
        while idx + 1 < extents.len() && Self::try_merge_sparse_byte_pair(extents, idx) {}
    }

    fn try_merge_sparse_byte_pair(extents: &mut Vec<(u64, Vec<u8>)>, left: usize) -> bool {
        let right = left + 1;
        if right >= extents.len() {
            return false;
        }
        if extents[left].1.is_empty() || extents[right].1.is_empty() {
            return false;
        }
        let left_end = extents[left].0.saturating_add(extents[left].1.len() as u64);
        if left_end != extents[right].0 {
            return false;
        }
        let right_len = extents[right].1.len();
        if extents[left].1.len().saturating_add(right_len) > SPARSE_BYTE_EXTENT_MERGE_LIMIT {
            return false;
        }

        let (_, right_data) = extents.remove(right);
        if extents[left].1.try_reserve_exact(right_data.len()).is_err() {
            extents.insert(right, (left_end, right_data));
            return false;
        }
        extents[left].1.extend_from_slice(&right_data);
        true
    }

    fn clear_sparse_repeat_extents(extents: &mut Vec<(u64, u64, u8)>, offset: u64, end: u64) {
        if offset >= end || extents.is_empty() {
            return;
        }
        let mut idx = extents.partition_point(|(_, extent_end, _)| *extent_end <= offset);
        while idx < extents.len() {
            let (extent_start, extent_end, byte) = extents[idx];
            if extent_start >= end {
                break;
            }
            if extent_start < offset {
                if extent_end > end {
                    extents[idx].1 = offset;
                    extents.insert(idx + 1, (end, extent_end, byte));
                    break;
                }
                extents[idx].1 = offset;
                idx += 1;
                continue;
            }
            if extent_end > end {
                extents[idx] = (end, extent_end, byte);
                break;
            } else {
                extents.remove(idx);
            }
        }
    }

    fn insert_sparse_repeat_extent(
        extents: &mut Vec<(u64, u64, u8)>,
        offset: u64,
        end: u64,
        byte: u8,
    ) {
        if offset >= end {
            return;
        }
        Self::clear_sparse_repeat_extents(extents, offset, end);
        let insert_at = extents.partition_point(|(start, _, _)| *start < offset);
        extents.insert(insert_at, (offset, end, byte));
        Self::merge_sparse_repeat_neighbors(extents, insert_at);
    }

    fn merge_sparse_repeat_neighbors(extents: &mut Vec<(u64, u64, u8)>, mut idx: usize) {
        if extents.is_empty() || idx >= extents.len() {
            return;
        }
        while idx > 0 && Self::try_merge_sparse_repeat_pair(extents, idx - 1) {
            idx -= 1;
        }
        while idx + 1 < extents.len() && Self::try_merge_sparse_repeat_pair(extents, idx) {}
    }

    fn try_merge_sparse_repeat_pair(extents: &mut Vec<(u64, u64, u8)>, left: usize) -> bool {
        let right = left + 1;
        if right >= extents.len() {
            return false;
        }
        let (_, left_end, left_byte) = extents[left];
        let (right_start, right_end, right_byte) = extents[right];
        if left_byte != right_byte || left_end < right_start {
            return false;
        }
        extents[left].1 = left_end.max(right_end);
        extents.remove(right);
        true
    }

    fn credit_path_free_512_blocks(&self, path: String, blocks: u64) {
        if blocks == 0 {
            return;
        }
        let mut all_data = self.path_sparse_data.lock();
        let extents = all_data.entry(path).or_default();
        let current = Self::take_path_free_512_block_credit(extents);
        Self::store_path_free_512_block_credit(extents, current.saturating_add(blocks));
    }

    pub(super) fn path_free_512_blocks(&self, path: &str) -> u64 {
        let all_data = self.path_sparse_data.lock();
        let Some(extents) = all_data.get(path) else {
            return 0;
        };
        extents
            .iter()
            .filter_map(|(offset, data)| {
                if data.is_empty() && *offset >= PATH_FREE_BLOCK_CREDIT_BASE {
                    Some(offset.saturating_sub(PATH_FREE_BLOCK_CREDIT_BASE))
                } else {
                    None
                }
            })
            .fold(0u64, u64::saturating_add)
    }

    pub(super) fn consume_path_free_512_blocks(&self, path: &str, blocks: u64) -> bool {
        if blocks == 0 {
            return true;
        }
        let mut all_data = self.path_sparse_data.lock();
        let Some(extents) = all_data.get_mut(path) else {
            return false;
        };
        let current = Self::take_path_free_512_block_credit(extents);
        if current < blocks {
            Self::store_path_free_512_block_credit(extents, current);
            return false;
        }
        Self::store_path_free_512_block_credit(extents, current - blocks);
        if extents.is_empty() {
            all_data.remove(path);
        }
        true
    }

    fn missing_data_range_512_blocks(ranges: &[(u64, u64)], start: u64, end: u64) -> u64 {
        // All path_data_ranges mutators keep ranges ordered and non-overlapping.
        // Query hot paths can therefore scan in place instead of cloning and
        // sorting the whole vector on every write-capacity check.
        let mut cursor = start;
        let mut missing = 0u64;
        for &(range_start, range_end) in ranges {
            if range_end <= cursor {
                continue;
            }
            let covered_start = range_start.max(start);
            let covered_end = range_end.min(end);
            if covered_end <= covered_start {
                continue;
            }
            if covered_start > cursor {
                missing = missing.saturating_add(
                    covered_start.saturating_sub(cursor) / Self::sparse_data_block_size(),
                );
            }
            cursor = cursor.max(covered_end);
            if cursor >= end {
                break;
            }
        }
        if cursor < end {
            missing =
                missing.saturating_add(end.saturating_sub(cursor) / Self::sparse_data_block_size());
        }
        missing
    }

    pub(super) fn clear_path_data_range(&self, path: String, offset: u64, len: u64) {
        if len == 0 {
            return;
        }
        let clear_end = offset.saturating_add(len);
        let start = Self::data_range_block_floor(offset);
        let end = Self::data_range_block_ceil(clear_end);
        if start >= end {
            return;
        }

        let mut all_data = self.path_sparse_data.lock();
        let remove_empty = if let Some(extents) = all_data.get_mut(path.as_str()) {
            Self::clear_sparse_byte_extents(extents, offset, clear_end);
            if extents.is_empty() { true } else { false }
        } else {
            false
        };
        if remove_empty {
            all_data.remove(path.as_str());
        }
        drop(all_data);

        let mut all_repeats = self.path_sparse_repeats.lock();
        let remove_empty = if let Some(extents) = all_repeats.get_mut(path.as_str()) {
            Self::clear_sparse_repeat_extents(extents, offset, clear_end);
            extents.is_empty()
        } else {
            false
        };
        if remove_empty {
            all_repeats.remove(path.as_str());
        }
        drop(all_repeats);

        let mut all_ranges = self.path_data_ranges.lock();
        let Some(ranges) = all_ranges.get_mut(path.as_str()) else {
            return;
        };
        let mut retained = Vec::new();
        let mut freed_blocks = 0u64;
        for (range_start, range_end) in ranges.drain(..) {
            if range_end <= start || range_start >= end {
                retained.push((range_start, range_end));
                continue;
            }
            let freed_start = range_start.max(start);
            let freed_end = range_end.min(end);
            if freed_start < freed_end {
                freed_blocks = freed_blocks.saturating_add(
                    freed_end.saturating_sub(freed_start) / Self::sparse_data_block_size(),
                );
            }
            if range_start < start {
                retained.push((range_start, start));
            }
            if range_end > end {
                retained.push((end, range_end));
            }
        }
        *ranges = retained;
        drop(all_ranges);

        self.credit_path_free_512_blocks(path, freed_blocks);
    }

    pub(super) fn path_allocated_512_blocks(&self, path: &str, logical_size: u64) -> Option<u64> {
        let all_ranges = self.path_data_ranges.lock();
        let ranges = all_ranges.get(path)?;
        let mut blocks = 0u64;
        for &(start, end) in ranges {
            if start >= logical_size || start >= end {
                continue;
            }
            let start = Self::data_range_block_floor(start);
            let end = Self::data_range_block_ceil(end.min(logical_size));
            blocks =
                blocks.saturating_add(end.saturating_sub(start) / Self::sparse_data_block_size());
        }
        Some(blocks)
    }

    pub(super) fn clear_path_sparse_file(&self, path: &str) {
        self.path_sparse_sizes.lock().remove(path);
        self.path_sparse_data.lock().remove(path);
        self.path_sparse_repeats.lock().remove(path);
        self.path_data_ranges.lock().remove(path);
    }

    pub(super) fn move_path_sparse_file(&self, old_path: &str, new_path: String) {
        {
            let mut sizes = self.path_sparse_sizes.lock();
            if let Some(size) = sizes.remove(old_path) {
                sizes.insert(new_path.clone(), size);
            } else {
                sizes.remove(new_path.as_str());
            }
        }
        let mut all_data = self.path_sparse_data.lock();
        if let Some(data) = all_data.remove(old_path) {
            all_data.insert(new_path.clone(), data);
        } else {
            all_data.remove(new_path.as_str());
        }
        drop(all_data);

        let mut all_repeats = self.path_sparse_repeats.lock();
        if let Some(repeats) = all_repeats.remove(old_path) {
            all_repeats.insert(new_path.clone(), repeats);
        } else {
            all_repeats.remove(new_path.as_str());
        }
        drop(all_repeats);

        let mut all_ranges = self.path_data_ranges.lock();
        if let Some(ranges) = all_ranges.remove(old_path) {
            all_ranges.insert(new_path, ranges);
        } else {
            all_ranges.remove(new_path.as_str());
        }
    }

    pub(super) fn truncate_path_sparse_file(&self, path: String, size: u64) {
        self.path_sparse_sizes.lock().insert(path.clone(), size);

        let mut all_data = self.path_sparse_data.lock();
        let remove_empty = if let Some(extents) = all_data.get_mut(path.as_str()) {
            let mut retained = Vec::new();
            for (offset, mut data) in extents.drain(..) {
                if offset >= size {
                    continue;
                }
                let keep = cmp::min(data.len(), size.saturating_sub(offset) as usize);
                if keep > 0 {
                    data.truncate(keep);
                    retained.push((offset, data));
                }
            }
            if retained.is_empty() {
                true
            } else {
                *extents = retained;
                false
            }
        } else {
            false
        };
        if remove_empty {
            all_data.remove(path.as_str());
        }
        drop(all_data);

        let mut all_repeats = self.path_sparse_repeats.lock();
        let remove_empty = if let Some(extents) = all_repeats.get_mut(path.as_str()) {
            extents.retain_mut(|(start, end, _)| {
                if *start >= size {
                    false
                } else {
                    *end = (*end).min(size);
                    *start < *end
                }
            });
            extents.is_empty()
        } else {
            false
        };
        if remove_empty {
            all_repeats.remove(path.as_str());
        }

        let mut all_ranges = self.path_data_ranges.lock();
        if size == 0 {
            all_ranges.remove(path.as_str());
        } else {
            let retained = all_ranges
                .remove(path.as_str())
                .unwrap_or_default()
                .into_iter()
                .filter_map(|(start, end)| {
                    if start >= size {
                        None
                    } else {
                        Some((start, end.min(size)))
                    }
                })
                .collect();
            all_ranges.insert(path, retained);
        }
    }

    pub(super) fn write_path_sparse_data(
        &self,
        path: String,
        offset: u64,
        data: &[u8],
    ) -> Result<(), LinuxError> {
        let end = offset.saturating_add(data.len() as u64);
        let logical_size = self.path_sparse_size(path.as_str()).unwrap_or(0).max(end);
        self.set_path_sparse_size(path.clone(), logical_size);
        if data.is_empty() {
            return Ok(());
        }
        self.mark_path_data_range(path.clone(), offset, data.len() as u64);

        let repeated = Self::repeated_sparse_byte(data);
        let mut all_repeats = self.path_sparse_repeats.lock();
        let repeats_empty = if let Some(extents) = all_repeats.get_mut(path.as_str()) {
            Self::clear_sparse_repeat_extents(extents, offset, end);
            extents.is_empty()
        } else {
            false
        };
        if repeats_empty {
            all_repeats.remove(path.as_str());
        }
        if let Some(byte) = repeated {
            let extents = all_repeats.entry(path.clone()).or_default();
            Self::insert_sparse_repeat_extent(extents, offset, end, byte);
        }
        drop(all_repeats);

        let mut all_data = self.path_sparse_data.lock();
        let extents = all_data.entry(path).or_default();
        Self::clear_sparse_byte_extents(extents, offset, end);
        if repeated.is_none() {
            Self::insert_sparse_byte_extent(extents, offset, data)?;
        }
        if extents.is_empty() {
            all_data.retain(|_, data_extents| !data_extents.is_empty());
        }
        Ok(())
    }

    pub(super) fn copy_path_sparse_data(&self, path: &str, offset: u64, dst: &mut [u8]) {
        if dst.is_empty() {
            return;
        }
        let end = offset.saturating_add(dst.len() as u64);
        let all_repeats = self.path_sparse_repeats.lock();
        if let Some(extents) = all_repeats.get(path) {
            let first = extents.partition_point(|(_, extent_end, _)| *extent_end <= offset);
            for (extent_offset, extent_end, byte) in &extents[first..] {
                if *extent_offset >= end {
                    break;
                }
                if *extent_end <= offset || *extent_offset >= end {
                    continue;
                }
                let copy_start = (*extent_offset).max(offset);
                let copy_end = (*extent_end).min(end);
                let dst_start = copy_start.saturating_sub(offset) as usize;
                let dst_end = copy_end.saturating_sub(offset) as usize;
                dst[dst_start..dst_end].fill(*byte);
            }
        }
        drop(all_repeats);

        let all_data = self.path_sparse_data.lock();
        let Some(extents) = all_data.get(path) else {
            return;
        };
        let first = extents.partition_point(|(extent_offset, data)| {
            extent_offset.saturating_add(data.len() as u64) <= offset
        });
        for (extent_offset, data) in &extents[first..] {
            if data.is_empty() {
                continue;
            }
            let extent_end = extent_offset.saturating_add(data.len() as u64);
            if *extent_offset >= end {
                break;
            }
            if extent_end <= offset || *extent_offset >= end {
                continue;
            }
            let copy_start = (*extent_offset).max(offset);
            let copy_end = extent_end.min(end);
            let dst_start = copy_start.saturating_sub(offset) as usize;
            let src_start = copy_start.saturating_sub(*extent_offset) as usize;
            let len = copy_end.saturating_sub(copy_start) as usize;
            dst[dst_start..dst_start + len].copy_from_slice(&data[src_start..src_start + len]);
        }
    }

    pub(super) fn clear_path_chown_special_bits(&self, path: &str, current_mode: u32) {
        let mode = self
            .path_mode(path)
            .unwrap_or(current_mode & FILE_MODE_PERMISSION_MASK);
        let mut updated_mode = mode & !FILE_MODE_SET_UID;
        let is_directory = current_mode & ST_MODE_TYPE_MASK == ST_MODE_DIR;
        if !is_directory && mode & FILE_MODE_GROUP_EXECUTE != 0 {
            updated_mode &= !FILE_MODE_SET_GID;
        }
        self.set_path_mode(path.to_string(), updated_mode);
    }
}

fn normalize_symlink_target(link_path: &str, target: &str) -> Option<String> {
    if target.starts_with('/') {
        normalize_path("/", target)
    } else {
        let parent = link_path
            .rsplit_once('/')
            .map(|(parent, _)| parent)
            .unwrap_or("/");
        let parent = if parent.is_empty() { "/" } else { parent };
        normalize_path(parent, target)
    }
}

pub(super) fn apply_recorded_path_metadata(
    process: &UserProcess,
    path: &str,
    mut st: general::stat,
) -> general::stat {
    if let Some(ty) = process.path_special_mode(path) {
        st.st_mode = (st.st_mode & !ST_MODE_TYPE_MASK) | ty;
        if matches!(ty, ST_MODE_CHR | ST_MODE_BLK) {
            st.st_rdev = process.path_rdev(path).unwrap_or(0) as _;
        }
    }
    if let Some(mode) = process.path_mode(path) {
        st.st_mode = (st.st_mode & !FILE_MODE_PERMISSION_MASK) | mode;
    }
    if let Some((uid, gid)) = process.path_owner(path) {
        st.st_uid = uid;
        st.st_gid = gid;
    }
    if let Some(ino) = process.path_inode_override(path) {
        st.st_ino = ino;
    }
    if let Some(nlink) = process.path_hardlink_count(path) {
        st.st_nlink = nlink as _;
    }
    if let Some(size) = process.path_sparse_size(path) {
        st.st_size = size.min(i64::MAX as u64) as _;
    }
    if let Some(blocks) = process.path_allocated_512_blocks(path, st.st_size.max(0) as u64) {
        st.st_blocks = blocks.min(i64::MAX as u64) as _;
    }
    if let Some(times) = process.path_times(path) {
        times.apply_to_stat(&mut st);
    }
    st
}

pub(super) fn canonical_permission_path(path: String) -> String {
    dev_shm_host_path(path.as_str()).unwrap_or(path)
}

pub(super) fn fd_entry_path(entry: &FdEntry) -> Option<&str> {
    match entry {
        FdEntry::DevCpuDmaLatency(_) => Some("/dev/cpu_dma_latency"),
        FdEntry::BlockDevice(dev) => Some(dev.path.as_str()),
        FdEntry::File(file) => Some(file.path.as_str()),
        FdEntry::Directory(dir) => Some(dir.path.as_str()),
        FdEntry::ProcFdDir(dir) => Some(dir.path.as_str()),
        FdEntry::SyntheticDir(dir) => Some(dir.path.as_str()),
        FdEntry::Path(path) => Some(path.path.as_str()),
        FdEntry::MemoryFile(file) => Some(file.path.as_str()),
        FdEntry::Memfd(_) => None,
        FdEntry::ProcPagemap(file) => Some(file.path.as_str()),
        FdEntry::ProcTimerSlack(file) => Some(file.path.as_str()),
        FdEntry::ProcMqQueuesMax(_) => Some("/proc/sys/fs/mqueue/queues_max"),
        FdEntry::ProcSysFile(file) => Some(file.path()),
        _ => None,
    }
}

pub(super) fn sys_faccessat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    mode: usize,
    flags: usize,
) -> isize {
    if mode & !ACCESS_MODE_MASK != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let supported_flags =
        (general::AT_EACCESS | general::AT_SYMLINK_NOFOLLOW | general::AT_EMPTY_PATH) as usize;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let mut fds = process.fds.lock();
    let (resolved_path, stat, parents_already_reached) = if path.is_empty() {
        if flags & general::AT_EMPTY_PATH as usize == 0 {
            return neg_errno(LinuxError::ENOENT);
        }
        if dirfd as i32 == general::AT_FDCWD {
            match fds.path_stat(process, general::AT_FDCWD, ".") {
                Ok((path, stat)) => (path, stat, false),
                Err(err) => return neg_errno(err),
            }
        } else {
            match fds.stat_with_recorded_path(process, dirfd as i32) {
                Ok((path, stat)) => (path.unwrap_or_default(), stat, true),
                Err(err) => return neg_errno(err),
            }
        }
    } else if flags & general::AT_SYMLINK_NOFOLLOW as usize != 0 {
        let resolved_path = match fds.resolve_path(process, dirfd as i32, path.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        };
        let resolved_path = match process.resolve_parent_symlinks(resolved_path.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        };
        let stat = match process.path_symlink_stat(resolved_path.as_str()) {
            Some(stat) => stat,
            None => match lstat_absolute_path(process, resolved_path.as_str()) {
                Ok(stat) => stat,
                Err(err) => return neg_errno(err),
            },
        };
        (resolved_path, stat, false)
    } else {
        match fds.path_stat(process, dirfd as i32, path.as_str()) {
            Ok((path, stat)) => (path, stat, false),
            Err(err) => return neg_errno(err),
        }
    };
    let use_effective_ids = flags & general::AT_EACCESS as usize != 0;
    let uid = if use_effective_ids {
        process.uid()
    } else {
        process.real_uid()
    };
    let gid = if use_effective_ids {
        process.gid()
    } else {
        process.real_gid()
    };
    let parents_searchable = if parents_already_reached || resolved_path.is_empty() {
        true
    } else {
        match fds.parent_dirs_searchable(process, resolved_path.as_str(), uid, gid) {
            Ok(searchable) => searchable,
            Err(err) => return neg_errno(err),
        }
    };
    if !parents_searchable {
        return neg_errno_code(LINUX_EACCES);
    }
    if mode & ACCESS_W_OK != 0 && process.path_on_readonly_mount(resolved_path.as_str()) {
        return neg_errno(LinuxError::EROFS);
    }
    if access_allowed(&stat, mode, uid, gid) {
        0
    } else {
        neg_errno_code(LINUX_EACCES)
    }
}

pub(super) fn sys_truncate(process: &UserProcess, pathname: usize, length: usize) -> isize {
    let length = length as isize;
    if length < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let length = length as u64;
    if length > process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current() {
        return neg_errno(LinuxError::EFBIG);
    }
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if path.is_empty() {
        return neg_errno(LinuxError::ENOENT);
    }
    if path.len() >= LINUX_PATH_MAX {
        return neg_errno(LinuxError::ENAMETOOLONG);
    }

    let resolved_path = {
        let table = process.fds.lock();
        match resolve_dirfd_path(process, &table, general::AT_FDCWD, path.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        }
    };
    let target_path = match process.resolve_path_symlink(resolved_path.as_str()) {
        Ok(Some(target)) => target,
        Ok(None) => resolved_path,
        Err(err) => return neg_errno(err),
    };
    let target_path = dev_shm_host_path(target_path.as_str()).unwrap_or(target_path);
    let st = match {
        let mut fds = process.fds.lock();
        fds.stat_path(process, general::AT_FDCWD, target_path.as_str())
    } {
        Ok(st) => st,
        Err(err) => return neg_errno(err),
    };
    if st.st_mode & ST_MODE_TYPE_MASK == ST_MODE_DIR {
        return neg_errno(LinuxError::EISDIR);
    }
    if process.path_on_readonly_mount(target_path.as_str()) {
        return neg_errno(LinuxError::EROFS);
    }
    if !access_allowed(&st, ACCESS_W_OK, process.fs_uid(), process.fs_gid()) {
        return neg_errno(LinuxError::EACCES);
    }

    let mut opts = OpenOptions::new();
    opts.write(true);
    let file = match axfs::fops::File::open(target_path.as_str(), &opts) {
        Ok(file) => file,
        Err(err) => return neg_errno(LinuxError::from(err)),
    };
    let physical_size = match file.get_attr() {
        Ok(attr) => attr.size(),
        Err(err) => return neg_errno(LinuxError::from(err)),
    };
    process.ensure_path_data_ranges(target_path.clone(), physical_size);
    if length <= physical_size || length < MAX_IN_MEMORY_FILE_SIZE {
        if let Err(err) = file.truncate(length) {
            return neg_errno(LinuxError::from(err));
        }
    }
    process.truncate_path_sparse_file(target_path, length);
    0
}

pub(super) fn sys_symlinkat(
    process: &UserProcess,
    target: usize,
    newdirfd: usize,
    linkpath: usize,
) -> isize {
    let target = match read_cstr(process, target) {
        Ok(target) => target,
        Err(err) => return neg_errno(err),
    };
    let linkpath = match read_cstr(process, linkpath) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if target.is_empty() || linkpath.is_empty() {
        return neg_errno(LinuxError::ENOENT);
    }
    if target.len() >= LINUX_PATH_MAX || linkpath.len() >= LINUX_PATH_MAX {
        return neg_errno(LinuxError::ENAMETOOLONG);
    }
    let resolved_path = {
        let table = process.fds.lock();
        match resolve_dirfd_path(process, &table, newdirfd as i32, linkpath.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        }
    };
    let resolved_path = match process.resolve_parent_symlinks(resolved_path.as_str()) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if process.path_symlink(resolved_path.as_str()).is_some()
        || axfs::api::metadata(resolved_path.as_str()).is_ok()
    {
        return neg_errno(LinuxError::EEXIST);
    }
    if let Err(err) = check_parent_write_search_permission(process, resolved_path.as_str()) {
        return neg_errno(err);
    }
    process.set_path_symlink(resolved_path, target);
    0
}

pub(super) fn sys_umask(process: &UserProcess, mask: usize) -> isize {
    process.replace_umask(mask as u32) as isize
}

pub(super) fn sys_fchmod(process: &UserProcess, fd: usize, mode: usize) -> isize {
    let (path, st) = {
        let mut fds = process.fds.lock();
        if matches!(fds.entry(fd as i32), Ok(FdEntry::Path(_))) {
            return neg_errno(LinuxError::EBADF);
        }
        match fds.stat_with_recorded_path(process, fd as i32) {
            Ok((path, st)) => (path, st),
            Err(err) => return neg_errno(err),
        }
    };
    if let Some(path) = path.as_deref() {
        if process.path_on_readonly_mount(path) {
            return neg_errno(LinuxError::EROFS);
        }
    }
    if !chmod_permission_allowed(process, &st) {
        return neg_errno(LinuxError::EPERM);
    }
    if let Some(path) = path {
        process.set_path_mode(path, chmod_effective_mode(process, &st, mode as u32));
    }
    0
}

pub(super) fn sys_fchmodat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    mode: usize,
    flags: usize,
) -> isize {
    let flags = flags as u32;
    let supported_flags = general::AT_SYMLINK_NOFOLLOW | general::AT_EMPTY_PATH;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if path.len() >= LINUX_PATH_MAX {
        return neg_errno(LinuxError::ENAMETOOLONG);
    }
    let mode = mode as u32;
    if let Some(fd) = proc_self_fd_number(path.as_str()) {
        let mut fds = process.fds.lock();
        if matches!(fds.entry(fd), Ok(FdEntry::Path(_))) {
            return neg_errno(LinuxError::EBADF);
        }
        let (path, st) = match fds.stat_with_recorded_path(process, fd) {
            Ok((path, st)) => (path, st),
            Err(err) => return neg_errno(err),
        };
        if let Some(path) = path.as_deref() {
            if process.path_on_readonly_mount(path) {
                return neg_errno(LinuxError::EROFS);
            }
        }
        if !chmod_permission_allowed(process, &st) {
            return neg_errno(LinuxError::EPERM);
        }
        if let Some(path) = path {
            process.set_path_mode(path, chmod_effective_mode(process, &st, mode));
        }
        return 0;
    }
    if path.is_empty() {
        if matches!(process.fds.lock().entry(dirfd as i32), Ok(FdEntry::Path(_))) {
            return neg_errno(LinuxError::EBADF);
        }
        if flags & general::AT_EMPTY_PATH == 0 {
            return neg_errno(LinuxError::ENOENT);
        }
        if dirfd as i32 == general::AT_FDCWD {
            let cwd = process.cwd();
            return match axfs::api::metadata(cwd.as_str()) {
                Ok(_) => {
                    let mut fds = process.fds.lock();
                    let st = match fds.stat_path(process, general::AT_FDCWD, ".") {
                        Ok(st) => st,
                        Err(err) => return neg_errno(err),
                    };
                    if process.path_on_readonly_mount(cwd.as_str()) {
                        return neg_errno(LinuxError::EROFS);
                    }
                    if !chmod_permission_allowed(process, &st) {
                        return neg_errno(LinuxError::EPERM);
                    }
                    process.set_path_mode(cwd, chmod_effective_mode(process, &st, mode));
                    0
                }
                Err(err) => neg_errno(LinuxError::from(err)),
            };
        }
        let (path, st) = {
            let mut fds = process.fds.lock();
            if matches!(fds.entry(dirfd as i32), Ok(FdEntry::Path(_))) {
                return neg_errno(LinuxError::EBADF);
            }
            match fds.stat_with_recorded_path(process, dirfd as i32) {
                Ok((path, st)) => (path, st),
                Err(err) => return neg_errno(err),
            }
        };
        if let Some(path) = path.as_deref() {
            if process.path_on_readonly_mount(path) {
                return neg_errno(LinuxError::EROFS);
            }
        }
        if !chmod_permission_allowed(process, &st) {
            return neg_errno(LinuxError::EPERM);
        }
        if let Some(path) = path {
            process.set_path_mode(path, chmod_effective_mode(process, &st, mode));
        }
        return 0;
    }

    let mut fds = process.fds.lock();
    let resolved_path = match fds.resolve_path(process, dirfd as i32, path.as_str()) {
        Ok(path) => path,
        Err(err) => {
            return neg_errno(err);
        }
    };
    match fds.parent_dirs_searchable(
        process,
        resolved_path.as_str(),
        process.fs_uid(),
        process.fs_gid(),
    ) {
        Ok(true) => {}
        Ok(false) => return neg_errno(LinuxError::EACCES),
        Err(err) => return neg_errno(err),
    }
    if axfs::api::metadata(resolved_path.as_str()).is_ok() {
        let st = match fds.stat_path(process, dirfd as i32, path.as_str()) {
            Ok(st) => st,
            Err(err) => return neg_errno(err),
        };
        if process.path_on_readonly_mount(resolved_path.as_str()) {
            return neg_errno(LinuxError::EROFS);
        }
        if !chmod_permission_allowed(process, &st) {
            return neg_errno(LinuxError::EPERM);
        }
        process.set_path_mode(resolved_path, chmod_effective_mode(process, &st, mode));
        return 0;
    }
    match fds.stat_path(process, dirfd as i32, path.as_str()) {
        Ok(st) => {
            if process.path_on_readonly_mount(resolved_path.as_str()) {
                return neg_errno(LinuxError::EROFS);
            }
            if !chmod_permission_allowed(process, &st) {
                return neg_errno(LinuxError::EPERM);
            }
            process.set_path_mode(resolved_path, chmod_effective_mode(process, &st, mode));
            0
        }
        Err(err) => neg_errno(err),
    }
}

fn chmod_permission_allowed(process: &UserProcess, st: &general::stat) -> bool {
    process.uid() == 0 || st.st_uid as u32 == process.uid()
}

fn chmod_effective_mode(process: &UserProcess, st: &general::stat, mode: u32) -> u32 {
    let mut mode = mode;
    if process.uid() != 0
        && st.st_mode & ST_MODE_DIR != 0
        && mode & FILE_MODE_SET_GID != 0
        && !process.has_group(st.st_gid)
    {
        mode &= !FILE_MODE_SET_GID;
    }
    mode
}

pub(super) fn sys_fchown(process: &UserProcess, fd: usize, owner: usize, group: usize) -> isize {
    let (owner, group) = match chown_ids(owner, group) {
        Ok(ids) => ids,
        Err(err) => return neg_errno(err),
    };
    let (path, st) = {
        let mut fds = process.fds.lock();
        if matches!(fds.entry(fd as i32), Ok(FdEntry::Path(_))) {
            return neg_errno(LinuxError::EBADF);
        }
        match fds.stat_with_recorded_path(process, fd as i32) {
            Ok((path, st)) => (path, st),
            Err(err) => return neg_errno(err),
        }
    };
    apply_chown_metadata(process, path, &st, owner, group)
}

fn resolve_xattr_path(
    process: &UserProcess,
    dirfd: i32,
    path: &str,
    follow_symlink: bool,
) -> Result<String, LinuxError> {
    if path.is_empty() {
        return Err(LinuxError::ENOENT);
    }
    if path.len() >= LINUX_PATH_MAX {
        return Err(LinuxError::ENAMETOOLONG);
    }
    let mut fds = process.fds.lock();
    let resolved_path = fds.resolve_path(process, dirfd, path)?;
    if !follow_symlink && process.path_symlink_stat(resolved_path.as_str()).is_some() {
        return Ok(resolved_path);
    }
    let (record_path, _) = fds.path_stat(process, dirfd, path)?;
    Ok(record_path)
}

fn resolve_fd_xattr_path(process: &UserProcess, fd: i32) -> Result<String, LinuxError> {
    let mut fds = process.fds.lock();
    if matches!(fds.entry(fd), Ok(FdEntry::Path(_))) {
        return Err(LinuxError::EBADF);
    }
    let (path, _) = fds.stat_with_recorded_path(process, fd)?;
    path.ok_or(LinuxError::ENODATA)
}

fn read_xattr_name(process: &UserProcess, name: usize) -> Result<String, LinuxError> {
    if name == 0 {
        return Err(LinuxError::EFAULT);
    }
    let name = read_cstr(process, name)?;
    if name.is_empty() || name.len() > XATTR_NAME_MAX {
        return Err(LinuxError::ERANGE);
    }
    Ok(name)
}

fn check_inode_flags_allow_xattr_mutation(
    process: &UserProcess,
    path: &str,
) -> Result<(), LinuxError> {
    if matches!(
        process.path_special_mode(path),
        Some(ST_MODE_FIFO | ST_MODE_CHR | ST_MODE_BLK | ST_MODE_SOCKET)
    ) {
        return Err(LinuxError::EPERM);
    }
    let flags = process.path_inode_flags(path);
    if flags & (general::FS_IMMUTABLE_FL | general::FS_APPEND_FL) != 0 {
        Err(LinuxError::EPERM)
    } else {
        Ok(())
    }
}

fn sys_setxattr_for_path(
    process: &UserProcess,
    path: String,
    name: usize,
    value: usize,
    size: usize,
    flags: usize,
) -> isize {
    let name = match read_xattr_name(process, name) {
        Ok(name) => name,
        Err(err) => return neg_errno(err),
    };
    if size > XATTR_SIZE_MAX {
        return neg_errno(LinuxError::E2BIG);
    }
    let value = match read_user_bytes(process, value, size) {
        Ok(value) => value,
        Err(err) => return neg_errno(err),
    };
    if let Err(err) = check_inode_flags_allow_xattr_mutation(process, path.as_str()) {
        return neg_errno(err);
    }
    process
        .set_path_xattr(path, name, value, flags)
        .map_or_else(neg_errno, |_| 0)
}

fn sys_getxattr_for_path(
    process: &UserProcess,
    path: String,
    name: usize,
    value: usize,
    size: usize,
) -> isize {
    let name = match read_xattr_name(process, name) {
        Ok(name) => name,
        Err(err) => return neg_errno(err),
    };
    let bytes = match process.get_path_xattr(path.as_str(), name.as_str()) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    if value == 0 || size == 0 {
        return bytes.len() as isize;
    }
    if size < bytes.len() {
        return neg_errno(LinuxError::ERANGE);
    }
    write_user_bytes(process, value, bytes.as_slice())
        .map_or_else(neg_errno, |_| bytes.len() as isize)
}

fn sys_listxattr_for_path(process: &UserProcess, path: String, list: usize, size: usize) -> isize {
    let bytes = process.list_path_xattrs(path.as_str());
    if list == 0 || size == 0 {
        return bytes.len() as isize;
    }
    if size < bytes.len() {
        return neg_errno(LinuxError::ERANGE);
    }
    write_user_bytes(process, list, bytes.as_slice())
        .map_or_else(neg_errno, |_| bytes.len() as isize)
}

fn sys_removexattr_for_path(process: &UserProcess, path: String, name: usize) -> isize {
    let name = match read_xattr_name(process, name) {
        Ok(name) => name,
        Err(err) => return neg_errno(err),
    };
    if let Err(err) = check_inode_flags_allow_xattr_mutation(process, path.as_str()) {
        return neg_errno(err);
    }
    process
        .remove_path_xattr(path.as_str(), name.as_str())
        .map_or_else(neg_errno, |_| 0)
}

pub(super) fn sys_setxattr(
    process: &UserProcess,
    pathname: usize,
    name: usize,
    value: usize,
    size: usize,
    flags: usize,
) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), true) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_setxattr_for_path(process, path, name, value, size, flags)
}

pub(super) fn sys_lsetxattr(
    process: &UserProcess,
    pathname: usize,
    name: usize,
    value: usize,
    size: usize,
    flags: usize,
) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), false) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_setxattr_for_path(process, path, name, value, size, flags)
}

pub(super) fn sys_fsetxattr(
    process: &UserProcess,
    fd: usize,
    name: usize,
    value: usize,
    size: usize,
    flags: usize,
) -> isize {
    let path = match resolve_fd_xattr_path(process, fd as i32) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_setxattr_for_path(process, path, name, value, size, flags)
}

pub(super) fn sys_getxattr(
    process: &UserProcess,
    pathname: usize,
    name: usize,
    value: usize,
    size: usize,
) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), true) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_getxattr_for_path(process, path, name, value, size)
}

pub(super) fn sys_lgetxattr(
    process: &UserProcess,
    pathname: usize,
    name: usize,
    value: usize,
    size: usize,
) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), false) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_getxattr_for_path(process, path, name, value, size)
}

pub(super) fn sys_fgetxattr(
    process: &UserProcess,
    fd: usize,
    name: usize,
    value: usize,
    size: usize,
) -> isize {
    let path = match resolve_fd_xattr_path(process, fd as i32) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_getxattr_for_path(process, path, name, value, size)
}

pub(super) fn sys_listxattr(
    process: &UserProcess,
    pathname: usize,
    list: usize,
    size: usize,
) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), true) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_listxattr_for_path(process, path, list, size)
}

pub(super) fn sys_llistxattr(
    process: &UserProcess,
    pathname: usize,
    list: usize,
    size: usize,
) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), false) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_listxattr_for_path(process, path, list, size)
}

pub(super) fn sys_flistxattr(process: &UserProcess, fd: usize, list: usize, size: usize) -> isize {
    let path = match resolve_fd_xattr_path(process, fd as i32) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_listxattr_for_path(process, path, list, size)
}

pub(super) fn sys_removexattr(process: &UserProcess, pathname: usize, name: usize) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), true) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_removexattr_for_path(process, path, name)
}

pub(super) fn sys_lremovexattr(process: &UserProcess, pathname: usize, name: usize) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), false) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_removexattr_for_path(process, path, name)
}

pub(super) fn sys_fremovexattr(process: &UserProcess, fd: usize, name: usize) -> isize {
    let path = match resolve_fd_xattr_path(process, fd as i32) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_removexattr_for_path(process, path, name)
}

pub(super) fn sys_fchownat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    owner: usize,
    group: usize,
    flags: usize,
) -> isize {
    let flags = flags as u32;
    let supported_flags = general::AT_SYMLINK_NOFOLLOW
        | general::AT_NO_AUTOMOUNT
        | general::AT_EMPTY_PATH
        | general::AT_STATX_SYNC_TYPE;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let (owner, group) = match chown_ids(owner, group) {
        Ok(ids) => ids,
        Err(err) => return neg_errno(err),
    };
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if let Some(fd) = proc_self_fd_number(path.as_str()) {
        let mut fds = process.fds.lock();
        if matches!(fds.entry(fd), Ok(FdEntry::Path(_))) {
            return neg_errno(LinuxError::EBADF);
        }
        let (path, st) = match fds.stat_with_recorded_path(process, fd) {
            Ok((path, st)) => (path, st),
            Err(err) => return neg_errno(err),
        };
        return apply_chown_metadata(process, path, &st, owner, group);
    }
    let (record_path, st) = if path.is_empty() {
        if matches!(process.fds.lock().entry(dirfd as i32), Ok(FdEntry::Path(_))) {
            return neg_errno(LinuxError::EBADF);
        }
        if flags & general::AT_EMPTY_PATH == 0 {
            return neg_errno(LinuxError::ENOENT);
        }
        if dirfd as i32 == general::AT_FDCWD {
            let cwd = process.cwd();
            let st = match process
                .fds
                .lock()
                .stat_path(process, general::AT_FDCWD, ".")
            {
                Ok(st) => st,
                Err(err) => return neg_errno(err),
            };
            (Some(cwd), st)
        } else {
            match process
                .fds
                .lock()
                .stat_with_recorded_path(process, dirfd as i32)
            {
                Ok((path, st)) => (path, st),
                Err(err) => return neg_errno(err),
            }
        }
    } else {
        let mut fds = process.fds.lock();
        let (resolved_path, st) = if flags & general::AT_SYMLINK_NOFOLLOW != 0 {
            let resolved_path = match resolve_dirfd_path(process, &fds, dirfd as i32, path.as_str())
            {
                Ok(path) => path,
                Err(err) => return neg_errno(err),
            };
            let resolved_path = match process.resolve_parent_symlinks(resolved_path.as_str()) {
                Ok(path) => path,
                Err(err) => return neg_errno(err),
            };
            let st = match process.path_symlink_stat(resolved_path.as_str()) {
                Some(st) => st,
                None => match lstat_absolute_path(process, resolved_path.as_str()) {
                    Ok(st) => st,
                    Err(err) => return neg_errno(err),
                },
            };
            (resolved_path, st)
        } else {
            match fds.path_stat(process, dirfd as i32, path.as_str()) {
                Ok(result) => result,
                Err(err) => return neg_errno(err),
            }
        };
        match fds.parent_dirs_searchable(
            process,
            resolved_path.as_str(),
            process.fs_uid(),
            process.fs_gid(),
        ) {
            Ok(true) => {}
            Ok(false) => return neg_errno(LinuxError::EACCES),
            Err(err) => return neg_errno(err),
        }
        (Some(resolved_path), st)
    };
    apply_chown_metadata(process, record_path, &st, owner, group)
}

fn proc_self_fd_number(path: &str) -> Option<i32> {
    let rest = path
        .strip_prefix("/proc/self/fd/")
        .or_else(|| path.strip_prefix("/dev/fd/"))?;
    if rest.is_empty() || rest.contains('/') {
        return None;
    }
    rest.parse().ok()
}

pub(super) fn fd_entry_statfs_path(entry: &FdEntry) -> Option<&str> {
    match entry {
        FdEntry::DevNull(_) => Some("/dev/null"),
        FdEntry::DevCpuDmaLatency(_) => Some("/dev/cpu_dma_latency"),
        FdEntry::Rtc => Some("/dev/misc/rtc"),
        FdEntry::Pipe(_) => Some("pipe:"),
        FdEntry::Socket(_) | FdEntry::LocalSocket(_) => Some("socket:"),
        _ => fd_entry_path(entry),
    }
}

fn statfs_type_for_path(path: Option<&str>) -> i64 {
    match path {
        Some(path) if path == "/proc" || path.starts_with("/proc/") => PROC_SUPER_MAGIC,
        Some(path) if path == "/sys" || path.starts_with("/sys/") => SYSFS_MAGIC,
        Some(path) if path == "/dev" || path.starts_with("/dev/") => DEVFS_MAGIC,
        // The harness scratch tree is backed by the kernel's ramfs-like
        // in-memory filesystem. Reporting it as tmpfs makes Linux-visible
        // callers skip tmpfs-specific unsupported features (for example
        // O_DIRECT checks) even though this VFS accepts those operations with
        // buffered semantics.
        Some(path) if path == "/tmp" || path.starts_with("/tmp/") => RAMFS_MAGIC,
        Some(path) if path == "/dev/shm" || path.starts_with("/dev/shm/") => TMPFS_MAGIC,
        Some(path) if path.starts_with("pipe:") => PIPEFS_MAGIC,
        _ => EXT4_SUPER_MAGIC,
    }
}

pub(super) const ST_NOSYMFOLLOW_FLAG: u64 = 0x2000;

pub(super) fn generic_statfs(path: Option<&str>) -> general::statfs {
    let alloc = global_allocator();
    let reported_file_blocks = (MAX_IN_MEMORY_FILE_SIZE / STATFS_BLOCK_SIZE as u64).max(1) as i64;
    let available_pages = (alloc.available_pages() as i64).min(reported_file_blocks);
    let total_pages = (alloc.used_pages() as i64 + available_pages).max(1);
    let fs_type = statfs_type_for_path(path);
    general::statfs {
        f_type: fs_type as _,
        f_bsize: STATFS_BLOCK_SIZE as _,
        f_blocks: total_pages as _,
        f_bfree: available_pages as _,
        f_bavail: available_pages as _,
        f_files: total_pages as _,
        f_ffree: available_pages as _,
        f_fsid: general::__kernel_fsid_t {
            val: [fs_type as i32, 0],
        },
        f_namelen: STATFS_NAME_MAX as _,
        f_frsize: STATFS_BLOCK_SIZE as _,
        f_flags: 0,
        f_spare: [0; 4],
    }
}

pub(super) fn dirent_type(ty: FileType) -> u32 {
    match ty {
        FileType::Dir => general::DT_DIR,
        FileType::CharDevice => general::DT_CHR,
        FileType::BlockDevice => general::DT_BLK,
        FileType::Fifo => general::DT_FIFO,
        FileType::Socket => general::DT_SOCK,
        FileType::SymLink => general::DT_LNK,
        _ => general::DT_REG,
    }
}

fn synthetic_char_stat(ino: u64, mode: u32, rdev: u64) -> general::stat {
    let mut st: general::stat = unsafe { core::mem::zeroed() };
    st.st_ino = ino;
    st.st_mode = mode;
    st.st_nlink = 1;
    st.st_rdev = rdev as _;
    st.st_blksize = 512;
    st
}

pub(super) fn stdio_stat(readable: bool) -> general::stat {
    let perm = if readable { 0o440 } else { 0o220 };
    synthetic_char_stat(1, ST_MODE_CHR | perm, 0)
}

pub(super) fn synthetic_char_stat_for_path(path: &str, mode: u32) -> general::stat {
    let rdev = match path {
        "/dev/null" => DEV_NULL_RDEV,
        "/dev/zero" => DEV_ZERO_RDEV,
        "/dev/cpu_dma_latency" => DEV_CPU_DMA_LATENCY_RDEV,
        _ => 0,
    };
    synthetic_char_stat(path_inode(Some(path)), mode, rdev)
}

pub(super) fn synthetic_block_stat_for_path(path: &str, mode: u32) -> general::stat {
    let rdev = match normalize_path("/", path).as_deref() {
        Some(path) if is_synthetic_virtio_block_path(path) => DEV_VDA_RDEV,
        Some(_) | None => 0,
    };
    let mut st = synthetic_char_stat(path_inode(Some(path)), mode, rdev);
    if rdev == DEV_VDA_RDEV {
        st.st_size = SYNTHETIC_BLOCK_DEVICE_SIZE.min(i64::MAX as u64) as _;
        st.st_blocks = (SYNTHETIC_BLOCK_DEVICE_SIZE / 512).min(i64::MAX as u64) as _;
    }
    st
}

fn is_synthetic_virtio_block_path(path: &str) -> bool {
    let Some(name) = path.strip_prefix("/dev/") else {
        return false;
    };
    name == "vda"
}

pub(super) fn dev_null_stat() -> general::stat {
    synthetic_char_stat_for_path("/dev/null", ST_MODE_CHR | 0o220)
}

pub(super) fn dev_zero_stat() -> general::stat {
    synthetic_char_stat_for_path("/dev/zero", ST_MODE_CHR | 0o666)
}

pub(super) fn dev_cpu_dma_latency_stat() -> general::stat {
    synthetic_char_stat_for_path("/dev/cpu_dma_latency", ST_MODE_CHR | 0o600)
}

pub(super) fn path_inode(path: Option<&str>) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    const COMPAT_INO_MAX: u64 = i32::MAX as u64;
    let Some(path) = path else {
        return 1;
    };
    let mut hash = FNV_OFFSET;
    for &byte in path.as_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    (hash % COMPAT_INO_MAX) + 1
}

pub(super) fn file_type_mode(ty: FileType) -> u32 {
    match ty {
        FileType::Dir => ST_MODE_DIR,
        FileType::BlockDevice => ST_MODE_BLK,
        FileType::CharDevice => ST_MODE_CHR,
        FileType::Fifo => ST_MODE_FIFO,
        FileType::SymLink => ST_MODE_LNK,
        FileType::Socket => ST_MODE_SOCKET,
        _ => ST_MODE_FILE,
    }
}

pub(super) fn sys_newfstatat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    statbuf: usize,
    flags: usize,
) -> isize {
    let flags = flags as u32;
    let supported_flags = general::AT_SYMLINK_NOFOLLOW
        | general::AT_EMPTY_PATH
        | general::AT_NO_AUTOMOUNT
        | general::AT_STATX_SYNC_TYPE;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if pathname == 0 {
        if flags & general::AT_EMPTY_PATH == 0 {
            return neg_errno(LinuxError::EFAULT);
        }
        let st = match stat_empty_path(process, dirfd as i32) {
            Ok(st) => st,
            Err(err) => return neg_errno(err),
        };
        return write_user_value(process, statbuf, &st);
    }
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let st = if path.is_empty() && flags & general::AT_EMPTY_PATH != 0 {
        match stat_empty_path(process, dirfd as i32) {
            Ok(st) => st,
            Err(err) => return neg_errno(err),
        }
    } else if path.is_empty() {
        return neg_errno(LinuxError::ENOENT);
    } else {
        if flags & general::AT_SYMLINK_NOFOLLOW != 0 {
            let resolved_path = {
                let table = process.fds.lock();
                match resolve_dirfd_path(process, &table, dirfd as i32, path.as_str()) {
                    Ok(path) => path,
                    Err(err) => return neg_errno(err),
                }
            };
            let resolved_path = match process.resolve_parent_symlinks(resolved_path.as_str()) {
                Ok(path) => path,
                Err(err) => return neg_errno(err),
            };
            if let Some(st) = process.path_symlink_stat(resolved_path.as_str()) {
                st
            } else {
                match lstat_absolute_path(process, resolved_path.as_str()) {
                    Ok(st) => st,
                    Err(err) => return neg_errno(err),
                }
            }
        } else {
            match process
                .fds
                .lock()
                .stat_path(process, dirfd as i32, path.as_str())
            {
                Ok(st) => st,
                Err(err) => return neg_errno(err),
            }
        }
    };
    write_user_value(process, statbuf, &st)
}

pub(super) fn sys_fstat(process: &UserProcess, fd: usize, statbuf: usize) -> isize {
    let st = match process
        .fds
        .lock()
        .stat_with_recorded_path(process, fd as i32)
    {
        Ok((_, st)) => st,
        Err(err) => return neg_errno(err),
    };
    write_user_value(process, statbuf, &st)
}

fn statx_attributes_from_inode_flags(flags: u32) -> u64 {
    let mut attrs = 0u64;
    if flags & general::FS_IMMUTABLE_FL != 0 {
        attrs |= general::STATX_ATTR_IMMUTABLE as u64;
    }
    if flags & general::FS_APPEND_FL != 0 {
        attrs |= general::STATX_ATTR_APPEND as u64;
    }
    if flags & general::FS_NODUMP_FL != 0 {
        attrs |= general::STATX_ATTR_NODUMP as u64;
    }
    attrs
}

fn statx_attributes_for_path(process: &UserProcess, path: Option<&str>, inode_flags: u32) -> u64 {
    let mut attrs = statx_attributes_from_inode_flags(inode_flags);
    if path.is_some_and(|path| process.has_mount_point(path)) {
        attrs |= general::STATX_ATTR_MOUNT_ROOT as u64;
    }
    attrs
}

fn stat_to_statx(st: general::stat, attributes: u64) -> general::statx {
    let mut stx: general::statx = unsafe { core::mem::zeroed() };
    stx.stx_mask = general::STATX_BASIC_STATS;
    stx.stx_blksize = st.st_blksize as _;
    stx.stx_nlink = st.st_nlink as _;
    stx.stx_uid = st.st_uid as _;
    stx.stx_gid = st.st_gid as _;
    stx.stx_mode = st.st_mode as _;
    stx.stx_ino = st.st_ino as _;
    stx.stx_size = st.st_size as _;
    stx.stx_blocks = st.st_blocks as _;
    stx.stx_attributes = attributes as _;
    stx.stx_attributes_mask = (general::STATX_ATTR_IMMUTABLE
        | general::STATX_ATTR_APPEND
        | general::STATX_ATTR_NODUMP
        | general::STATX_ATTR_MOUNT_ROOT) as _;
    stx.stx_dev_major = ((st.st_dev as u64) >> 8) as _;
    stx.stx_dev_minor = ((st.st_dev as u64) & 0xff) as _;
    stx.stx_rdev_major = ((st.st_rdev as u64) >> 8) as _;
    stx.stx_rdev_minor = ((st.st_rdev as u64) & 0xff) as _;
    stx.stx_atime.tv_sec = st.st_atime as _;
    stx.stx_atime.tv_nsec = st.st_atime_nsec as _;
    stx.stx_mtime.tv_sec = st.st_mtime as _;
    stx.stx_mtime.tv_nsec = st.st_mtime_nsec as _;
    stx.stx_ctime.tv_sec = st.st_ctime as _;
    stx.stx_ctime.tv_nsec = st.st_ctime_nsec as _;
    stx
}

pub(super) fn sys_statx(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    flags: usize,
    mask: usize,
    statxbuf: usize,
) -> isize {
    if statxbuf == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let flags = flags as u32;
    let supported_flags = general::AT_SYMLINK_NOFOLLOW
        | general::AT_EMPTY_PATH
        | general::AT_NO_AUTOMOUNT
        | general::AT_STATX_SYNC_TYPE;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let mask = mask as u32;
    let supported_mask = general::STATX_BASIC_STATS
        | general::STATX_BTIME
        | general::STATX_MNT_ID
        | general::STATX_DIOALIGN
        | general::STATX_MNT_ID_UNIQUE
        | general::STATX_SUBVOL
        | general::STATX_WRITE_ATOMIC
        | general::STATX_DIO_READ_ALIGN;
    if mask & !supported_mask != 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let (st, attributes) = if pathname == 0 {
        if flags & general::AT_EMPTY_PATH == 0 {
            return neg_errno(LinuxError::EFAULT);
        }
        match stat_empty_path_with_attributes(process, dirfd as i32) {
            Ok(result) => result,
            Err(err) => return neg_errno(err),
        }
    } else {
        let path = match read_cstr(process, pathname) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        };
        if path.is_empty() && flags & general::AT_EMPTY_PATH != 0 {
            match stat_empty_path_with_attributes(process, dirfd as i32) {
                Ok(result) => result,
                Err(err) => return neg_errno(err),
            }
        } else if path.is_empty() {
            return neg_errno(LinuxError::ENOENT);
        } else if flags & general::AT_SYMLINK_NOFOLLOW != 0 {
            let resolved_path = {
                let table = process.fds.lock();
                match resolve_dirfd_path(process, &table, dirfd as i32, path.as_str()) {
                    Ok(path) => path,
                    Err(err) => return neg_errno(err),
                }
            };
            let resolved_path = match process.resolve_parent_symlinks(resolved_path.as_str()) {
                Ok(path) => path,
                Err(err) => return neg_errno(err),
            };
            let inode_flags = process.path_inode_flags(resolved_path.as_str());
            let attributes =
                statx_attributes_for_path(process, Some(resolved_path.as_str()), inode_flags);
            if let Some(st) = process.path_symlink_stat(resolved_path.as_str()) {
                (st, attributes)
            } else {
                match lstat_absolute_path(process, resolved_path.as_str()) {
                    Ok(st) => (st, attributes),
                    Err(err) => return neg_errno(err),
                }
            }
        } else {
            let resolved_path = {
                let table = process.fds.lock();
                match resolve_dirfd_path(process, &table, dirfd as i32, path.as_str()) {
                    Ok(path) => path,
                    Err(err) => return neg_errno(err),
                }
            };
            let inode_flags = process.path_inode_flags(resolved_path.as_str());
            let attributes =
                statx_attributes_for_path(process, Some(resolved_path.as_str()), inode_flags);
            match process
                .fds
                .lock()
                .stat_path(process, dirfd as i32, path.as_str())
            {
                Ok(st) => (st, attributes),
                Err(err) => return neg_errno(err),
            }
        }
    };
    write_user_value(process, statxbuf, &stat_to_statx(st, attributes))
}

fn stat_empty_path(process: &UserProcess, dirfd: i32) -> Result<general::stat, LinuxError> {
    if dirfd == general::AT_FDCWD {
        process
            .fds
            .lock()
            .stat_path(process, general::AT_FDCWD, ".")
    } else {
        process
            .fds
            .lock()
            .stat_with_recorded_path(process, dirfd)
            .map(|(_, st)| st)
    }
}

fn stat_empty_path_with_attributes(
    process: &UserProcess,
    dirfd: i32,
) -> Result<(general::stat, u64), LinuxError> {
    if dirfd == general::AT_FDCWD {
        let cwd = process.cwd();
        stat_empty_path(process, dirfd).map(|st| {
            (
                st,
                statx_attributes_for_path(process, Some(cwd.as_str()), 0),
            )
        })
    } else {
        process
            .fds
            .lock()
            .stat_with_recorded_path(process, dirfd)
            .map(|(path, st)| {
                let inode_flags = path
                    .as_deref()
                    .map(|path| process.path_inode_flags(path))
                    .unwrap_or(0);
                let attributes = statx_attributes_for_path(process, path.as_deref(), inode_flags);
                (st, attributes)
            })
    }
}

pub(super) fn sys_readlinkat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    buf: usize,
    bufsiz: usize,
) -> isize {
    if bufsiz == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if path.len() >= LINUX_PATH_MAX {
        return neg_errno(LinuxError::ENAMETOOLONG);
    }
    let resolved_path = if path.is_empty() {
        if dirfd as i32 == general::AT_FDCWD {
            return neg_errno(LinuxError::ENOENT);
        }
        let table = process.fds.lock();
        match table.entry(dirfd as i32).and_then(|entry| {
            fd_entry_path(entry)
                .map(ToString::to_string)
                .ok_or(LinuxError::EINVAL)
        }) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        }
    } else {
        let table = process.fds.lock();
        match resolve_dirfd_path(process, &table, dirfd as i32, path.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        }
    };
    let resolved_path = match process.resolve_parent_symlinks(resolved_path.as_str()) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if process.fs_uid() != 0 {
        let mut table = process.fds.lock();
        match table.parent_dirs_searchable(
            process,
            resolved_path.as_str(),
            process.fs_uid(),
            process.fs_gid(),
        ) {
            Ok(true) => {}
            Ok(false) => return neg_errno(LinuxError::EACCES),
            Err(err) => return neg_errno(err),
        }
    }
    let target = if let Some(fd) = proc_self_fd_number(resolved_path.as_str()) {
        {
            let table = process.fds.lock();
            match table.entry(fd) {
                Ok(entry) => match fd_entry_path(entry) {
                    Some(path) => path.to_string(),
                    None => return neg_errno(LinuxError::EINVAL),
                },
                Err(err) => return neg_errno(err),
            }
        }
    } else if let Some(target) = proc_exe_link_target(process, resolved_path.as_str()) {
        target
    } else if let Some(target) = process.path_symlink(resolved_path.as_str()) {
        target
    } else {
        match axfs::api::read_link(resolved_path.as_str()) {
            Ok(target) => target,
            Err(err) => return neg_errno(LinuxError::from(err)),
        }
    };
    let bytes = target.as_bytes();
    let copy_len = cmp::min(bytes.len(), bufsiz);
    let Some(dst) = UserPtr::<u8, Write>::new(UserAddr::new(buf)).slice(copy_len) else {
        return neg_errno(LinuxError::EFAULT);
    };
    match write_user_slice(process, dst, &bytes[..copy_len]) {
        Ok(()) => copy_len as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_utimensat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    times: usize,
    flags: usize,
) -> isize {
    let flags = flags as u32;
    let supported_flags = general::AT_SYMLINK_NOFOLLOW | general::AT_EMPTY_PATH;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let request = match read_utime_request(process, times) {
        Ok(request) => request,
        Err(err) => return neg_errno(err),
    };

    let (record_path, st) = match utimensat_target(process, dirfd as i32, pathname, flags) {
        Ok(target) => target,
        Err(err) => return neg_errno(err),
    };

    if request.both_omit() {
        return 0;
    }
    if let Some(path) = record_path.as_deref() {
        if process.path_on_readonly_mount(path) {
            return neg_errno(LinuxError::EROFS);
        }
    }
    if let Err(err) = check_utimensat_permission(process, &st, request) {
        return neg_errno(err);
    }

    let Some(path) = record_path else {
        return 0;
    };
    let now = match realtime_timespec() {
        Ok(now) => now,
        Err(err) => return neg_errno(err),
    };
    let current = process
        .path_times(path.as_str())
        .unwrap_or_else(|| PathTimes::from_stat(&st));
    let new_times = PathTimes {
        atime: apply_utime_selection(request.atime, current.atime, now),
        mtime: apply_utime_selection(request.mtime, current.mtime, now),
        ctime: now,
    };
    process.set_path_times(path, new_times);
    0
}

fn read_utime_request(process: &UserProcess, times: usize) -> Result<UtimeRequest, LinuxError> {
    if times == 0 {
        return Ok(UtimeRequest {
            atime: UtimeSelection::Now,
            mtime: UtimeSelection::Now,
        });
    }
    let atime = read_user_value::<general::timespec>(process, times)?;
    let mtime_addr = times
        .checked_add(size_of::<general::timespec>())
        .ok_or(LinuxError::EFAULT)?;
    let mtime = read_user_value::<general::timespec>(process, mtime_addr)?;
    Ok(UtimeRequest {
        atime: parse_utime_selection(atime)?,
        mtime: parse_utime_selection(mtime)?,
    })
}

fn parse_utime_selection(ts: general::timespec) -> Result<UtimeSelection, LinuxError> {
    let nsec = ts.tv_nsec as i64;
    if nsec == general::UTIME_NOW as i64 {
        Ok(UtimeSelection::Now)
    } else if nsec == general::UTIME_OMIT as i64 {
        Ok(UtimeSelection::Omit)
    } else if (0..NSEC_PER_SEC).contains(&nsec) {
        Ok(UtimeSelection::Set(ts))
    } else {
        Err(LinuxError::EINVAL)
    }
}

fn utimensat_target(
    process: &UserProcess,
    dirfd: i32,
    pathname: usize,
    flags: u32,
) -> Result<(Option<String>, general::stat), LinuxError> {
    if pathname == 0 {
        if dirfd == general::AT_FDCWD {
            return Err(LinuxError::EFAULT);
        }
        return utimensat_fd_target(process, dirfd);
    }
    let path = read_cstr(process, pathname)?;
    if path.len() >= LINUX_PATH_MAX {
        return Err(LinuxError::ENAMETOOLONG);
    }
    if let Some(fd) = proc_self_fd_number(path.as_str()) {
        return utimensat_fd_target(process, fd);
    }
    if path.is_empty() {
        if flags & general::AT_EMPTY_PATH == 0 {
            return Err(LinuxError::ENOENT);
        }
        return utimensat_fd_target(process, dirfd);
    }

    let abs_path = {
        let table = process.fds.lock();
        resolve_dirfd_path(process, &table, dirfd, path.as_str())?
    };
    if flags & general::AT_SYMLINK_NOFOLLOW != 0 {
        let resolved_path = process.resolve_parent_symlinks(abs_path.as_str())?;
        if let Some(st) = process.path_symlink_stat(resolved_path.as_str()) {
            return Ok((Some(resolved_path), st));
        }
        let st = lstat_absolute_path(process, resolved_path.as_str())?;
        return Ok((Some(resolved_path), st));
    }

    let resolved_path = process
        .resolve_path_symlink(abs_path.as_str())?
        .unwrap_or(abs_path);
    let st = process
        .fds
        .lock()
        .stat_path(process, general::AT_FDCWD, resolved_path.as_str())?;
    Ok((Some(resolved_path), st))
}

fn utimensat_fd_target(
    process: &UserProcess,
    fd: i32,
) -> Result<(Option<String>, general::stat), LinuxError> {
    let mut table = process.fds.lock();
    table.stat_with_recorded_path(process, fd)
}

fn check_utimensat_permission(
    process: &UserProcess,
    st: &general::stat,
    request: UtimeRequest,
) -> Result<(), LinuxError> {
    let uid = process.fs_uid();
    if uid == 0 {
        return Ok(());
    }
    let owner = st.st_uid as u32 == uid;
    if request.both_now() {
        if owner || access_allowed(st, ACCESS_W_OK, uid, process.fs_gid()) {
            Ok(())
        } else {
            Err(LinuxError::EACCES)
        }
    } else if owner {
        Ok(())
    } else {
        Err(LinuxError::EPERM)
    }
}

fn apply_utime_selection(
    selection: UtimeSelection,
    current: general::timespec,
    now: general::timespec,
) -> general::timespec {
    match selection {
        UtimeSelection::Set(ts) => ts,
        UtimeSelection::Now => now,
        UtimeSelection::Omit => current,
    }
}

pub(super) fn sys_statfs(process: &UserProcess, pathname: usize, statfsbuf: usize) -> isize {
    if statfsbuf == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let st = match process
        .fds
        .lock()
        .statfs_path(process, general::AT_FDCWD, path.as_str())
    {
        Ok(st) => st,
        Err(err) => return neg_errno(err),
    };
    write_user_value(process, statfsbuf, &st)
}

pub(super) fn sys_fstatfs(process: &UserProcess, fd: usize, statfsbuf: usize) -> isize {
    if statfsbuf == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let st = match process.fds.lock().statfs(fd as i32) {
        Ok(st) => st,
        Err(err) => return neg_errno(err),
    };
    write_user_value(process, statfsbuf, &st)
}
