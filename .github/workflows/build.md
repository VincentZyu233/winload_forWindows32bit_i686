# Build Workflow

## Overview

This workflow is intentionally small: it builds one legacy Windows artifact only.

- Target: `i686-pc-windows-gnu`
- Features: `--no-default-features`
- Output: `winload-windows-i686-mingw-no-npcap.exe`
- Intended test host: Windows 7 32-bit VM

## Triggers

The workflow runs on:

- `push` to `main`
- manual `workflow_dispatch`

For push events, commit message keywords control whether the job runs.

## Keywords

| Commit message keyword | Build | Release | Notes |
|---|---:|---:|---|
| `build action` | ✅ | ❌ | Build only |
| `build release` | ✅ | ✅ | Build and create GitHub Release |

`build release` is the keyword to use when you want CI to publish a GitHub Release.

## Rust and toolchain

The workflow uses:

- `stable` Rust toolchain
- `i686-pc-windows-gnu` target
- MSYS2 MinGW32 toolchain

That combination is the most practical choice for a Win7 32-bit test VM.

## Usage examples

```bash
# Build from a normal commit
git commit -m "ci: test build only (build action)"

# Build and create release
git commit -m "ci: test release pipeline (build release)"
```

## Output

After the workflow finishes, download the artifact named:

`winload-windows-i686-mingw-no-npcap`

The artifact contains:

`winload-windows-i686-mingw-no-npcap.exe`
