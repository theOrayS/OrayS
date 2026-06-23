use std::string::{String, ToString};
use std::vec::Vec;

use super::linux_abi::{LEGACY_TESTSUITE_STAGE_ROOT, TESTSUITE_STAGE_ROOT};

pub(super) fn current_cwd() -> String {
    std::env::current_dir().unwrap_or_else(|_| "/".into())
}

pub(super) fn resolve_host_path(cwd: String, path: &str) -> Result<String, String> {
    normalize_path(cwd.as_str(), path).ok_or_else(|| format!("invalid path: {path}"))
}

pub(super) fn normalize_path(base: &str, path: &str) -> Option<String> {
    let mut parts = Vec::new();
    let input = if path.starts_with('/') {
        path.to_string()
    } else if base == "/" {
        format!("/{path}")
    } else {
        format!("{}/{}", base.trim_end_matches('/'), path)
    };
    for part in input.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }
    let mut normalized = String::from("/");
    normalized.push_str(&parts.join("/"));
    Some(normalized)
}

pub(super) fn derive_exec_root_from_path(path: &str) -> String {
    if path == "/musl" || path.starts_with("/musl/") {
        return "/musl".into();
    }
    if path == "/glibc" || path.starts_with("/glibc/") {
        return "/glibc".into();
    }
    if let Some(root) = staged_exec_root(path, TESTSUITE_STAGE_ROOT) {
        return root.into();
    }
    if let Some(root) = staged_exec_root(path, LEGACY_TESTSUITE_STAGE_ROOT) {
        return root.into();
    }
    "/".into()
}

fn staged_exec_root(path: &str, stage_root: &str) -> Option<&'static str> {
    let rest = path.strip_prefix(stage_root)?;
    match rest {
        "/m" => Some("/musl"),
        "/g" => Some("/glibc"),
        _ if rest.starts_with("/m/") => Some("/musl"),
        _ if rest.starts_with("/g/") => Some("/glibc"),
        "/musl" => Some("/musl"),
        "/glibc" => Some("/glibc"),
        _ if rest.starts_with("/musl/") => Some("/musl"),
        _ if rest.starts_with("/glibc/") => Some("/glibc"),
        _ => None,
    }
}

pub(super) fn resolve_runtime_support_file(exec_root: &str, path: &str) -> Result<String, String> {
    let candidates = if path.starts_with('/') {
        runtime_absolute_path_candidates(exec_root, path)
    } else if !path.contains('/') {
        runtime_library_name_candidates(exec_root, path)
    } else {
        vec![normalize_path("/", path).ok_or_else(|| format!("invalid path: {path}"))?]
    };
    candidates
        .into_iter()
        .find(|candidate| matches!(std::fs::metadata(candidate), Ok(meta) if meta.is_file()))
        .ok_or_else(|| format!("runtime support file not found: {path}"))
}

