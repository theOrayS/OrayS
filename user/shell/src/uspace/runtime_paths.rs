use std::string::{String, ToString};
use std::vec::Vec;

use super::linux_abi::{LEGACY_TESTSUITE_STAGE_ROOT, TESTSUITE_STAGE_ROOT};

pub(super) fn current_cwd() -> String {
    std::env::current_dir().unwrap_or_else(|_| "/".into())
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

fn runtime_root_candidates(exec_root: &str, path: &str) -> Vec<String> {
    let name = path.rsplit('/').next().unwrap_or(path);
    let mut roots = Vec::new();
    let mut push = |root: &str| {
        if !roots.iter().any(|item| item == root) {
            roots.push(root.to_string());
        }
    };
    if exec_root == "/" {
        push(exec_root);
    }
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

const EXEC_PATH_ENOMEM_PREFIX: &str = "exec-loader-ENOMEM: ";

fn exec_path_enomem(context: &str) -> String {
    let mut out = String::new();
    let _ = out.try_reserve_exact(EXEC_PATH_ENOMEM_PREFIX.len() + context.len());
    out.push_str(EXEC_PATH_ENOMEM_PREFIX);
    out.push_str(context);
    out
}

fn try_owned(value: &str, context: &str) -> Result<String, String> {
    let mut out = String::new();
    out.try_reserve_exact(value.len())
        .map_err(|_| exec_path_enomem(context))?;
    out.push_str(value);
    Ok(out)
}

fn try_join_parts(parts: &[&str], context: &str) -> Result<String, String> {
    let len = parts.iter().try_fold(0usize, |len, part| {
        len.checked_add(part.len())
            .ok_or_else(|| exec_path_enomem(context))
    })?;
    let mut out = String::new();
    out.try_reserve_exact(len)
        .map_err(|_| exec_path_enomem(context))?;
    for part in parts {
        out.push_str(part);
    }
    Ok(out)
}

fn try_push_path_parts<'a>(
    parts: &mut Vec<&'a str>,
    input: &'a str,
    context: &str,
) -> Result<(), String> {
    for part in input.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => {
                parts
                    .try_reserve_exact(1)
                    .map_err(|_| exec_path_enomem(context))?;
                parts.push(part);
            }
        }
    }
    Ok(())
}

pub(super) fn try_normalize_path(base: &str, path: &str) -> Result<Option<String>, String> {
    let mut parts = Vec::new();
    if path.starts_with('/') {
        try_push_path_parts(&mut parts, path, "normalize absolute path")?;
    } else {
        try_push_path_parts(&mut parts, base, "normalize base path")?;
        try_push_path_parts(&mut parts, path, "normalize relative path")?;
    }

    let byte_len = if parts.is_empty() {
        1
    } else {
        parts
            .iter()
            .try_fold(1usize, |len, part| {
                len.checked_add(part.len())
                    .and_then(|len| len.checked_add(1))
                    .ok_or_else(|| exec_path_enomem("normalize path length overflow"))
            })?
            .saturating_sub(1)
    };
    let mut normalized = String::new();
    normalized
        .try_reserve_exact(byte_len)
        .map_err(|_| exec_path_enomem("normalize path"))?;
    normalized.push('/');
    for (idx, part) in parts.iter().enumerate() {
        if idx != 0 {
            normalized.push('/');
        }
        normalized.push_str(part);
    }
    Ok(Some(normalized))
}

pub(super) fn try_resolve_host_path(cwd: &str, path: &str) -> Result<String, String> {
    try_normalize_path(cwd, path)?.ok_or_else(|| {
        try_join_parts(&["invalid path: ", path], "report invalid exec path")
            .unwrap_or_else(|_| exec_path_enomem("report invalid exec path"))
    })
}

pub(super) fn try_derive_exec_root_from_path(path: &str) -> Result<String, String> {
    if path == "/musl" || path.starts_with("/musl/") {
        return try_owned("/musl", "copy musl exec root");
    }
    if path == "/glibc" || path.starts_with("/glibc/") {
        return try_owned("/glibc", "copy glibc exec root");
    }
    if let Some(root) = staged_exec_root(path, TESTSUITE_STAGE_ROOT) {
        return try_owned(root, "copy staged exec root");
    }
    if let Some(root) = staged_exec_root(path, LEGACY_TESTSUITE_STAGE_ROOT) {
        return try_owned(root, "copy staged exec root");
    }
    try_owned("/", "copy default exec root")
}

