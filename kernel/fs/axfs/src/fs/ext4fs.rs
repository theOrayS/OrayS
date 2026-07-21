use alloc::{
    boxed::Box,
    collections::{BTreeMap, VecDeque},
    format,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use axfs_vfs::{
    VfsDirEntry, VfsError, VfsNodeAttr, VfsNodeOps, VfsNodePerm, VfsNodeRef, VfsNodeType, VfsOps,
    VfsResult,
};
use axsync::Mutex;
use core::{cell::UnsafeCell, cmp, error::Error, fmt};
use ext4_view::{Ext4, Ext4Error, Ext4Read, FileType, Metadata};

use crate::dev::Disk;

const BLOCK_SIZE: u64 = 512;
const EXT4_METADATA_CACHE_CAP: usize = 1024;
const EXT4_READ_OBSERVED_CAP: usize = 1024;
const EXT4_READ_CACHE_MAX_ENTRIES: usize = 96;
const EXT4_READ_CACHE_MAX_FILE_BYTES: usize = 4 * 1024 * 1024;
const EXT4_READ_CACHE_TOTAL_BYTES: usize = 32 * 1024 * 1024;
const EXT4_DIR_OBSERVED_CAP: usize = 256;
const EXT4_DIR_CACHE_MAX_DIRS: usize = 64;
const EXT4_DIR_CACHE_MAX_ENTRIES_PER_DIR: usize = 2048;

pub struct Ext4FileSystem {
    root_dir: Arc<Ext4DirNode>,
}

struct LockedExt4 {
    lock: Mutex<()>,
    inner: UnsafeCell<Ext4>,
    metadata_cache: Mutex<Ext4MetadataCache>,
    read_cache: Mutex<Ext4ReadCache>,
    dir_cache: Mutex<Ext4DirCache>,
}

struct Ext4Disk(Disk);

#[derive(Debug)]
struct Ext4DiskReadError;

struct Ext4MetadataCache {
    entries: BTreeMap<String, Metadata>,
    lru: VecDeque<String>,
}

struct Ext4ReadCache {
    entries: BTreeMap<String, Arc<[u8]>>,
    lru: VecDeque<String>,
    observed_reads: BTreeMap<String, u8>,
    reserved_fills: BTreeMap<String, usize>,
    total_bytes: usize,
    reserved_bytes: usize,
}

struct Ext4DirCache {
    entries: BTreeMap<String, Arc<[CachedExt4DirEntry]>>,
    lru: VecDeque<String>,
    observed_reads: BTreeMap<String, u8>,
    uncacheable: BTreeMap<String, ()>,
}

struct CachedExt4DirEntry {
    name: String,
    ty: VfsNodeType,
}

struct Ext4FileNode {
    fs: Arc<LockedExt4>,
    path: String,
}

struct Ext4DirNode {
    fs: Arc<LockedExt4>,
    path: String,
}

impl Ext4FileSystem {
    pub fn new(disk: Disk) -> Self {
        let ext4 =
            Ext4::load(Box::new(Ext4Disk(disk))).expect("failed to initialize ext4 filesystem");
        let fs = Arc::new(LockedExt4::new(ext4));
        Self {
            root_dir: Arc::new(Ext4DirNode::new(fs, "/".into())),
        }
    }
}

impl LockedExt4 {
    fn new(inner: Ext4) -> Self {
        Self {
            lock: Mutex::new(()),
            inner: UnsafeCell::new(inner),
            metadata_cache: Mutex::new(Ext4MetadataCache::new()),
            read_cache: Mutex::new(Ext4ReadCache::new()),
            dir_cache: Mutex::new(Ext4DirCache::new()),
        }
    }

    fn with<R>(&self, f: impl FnOnce(&Ext4) -> VfsResult<R>) -> VfsResult<R> {
        let _guard = self.lock.lock();
        // SAFETY: all access goes through this method and is serialized by `lock`.
        let fs = unsafe { &*self.inner.get() };
        f(fs)
    }

    fn metadata(&self, path: &str) -> VfsResult<Metadata> {
        if let Some(metadata) = self.metadata_cache.lock().get(path) {
            return Ok(metadata);
        }

        let metadata = self.with(|fs| fs.metadata(path).map_err(as_vfs_err))?;
        self.metadata_cache.lock().insert(path, metadata.clone());
        Ok(metadata)
    }

    fn symlink_metadata(&self, path: &str) -> VfsResult<Metadata> {
        self.with(|fs| fs.symlink_metadata(path).map_err(as_vfs_err))
    }

    fn read_link(&self, path: &str) -> VfsResult<String> {
        self.with(|fs| {
            fs.read_link(path)
                .map(|target| target.display().to_string())
                .map_err(as_vfs_err)
        })
    }

    fn read_at(&self, path: &str, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        if !buf.is_empty()
            && let Some(data) = self.read_cache.lock().get(path)
        {
            return Ok(copy_cached_data(&data, offset, buf));
        }

        let metadata = self.metadata(path)?;
        if !metadata.file_type().is_regular_file() {
            return self.read_at_uncached(path, offset, buf);
        }
        if buf.is_empty() {
            return Ok(0);
        }
        if offset >= metadata.len() {
            return Ok(0);
        }

        let cacheable_len = usize::try_from(metadata.len())
            .ok()
            .filter(|len| *len <= EXT4_READ_CACHE_MAX_FILE_BYTES);
        if let Some(file_len) = cacheable_len
            && self.read_cache.lock().start_cache_fill(path, file_len)
        {
            match self.with(|fs| fs.read(path).map_err(as_vfs_err)) {
                Ok(data) => {
                    let data = Arc::<[u8]>::from(data.into_boxed_slice());
                    self.read_cache.lock().finish_cache_fill(path, data.clone());
                    return Ok(copy_cached_data(&data, offset, buf));
                }
                Err(_) => {
                    self.read_cache.lock().abort_cache_fill(path);
                }
            }
        }

        self.read_at_uncached(path, offset, buf)
    }

    fn read_at_uncached(&self, path: &str, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        self.with(|fs| {
            let mut file = fs.open(path).map_err(as_vfs_err)?;
            file.seek_to(offset).map_err(as_vfs_err)?;
            file.read_bytes(buf).map_err(as_vfs_err)
        })
    }

    fn read_dir(
        &self,
        path: &str,
        start_idx: usize,
        dirents: &mut [VfsDirEntry],
    ) -> VfsResult<usize> {
        if dirents.is_empty() {
            return Ok(0);
        }

        if let Some(entries) = self.dir_cache.lock().get(path) {
            return Ok(copy_cached_dir_entries(&entries, start_idx, dirents));
        }

        if self.dir_cache.lock().note_cacheable_read(path) {
            match self.read_dir_all_cacheable(path) {
                Ok(Some(entries)) => {
                    let entries = Arc::<[CachedExt4DirEntry]>::from(entries.into_boxed_slice());
                    self.dir_cache.lock().insert(path, entries.clone());
                    return Ok(copy_cached_dir_entries(&entries, start_idx, dirents));
                }
                Ok(None) => self.dir_cache.lock().mark_uncacheable(path),
                Err(_) => {}
            }
        }

        self.read_dir_uncached(path, start_idx, dirents)
    }

    fn read_dir_all_cacheable(&self, path: &str) -> VfsResult<Option<Vec<CachedExt4DirEntry>>> {
        self.with(|fs| {
            let iter = fs.read_dir(path).map_err(as_vfs_err)?;
            let mut entries = Vec::new();
            for entry in iter {
                let entry = entry.map_err(as_vfs_err)?;
                if entries.len() >= EXT4_DIR_CACHE_MAX_ENTRIES_PER_DIR {
                    return Ok(None);
                }
                entries.push(CachedExt4DirEntry {
                    name: entry.file_name().display().to_string(),
                    ty: map_file_type(entry.file_type().map_err(as_vfs_err)?),
                });
            }
            Ok(Some(entries))
        })
    }

    fn read_dir_uncached(
        &self,
        path: &str,
        start_idx: usize,
        dirents: &mut [VfsDirEntry],
    ) -> VfsResult<usize> {
        self.with(|fs| {
            let mut iter = fs.read_dir(path).map_err(as_vfs_err)?;
            for _ in 0..start_idx {
                match iter.next().transpose().map_err(as_vfs_err)? {
                    Some(_) => {}
                    None => return Ok(0),
                }
            }

            for (i, out_entry) in dirents.iter_mut().enumerate() {
                match iter.next().transpose().map_err(as_vfs_err)? {
                    Some(entry) => {
                        let name = entry.file_name().display().to_string();
                        let ty = map_file_type(entry.file_type().map_err(as_vfs_err)?);
                        *out_entry = VfsDirEntry::new(&name, ty);
                    }
                    None => return Ok(i),
                }
            }
            Ok(dirents.len())
        })
    }
}

// SAFETY: the inner `Ext4` object is only accessed while holding `lock`.
unsafe impl Send for LockedExt4 {}
// SAFETY: the inner `Ext4` object is only accessed while holding `lock`.
unsafe impl Sync for LockedExt4 {}

impl fmt::Display for Ext4DiskReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to read from ext4 disk")
    }
}

