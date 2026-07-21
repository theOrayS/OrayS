//! [`std::fs`]-like high-level filesystem manipulation operations.

mod dir;
mod file;

pub use self::dir::{DirBuilder, DirEntry, ReadDir};
pub use self::file::{File, FileType, Metadata, OpenOptions, Permissions};

use alloc::{string::String, vec::Vec};
use axio::{self as io, prelude::*};

/// Mounts a FAT filesystem backed by a block device at an absolute path.
///
/// This is intentionally a low-level kernel VFS operation. Callers must pass a
/// real block-device implementation; compatibility layers should not use this
/// to alias a mount target to an unrelated existing directory.
pub fn mount_fatfs(
    path: &'static str,
    dev: axdriver::AxBlockDevice,
    format: bool,
) -> io::Result<()> {
    crate::root::mount_fatfs(path, dev, format).map_err(Into::into)
}

/// Unmounts a filesystem mounted through the kernel VFS mount table.
pub fn umount(path: &str) -> io::Result<()> {
    crate::root::umount(path).map_err(Into::into)
}

/// Returns an iterator over the entries within a directory.
pub fn read_dir(path: &str) -> io::Result<ReadDir<'_>> {
    ReadDir::new(path)
}

/// Returns the canonical, absolute form of a path with all intermediate
/// components normalized.
pub fn canonicalize(path: &str) -> io::Result<String> {
    crate::root::absolute_path(path)
}

/// Returns the current working directory as a [`String`].
pub fn current_dir() -> io::Result<String> {
    crate::root::current_dir()
}

/// Changes the current working directory to the specified path.
pub fn set_current_dir(path: &str) -> io::Result<()> {
    crate::root::set_current_dir(path)
}

/// Read the entire contents of a file into a bytes vector.
pub fn read(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

/// Read the entire contents of a file into a string.
pub fn read_to_string(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

/// Write a slice as the entire contents of a file.
pub fn write<C: AsRef<[u8]>>(path: &str, contents: C) -> io::Result<()> {
    File::create(path)?.write_all(contents.as_ref())
}

/// Given a path, query the file system to get information about a file,
/// directory, etc.
pub fn metadata(path: &str) -> io::Result<Metadata> {
    crate::root::lookup(None, path)?
        .get_attr()
        .map(Metadata::from_attr)
        .map_err(Into::into)
}

/// Query metadata without following a symbolic link in the final path
/// component.
pub fn symlink_metadata(path: &str) -> io::Result<Metadata> {
    let node = crate::root::lookup(None, path)?;
    let attr = if path.ends_with('/') {
        node.get_attr()
    } else {
        node.get_link_attr()
    };
    attr.map(Metadata::from_attr).map_err(Into::into)
}

/// Read the target stored in a symbolic link.
pub fn read_link(path: &str) -> io::Result<String> {
    crate::root::lookup(None, path)?
        .read_link()
        .map_err(Into::into)
}

/// Creates a new, empty directory at the provided path.
pub fn create_dir(path: &str) -> io::Result<()> {
    DirBuilder::new().create(path)
}

/// Recursively create a directory and all of its parent components if they
/// are missing.
pub fn create_dir_all(path: &str) -> io::Result<()> {
    DirBuilder::new().recursive(true).create(path)
}

/// Removes an empty directory.
pub fn remove_dir(path: &str) -> io::Result<()> {
    crate::root::remove_dir(None, path)
}

/// Removes a file from the filesystem.
pub fn remove_file(path: &str) -> io::Result<()> {
    crate::root::remove_file(None, path)
}

/// Rename a file or directory to a new name.
/// Delete the original file if `old` already exists.
///
/// This only works then the new path is in the same mounted fs.
pub fn rename(old: &str, new: &str) -> io::Result<()> {
    crate::root::rename(old, new)
}