pub(super) fn try_resolve_runtime_support_file(
    exec_root: &str,
    path: &str,
) -> Result<String, String> {
    let candidates = if path.starts_with('/') {
        try_runtime_absolute_path_candidates(exec_root, path)?
    } else if !path.contains('/') {
        try_runtime_library_name_candidates(exec_root, path)?
    } else {
        let mut candidates = Vec::new();
        let normalized = try_normalize_path("/", path)?.ok_or_else(|| {
            try_join_parts(&["invalid path: ", path], "report invalid runtime path")
                .unwrap_or_else(|_| exec_path_enomem("report invalid runtime path"))
        })?;
        try_push_candidate(
            &mut candidates,
            normalized,
            "record runtime support candidate",
        )?;
        candidates
    };
    candidates
        .into_iter()
        .find(|candidate| matches!(std::fs::metadata(candidate), Ok(meta) if meta.is_file()))
        .ok_or_else(|| {
            try_join_parts(
                &["runtime support file not found: ", path],
                "report missing runtime support",
            )
            .unwrap_or_else(|_| exec_path_enomem("report missing runtime support"))
        })
}

pub(super) fn try_runtime_absolute_path_candidates(
    exec_root: &str,
    path: &str,
) -> Result<Vec<String>, String> {
    let Some(normalized) = try_normalize_path("/", path)? else {
        return Ok(Vec::new());
    };
    let mut candidates = Vec::new();
    try_push_candidate(
        &mut candidates,
        try_owned(normalized.as_str(), "copy normalized runtime path")?,
        "record normalized runtime path",
    )?;
    for root in try_runtime_root_candidates(exec_root, normalized.as_str())? {
        if normalized == "/lib" || normalized.starts_with("/lib/") {
            try_push_joined_root(&mut candidates, root.as_str(), normalized.as_str())?;
            if normalized == "/lib" {
                try_push_joined_root(&mut candidates, root.as_str(), "/lib64")?;
            } else if let Some(suffix) = normalized.strip_prefix("/lib/") {
                let lib64 = try_join_parts(&["/lib64/", suffix], "prepare lib64 alias")?;
                try_push_joined_root(&mut candidates, root.as_str(), lib64.as_str())?;
                try_push_multiarch_runtime_aliases(&mut candidates, root.as_str(), suffix)?;
            }
        } else if normalized == "/lib64" || normalized.starts_with("/lib64/") {
            try_push_joined_root(&mut candidates, root.as_str(), normalized.as_str())?;
            if normalized == "/lib64" {
                try_push_joined_root(&mut candidates, root.as_str(), "/lib")?;
            } else if let Some(suffix) = normalized.strip_prefix("/lib64/") {
                let lib = try_join_parts(&["/lib/", suffix], "prepare lib alias")?;
                try_push_joined_root(&mut candidates, root.as_str(), lib.as_str())?;
                try_push_multiarch_runtime_aliases(&mut candidates, root.as_str(), suffix)?;
            }
        } else if normalized == "/usr/lib" || normalized.starts_with("/usr/lib/") {
            try_push_joined_root(&mut candidates, root.as_str(), normalized.as_str())?;
            if normalized == "/usr/lib" {
                try_push_joined_root(&mut candidates, root.as_str(), "/lib")?;
                try_push_joined_root(&mut candidates, root.as_str(), "/lib64")?;
            } else if let Some(suffix) = normalized.strip_prefix("/usr/lib/") {
                let lib = try_join_parts(&["/lib/", suffix], "prepare usr lib alias")?;
                let lib64 = try_join_parts(&["/lib64/", suffix], "prepare usr lib64 alias")?;
                try_push_joined_root(&mut candidates, root.as_str(), lib.as_str())?;
                try_push_joined_root(&mut candidates, root.as_str(), lib64.as_str())?;
                try_push_multiarch_runtime_aliases(&mut candidates, root.as_str(), suffix)?;
            }
        } else if normalized == "/usr/lib64" || normalized.starts_with("/usr/lib64/") {
            try_push_joined_root(&mut candidates, root.as_str(), normalized.as_str())?;
            if normalized == "/usr/lib64" {
                try_push_joined_root(&mut candidates, root.as_str(), "/lib64")?;
                try_push_joined_root(&mut candidates, root.as_str(), "/lib")?;
            } else if let Some(suffix) = normalized.strip_prefix("/usr/lib64/") {
                let lib64 = try_join_parts(&["/lib64/", suffix], "prepare usr lib64 alias")?;
                let lib = try_join_parts(&["/lib/", suffix], "prepare usr lib alias")?;
                try_push_joined_root(&mut candidates, root.as_str(), lib64.as_str())?;
                try_push_joined_root(&mut candidates, root.as_str(), lib.as_str())?;
                try_push_multiarch_runtime_aliases(&mut candidates, root.as_str(), suffix)?;
            }
        } else if normalized.starts_with("/etc/ld") {
            try_push_joined_root(&mut candidates, root.as_str(), normalized.as_str())?;
        } else if normalized == "/bin"
            || normalized.starts_with("/bin/")
            || normalized == "/usr/bin"
            || normalized.starts_with("/usr/bin/")
        {
            try_push_joined_root(&mut candidates, root.as_str(), normalized.as_str())?;
            let prefix = if normalized == "/bin" || normalized.starts_with("/bin/") {
                "/bin"
            } else {
                "/usr/bin"
            };
            if normalized == prefix {
                try_push_candidate(
                    &mut candidates,
                    try_owned(root.as_str(), "copy runtime command root")?,
                    "record runtime command root",
                )?;
            } else if let Some(suffix) = normalized.strip_prefix(prefix) {
                try_push_joined_root(&mut candidates, root.as_str(), suffix)?;
                if is_runtime_shell_command(suffix.trim_start_matches('/')) {
                    try_push_joined_root(&mut candidates, root.as_str(), "/busybox")?;
                }
            }
        }
        try_push_musl_loader_aliases(&mut candidates, root.as_str(), normalized.as_str())?;
    }
    Ok(candidates)
}