impl Error for Ext4DiskReadError {}

impl Ext4MetadataCache {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            lru: VecDeque::new(),
        }
    }

    fn get(&mut self, path: &str) -> Option<Metadata> {
        let metadata = self.entries.get(path).cloned()?;
        self.touch(path);
        Some(metadata)
    }

    fn insert(&mut self, path: &str, metadata: Metadata) {
        if self.entries.contains_key(path) {
            self.entries.insert(path.into(), metadata);
            self.touch(path);
            return;
        }

        while self.entries.len() >= EXT4_METADATA_CACHE_CAP {
            let Some(evicted) = self.lru.pop_back() else {
                break;
            };
            self.entries.remove(evicted.as_str());
        }

        let path = path.to_string();
        self.entries.insert(path.clone(), metadata);
        self.lru.push_front(path);
    }

    fn touch(&mut self, path: &str) {
        if let Some(pos) = self.lru.iter().position(|entry| entry.as_str() == path) {
            let entry = self.lru.remove(pos).expect("LRU position exists");
            self.lru.push_front(entry);
        }
    }
}

impl Ext4ReadCache {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            lru: VecDeque::new(),
            observed_reads: BTreeMap::new(),
            reserved_fills: BTreeMap::new(),
            total_bytes: 0,
            reserved_bytes: 0,
        }
    }

    fn get(&mut self, path: &str) -> Option<Arc<[u8]>> {
        let data = self.entries.get(path).cloned()?;
        self.touch(path);
        Some(data)
    }

    fn start_cache_fill(&mut self, path: &str, len: usize) -> bool {
        if !self.observed_reads.contains_key(path)
            && self.observed_reads.len() >= EXT4_READ_OBSERVED_CAP
        {
            self.observed_reads.clear();
        }
        let reads = self.observed_reads.entry(path.to_string()).or_insert(0);
        let should_fill = *reads > 0;
        *reads = reads.saturating_add(1).min(2);
        if !should_fill {
            return false;
        }
        if self.entries.contains_key(path)
            || self.reserved_fills.contains_key(path)
            || len > EXT4_READ_CACHE_MAX_FILE_BYTES
            || len > EXT4_READ_CACHE_TOTAL_BYTES
        {
            return false;
        }

        self.evict_until_room(len);
        if self
            .total_bytes
            .saturating_add(self.reserved_bytes)
            .saturating_add(len)
            > EXT4_READ_CACHE_TOTAL_BYTES
        {
            return false;
        }

        self.reserved_fills.insert(path.to_string(), len);
        self.reserved_bytes = self.reserved_bytes.saturating_add(len);
        self.observed_reads.remove(path);
        true
    }

    fn finish_cache_fill(&mut self, path: &str, data: Arc<[u8]>) {
        self.release_reserved_fill(path);
        self.insert(path, data);
    }

    fn abort_cache_fill(&mut self, path: &str) {
        self.release_reserved_fill(path);
    }

    fn release_reserved_fill(&mut self, path: &str) {
        if let Some(len) = self.reserved_fills.remove(path) {
            self.reserved_bytes = self.reserved_bytes.saturating_sub(len);
        }
    }

    fn insert(&mut self, path: &str, data: Arc<[u8]>) {
        let len = data.len();
        if len > EXT4_READ_CACHE_MAX_FILE_BYTES || len > EXT4_READ_CACHE_TOTAL_BYTES {
            return;
        }

        if let Some(old) = self.entries.remove(path) {
            self.total_bytes = self.total_bytes.saturating_sub(old.len());
            self.remove_lru(path);
        }

        self.evict_until_room(len);

        if self
            .total_bytes
            .saturating_add(self.reserved_bytes)
            .saturating_add(len)
            > EXT4_READ_CACHE_TOTAL_BYTES
        {
            return;
        }

        let path = path.to_string();
        self.total_bytes += len;
        self.entries.insert(path.clone(), data);
        self.observed_reads.remove(path.as_str());
        self.lru.push_front(path);
    }

    fn touch(&mut self, path: &str) {
        if let Some(pos) = self.lru.iter().position(|entry| entry.as_str() == path) {
            let entry = self.lru.remove(pos).expect("LRU position exists");
            self.lru.push_front(entry);
        }
    }

    fn remove_lru(&mut self, path: &str) {
        if let Some(pos) = self.lru.iter().position(|entry| entry.as_str() == path) {
            self.lru.remove(pos);
        }
    }

    fn evict_until_room(&mut self, len: usize) {
        while !self.entries.is_empty()
            && (self.entries.len() >= EXT4_READ_CACHE_MAX_ENTRIES
                || self
                    .total_bytes
                    .saturating_add(self.reserved_bytes)
                    .saturating_add(len)
                    > EXT4_READ_CACHE_TOTAL_BYTES)
        {
            let Some(evicted) = self.lru.pop_back() else {
                break;
            };
            if let Some(old) = self.entries.remove(evicted.as_str()) {
                self.total_bytes = self.total_bytes.saturating_sub(old.len());
            }
        }
    }
}