pub(super) fn runtime_absolute_path_candidates(exec_root: &str, path: &str) -> Vec<String> {
    let Some(normalized) = normalize_path("/", path) else {
        return Vec::new();
    };
    let mut candidates = vec![normalized.clone()];
    for root in runtime_root_candidates(exec_root, normalized.as_str()) {
        if normalized == "/lib" || normalized.starts_with("/lib/") {
            push_runtime_candidate(
                &mut candidates,
                join_runtime_root(root.as_str(), normalized.as_str()),
            );
            if normalized == "/lib" {
                push_runtime_candidate(&mut candidates, join_runtime_root(root.as_str(), "/lib64"));
            } else if let Some(suffix) = normalized.strip_prefix("/lib/") {
                push_runtime_candidate(
                    &mut candidates,
                    join_runtime_root(root.as_str(), format!("/lib64/{suffix}").as_str()),
                );
                push_multiarch_runtime_aliases(&mut candidates, root.as_str(), suffix);
            }
        } else if normalized == "/lib64" || normalized.starts_with("/lib64/") {
            push_runtime_candidate(
                &mut candidates,
                join_runtime_root(root.as_str(), normalized.as_str()),
            );
            if normalized == "/lib64" {
                push_runtime_candidate(&mut candidates, join_runtime_root(root.as_str(), "/lib"));
            } else if let Some(suffix) = normalized.strip_prefix("/lib64/") {
                push_runtime_candidate(
                    &mut candidates,
                    join_runtime_root(root.as_str(), format!("/lib/{suffix}").as_str()),
                );
                push_multiarch_runtime_aliases(&mut candidates, root.as_str(), suffix);
            }
        } else if normalized == "/usr/lib" || normalized.starts_with("/usr/lib/") {
            push_runtime_candidate(
                &mut candidates,
                join_runtime_root(root.as_str(), normalized.as_str()),
            );
            if normalized == "/usr/lib" {
                push_runtime_candidate(&mut candidates, join_runtime_root(root.as_str(), "/lib"));
                push_runtime_candidate(&mut candidates, join_runtime_root(root.as_str(), "/lib64"));
            } else if let Some(suffix) = normalized.strip_prefix("/usr/lib/") {
                push_runtime_candidate(
                    &mut candidates,
                    join_runtime_root(root.as_str(), format!("/lib/{suffix}").as_str()),
                );
                push_runtime_candidate(
                    &mut candidates,
                    join_runtime_root(root.as_str(), format!("/lib64/{suffix}").as_str()),
                );
                push_multiarch_runtime_aliases(&mut candidates, root.as_str(), suffix);
            }
        } else if normalized == "/usr/lib64" || normalized.starts_with("/usr/lib64/") {
            push_runtime_candidate(
                &mut candidates,
                join_runtime_root(root.as_str(), normalized.as_str()),
            );
            if normalized == "/usr/lib64" {
                push_runtime_candidate(&mut candidates, join_runtime_root(root.as_str(), "/lib64"));
                push_runtime_candidate(&mut candidates, join_runtime_root(root.as_str(), "/lib"));
            } else if let Some(suffix) = normalized.strip_prefix("/usr/lib64/") {
                push_runtime_candidate(
                    &mut candidates,
                    join_runtime_root(root.as_str(), format!("/lib64/{suffix}").as_str()),
                );
                push_runtime_candidate(
                    &mut candidates,
                    join_runtime_root(root.as_str(), format!("/lib/{suffix}").as_str()),
                );
                push_multiarch_runtime_aliases(&mut candidates, root.as_str(), suffix);
            }
        } else if normalized.starts_with("/etc/ld") {
            push_runtime_candidate(
                &mut candidates,
                join_runtime_root(root.as_str(), normalized.as_str()),
            );
        } else if normalized == "/bin"
            || normalized.starts_with("/bin/")
            || normalized == "/usr/bin"
            || normalized.starts_with("/usr/bin/")
        {
            push_runtime_candidate(
                &mut candidates,
                join_runtime_root(root.as_str(), normalized.as_str()),
            );
            let prefix = if normalized == "/bin" || normalized.starts_with("/bin/") {
                "/bin"
            } else {
                "/usr/bin"
            };
            if normalized == prefix {
                push_runtime_candidate(&mut candidates, Some(root.to_string()));
            } else if let Some(suffix) = normalized.strip_prefix(prefix) {
                push_runtime_candidate(&mut candidates, join_runtime_root(root.as_str(), suffix));
                if is_runtime_shell_command(suffix.trim_start_matches('/')) {
                    push_runtime_candidate(
                        &mut candidates,
                        join_runtime_root(root.as_str(), "/busybox"),
                    );
                }
            }
        }
        push_musl_loader_aliases(&mut candidates, root.as_str(), normalized.as_str());
    }
    candidates
}

pub(super) fn staged_cwd_absolute_path_candidates(cwd: &str, path: &str) -> Vec<String> {
    let Some(normalized) = normalize_path("/", path) else {
        return Vec::new();
    };
    let Some(prefix) = runtime_command_prefix(normalized.as_str()) else {
        return Vec::new();
    };
    let mut candidates = Vec::new();
    for root in staged_cwd_roots(cwd) {
        push_runtime_candidate(
            &mut candidates,
            join_runtime_root(root.as_str(), normalized.as_str()),
        );
        if normalized == prefix {
            push_runtime_candidate(&mut candidates, Some(root));
        } else if let Some(suffix) = normalized.strip_prefix(prefix) {
            push_runtime_candidate(&mut candidates, join_runtime_root(root.as_str(), suffix));
        }
    }
    candidates
}

pub(super) fn runtime_library_name_candidates(exec_root: &str, name: &str) -> Vec<String> {
    if name.contains('/') || !looks_like_runtime_library_name(name) {
        return Vec::new();
    }
    let mut candidates = Vec::new();
    for root in runtime_root_candidates(exec_root, name) {
        push_runtime_candidate(
            &mut candidates,
            join_runtime_root(root.as_str(), format!("/lib/{name}").as_str()),
        );
        push_runtime_candidate(
            &mut candidates,
            join_runtime_root(root.as_str(), format!("/lib64/{name}").as_str()),
        );
        push_runtime_candidate(
            &mut candidates,
            join_runtime_root(root.as_str(), format!("/usr/lib/{name}").as_str()),
        );
        push_runtime_candidate(
            &mut candidates,
            join_runtime_root(root.as_str(), format!("/usr/lib64/{name}").as_str()),
        );
        push_musl_loader_aliases(&mut candidates, root.as_str(), name);
    }
    candidates
}

