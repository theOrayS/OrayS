<a id="worktree-disk-and-cleanup"></a>
# Worktree, disk, and cleanup

<a id="worktree-disk-and-cleanup-disk-gate"></a>
## Disk gate

Before and after long builds, full evaluators, QEMU, Docker, vendoring, large logs, or image creation, run:

```bash
df -h / /root
```

When Codex/OMX storage is relevant, add:

```bash
du -sh /root/.codex .omx 2>/dev/null
omx state list-active --json
omx status
```

If `/` is around 85% used or has less than roughly 10 GiB free, pause new heavy work and inventory usage before deleting anything. Use bounded `du` queries and inspect repository-root images and known evaluation-run directories.

<a id="worktree-disk-and-cleanup-cleanup-order"></a>
## Cleanup order

1. Expired temporary files and reproducible build/cache outputs with no active users.
2. Obsolete raw logs only after preserving required summaries/evidence.
3. Inactive OMX/Codex transient cache/session debris after proving it is inactive.
4. Larger generated images or vendored/build data only with explicit ownership and a reproducible restoration path.

Never delete source, credentials, memory, skills/prompts/agents, authentication, active session/team/goal state, required `.omx` evidence, or another worktree's files. Do not remove caches merely because their directory is large.

<a id="worktree-disk-and-cleanup-closure"></a>
## Closure

Re-run `git status --porcelain=v1 -uall`, disk measurements, and active-state checks. Report paths removed, reclaimed space, protected paths retained, and any residual pressure or unverifiable ownership.