pub(super) fn try_staged_cwd_absolute_path_candidates(
    cwd: &str,
    path: &str,
) -> Result<Vec<String>, String> {
    let Some(normalized) = try_normalize_path("/", path)? else {
        return Ok(Vec::new());
    };
    let Some(prefix) = runtime_command_prefix(normalized.as_str()) else {
        return Ok(Vec::new());
    };
    let mut candidates = Vec::new();
    for root in try_staged_cwd_roots(cwd)? {
        try_push_joined_root(&mut candidates, root.as_str(), normalized.as_str())?;
        if normalized == prefix {
            try_push_candidate(&mut candidates, root, "record staged cwd root")?;
        } else if let Some(suffix) = normalized.strip_prefix(prefix) {
            try_push_joined_root(&mut candidates, root.as_str(), suffix)?;
        }
    }
    Ok(candidates)
}

pub(super) fn try_runtime_library_name_candidates(
    exec_root: &str,
    name: &str,
) -> Result<Vec<String>, String> {
    if name.contains('/') || !looks_like_runtime_library_name(name) {
        return Ok(Vec::new());
    }
    let mut candidates = Vec::new();
    for root in try_runtime_root_candidates(exec_root, name)? {
        let lib = try_join_parts(&["/lib/", name], "prepare runtime lib path")?;
        let lib64 = try_join_parts(&["/lib64/", name], "prepare runtime lib64 path")?;
        let usr_lib = try_join_parts(&["/usr/lib/", name], "prepare runtime usr lib path")?;
        let usr_lib64 = try_join_parts(&["/usr/lib64/", name], "prepare runtime usr lib64 path")?;
        try_push_joined_root(&mut candidates, root.as_str(), lib.as_str())?;
        try_push_joined_root(&mut candidates, root.as_str(), lib64.as_str())?;
        try_push_joined_root(&mut candidates, root.as_str(), usr_lib.as_str())?;
        try_push_joined_root(&mut candidates, root.as_str(), usr_lib64.as_str())?;
        try_push_musl_loader_aliases(&mut candidates, root.as_str(), name)?;
    }
    Ok(candidates)
}

fn try_staged_cwd_roots(cwd: &str) -> Result<Vec<String>, String> {
    let mut roots = Vec::new();
    try_push_staged_cwd_roots(&mut roots, cwd, TESTSUITE_STAGE_ROOT)?;
    try_push_staged_cwd_roots(&mut roots, cwd, LEGACY_TESTSUITE_STAGE_ROOT)?;
    Ok(roots)
}

