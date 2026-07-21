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
    sync::{Arc, Weak},
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
    nodes: Mutex<NodeCache>,
    mutation: Mutex<()>,
}

struct NodeCache {
    entries: BTreeMap<String, Weak<OverlayNode>>,
    sweep_at: usize,
}

struct OverlayNode {
    state: Arc<OverlayState>,
    location: Mutex<NodeLocation>,
}

struct NodeLocation {
    path: String,
    backing: VfsNodeRef,
    upper: bool,
    linked: bool,
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
            nodes: Mutex::new(NodeCache {
                entries: BTreeMap::new(),
                sweep_at: 4096,
            }),
            mutation: Mutex::new(()),
        });
        let root = Arc::new(OverlayNode {
            state: state.clone(),
            location: Mutex::new(NodeLocation {
                path: "/".into(),
                backing: state.upper_root.clone(),
                upper: true,
                linked: true,
            }),
        });
        state
            .nodes
            .lock()
            .entries
            .insert("/".into(), Arc::downgrade(&root));
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
        self.visible_binding(path).map(|(node, _)| node)
    }

    fn visible_binding(&self, path: &str) -> Option<(VfsNodeRef, bool)> {
        self.upper_node(path)
            .map(|node| (node, true))
            .or_else(|| self.lower_node(path).map(|node| (node, false)))
    }

    fn node_at_locked(self: &Arc<Self>, path: String) -> VfsResult<VfsNodeRef> {
        let mut cache = self.nodes.lock();
        if let Some(node) = cache.entries.get(path.as_str()).and_then(Weak::upgrade) {
            return Ok(node);
        }
        cache.entries.remove(path.as_str());

        if cache.entries.len() >= cache.sweep_at {
            cache.entries.retain(|_, node| node.strong_count() > 0);
            cache.sweep_at = cache.entries.len().saturating_mul(2).max(4096);
        }

        let (backing, upper) = self
            .visible_binding(path.as_str())
            .ok_or(VfsError::NotFound)?;
        let node = Arc::new(OverlayNode {
            state: self.clone(),
            location: Mutex::new(NodeLocation {
                path: path.clone(),
                backing,
                upper,
                linked: true,
            }),
        });
        cache.entries.insert(path, Arc::downgrade(&node));
        Ok(node)
    }

    fn cached_node(&self, path: &str) -> Option<Arc<OverlayNode>> {
        self.nodes.lock().entries.get(path).and_then(Weak::upgrade)
    }

    fn update_cached_binding(&self, path: &str, backing: VfsNodeRef, upper: bool) {
        let Some(node) = self.cached_node(path) else {
            return;
        };
        let mut location = node.location.lock();
        if location.linked && location.path == path {
            location.backing = backing;
            location.upper = upper;
        }
    }

    fn detach_cached_tree(&self, path: &str) {
        let prefix = format!("{path}/");
        let mut cache = self.nodes.lock();
        let paths: Vec<_> = cache
            .entries
            .keys()
            .filter(|entry| entry.as_str() == path || entry.starts_with(prefix.as_str()))
            .cloned()
            .collect();
        for old_path in paths {
            if let Some(node) = cache
                .entries
                .remove(old_path.as_str())
                .and_then(|node| node.upgrade())
            {
                node.location.lock().linked = false;
            }
        }
    }

    fn move_cached_tree(&self, src: &str, dst: &str) {
        let prefix = format!("{src}/");
        let mut cache = self.nodes.lock();
        let paths: Vec<_> = cache
            .entries
            .keys()
            .filter(|entry| entry.as_str() == src || entry.starts_with(prefix.as_str()))
            .cloned()
            .collect();
        for old_path in paths {
            let Some(node) = cache
                .entries
                .remove(old_path.as_str())
                .and_then(|node| node.upgrade())
            else {
                continue;
            };
            let new_path = if old_path == src {
                dst.to_string()
            } else {
                format!("{dst}{}", &old_path[src.len()..])
            };
            {
                let mut location = node.location.lock();
                location.path = new_path.clone();
                location.linked = true;
            }
            cache.entries.insert(new_path, Arc::downgrade(&node));
        }
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
                self.update_cached_binding(current.as_str(), node, true);
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
            self.update_cached_binding(current.as_str(), upper, true);
        }
        let upper = self.upper_node(path).ok_or(VfsError::NotFound)?;
        self.update_cached_binding(path, upper.clone(), true);
        Ok(upper)
    }

    fn ensure_copy_up_file_locked(&self, path: &str) -> VfsResult<VfsNodeRef> {
        if let Some(upper) = self.upper_node(path) {
            return if upper.get_attr()?.is_file() {
                self.update_cached_binding(path, upper.clone(), true);
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
        self.update_cached_binding(path, upper.clone(), true);
        Ok(upper)
    }

    fn ensure_empty_upper_file_locked(&self, path: &str) -> VfsResult<VfsNodeRef> {
        if let Some(upper) = self.upper_node(path) {
            if !upper.get_attr()?.is_file() {
                return Err(VfsError::IsADirectory);
            }
            upper.truncate(0)?;
            self.update_cached_binding(path, upper.clone(), true);
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
            Ok(()) | Err(VfsError::Unsupported) => {
                self.update_cached_binding(path, upper.clone(), true);
                Ok(upper)
            }
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
    fn snapshot(&self) -> (String, VfsNodeRef, bool, bool) {
        let location = self.location.lock();
        (
            location.path.clone(),
            location.backing.clone(),
            location.upper,
            location.linked,
        )
    }

    fn resolve(&self, path: &str) -> String {
        if path.starts_with('/') {
            axfs_vfs::path::canonicalize(path)
        } else {
            let location = self.location.lock();
            join_path(location.path.as_str(), path)
        }
    }
}

impl VfsNodeOps for OverlayNode {
    fn get_attr(&self) -> VfsResult<VfsNodeAttr> {
        self.location.lock().backing.get_attr()
    }

    fn set_perm(&self, perm: axfs_vfs::VfsNodePerm) -> VfsResult {
        let _guard = self.state.mutation.lock();
        let (path, backing, upper, linked) = self.snapshot();
        if upper {
            return backing.set_perm(perm);
        }
        if !linked {
            return Err(VfsError::NotFound);
        }
        let attr = backing.get_attr()?;
        let upper = if attr.is_dir() {
            self.state.ensure_upper_dir_locked(path.as_str())?
        } else if attr.is_file() {
            self.state.ensure_copy_up_file_locked(path.as_str())?
        } else {
            return Err(VfsError::Unsupported);
        };
        upper.set_perm(perm)
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        self.location.lock().backing.read_at(offset, buf)
    }

    fn write_at(&self, offset: u64, buf: &[u8]) -> VfsResult<usize> {
        let _guard = self.state.mutation.lock();
        let (path, backing, upper, linked) = self.snapshot();
        if upper {
            backing.write_at(offset, buf)
        } else if linked {
            self.state
                .ensure_copy_up_file_locked(path.as_str())?
                .write_at(offset, buf)
        } else {
            Err(VfsError::NotFound)
        }
    }

    fn fsync(&self) -> VfsResult {
        let (_, backing, upper, _) = self.snapshot();
        if upper { backing.fsync() } else { Ok(()) }
    }

    fn truncate(&self, size: u64) -> VfsResult {
        let _guard = self.state.mutation.lock();
        let (path, backing, upper, linked) = self.snapshot();
        if upper {
            return backing.truncate(size);
        }
        if !linked {
            return Err(VfsError::NotFound);
        }
        if size == 0 {
            self.state.ensure_empty_upper_file_locked(path.as_str())?;
            Ok(())
        } else {
            self.state
                .ensure_copy_up_file_locked(path.as_str())?
                .truncate(size)
        }
    }

    fn parent(&self) -> Option<VfsNodeRef> {
        let _guard = self.state.mutation.lock();
        let (path, backing, _, linked) = self.snapshot();
        if path == "/" || !linked || !backing.get_attr().ok()?.is_dir() {
            None
        } else {
            self.state
                .node_at_locked(parent_path(path.as_str()).to_string())
                .ok()
        }
    }

    fn lookup(self: Arc<Self>, path: &str) -> VfsResult<VfsNodeRef> {
        let _guard = self.state.mutation.lock();
        let (_, backing, _, linked) = self.snapshot();
        if !linked {
            return Err(VfsError::NotFound);
        }
        if !backing.get_attr()?.is_dir() {
            return Err(VfsError::NotADirectory);
        }
        let resolved = self.resolve(path);
        self.state.node_at_locked(resolved)
    }

    fn create(&self, path: &str, ty: VfsNodeType) -> VfsResult {
        if !matches!(ty, VfsNodeType::File | VfsNodeType::Dir) {
            return Err(VfsError::Unsupported);
        }
        let _guard = self.state.mutation.lock();
        let (_, backing, _, linked) = self.snapshot();
        if !linked {
            return Err(VfsError::NotFound);
        }
        if !backing.get_attr()?.is_dir() {
            return Err(VfsError::NotADirectory);
        }
        let resolved = self.resolve(path);
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
        let _guard = self.state.mutation.lock();
        let (_, backing, _, linked) = self.snapshot();
        if !linked {
            return Err(VfsError::NotFound);
        }
        if !backing.get_attr()?.is_dir() {
            return Err(VfsError::NotADirectory);
        }
        let resolved = self.resolve(path);
        let node = self
            .state
            .visible_node(resolved.as_str())
            .ok_or(VfsError::NotFound)?;
        if node.get_attr()?.is_dir() && !self.state.merged_entries(resolved.as_str())?.is_empty() {
            return Err(VfsError::DirectoryNotEmpty);
        }

        let has_lower = self.state.lower_node_raw(resolved.as_str()).is_some();
        if let Some(open_node) = self.state.cached_node(resolved.as_str()) {
            let (_, backing, upper, _) = open_node.snapshot();
            if !upper && backing.get_attr()?.is_file() {
                self.state.ensure_copy_up_file_locked(resolved.as_str())?;
            }
        }
        if self.state.upper_node(resolved.as_str()).is_some() {
            self.state
                .upper_root
                .remove(OverlayState::relative(resolved.as_str()))?;
        }
        if has_lower {
            self.state.add_whiteout(resolved.as_str());
        }
        self.state.detach_cached_tree(resolved.as_str());
        Ok(())
    }

    fn read_dir(&self, start_idx: usize, dirents: &mut [VfsDirEntry]) -> VfsResult<usize> {
        let _guard = self.state.mutation.lock();
        let (path, backing, _, linked) = self.snapshot();
        if !linked {
            return Err(VfsError::NotFound);
        }
        if !backing.get_attr()?.is_dir() {
            return Err(VfsError::NotADirectory);
        }
        let entries = self.state.merged_entries(path.as_str())?;
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
        let _guard = self.state.mutation.lock();
        let (_, backing, _, linked) = self.snapshot();
        if !linked {
            return Err(VfsError::NotFound);
        }
        if !backing.get_attr()?.is_dir() {
            return Err(VfsError::NotADirectory);
        }
        let src = self.resolve(src_path);
        let dst = self.resolve(dst_path);
        let src_attr = self
            .state
            .visible_node(src.as_str())
            .ok_or(VfsError::NotFound)?
            .get_attr()?;
        if src == dst {
            return Ok(());
        }

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
            if self.state.cached_node(dst.as_str()).is_some() {
                if dst_attr.is_file() {
                    self.state.ensure_copy_up_file_locked(dst.as_str())?;
                } else if dst_attr.is_dir() {
                    let upper = self.state.ensure_upper_dir_locked(dst.as_str())?;
                    match upper.set_perm(dst_attr.perm()) {
                        Ok(()) | Err(VfsError::Unsupported) => {}
                        Err(err) => return Err(err),
                    }
                }
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
        self.state.detach_cached_tree(dst.as_str());
        self.state.move_cached_tree(src.as_str(), dst.as_str());
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
