# winload_forWindows32bit_i686

这是 `winload` 的旧版 Windows 构建仓库，用于 32-bit Windows 7 虚拟机测试。

## 上游仓库

- GitHub: https://github.com/VincentZyuApps/winload

## 构建目标

- `i686-pc-windows-gnu`
- `--no-default-features`
- 产物：`winload-windows-i686-mingw-no-npcap.exe`

## 触发方式

GitHub Actions 通过 commit 信息关键词触发：

- `build action`
- `build legacy`

做 Win7 32-bit VM 测试时，推荐直接用 `build legacy`。
