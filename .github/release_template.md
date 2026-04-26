<div align=center>

[![Downloads](https://img.shields.io/github/downloads/__REPO__/__VERSION__/total?style=flat-square&logo=github)](https://github.com/__REPO__/releases/__VERSION__)

</div>

### ⬇️ Downloads

> **i686 (32-bit)**: Npcap SDK 1.13 的 32 位库文件位于 `Lib/` 根目录（非 `Lib/x86`），`--npcap` 功能完整可用。不提供 DEB/RPM 包，也不推送到 npm / Scoop / AUR 源。

| Arch | Windows | Linux | macOS | Android |
|------|---------|-------|-------|---------|
| **x86_64** | [![exe](https://img.shields.io/badge/exe-x64-0078D6.svg?logo=windows)](__BASE_URL__/winload-windows-x86_64-__VERSION__.exe) | [![binary](https://img.shields.io/badge/binary-x64-E95420.svg?logo=linux)](__BASE_URL__/winload-linux-x86_64-__VERSION__) · [![deb](https://img.shields.io/badge/deb-x64-D70A53.svg?logo=debian)](__BASE_URL__/winload-linux-x86_64-__VERSION__.deb) · [![rpm](https://img.shields.io/badge/rpm-x64-F1B42F.svg?logo=redhat)](__BASE_URL__/winload-linux-x86_64-__VERSION__.rpm) | [![binary](https://img.shields.io/badge/binary-Intel-000000.svg?logo=apple)](__BASE_URL__/winload-macos-x86_64-__VERSION__) | [![binary](https://img.shields.io/badge/binary-x64-96ed89.svg?logo=android)](__BASE_URL__/winload-android-x86_64-__VERSION__) |
| **ARM64** | [![exe](https://img.shields.io/badge/exe-ARM64-0078D6.svg?logo=windows)](__BASE_URL__/winload-windows-aarch64-__VERSION__.exe) | [![binary](https://img.shields.io/badge/binary-ARM64-E95420.svg?logo=linux)](__BASE_URL__/winload-linux-aarch64-__VERSION__) · [![deb](https://img.shields.io/badge/deb-ARM64-D70A53.svg?logo=debian)](__BASE_URL__/winload-linux-aarch64-__VERSION__.deb) · [![rpm](https://img.shields.io/badge/rpm-ARM64-F1B42F.svg?logo=redhat)](__BASE_URL__/winload-linux-aarch64-__VERSION__.rpm) | [![binary](https://img.shields.io/badge/binary-Apple_Silicon-000000.svg?logo=apple)](__BASE_URL__/winload-macos-aarch64-__VERSION__) | [![binary](https://img.shields.io/badge/binary-ARM64-168039.svg?logo=android)](__BASE_URL__/winload-android-aarch64-__VERSION__) |
| **i686 (32-bit)** | [![exe](https://img.shields.io/badge/exe-i686-0078D6.svg?logo=windows)](__BASE_URL__/winload-windows-i686-__VERSION__.exe) | [![binary](https://img.shields.io/badge/binary-i686-E95420.svg?logo=linux)](__BASE_URL__/winload-linux-i686-__VERSION__) | — | — |

### 📥 Quick Install

**Python (pip):**
```bash
pip install winload==__PYPI_VER__
# or with uv
uv pip install winload==__PYPI_VER__
```
> 📄 [PyPI](https://pypi.org/project/winload/__PYPI_VER__/)

**npm (cross-platform):**
```bash
npm install -g @vincentzyuapps/winload@__PLAIN_VER__
```
> 📄 [npm](https://www.npmjs.com/package/@vincentzyuapps/winload/v/__PLAIN_VER__)

**Cargo (build from source):**
```bash
cargo install winload@__PLAIN_VER__
```
> 📄 [crates.io](https://crates.io/crates/winload/__PLAIN_VER__)

**Windows (Scoop):**
```powershell
scoop bucket add vincentzyu https://github.com/VincentZyuApps/scoop-bucket
scoop install winload@__PLAIN_VER__
```
> 📄 [Scoop bucket](https://github.com/VincentZyuApps/scoop-bucket)

**Arch Linux (AUR):**
```bash
paru -S winload-rust-bin
```
> 📄 [winload-rust-bin](https://aur.archlinux.org/packages/winload-rust-bin)

**One-liner install for Linux (Debian/Ubuntu/RHEL/Fedora and derivatives):**
```bash
curl -fsSL https://raw.githubusercontent.com/__REPO__/main/docs/install_scripts/install.sh | bash
# or install this specific version:
WINLOAD_VERSION=__VERSION__ bash -c "$(curl -fsSL https://raw.githubusercontent.com/__REPO__/main/docs/install_scripts/install.sh)"
```
> 📄 [View install script source](https://github.com/__REPO__/blob/main/docs/install_scripts/install.sh)

**🇨🇳 Gitee mirror (faster in China):**
```bash
curl -fsSL https://gitee.com/vincent-zyu/winload/raw/main/docs/install_scripts/install_gitee.sh | bash
# or install this specific version:
WINLOAD_VERSION=__VERSION__ bash -c "$(curl -fsSL https://gitee.com/vincent-zyu/winload/raw/main/docs/install_scripts/install_gitee.sh)"
```
> 📄 [View Gitee install script](https://gitee.com/vincent-zyu/winload/blob/main/docs/install_scripts/install_gitee.sh)

> ⚠️ These two `.sh` install scripts only support systems with **apt or dnf** on **x86_64 / aarch64**. For other platforms, use **npm** or **Cargo**.
>
> ⚠️ 以上两个`.sh`安装脚本仅适用于使用 **apt 或 dnf** 的 **x86_64 / aarch64** 系统。其他平台请使用 **npm** 或 **Cargo** 安装.
