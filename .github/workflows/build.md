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

| Commit message keyword | Build | Notes |
|---|---:|---|
| `build action` | ✅ | Alias for the legacy build path for now |
| `build legacy` | ✅ | Explicit legacy Windows 7 build |

Both keywords currently trigger the same single artifact. `build legacy` is the recommended one for VM testing.

## Rust and toolchain

The workflow uses:

- `stable` Rust toolchain
- `i686-pc-windows-gnu` target
- MSYS2 MinGW32 toolchain

That combination is the most practical choice for a Win7 32-bit test VM.

## Usage examples

```bash
# Build from a normal commit
git commit -m "ci: test legacy build (build legacy)"

# Same build, kept as a generic CI trigger alias
git commit -m "ci: verify workflow (build action)"
```

## Output

After the workflow finishes, download the artifact named:

`winload-windows-i686-mingw-no-npcap`

The artifact contains:

`winload-windows-i686-mingw-no-npcap.exe`