impl Ext4DirCache {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            lru: VecDeque::new(),
            observed_reads: BTreeMap::new(),
            uncacheable: BTreeMap::new(),
        }
    }

    fn get(&mut self, path: &str) -> Option<Arc<[CachedExt4DirEntry]>> {
        let entries = self.entries.get(path).cloned()?;
        self.touch(path);
        Some(entries)
    }

    fn note_cacheable_read(&mut self, path: &str) -> bool {
        if self.uncacheable.contains_key(path) {
            return false;
        }
        if !self.observed_reads.contains_key(path)
            && self.observed_reads.len() >= EXT4_DIR_OBSERVED_CAP
        {
            self.observed_reads.clear();
        }
        let reads = self.observed_reads.entry(path.to_string()).or_insert(0);
        let should_fill = *reads > 0;
        *reads = reads.saturating_add(1).min(2);
        should_fill
    }

    fn mark_uncacheable(&mut self, path: &str) {
        if self.uncacheable.contains_key(path) {
            return;
        }
        if self.uncacheable.len() >= EXT4_DIR_OBSERVED_CAP {
            self.uncacheable.clear();
        }
        self.observed_reads.remove(path);
        self.uncacheable.insert(path.to_string(), ());
    }

    fn insert(&mut self, path: &str, entries: Arc<[CachedExt4DirEntry]>) {
        if entries.len() > EXT4_DIR_CACHE_MAX_ENTRIES_PER_DIR {
            return;
        }

        if self.entries.contains_key(path) {
            self.entries.insert(path.into(), entries);
            self.touch(path);
            return;
        }

        while self.entries.len() >= EXT4_DIR_CACHE_MAX_DIRS {
            let Some(evicted) = self.lru.pop_back() else {
                break;
            };
            self.entries.remove(evicted.as_str());
        }

        let path = path.to_string();
        self.entries.insert(path.clone(), entries);
        self.observed_reads.remove(path.as_str());
        self.uncacheable.remove(path.as_str());
        self.lru.push_front(path);
    }

    fn touch(&mut self, path: &str) {
        if let Some(pos) = self.lru.iter().position(|entry| entry.as_str() == path) {
            let entry = self.lru.remove(pos).expect("LRU position exists");
            self.lru.push_front(entry);
        }
    }
}

