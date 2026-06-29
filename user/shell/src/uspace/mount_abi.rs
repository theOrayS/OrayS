use axerrno::LinuxError;
use linux_raw_sys::general;
use std::boxed::Box;
use std::string::{String, ToString};

use super::fd_table::{
    resolve_dirfd_path, synthetic_block_device_for_mount, synthetic_block_device_is_uninitialized,
};
use super::linux_abi::{ST_MODE_DIR, ST_MODE_TYPE_MASK, neg_errno};
use super::runtime_paths::normalize_path;
use super::user_memory::read_cstr;
use super::{MountPoint, UserProcess};

const SUPPORTED_MOUNT_FLAGS: u32 = general::MS_BIND
    | general::MS_REC
    | general::MS_SILENT
    | general::MS_RDONLY
    | general::MS_REMOUNT
    | general::MS_NOSYMFOLLOW;

const SUPPORTED_UMOUNT_FLAGS: u32 = general::MNT_FORCE
    | general::MNT_DETACH
    | general::MNT_EXPIRE
    | general::UMOUNT_NOFOLLOW
    | general::UMOUNT_UNUSED;

impl UserProcess {
    pub(super) fn translate_mount_path(&self, path: &str) -> String {
        let mount_points = self.mount_points.lock();
        let mut best: Option<(&str, &MountPoint)> = None;
        for (target, mount) in mount_points.iter() {
            if mount_path_rest(path, target.as_str()).is_none() {
                continue;
            }
            if best.is_none_or(|(best_target, _)| target.len() > best_target.len()) {
                best = Some((target.as_str(), mount));
            }
        }
        let Some((target, mount)) = best else {
            return path.to_string();
        };
        let rest = mount_path_rest(path, target).unwrap_or("");
        join_mount_source(mount.source_root.as_str(), rest)
    }

    fn best_mount_flag(&self, path: &str, flag: impl Fn(&MountPoint) -> bool) -> bool {
        let mount_points = self.mount_points.lock();
        let mut best: Option<(&str, bool)> = None;
        for (target, mount) in mount_points.iter() {
            if mount_path_rest(path, target.as_str()).is_none() {
                continue;
            }
            if best.is_none_or(|(best_target, _)| target.len() > best_target.len()) {
                best = Some((target.as_str(), flag(mount)));
            }
        }
        best.is_some_and(|(_, enabled)| enabled)
    }

    pub(super) fn path_on_readonly_mount(&self, path: &str) -> bool {
        self.best_mount_flag(path, |mount| mount.readonly)
    }

    pub(super) fn path_on_nosymfollow_mount(&self, path: &str) -> bool {
        self.best_mount_flag(path, |mount| mount.nosymfollow)
    }

    pub(super) fn paths_cross_mount(&self, lhs: &str, rhs: &str) -> bool {
        fn best_mount_target<'a>(
            mount_points: &'a std::collections::BTreeMap<String, MountPoint>,
            path: &str,
        ) -> Option<&'a str> {
            let mut best: Option<&str> = None;
            for target in mount_points.keys() {
                if mount_path_rest(path, target.as_str()).is_none() {
                    continue;
                }
                if best.is_none_or(|best_target| target.len() > best_target.len()) {
                    best = Some(target.as_str());
                }
            }
            best
        }

        let mount_points = self.mount_points.lock();
        best_mount_target(&mount_points, lhs) != best_mount_target(&mount_points, rhs)
    }

    fn add_mount_point(
        &self,
        target: String,
        source_root: String,
        readonly: bool,
        nosymfollow: bool,
        tmpfs_size_limit: Option<u64>,
        remount: bool,
    ) {
        let mut mount_points = self.mount_points.lock();
        if remount {
            if let Some(mount) = mount_points.get_mut(target.as_str()) {
                mount.readonly = readonly;
                mount.nosymfollow = nosymfollow;
                mount.tmpfs_size_limit = tmpfs_size_limit;
                return;
            }
        }
        mount_points.insert(
            target,
            MountPoint {
                source_root,
                readonly,
                nosymfollow,
                tmpfs_size_limit,
            },
        );
    }

    pub(super) fn has_mount_point(&self, target: &str) -> bool {
        self.mount_points.lock().contains_key(target)
    }

    fn remove_mount_point(&self, target: &str) -> bool {
        self.mount_points.lock().remove(target).is_some()
    }

    pub(super) fn tmpfs_free_512_blocks_for_path(&self, path: &str) -> Option<u64> {
        let mount_points = self.mount_points.lock();
        let mut best: Option<(&str, u64)> = None;
        for (target, mount) in mount_points.iter() {
            let Some(limit) = mount.tmpfs_size_limit else {
                continue;
            };
            if mount_path_rest(path, target.as_str()).is_none() {
                continue;
            }
            if best.is_none_or(|(best_target, _)| target.len() > best_target.len()) {
                best = Some((target.as_str(), limit));
            }
        }
        let (target, limit) = best?;
        let limit_blocks = limit.saturating_add(511) / 512;
        let used_blocks = {
            let all_ranges = self.path_data_ranges.lock();
            all_ranges
                .iter()
                .filter(|(range_path, _)| mount_path_rest(range_path.as_str(), target).is_some())
                .flat_map(|(_, ranges)| ranges.iter())
                .map(|(start, end)| end.saturating_sub(*start) / 512)
                .fold(0u64, u64::saturating_add)
        };
        Some(limit_blocks.saturating_sub(used_blocks))
    }
}