fn runtime_command_prefix(path: &str) -> Option<&'static str> {
    if path == "/bin" || path.starts_with("/bin/") {
        Some("/bin")
    } else if path == "/usr/bin" || path.starts_with("/usr/bin/") {
        Some("/usr/bin")
    } else {
        None
    }
}

fn is_runtime_shell_command(name: &str) -> bool {
    matches!(name, "sh" | "ash" | "bash")
}

fn staged_cwd_roots(cwd: &str) -> Vec<String> {
    let mut roots = Vec::new();
    push_staged_cwd_roots(&mut roots, cwd, TESTSUITE_STAGE_ROOT);
    push_staged_cwd_roots(&mut roots, cwd, LEGACY_TESTSUITE_STAGE_ROOT);
    roots
}

fn push_staged_cwd_roots(roots: &mut Vec<String>, cwd: &str, stage_root: &str) {
    let Some(rest) = cwd.strip_prefix(stage_root) else {
        return;
    };
    let rest = rest.trim_start_matches('/');
    if rest.is_empty() {
        return;
    }

    let mut parts = rest.split('/').filter(|part| !part.is_empty());
    let Some(suite) = parts.next() else {
        return;
    };
    let Some(group) = parts.next() else {
        return;
    };
    let group_root = format!("{}/{}/{}", stage_root.trim_end_matches('/'), suite, group);
    push_runtime_candidate(roots, Some(group_root));
    push_runtime_candidate(roots, Some(cwd.to_string()));
}

fn runtime_root_candidates(exec_root: &str, path: &str) -> Vec<String> {
    let name = path.rsplit('/').next().unwrap_or(path);
    let mut roots = Vec::new();
    let mut push = |root: &str| {
        if !roots.iter().any(|item| item == root) {
            roots.push(root.to_string());
        }
    };
    if is_glibc_runtime_name(name) {
        push("/glibc");
    }
    if is_musl_runtime_name(name) {
        push("/musl");
    }
    if exec_root != "/" {
        push(exec_root);
    }
    push("/musl");
    push("/glibc");
    roots
}

fn join_runtime_root(root: &str, path: &str) -> Option<String> {
    let normalized = normalize_path("/", path)?;
    if root == "/" {
        return Some(normalized);
    }
    let rel = normalized.trim_start_matches('/');
    Some(if rel.is_empty() {
        root.to_string()
    } else {
        format!("{}/{}", root.trim_end_matches('/'), rel)
    })
}

pub(super) fn push_runtime_candidate(candidates: &mut Vec<String>, candidate: Option<String>) {
    let Some(candidate) = candidate else {
        return;
    };
    if !candidates.iter().any(|item| item == &candidate) {
        candidates.push(candidate);
    }
}

fn push_multiarch_runtime_aliases(candidates: &mut Vec<String>, root: &str, suffix: &str) {
    let Some((_, tail)) = suffix.split_once('/') else {
        return;
    };
    if tail.is_empty() {
        return;
    }
    push_runtime_candidate(
        candidates,
        join_runtime_root(root, format!("/lib/{tail}").as_str()),
    );
    push_runtime_candidate(
        candidates,
        join_runtime_root(root, format!("/lib64/{tail}").as_str()),
    );
}

fn push_musl_loader_aliases(candidates: &mut Vec<String>, root: &str, path: &str) {
    let name = path.rsplit('/').next().unwrap_or(path);
    if !name.starts_with("ld-musl-") || !name.ends_with(".so.1") {
        return;
    }
    push_runtime_candidate(candidates, join_runtime_root(root, "/lib/libc.so"));
    push_runtime_candidate(candidates, join_runtime_root(root, "/lib64/libc.so"));
}

fn is_glibc_runtime_name(name: &str) -> bool {
    name.starts_with("ld-linux-") || name.ends_with(".so.6")
}

fn is_musl_runtime_name(name: &str) -> bool {
    name.starts_with("ld-musl-") || name == "libc.so"
}

fn looks_like_runtime_library_name(name: &str) -> bool {
    name.starts_with("ld-") || name.contains(".so")
}