fn copy_cached_data(data: &[u8], offset: u64, buf: &mut [u8]) -> usize {
    let Ok(start) = usize::try_from(offset) else {
        return 0;
    };
    if start >= data.len() {
        return 0;
    }
    let read_len = cmp::min(buf.len(), data.len() - start);
    buf[..read_len].copy_from_slice(&data[start..start + read_len]);
    read_len
}

fn copy_cached_dir_entries(
    entries: &[CachedExt4DirEntry],
    start_idx: usize,
    dirents: &mut [VfsDirEntry],
) -> usize {
    if start_idx >= entries.len() {
        return 0;
    }
    let read_len = cmp::min(dirents.len(), entries.len() - start_idx);
    for (out, entry) in dirents
        .iter_mut()
        .zip(entries[start_idx..start_idx + read_len].iter())
    {
        *out = VfsDirEntry::new(entry.name.as_str(), entry.ty);
    }
    read_len
}

impl Ext4Read for Ext4Disk {
    fn read(
        &mut self,
        start_byte: u64,
        dst: &mut [u8],
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.0.set_position(start_byte);
        let mut filled = 0;
        while filled < dst.len() {
            let read = self
                .0
                .read_one(&mut dst[filled..])
                .map_err(|_| Box::new(Ext4DiskReadError) as Box<dyn Error + Send + Sync>)?;
            if read == 0 {
                return Err(Box::new(Ext4DiskReadError));
            }
            filled += read;
        }
        Ok(())
    }
}