pub(super) fn sys_mount(
    process: &UserProcess,
    source: usize,
    target: usize,
    filesystemtype: usize,
    mountflags: usize,
    _data: usize,
) -> isize {
    let flags = mountflags as u32;
    if flags & !SUPPORTED_MOUNT_FLAGS != 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let source = match read_optional_cstr(process, source) {
        Ok(source) => source,
        Err(err) => return neg_errno(err),
    };
    let target = match read_cstr(process, target) {
        Ok(target) => target,
        Err(err) => return neg_errno(err),
    };
    let fstype = match read_optional_cstr(process, filesystemtype) {
        Ok(fstype) => fstype,
        Err(err) => return neg_errno(err),
    };
    let data = match read_optional_cstr(process, _data) {
        Ok(data) => data,
        Err(err) => return neg_errno(err),
    };
    let target_path = match resolve_mount_target_path(process, target.as_str()) {
        Ok(target_path) => target_path,
        Err(err) => return neg_errno(err),
    };
    if let Err(err) = ensure_mount_target_directory(process, target_path.as_str()) {
        return neg_errno(err);
    }
    if flags & general::MS_REMOUNT != 0 && !process.has_mount_point(target_path.as_str()) {
        return neg_errno(LinuxError::EINVAL);
    }
    let source_root = match resolve_mount_source(
        process,
        source.as_deref(),
        fstype.as_deref(),
        flags,
        target_path.as_str(),
    ) {
        Ok(source_root) => source_root,
        Err(err) => return neg_errno(err),
    };
    let readonly = flags & general::MS_RDONLY != 0;
    let nosymfollow = flags & general::MS_NOSYMFOLLOW != 0;
    let tmpfs_size_limit = if fstype.as_deref() == Some("tmpfs") {
        parse_tmpfs_size_limit(data.as_deref())
    } else {
        None
    };
    let remount = flags & general::MS_REMOUNT != 0;
    process.add_mount_point(
        target_path,
        source_root,
        readonly,
        nosymfollow,
        tmpfs_size_limit,
        remount,
    );
    0
}

