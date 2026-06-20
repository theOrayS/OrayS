use alloc::vec::Vec;
use axfs_vfs::{
    impl_vfs_non_dir_default, VfsError, VfsNodeAttr, VfsNodeOps, VfsNodePerm, VfsNodeType,
    VfsResult,
};
use spin::RwLock;

// The evaluator mounts /tmp and /var as ramfs. Keep a per-file ceiling so
// runaway scratch data cannot starve later user processes, but allow regular
// syscall tests such as glibc write01 to create their expected ~32 MiB file.
const MAX_RAMFS_FILE_SIZE: usize = 64 * 1024 * 1024;

/// The file node in the RAM filesystem.
///
/// It implements [`axfs_vfs::VfsNodeOps`].
pub struct FileNode {
    content: RwLock<Vec<u8>>,
    perm: RwLock<VfsNodePerm>,
}

impl FileNode {
    pub(super) const fn new() -> Self {
        Self::new_with_perm(VfsNodePerm::default_file())
    }

    pub(super) const fn new_with_perm(perm: VfsNodePerm) -> Self {
        Self {
            content: RwLock::new(Vec::new()),
            perm: RwLock::new(perm),
        }
    }
}

impl VfsNodeOps for FileNode {
    fn get_attr(&self) -> VfsResult<VfsNodeAttr> {
        Ok(VfsNodeAttr::new(
            *self.perm.read(),
            VfsNodeType::File,
            self.content.read().len() as _,
            0,
        ))
    }

    fn set_perm(&self, perm: VfsNodePerm) -> VfsResult {
        *self.perm.write() = perm;
        Ok(())
    }

    fn truncate(&self, size: u64) -> VfsResult {
        if size > MAX_RAMFS_FILE_SIZE as u64 {
            return Err(VfsError::StorageFull);
        }
        let mut content = self.content.write();
        if size < content.len() as u64 {
            content.truncate(size as _);
        } else {
            let additional = (size as usize).saturating_sub(content.len());
            content
                .try_reserve_exact(additional)
                .map_err(|_| VfsError::StorageFull)?;
            content.resize(size as _, 0);
        }
        Ok(())
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        let content = self.content.read();
        let start = content.len().min(offset as usize);
        let end = content.len().min(offset as usize + buf.len());
        let src = &content[start..end];
        buf[..src.len()].copy_from_slice(src);
        Ok(src.len())
    }

    fn write_at(&self, offset: u64, buf: &[u8]) -> VfsResult<usize> {
        let offset = offset as usize;
        let Some(end) = offset.checked_add(buf.len()) else {
            return Err(VfsError::StorageFull);
        };
        if end > MAX_RAMFS_FILE_SIZE {
            return Err(VfsError::StorageFull);
        }
        let mut content = self.content.write();
        if end > content.len() {
            let additional = end.saturating_sub(content.len());
            content
                .try_reserve_exact(additional)
                .map_err(|_| VfsError::StorageFull)?;
            content.resize(end, 0);
        }
        let dst = &mut content[offset..end];
        dst.copy_from_slice(&buf[..dst.len()]);
        Ok(buf.len())
    }

    fn fsync(&self) -> VfsResult {
        // ramfs is already memory-resident.  There is no lower block device to
        // flush, and all writes are visible once write_at returns.
        Ok(())
    }

    impl_vfs_non_dir_default! {}
}