impl Ext4FileNode {
    fn new(fs: Arc<LockedExt4>, path: String) -> Self {
        Self { fs, path }
    }

    fn metadata(&self) -> VfsResult<Metadata> {
        self.fs.metadata(self.path.as_str())
    }
}

impl Ext4DirNode {
    fn new(fs: Arc<LockedExt4>, path: String) -> Self {
        Self { fs, path }
    }

    fn metadata(&self) -> VfsResult<Metadata> {
        self.fs.metadata(self.path.as_str())
    }

    fn child_path(&self, path: &str) -> String {
        if path.starts_with('/') {
            axfs_vfs::path::canonicalize(path)
        } else if self.path == "/" {
            axfs_vfs::path::canonicalize(&format!("/{}", path))
        } else {
            axfs_vfs::path::canonicalize(&format!("{}/{}", self.path, path))
        }
    }

    fn parent_path(&self) -> Option<String> {
        if self.path == "/" {
            None
        } else {
            Some(axfs_vfs::path::canonicalize(&(self.path.clone() + "/..")))
        }
    }
}

impl VfsNodeOps for Ext4FileNode {
    axfs_vfs::impl_vfs_non_dir_default! {}

    fn get_attr(&self) -> VfsResult<VfsNodeAttr> {
        let metadata = self.metadata()?;
        Ok(vfs_attr_from_metadata(&metadata))
    }

    fn get_link_attr(&self) -> VfsResult<VfsNodeAttr> {
        let metadata = self.fs.symlink_metadata(self.path.as_str())?;
        Ok(vfs_attr_from_metadata(&metadata))
    }

    fn read_link(&self) -> VfsResult<String> {
        self.fs.read_link(self.path.as_str())
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        self.fs.read_at(self.path.as_str(), offset, buf)
    }

    fn write_at(&self, _offset: u64, _buf: &[u8]) -> VfsResult<usize> {
        Err(VfsError::ReadOnlyFilesystem)
    }

    fn fsync(&self) -> VfsResult {
        Err(VfsError::ReadOnlyFilesystem)
    }

    fn truncate(&self, _size: u64) -> VfsResult {
        Err(VfsError::ReadOnlyFilesystem)
    }
}

impl VfsNodeOps for Ext4DirNode {
    axfs_vfs::impl_vfs_dir_default! {}

