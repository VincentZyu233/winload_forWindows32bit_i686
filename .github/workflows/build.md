# Build Workflow

## Overview

This workflow is intentionally small: it builds one legacy Windows artifact only.

- Target: `i686-pc-windows-gnu`
- Features: `--no-default-features`
- Output: `winload-windows-i686-mingw-no-npcap.exe`
- Intended test host: Windows 7 32-bit VM

## Pipeline

The workflow runs in up to 3 sequential jobs:

```
check ──→ build ──→ release
```

| Stage | Job | Runs on | What it does |
|-------|-----|---------|-------------|
| 1 | `check` | ubuntu-latest | Parse commit message, determine which jobs to run |
| 2 | `build` | windows-latest | Compile with MSYS2 MinGW32, upload artifact |
| 3 | `release` | ubuntu-latest | Download artifact, create GitHub Release |

Each stage only runs if its trigger condition is met (see Keywords below).

## Triggers

The workflow runs on:

- `push` to `main`
- manual `workflow_dispatch`

For push events, commit message keywords control which jobs run.

## Keywords

| Commit message keyword | check | build | release |
|---|---:|---:|---:|
| *(none / other)* | ✅ | ❌ | ❌ |
| `build action` / `build legacy` | ✅ | ✅ | ❌ |
| `build release` | ✅ | ✅ | ✅ |

- No keyword → workflow stops after `check` (no build, no release).
- `build action` or `build legacy` → build only, artifact uploaded but no GitHub Release.
- `build release` → full pipeline: build + create GitHub Release.

## Rust and toolchain

The workflow uses:

- `stable` Rust toolchain
- `i686-pc-windows-gnu` target
- MSYS2 MinGW32 toolchain

That combination is the most practical choice for a Win7 32-bit test VM.

## Usage examples

```bash
# No keyword — check passes but nothing else runs
git commit -m "docs: update readme"

# Build only
git commit -m "ci: test build only (build action)"

# Build and create release
git commit -m "ci: test release pipeline (build release)"
```

## Output

After the workflow finishes, download the artifact named:

`winload-windows-i686-mingw-no-npcap`

The artifact contains:

`winload-windows-i686-mingw-no-npcap.exe`