fn try_push_staged_cwd_roots(
    roots: &mut Vec<String>,
    cwd: &str,
    stage_root: &str,
) -> Result<(), String> {
    let Some(rest) = cwd.strip_prefix(stage_root) else {
        return Ok(());
    };
    let rest = rest.trim_start_matches('/');
    if rest.is_empty() {
        return Ok(());
    }

    let mut parts = rest.split('/').filter(|part| !part.is_empty());
    let Some(suite) = parts.next() else {
        return Ok(());
    };
    let Some(group) = parts.next() else {
        return Ok(());
    };
    let group_root = try_join_parts(
        &[stage_root.trim_end_matches('/'), "/", suite, "/", group],
        "prepare staged cwd root",
    )?;
    try_push_candidate(roots, group_root, "record staged cwd root")?;
    try_push_candidate(
        roots,
        try_owned(cwd, "copy staged cwd")?,
        "record staged cwd",
    )
}

fn try_runtime_root_candidates(exec_root: &str, path: &str) -> Result<Vec<String>, String> {
    let name = path.rsplit('/').next().unwrap_or(path);
    let mut roots = Vec::new();
    if exec_root == "/" {
        try_push_candidate_from_str(&mut roots, exec_root, "record standard runtime root")?;
    }
    if is_glibc_runtime_name(name) {
        try_push_candidate_from_str(&mut roots, "/glibc", "record glibc runtime root")?;
    }
    if is_musl_runtime_name(name) {
        try_push_candidate_from_str(&mut roots, "/musl", "record musl runtime root")?;
    }
    if exec_root != "/" {
        try_push_candidate_from_str(&mut roots, exec_root, "record current runtime root")?;
    }
    try_push_candidate_from_str(&mut roots, "/musl", "record fallback musl root")?;
    try_push_candidate_from_str(&mut roots, "/glibc", "record fallback glibc root")?;
    Ok(roots)
}

fn try_join_runtime_root(root: &str, path: &str) -> Result<String, String> {
    let normalized =
        try_normalize_path("/", path)?.ok_or_else(|| exec_path_enomem("normalize runtime path"))?;
    if root == "/" {
        return Ok(normalized);
    }
    let rel = normalized.trim_start_matches('/');
    if rel.is_empty() {
        try_owned(root, "copy runtime root")
    } else {
        try_join_parts(
            &[root.trim_end_matches('/'), "/", rel],
            "join runtime root path",
        )
    }
}

fn try_push_joined_root(
    candidates: &mut Vec<String>,
    root: &str,
    path: &str,
) -> Result<(), String> {
    let joined = try_join_runtime_root(root, path)?;
    try_push_candidate(candidates, joined, "record runtime candidate")
}

fn try_push_candidate_from_str(
    candidates: &mut Vec<String>,
    candidate: &str,
    context: &str,
) -> Result<(), String> {
    let candidate = try_owned(candidate, context)?;
    try_push_candidate(candidates, candidate, context)
}

fn try_push_candidate(
    candidates: &mut Vec<String>,
    candidate: String,
    context: &str,
) -> Result<(), String> {
    if !candidates.iter().any(|item| item == &candidate) {
        candidates
            .try_reserve_exact(1)
            .map_err(|_| exec_path_enomem(context))?;
        candidates.push(candidate);
    }
    Ok(())
}

fn try_push_multiarch_runtime_aliases(
    candidates: &mut Vec<String>,
    root: &str,
    suffix: &str,
) -> Result<(), String> {
    let Some((_, tail)) = suffix.split_once('/') else {
        return Ok(());
    };
    if tail.is_empty() {
        return Ok(());
    }
    let lib = try_join_parts(&["/lib/", tail], "prepare multiarch lib alias")?;
    let lib64 = try_join_parts(&["/lib64/", tail], "prepare multiarch lib64 alias")?;
    try_push_joined_root(candidates, root, lib.as_str())?;
    try_push_joined_root(candidates, root, lib64.as_str())
}

fn try_push_musl_loader_aliases(
    candidates: &mut Vec<String>,
    root: &str,
    path: &str,
) -> Result<(), String> {
    let name = path.rsplit('/').next().unwrap_or(path);
    if !name.starts_with("ld-musl-") || !name.ends_with(".so.1") {
        return Ok(());
    }
    try_push_joined_root(candidates, root, "/lib/libc.so")?;
    try_push_joined_root(candidates, root, "/lib64/libc.so")
}
