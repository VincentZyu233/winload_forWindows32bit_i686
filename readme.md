# winload_forWindows32bit_i686

Legacy Windows build of `winload` for testing on 32-bit Windows 7 VMs.

## Upstream

- GitHub: https://github.com/VincentZyuApps/winload

## Build target

- `i686-pc-windows-gnu`
- `--no-default-features`
- Artifact: `winload-windows-i686-mingw-no-npcap.exe`

## Usage

The GitHub Actions workflow is driven by commit message keywords:

- `build action`
- `build legacy`

For Win7 32-bit VM testing, use `build legacy`.
