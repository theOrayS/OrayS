use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub const MAX_TEXT_BYTES: usize = 1024 * 1024;
const READ_CHUNK_BYTES: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsErrorKind {
    NotFound,
    AlreadyExists,
    PermissionDenied,
    InvalidInput,
    IsDirectory,
    NotDirectory,
    TooLarge,
    Unsupported,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsError {
    pub kind: FsErrorKind,
    pub operation: &'static str,
    pub path: String,
}

impl FsError {
    fn new(kind: FsErrorKind, operation: &'static str, path: &str) -> Self {
        Self {
            kind,
            operation,
            path: path.to_string(),
        }
    }

    pub fn invalid(operation: &'static str, path: &str) -> Self {
        Self::new(FsErrorKind::InvalidInput, operation, path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: u64,
}

pub fn join(base: &str, name: &str) -> Result<String, FsError> {
    if name.is_empty() || name == "." || name == ".." || name.bytes().any(|byte| byte == b'/') {
        return Err(FsError::new(FsErrorKind::InvalidInput, "join", name));
    }
    let base = if base.is_empty() { "/" } else { base };
    if base == "/" {
        Ok(alloc::format!("/{name}"))
    } else {
        Ok(alloc::format!("{}/{name}", base.trim_end_matches('/')))
    }
}

pub fn parent(path: &str) -> String {
    let trimmed = path.trim_end_matches('/');
    if trimmed.is_empty() || trimmed == "/" {
        return "/".to_string();
    }
    match trimmed.rfind('/') {
        Some(0) | None => "/".to_string(),
        Some(index) => trimmed[..index].to_string(),
    }
}

pub fn read_directory(path: &str) -> Result<Vec<DirectoryEntry>, FsError> {
    imp::read_directory(path)
}

pub fn read_bytes_bounded(path: &str, max_bytes: usize) -> Result<Vec<u8>, FsError> {
    imp::read_bytes_bounded(path, max_bytes)
}

pub fn read_text(path: &str) -> Result<String, FsError> {
    let bytes = read_bytes_bounded(path, MAX_TEXT_BYTES)?;
    String::from_utf8(bytes).map_err(|_| FsError::new(FsErrorKind::InvalidInput, "utf8", path))
}

#[cfg(any(feature = "orays", test, feature = "host-tools"))]
fn collect_bounded<F>(
    path: &str,
    max_bytes: usize,
    reported_size: u64,
    mut read: F,
) -> Result<Vec<u8>, FsError>
where
    F: FnMut(&mut [u8]) -> Result<usize, FsError>,
{
    let reported_size = usize::try_from(reported_size)
        .map_err(|_| FsError::new(FsErrorKind::TooLarge, "read", path))?;
    if reported_size > max_bytes {
        return Err(FsError::new(FsErrorKind::TooLarge, "read", path));
    }

    let mut bytes = Vec::with_capacity(reported_size);
    let mut chunk = [0u8; READ_CHUNK_BYTES];
    loop {
        let remaining = max_bytes.saturating_sub(bytes.len());
        // Probe one byte past the remaining budget so a file that grows after
        // the metadata check is still rejected without allocating past the
        // configured limit.
        let request = chunk.len().min(remaining.saturating_add(1));
        let count = read(&mut chunk[..request])?;
        if count == 0 {
            return Ok(bytes);
        }
        if count > remaining {
            return Err(FsError::new(FsErrorKind::TooLarge, "read", path));
        }
        bytes.extend_from_slice(&chunk[..count]);
    }
}

#[cfg(test)]
mod bounded_read_tests {
    use super::*;

    fn collect_fixture(source: &[u8], max_bytes: usize) -> Result<Vec<u8>, FsError> {
        let mut offset = 0usize;
        collect_bounded("fixture", max_bytes, 0, |buffer| {
            let count = buffer.len().min(source.len().saturating_sub(offset));
            buffer[..count].copy_from_slice(&source[offset..offset + count]);
            offset += count;
            Ok(count)
        })
    }

    #[test]
    fn bounded_reader_accepts_exact_limit() {
        assert_eq!(collect_fixture(b"1234", 4).unwrap(), b"1234");
    }

    #[test]
    fn bounded_reader_rejects_growth_past_reported_size() {
        assert_eq!(
            collect_fixture(b"12345", 4).unwrap_err().kind,
            FsErrorKind::TooLarge
        );
    }
}

pub fn write_text(path: &str, contents: &str) -> Result<(), FsError> {
    if contents.len() > MAX_TEXT_BYTES {
        return Err(FsError::new(FsErrorKind::TooLarge, "write", path));
    }
    imp::write(path, contents.as_bytes())
}

pub fn create_directory(path: &str) -> Result<(), FsError> {
    imp::create_directory(path)
}

pub fn rename(old: &str, new: &str) -> Result<(), FsError> {
    imp::rename(old, new)
}

pub fn remove(path: &str, is_directory: bool) -> Result<(), FsError> {
    imp::remove(path, is_directory)
}

#[cfg(feature = "orays")]
mod imp {
    use super::*;
    use axstd::io::prelude::*;

    fn map_error(operation: &'static str, path: &str, _error: axstd::io::Error) -> FsError {
        // axstd currently exposes a stable error display but not std::io::ErrorKind.
        // Keep the operation/path truthful and avoid guessing an errno class.
        FsError::new(FsErrorKind::Other, operation, path)
    }

    pub fn read_directory(path: &str) -> Result<Vec<DirectoryEntry>, FsError> {
        let iterator = axstd::fs::read_dir(path).map_err(|e| map_error("read_dir", path, e))?;
        let mut entries = Vec::new();
        for entry in iterator {
            let entry = entry.map_err(|e| map_error("read_dir_entry", path, e))?;
            let entry_path = entry.path();
            let metadata = axstd::fs::metadata(&entry_path)
                .map_err(|e| map_error("metadata", &entry_path, e))?;
            entries.push(DirectoryEntry {
                name: entry.file_name(),
                path: entry_path,
                is_directory: metadata.is_dir(),
                size: metadata.len(),
            });
        }
        entries.sort_by(|left, right| {
            right
                .is_directory
                .cmp(&left.is_directory)
                .then_with(|| left.name.cmp(&right.name))
        });
        Ok(entries)
    }

    pub fn read_bytes_bounded(path: &str, max_bytes: usize) -> Result<Vec<u8>, FsError> {
        let mut file = axstd::fs::File::open(path).map_err(|e| map_error("read", path, e))?;
        let reported_size = file
            .metadata()
            .map_err(|e| map_error("metadata", path, e))?
            .len();
        collect_bounded(path, max_bytes, reported_size, |buffer| {
            file.read(buffer).map_err(|e| map_error("read", path, e))
        })
    }

    pub fn write(path: &str, contents: &[u8]) -> Result<(), FsError> {
        axstd::fs::write(path, contents).map_err(|e| map_error("write", path, e))
    }

    pub fn create_directory(path: &str) -> Result<(), FsError> {
        axstd::fs::create_dir(path).map_err(|e| map_error("mkdir", path, e))
    }

    pub fn rename(old: &str, new: &str) -> Result<(), FsError> {
        axstd::fs::rename(old, new).map_err(|e| map_error("rename", old, e))
    }

    pub fn remove(path: &str, is_directory: bool) -> Result<(), FsError> {
        let result = if is_directory {
            axstd::fs::remove_dir(path)
        } else {
            axstd::fs::remove_file(path)
        };
        result.map_err(|e| map_error("remove", path, e))
    }
}

#[cfg(all(not(feature = "orays"), any(test, feature = "host-tools")))]
mod imp {
    use super::*;
    use std::io::{ErrorKind, Read};

    fn map_kind(kind: ErrorKind) -> FsErrorKind {
        match kind {
            ErrorKind::NotFound => FsErrorKind::NotFound,
            ErrorKind::AlreadyExists => FsErrorKind::AlreadyExists,
            ErrorKind::PermissionDenied => FsErrorKind::PermissionDenied,
            ErrorKind::InvalidInput | ErrorKind::InvalidData => FsErrorKind::InvalidInput,
            ErrorKind::IsADirectory => FsErrorKind::IsDirectory,
            ErrorKind::NotADirectory => FsErrorKind::NotDirectory,
            ErrorKind::Unsupported => FsErrorKind::Unsupported,
            _ => FsErrorKind::Other,
        }
    }

    fn map_error(operation: &'static str, path: &str, error: std::io::Error) -> FsError {
        FsError::new(map_kind(error.kind()), operation, path)
    }

    pub fn read_directory(path: &str) -> Result<Vec<DirectoryEntry>, FsError> {
        let iterator = std::fs::read_dir(path).map_err(|e| map_error("read_dir", path, e))?;
        let mut entries = Vec::new();
        for entry in iterator {
            let entry = entry.map_err(|e| map_error("read_dir_entry", path, e))?;
            let entry_path = entry.path();
            let display_path = entry_path.to_string_lossy().into_owned();
            let metadata = entry
                .metadata()
                .map_err(|e| map_error("metadata", &display_path, e))?;
            entries.push(DirectoryEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                path: display_path,
                is_directory: metadata.is_dir(),
                size: metadata.len(),
            });
        }
        entries.sort_by(|left, right| {
            right
                .is_directory
                .cmp(&left.is_directory)
                .then_with(|| left.name.cmp(&right.name))
        });
        Ok(entries)
    }

    pub fn read_bytes_bounded(path: &str, max_bytes: usize) -> Result<Vec<u8>, FsError> {
        let mut file = std::fs::File::open(path).map_err(|e| map_error("read", path, e))?;
        let reported_size = file
            .metadata()
            .map_err(|e| map_error("metadata", path, e))?
            .len();
        collect_bounded(path, max_bytes, reported_size, |buffer| {
            file.read(buffer).map_err(|e| map_error("read", path, e))
        })
    }

    pub fn write(path: &str, contents: &[u8]) -> Result<(), FsError> {
        std::fs::write(path, contents).map_err(|e| map_error("write", path, e))
    }

    pub fn create_directory(path: &str) -> Result<(), FsError> {
        std::fs::create_dir(path).map_err(|e| map_error("mkdir", path, e))
    }

    pub fn rename(old: &str, new: &str) -> Result<(), FsError> {
        std::fs::rename(old, new).map_err(|e| map_error("rename", old, e))
    }

    pub fn remove(path: &str, is_directory: bool) -> Result<(), FsError> {
        let result = if is_directory {
            std::fs::remove_dir(path)
        } else {
            std::fs::remove_file(path)
        };
        result.map_err(|e| map_error("remove", path, e))
    }
}

#[cfg(all(not(feature = "orays"), not(any(test, feature = "host-tools"))))]
mod imp {
    use super::*;

    fn unsupported(operation: &'static str, path: &str) -> FsError {
        FsError::new(FsErrorKind::Unsupported, operation, path)
    }

    pub fn read_directory(path: &str) -> Result<Vec<DirectoryEntry>, FsError> {
        Err(unsupported("read_dir", path))
    }
    pub fn read_bytes_bounded(path: &str, _max_bytes: usize) -> Result<Vec<u8>, FsError> {
        Err(unsupported("read", path))
    }
    pub fn write(path: &str, _contents: &[u8]) -> Result<(), FsError> {
        Err(unsupported("write", path))
    }
    pub fn create_directory(path: &str) -> Result<(), FsError> {
        Err(unsupported("mkdir", path))
    }
    pub fn rename(old: &str, _new: &str) -> Result<(), FsError> {
        Err(unsupported("rename", old))
    }
    pub fn remove(path: &str, _is_directory: bool) -> Result<(), FsError> {
        Err(unsupported("remove", path))
    }
}