pub(super) fn sys_umount2(process: &UserProcess, target: usize, flags: usize) -> isize {
    let flags = flags as u32;
    if flags & !SUPPORTED_UMOUNT_FLAGS != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let target = match read_cstr(process, target) {
        Ok(target) => target,
        Err(err) => return neg_errno(err),
    };
    let target_path = match resolve_mount_target_path(process, target.as_str()) {
        Ok(target_path) => target_path,
        Err(err) => return neg_errno(err),
    };
    if !process.remove_mount_point(target_path.as_str()) {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = axfs::api::umount(target_path.as_str()) {
        let err = LinuxError::from(err);
        if err != LinuxError::ENOENT {
            return neg_errno(err);
        }
    }
    0
}

fn read_optional_cstr(process: &UserProcess, ptr: usize) -> Result<Option<String>, LinuxError> {
    if ptr == 0 {
        Ok(None)
    } else {
        read_cstr(process, ptr).map(Some)
    }
}

fn resolve_mount_target_path(process: &UserProcess, target: &str) -> Result<String, LinuxError> {
    if target.is_empty() {
        return Err(LinuxError::ENOENT);
    }
    let cwd = process.cwd();
    normalize_path(cwd.as_str(), target).ok_or(LinuxError::EINVAL)
}

fn ensure_mount_target_directory(
    process: &UserProcess,
    target_path: &str,
) -> Result<(), LinuxError> {
    let st = process
        .fds
        .lock()
        .stat_path(process, general::AT_FDCWD, target_path)?;
    if st.st_mode as u32 & ST_MODE_TYPE_MASK != ST_MODE_DIR {
        return Err(LinuxError::ENOTDIR);
    }
    Ok(())
}

fn resolve_mount_source(
    process: &UserProcess,
    source: Option<&str>,
    fstype: Option<&str>,
    flags: u32,
    target_path: &str,
) -> Result<String, LinuxError> {
    if flags & general::MS_BIND != 0 {
        let source = source.ok_or(LinuxError::EINVAL)?;
        let source_path = {
            let fds = process.fds.lock();
            resolve_dirfd_path(process, &fds, general::AT_FDCWD, source)?
        };
        process
            .fds
            .lock()
            .stat_path(process, general::AT_FDCWD, source_path.as_str())?;
        return Ok(source_path);
    }

    match fstype.unwrap_or("") {
        "devtmpfs" | "devfs" => Ok("/dev".into()),
        "proc" | "procfs" => Ok("/proc".into()),
        "sysfs" => Ok("/sys".into()),
        "tmpfs" => Ok(target_path.into()),
        "vfat" | "msdos" | "fat" => {
            let source = source.ok_or(LinuxError::EINVAL)?;
            let source_path = {
                let fds = process.fds.lock();
                resolve_dirfd_path(process, &fds, general::AT_FDCWD, source)?
            };
            let Some(dev) = synthetic_block_device_for_mount(source_path.as_str()) else {
                return Err(LinuxError::EOPNOTSUPP);
            };
            let format = synthetic_block_device_is_uninitialized(source_path.as_str());
            let mount_path: &'static str = Box::leak(target_path.to_string().into_boxed_str());
            axfs::api::mount_fatfs(mount_path, dev, format).map_err(LinuxError::from)?;
            Ok(target_path.into())
        }
        "ext2" | "ext3" | "ext4" => Err(LinuxError::EOPNOTSUPP),
        "" => Err(LinuxError::EINVAL),
        _ => Err(LinuxError::ENODEV),
    }
}

fn parse_tmpfs_size_limit(data: Option<&str>) -> Option<u64> {
    data?
        .split(',')
        .filter_map(|option| option.strip_prefix("size="))
        .filter_map(parse_tmpfs_size_value)
        .next()
}

fn parse_tmpfs_size_value(value: &str) -> Option<u64> {
    let split = value
        .char_indices()
        .find(|(_, ch)| !ch.is_ascii_digit())
        .map(|(idx, _)| idx)
        .unwrap_or(value.len());
    if split == 0 {
        return None;
    }
    let number = value[..split].parse::<u64>().ok()?;
    let mut suffix = value[split..].trim();
    if suffix.ends_with('B') || suffix.ends_with('b') {
        suffix = &suffix[..suffix.len().saturating_sub(1)];
    }
    let multiplier = match suffix {
        "" => 1,
        "k" | "K" => 1024,
        "m" | "M" => 1024 * 1024,
        "g" | "G" => 1024 * 1024 * 1024,
        _ => return None,
    };
    number.checked_mul(multiplier)
}

fn mount_path_rest<'a>(path: &'a str, target: &str) -> Option<&'a str> {
    if path == target {
        return Some("");
    }
    path.strip_prefix(target)
        .and_then(|rest| rest.strip_prefix('/'))
}

fn join_mount_source(source: &str, rest: &str) -> String {
    if rest.is_empty() {
        return source.to_string();
    }
    if source == "/" {
        format!("/{rest}")
    } else {
        format!("{}/{rest}", source.trim_end_matches('/'))
    }
}
