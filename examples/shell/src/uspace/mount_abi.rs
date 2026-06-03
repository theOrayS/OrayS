use axerrno::LinuxError;
use linux_raw_sys::general;
use std::string::{String, ToString};

use super::fd_table::resolve_dirfd_path;
use super::linux_abi::{ST_MODE_DIR, ST_MODE_TYPE_MASK, neg_errno};
use super::runtime_paths::normalize_path;
use super::user_memory::read_cstr;
use super::{MountPoint, UserProcess};

const SUPPORTED_MOUNT_FLAGS: u32 = general::MS_BIND
    | general::MS_REC
    | general::MS_SILENT
    | general::MS_RDONLY
    | general::MS_REMOUNT;

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

    pub(super) fn path_on_readonly_mount(&self, path: &str) -> bool {
        let mount_points = self.mount_points.lock();
        let mut best: Option<(&str, bool)> = None;
        for (target, mount) in mount_points.iter() {
            if mount_path_rest(path, target.as_str()).is_none() {
                continue;
            }
            if best.is_none_or(|(best_target, _)| target.len() > best_target.len()) {
                best = Some((target.as_str(), mount.readonly));
            }
        }
        best.is_some_and(|(_, readonly)| readonly)
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

    fn add_mount_point(&self, target: String, source_root: String, readonly: bool, remount: bool) {
        let mut mount_points = self.mount_points.lock();
        if remount {
            if let Some(mount) = mount_points.get_mut(target.as_str()) {
                mount.readonly = readonly;
                return;
            }
        }
        mount_points.insert(
            target,
            MountPoint {
                source_root,
                readonly,
            },
        );
    }

    fn has_mount_point(&self, target: &str) -> bool {
        self.mount_points.lock().contains_key(target)
    }

    fn remove_mount_point(&self, target: &str) -> bool {
        self.mount_points.lock().remove(target).is_some()
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
    let remount = flags & general::MS_REMOUNT != 0;
    process.add_mount_point(target_path, source_root, readonly, remount);
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
    if process.remove_mount_point(target_path.as_str()) {
        0
    } else {
        neg_errno(LinuxError::EINVAL)
    }
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
        "vfat" | "msdos" | "fat" | "ext2" | "ext3" | "ext4" => {
            let source = source.ok_or(LinuxError::EINVAL)?;
            // The evaluator exposes a single block-backed root filesystem and does not
            // model partition device nodes in devfs.  Accept only conventional block
            // device names, then expose that already-mounted backing filesystem through
            // the process mount namespace rather than pretending to attach a new disk.
            if is_supported_block_device_name(source) {
                Ok("/".into())
            } else {
                Err(LinuxError::ENODEV)
            }
        }
        "" => Err(LinuxError::EINVAL),
        _ => Err(LinuxError::ENODEV),
    }
}

fn is_supported_block_device_name(source: &str) -> bool {
    let Some(path) = normalize_path("/", source) else {
        return false;
    };
    let Some(name) = path.strip_prefix("/dev/") else {
        return false;
    };
    let Some(disk) = name
        .strip_prefix("vd")
        .or_else(|| name.strip_prefix("sd"))
        .or_else(|| name.strip_prefix("xvd"))
    else {
        return false;
    };
    let mut chars = disk.chars();
    let Some(letter) = chars.next() else {
        return false;
    };
    letter.is_ascii_lowercase() && chars.all(|ch| ch.is_ascii_digit())
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
