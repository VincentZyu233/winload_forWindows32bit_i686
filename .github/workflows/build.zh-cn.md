# 构建工作流

## 概览

这个工作流只做一件事：构建一个旧版 Windows 专用产物。

- 目标：`i686-pc-windows-gnu`
- 特性：`--no-default-features`
- 输出：`winload-windows-i686-mingw-no-npcap.exe`
- 目标机器：Windows 7 32-bit VMware 虚拟机

## 触发方式

工作流会在下面情况运行：

- `push` 到 `main`
- 手动 `workflow_dispatch`

如果是 push 事件，是否构建由 commit 信息关键词控制。

## 关键词

| commit 关键词 | 构建 | Release | 说明 |
|---|---:|---:|---|
| `build action` | ✅ | ❌ | 只构建，不发 Release |
| `build release` | ✅ | ✅ | 构建并创建 GitHub Release |

如果你想让 CI 发 Release，就用 `build release`。

## Rust 和工具链

工作流使用：

- `stable` Rust 工具链
- `i686-pc-windows-gnu` target
- MSYS2 的 MinGW32 工具链

这是目前最适合 Win7 32-bit 虚拟机验证的组合。

## 使用示例

```bash
# 只构建
git commit -m "ci: test build only (build action)"

# 构建并发 Release
git commit -m "ci: test release pipeline (build release)"
```

## 构建结果

Workflow 完成后，下载这个 artifact：

`winload-windows-i686-mingw-no-npcap`

里面的文件是：

`winload-windows-i686-mingw-no-npcap.exe`
