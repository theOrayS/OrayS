//! Volatile writable overlay for a read-only lower filesystem.
//!
//! Competition root images use an ext4 lower layer that must remain an
//! immutable input.  This module combines that lower tree with a ramfs upper
//! tree.  Reads fall through to the lower layer, while every mutation is
//! copied up and remains memory-resident.  Whiteouts prevent removed or moved
//! lower nodes from becoming visible again.

use alloc::{
    collections::{BTreeMap, BTreeSet},
    format,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use axfs_ramfs::RamFileSystem;
use axfs_vfs::{
    FileSystemInfo, VfsDirEntry, VfsError, VfsNodeAttr, VfsNodeOps, VfsNodeRef, VfsNodeType,
    VfsOps, VfsResult,
};
use axsync::Mutex;

const COPY_CHUNK_SIZE: usize = 16 * 1024;

/// A filesystem with an immutable lower tree and a volatile ramfs upper tree.
pub struct OverlayFileSystem {
    state: Arc<OverlayState>,
    root: Arc<OverlayNode>,
}

struct OverlayState {
    lower: Arc<dyn VfsOps>,
    lower_root: VfsNodeRef,
    upper_root: VfsNodeRef,
    whiteouts: Mutex<BTreeSet<String>>,
    mutation: Mutex<()>,
}

struct OverlayNode {
    state: Arc<OverlayState>,
    path: String,
}

impl OverlayFileSystem {
    /// Creates an empty volatile upper layer above `lower`.
    pub fn new(lower: Arc<dyn VfsOps>) -> Self {
        let upper: Arc<dyn VfsOps> = Arc::new(RamFileSystem::new());
        let state = Arc::new(OverlayState {
            lower_root: lower.root_dir(),
            upper_root: upper.root_dir(),
            lower,
            whiteouts: Mutex::new(BTreeSet::new()),
            mutation: Mutex::new(()),
        });
        let root = Arc::new(OverlayNode {
            state: state.clone(),
            path: "/".into(),
        });
        Self { state, root }
    }
}

impl VfsOps for OverlayFileSystem {
    fn umount(&self) -> VfsResult {
        self.state.lower.umount()
    }

    fn statfs(&self) -> VfsResult<FileSystemInfo> {
        self.state.lower.statfs()
    }

    fn root_dir(&self) -> VfsNodeRef {
        self.root.clone()
    }
}

impl OverlayState {
    fn relative(path: &str) -> &str {
        path.trim_start_matches('/')
    }

    fn upper_node(&self, path: &str) -> Option<VfsNodeRef> {
        self.upper_root.clone().lookup(Self::relative(path)).ok()
    }

    fn lower_node_raw(&self, path: &str) -> Option<VfsNodeRef> {
        self.lower_root.clone().lookup(Self::relative(path)).ok()
    }

    fn lower_is_hidden(&self, path: &str) -> bool {
        let whiteouts = self.whiteouts.lock();
        let mut candidate = path;
        loop {
            if whiteouts.contains(candidate) {
                return true;
            }
            if candidate == "/" {
                return false;
            }
            candidate = parent_path(candidate);
        }
    }

    fn lower_node(&self, path: &str) -> Option<VfsNodeRef> {
        if self.lower_is_hidden(path) {
            None
        } else {
            self.lower_node_raw(path)
        }
    }

    fn visible_node(&self, path: &str) -> Option<VfsNodeRef> {
        self.upper_node(path).or_else(|| self.lower_node(path))
    }

    fn ensure_upper_dir_locked(&self, path: &str) -> VfsResult<VfsNodeRef> {
        if path == "/" || path.is_empty() {
            return Ok(self.upper_root.clone());
        }

        let mut current = String::new();
        for component in path.split('/').filter(|part| !part.is_empty()) {
            current.push('/');
            current.push_str(component);
            if let Some(node) = self.upper_node(current.as_str()) {
                if !node.get_attr()?.is_dir() {
                    return Err(VfsError::NotADirectory);
                }
                continue;
            }

            self.upper_root
                .create(Self::relative(current.as_str()), VfsNodeType::Dir)?;
            let upper = self
                .upper_node(current.as_str())
                .ok_or(VfsError::NotFound)?;
            if let Some(lower) = self.lower_node_raw(current.as_str()) {
                let attr = lower.get_attr()?;
                if !attr.is_dir() {
                    return Err(VfsError::NotADirectory);
                }
                match upper.set_perm(attr.perm()) {
                    Ok(()) | Err(VfsError::Unsupported) => {}
                    Err(err) => return Err(err),
                }
            }
        }
        self.upper_node(path).ok_or(VfsError::NotFound)
    }

    fn ensure_copy_up_file_locked(&self, path: &str) -> VfsResult<VfsNodeRef> {
        if let Some(upper) = self.upper_node(path) {
            return if upper.get_attr()?.is_file() {
                Ok(upper)
            } else {
                Err(VfsError::IsADirectory)
            };
        }

        let lower = self.lower_node(path).ok_or(VfsError::NotFound)?;
        let attr = lower.get_attr()?;
        if !attr.is_file() {
            return if attr.is_dir() {
                Err(VfsError::IsADirectory)
            } else {
                Err(VfsError::Unsupported)
            };
        }

        self.ensure_upper_dir_locked(parent_path(path))?;
        self.upper_root
            .create(Self::relative(path), VfsNodeType::File)?;
        let upper = self.upper_node(path).ok_or(VfsError::NotFound)?;
        if let Err(err) = upper.set_perm(attr.perm())
            && err != VfsError::Unsupported
        {
            let _ = self.upper_root.remove(Self::relative(path));
            return Err(err);
        }

        let mut buffer = Vec::new();
        if buffer.try_reserve_exact(COPY_CHUNK_SIZE).is_err() {
            let _ = self.upper_root.remove(Self::relative(path));
            return Err(VfsError::StorageFull);
        }
        buffer.resize(COPY_CHUNK_SIZE, 0);
        let mut offset = 0u64;
        loop {
            let count = match lower.read_at(offset, buffer.as_mut_slice()) {
                Ok(count) => count,
                Err(err) => {
                    let _ = self.upper_root.remove(Self::relative(path));
                    return Err(err);
                }
            };
            if count == 0 {
                break;
            }
            if let Err(err) = upper.write_at(offset, &buffer[..count]) {
                let _ = self.upper_root.remove(Self::relative(path));
                return Err(err);
            }
            let Some(next_offset) = offset.checked_add(count as u64) else {
                let _ = self.upper_root.remove(Self::relative(path));
                return Err(VfsError::StorageFull);
            };
            offset = next_offset;
        }
        Ok(upper)
    }

    fn ensure_empty_upper_file_locked(&self, path: &str) -> VfsResult<VfsNodeRef> {
        if let Some(upper) = self.upper_node(path) {
            if !upper.get_attr()?.is_file() {
                return Err(VfsError::IsADirectory);
            }
            upper.truncate(0)?;
            return Ok(upper);
        }

        let lower = self.lower_node(path).ok_or(VfsError::NotFound)?;
        let attr = lower.get_attr()?;
        if !attr.is_file() {
            return if attr.is_dir() {
                Err(VfsError::IsADirectory)
            } else {
                Err(VfsError::Unsupported)
            };
        }
        self.ensure_upper_dir_locked(parent_path(path))?;
        self.upper_root
            .create(Self::relative(path), VfsNodeType::File)?;
        let upper = self.upper_node(path).ok_or(VfsError::NotFound)?;
        match upper.set_perm(attr.perm()) {
            Ok(()) | Err(VfsError::Unsupported) => Ok(upper),
            Err(err) => {
                let _ = self.upper_root.remove(Self::relative(path));
                Err(err)
            }
        }
    }

    fn merged_entries(&self, path: &str) -> VfsResult<BTreeMap<String, VfsNodeType>> {
        let mut entries = BTreeMap::new();
        if let Some(lower) = self.lower_node(path) {
            collect_dir_entries(&lower, |name, ty| {
                let child = join_path(path, name.as_str());
                if !self.lower_is_hidden(child.as_str()) {
                    entries.insert(name, ty);
                }
            })?;
        }
        if let Some(upper) = self.upper_node(path) {
            collect_dir_entries(&upper, |name, ty| {
                entries.insert(name, ty);
            })?;
        }
        Ok(entries)
    }

    fn ensure_copy_up_tree_locked(&self, path: &str) -> VfsResult {
        let node = self.visible_node(path).ok_or(VfsError::NotFound)?;
        let attr = node.get_attr()?;
        if attr.is_file() {
            self.ensure_copy_up_file_locked(path)?;
            return Ok(());
        }
        if !attr.is_dir() {
            return Err(VfsError::Unsupported);
        }

        let upper = self.ensure_upper_dir_locked(path)?;
        match upper.set_perm(attr.perm()) {
            Ok(()) | Err(VfsError::Unsupported) => {}
            Err(err) => return Err(err),
        }
        let children = self.merged_entries(path)?;
        for name in children.keys() {
            self.ensure_copy_up_tree_locked(join_path(path, name.as_str()).as_str())?;
        }
        Ok(())
    }

    fn add_whiteout(&self, path: &str) {
        let mut whiteouts = self.whiteouts.lock();
        let mut ancestor = parent_path(path);
        loop {
            if whiteouts.contains(ancestor) {
                return;
            }
            if ancestor == "/" {
                break;
            }
            ancestor = parent_path(ancestor);
        }

        let descendant_prefix = format!("{path}/");
        whiteouts.retain(|entry| !entry.starts_with(descendant_prefix.as_str()));
        whiteouts.insert(path.to_string());
    }
}

impl OverlayNode {
    fn resolve(&self, path: &str) -> String {
        if path.starts_with('/') {
            axfs_vfs::path::canonicalize(path)
        } else {
            join_path(self.path.as_str(), path)
        }
    }

    fn new_at(&self, path: String) -> VfsNodeRef {
        Arc::new(Self {
            state: self.state.clone(),
            path,
        })
    }
}

impl VfsNodeOps for OverlayNode {
    fn get_attr(&self) -> VfsResult<VfsNodeAttr> {
        self.state
            .visible_node(self.path.as_str())
            .ok_or(VfsError::NotFound)?
            .get_attr()
    }

    fn set_perm(&self, perm: axfs_vfs::VfsNodePerm) -> VfsResult {
        let _guard = self.state.mutation.lock();
        let attr = self.get_attr()?;
        let upper = if attr.is_dir() {
            self.state.ensure_upper_dir_locked(self.path.as_str())?
        } else if attr.is_file() {
            self.state.ensure_copy_up_file_locked(self.path.as_str())?
        } else {
            return Err(VfsError::Unsupported);
        };
        upper.set_perm(perm)
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        self.state
            .visible_node(self.path.as_str())
            .ok_or(VfsError::NotFound)?
            .read_at(offset, buf)
    }

    fn write_at(&self, offset: u64, buf: &[u8]) -> VfsResult<usize> {
        let _guard = self.state.mutation.lock();
        self.state
            .ensure_copy_up_file_locked(self.path.as_str())?
            .write_at(offset, buf)
    }

    fn fsync(&self) -> VfsResult {
        if let Some(upper) = self.state.upper_node(self.path.as_str()) {
            upper.fsync()
        } else if self.state.lower_node(self.path.as_str()).is_some() {
            Ok(())
        } else {
            Err(VfsError::NotFound)
        }
    }

    fn truncate(&self, size: u64) -> VfsResult {
        let _guard = self.state.mutation.lock();
        if size == 0 {
            self.state
                .ensure_empty_upper_file_locked(self.path.as_str())?;
            Ok(())
        } else {
            self.state
                .ensure_copy_up_file_locked(self.path.as_str())?
                .truncate(size)
        }
    }

    fn parent(&self) -> Option<VfsNodeRef> {
        if self.path == "/" || !self.get_attr().ok()?.is_dir() {
            None
        } else {
            Some(self.new_at(parent_path(self.path.as_str()).to_string()))
        }
    }

    fn lookup(self: Arc<Self>, path: &str) -> VfsResult<VfsNodeRef> {
        let resolved = self.resolve(path);
        self.state
            .visible_node(resolved.as_str())
            .ok_or(VfsError::NotFound)?;
        Ok(self.new_at(resolved))
    }

    fn create(&self, path: &str, ty: VfsNodeType) -> VfsResult {
        if !matches!(ty, VfsNodeType::File | VfsNodeType::Dir) {
            return Err(VfsError::Unsupported);
        }
        let resolved = self.resolve(path);
        let _guard = self.state.mutation.lock();
        if self.state.visible_node(resolved.as_str()).is_some() {
            return Err(VfsError::AlreadyExists);
        }
        self.state
            .ensure_upper_dir_locked(parent_path(resolved.as_str()))?;
        self.state
            .upper_root
            .create(OverlayState::relative(resolved.as_str()), ty)
    }

    fn remove(&self, path: &str) -> VfsResult {
        let resolved = self.resolve(path);
        let _guard = self.state.mutation.lock();
        let node = self
            .state
            .visible_node(resolved.as_str())
            .ok_or(VfsError::NotFound)?;
        if node.get_attr()?.is_dir() && !self.state.merged_entries(resolved.as_str())?.is_empty() {
            return Err(VfsError::DirectoryNotEmpty);
        }

        let has_lower = self.state.lower_node_raw(resolved.as_str()).is_some();
        if self.state.upper_node(resolved.as_str()).is_some() {
            self.state
                .upper_root
                .remove(OverlayState::relative(resolved.as_str()))?;
        }
        if has_lower {
            self.state.add_whiteout(resolved.as_str());
        }
        Ok(())
    }

    fn read_dir(&self, start_idx: usize, dirents: &mut [VfsDirEntry]) -> VfsResult<usize> {
        let entries = self.state.merged_entries(self.path.as_str())?;
        let mut iter = entries.iter().skip(start_idx.saturating_sub(2));
        for (index, out) in dirents.iter_mut().enumerate() {
            match start_idx + index {
                0 => {
                    *out = VfsDirEntry::new(".", VfsNodeType::Dir);
                    continue;
                }
                1 => {
                    *out = VfsDirEntry::new("..", VfsNodeType::Dir);
                    continue;
                }
                _ => {}
            }
            let Some((name, ty)) = iter.next() else {
                return Ok(index);
            };
            *out = VfsDirEntry::new(name, *ty);
        }
        Ok(dirents.len())
    }

    fn rename(&self, src_path: &str, dst_path: &str) -> VfsResult {
        let src = self.resolve(src_path);
        let dst = self.resolve(dst_path);
        if src == dst {
            return Ok(());
        }

        let _guard = self.state.mutation.lock();
        let src_attr = self
            .state
            .visible_node(src.as_str())
            .ok_or(VfsError::NotFound)?
            .get_attr()?;
        if let Some(dst_node) = self.state.visible_node(dst.as_str()) {
            let dst_attr = dst_node.get_attr()?;
            match (src_attr.is_dir(), dst_attr.is_dir()) {
                (true, true) => {
                    if !self.state.merged_entries(dst.as_str())?.is_empty() {
                        return Err(VfsError::DirectoryNotEmpty);
                    }
                }
                (true, false) => return Err(VfsError::NotADirectory),
                (false, true) => return Err(VfsError::IsADirectory),
                (false, false) => {}
            }
        }
        self.state.ensure_copy_up_tree_locked(src.as_str())?;
        self.state
            .ensure_upper_dir_locked(parent_path(dst.as_str()))?;
        let src_has_lower = self.state.lower_node_raw(src.as_str()).is_some();
        let dst_has_lower = self.state.lower_node_raw(dst.as_str()).is_some();
        self.state.upper_root.rename(
            OverlayState::relative(src.as_str()),
            OverlayState::relative(dst.as_str()),
        )?;
        if src_has_lower {
            self.state.add_whiteout(src.as_str());
        }
        if dst_has_lower {
            self.state.add_whiteout(dst.as_str());
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

fn parent_path(path: &str) -> &str {
    if path == "/" {
        return "/";
    }
    path.rsplit_once('/')
        .map(|(parent, _)| if parent.is_empty() { "/" } else { parent })
        .unwrap_or("/")
}

fn join_path(base: &str, name: &str) -> String {
    axfs_vfs::path::canonicalize(
        if base == "/" {
            format!("/{name}")
        } else {
            format!("{base}/{name}")
        }
        .as_str(),
    )
}

fn collect_dir_entries(
    dir: &VfsNodeRef,
    mut consume: impl FnMut(String, VfsNodeType),
) -> VfsResult {
    let mut start = 0;
    loop {
        let mut batch: [VfsDirEntry; 16] = core::array::from_fn(|_| VfsDirEntry::default());
        let count = dir.read_dir(start, &mut batch)?;
        for entry in &batch[..count] {
            let Ok(name) = core::str::from_utf8(entry.name_as_bytes()) else {
                return Err(VfsError::InvalidData);
            };
            if name != "." && name != ".." {
                consume(name.to_string(), entry.entry_type());
            }
        }
        start += count;
        if count < batch.len() {
            return Ok(());
        }
    }
}