    fn get_attr(&self) -> VfsResult<VfsNodeAttr> {
        let metadata = self.metadata()?;
        Ok(vfs_attr_from_metadata(&metadata))
    }

    fn get_link_attr(&self) -> VfsResult<VfsNodeAttr> {
        let metadata = self.fs.symlink_metadata(self.path.as_str())?;
        Ok(vfs_attr_from_metadata(&metadata))
    }

    fn read_link(&self) -> VfsResult<String> {
        self.fs.read_link(self.path.as_str())
    }

    fn parent(&self) -> Option<VfsNodeRef> {
        self.parent_path()
            .map(|path| Arc::new(Self::new(self.fs.clone(), path)) as VfsNodeRef)
    }

    fn lookup(self: Arc<Self>, path: &str) -> VfsResult<VfsNodeRef> {
        let path = path.trim_matches('/');
        if path.is_empty() || path == "." {
            return Ok(self);
        }
        if let Some(rest) = path.strip_prefix("./") {
            return self.lookup(rest);
        }

        let path = self.child_path(path);
        let file_type = map_file_type(self.fs.metadata(path.as_str())?.file_type());
        if file_type.is_dir() {
            Ok(Arc::new(Self::new(self.fs.clone(), path)))
        } else {
            Ok(Arc::new(Ext4FileNode::new(self.fs.clone(), path)))
        }
    }

    fn create(&self, _path: &str, _ty: VfsNodeType) -> VfsResult {
        Err(VfsError::ReadOnlyFilesystem)
    }

    fn remove(&self, _path: &str) -> VfsResult {
        Err(VfsError::ReadOnlyFilesystem)
    }

    fn read_dir(&self, start_idx: usize, dirents: &mut [VfsDirEntry]) -> VfsResult<usize> {
        self.fs.read_dir(self.path.as_str(), start_idx, dirents)
    }

    fn rename(&self, _src_path: &str, _dst_path: &str) -> VfsResult {
        Err(VfsError::ReadOnlyFilesystem)
    }
}

impl VfsOps for Ext4FileSystem {
    fn root_dir(&self) -> VfsNodeRef {
        self.root_dir.clone()
    }
}

fn vfs_attr_from_metadata(metadata: &ext4_view::Metadata) -> VfsNodeAttr {
    VfsNodeAttr::new(
        VfsNodePerm::from_bits_truncate(metadata.mode()),
        map_file_type(metadata.file_type()),
        metadata.len(),
        metadata.len().div_ceil(BLOCK_SIZE),
    )
}

fn map_file_type(file_type: FileType) -> VfsNodeType {
    match file_type {
        FileType::BlockDevice => VfsNodeType::BlockDevice,
        FileType::CharacterDevice => VfsNodeType::CharDevice,
        FileType::Directory => VfsNodeType::Dir,
        FileType::Fifo => VfsNodeType::Fifo,
        FileType::Regular => VfsNodeType::File,
        FileType::Socket => VfsNodeType::Socket,
        FileType::Symlink => VfsNodeType::SymLink,
    }
}

fn as_vfs_err(err: Ext4Error) -> VfsError {
    match err {
        Ext4Error::NotFound => VfsError::NotFound,
        Ext4Error::IsADirectory => VfsError::IsADirectory,
        Ext4Error::NotADirectory => VfsError::NotADirectory,
        Ext4Error::Encrypted => VfsError::PermissionDenied,
        Ext4Error::NotUtf8 => VfsError::InvalidData,
        Ext4Error::Io(_) => VfsError::Io,
        Ext4Error::Incompatible(_) | Ext4Error::Corrupt(_) | Ext4Error::FileTooLarge => {
            VfsError::InvalidData
        }
        Ext4Error::NotAbsolute
        | Ext4Error::NotASymlink
        | Ext4Error::IsASpecialFile
        | Ext4Error::MalformedPath
        | Ext4Error::PathTooLong
        | Ext4Error::TooManySymlinks => VfsError::InvalidInput,
        _ => VfsError::InvalidData,
    }
}
