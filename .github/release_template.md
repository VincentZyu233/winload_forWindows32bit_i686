<div align=center>

[![Downloads](https://img.shields.io/github/downloads/__REPO__/__VERSION__/total?style=flat-square&logo=github)](https://github.com/__REPO__/releases/__VERSION__)

</div>

### ⬇️ Downloads


> **For i686 (32-bit)**: Linux binary only — no DEB/RPM packages. Windows & Linux binaries are not published to npm / Scoop / AUR.

| Arch | Windows | Linux | macOS | Android |
|------|---------|-------|-------|---------|
| **x86_64** | [![msvc-npcap](https://img.shields.io/badge/msvc--npcap-x64-0078D4.svg?logo=windows)](__BASE_URL__/winload-windows-x86_64-msvc-npcap-__VERSION__.exe) · [![msvc-no-npcap](https://img.shields.io/badge/msvc--no--npcap-x64-0078D4.svg?logo=windows)](__BASE_URL__/winload-windows-x86_64-msvc-nonpcap-__VERSION__.exe) · [![mingw](https://img.shields.io/badge/mingw-x64-0078D4.svg?logo=windows)](__BASE_URL__/winload-windows-x86_64-mingw-__VERSION__.exe) | [![binary](https://img.shields.io/badge/binary-x64-E95420.svg?logo=linux)](__BASE_URL__/winload-linux-x86_64-__VERSION__) · [![deb](https://img.shields.io/badge/deb-x64-D70A53.svg?logo=debian)](__BASE_URL__/winload-linux-x86_64-__VERSION__.deb) · [![rpm](https://img.shields.io/badge/rpm-x64-F1B42F.svg?logo=redhat)](__BASE_URL__/winload-linux-x86_64-__VERSION__.rpm) | [![binary](https://img.shields.io/badge/binary-Intel-000000.svg?logo=apple)](__BASE_URL__/winload-macos-x86_64-__VERSION__) | [![binary](https://img.shields.io/badge/binary-x64-96ed89.svg?logo=android)](__BASE_URL__/winload-android-x86_64-__VERSION__) |
| **ARM64** | [![msvc-npcap](https://img.shields.io/badge/msvc--npcap-ARM64-0099CC.svg?logo=windows)](__BASE_URL__/winload-windows-aarch64-msvc-npcap-__VERSION__.exe) · [![msvc-no-npcap](https://img.shields.io/badge/msvc--no--npcap-ARM64-0099CC.svg?logo=windows)](__BASE_URL__/winload-windows-aarch64-msvc-nonpcap-__VERSION__.exe) | [![binary](https://img.shields.io/badge/binary-ARM64-E95420.svg?logo=linux)](__BASE_URL__/winload-linux-aarch64-__VERSION__) · [![deb](https://img.shields.io/badge/deb-ARM64-D70A53.svg?logo=debian)](__BASE_URL__/winload-linux-aarch64-__VERSION__.deb) · [![rpm](https://img.shields.io/badge/rpm-ARM64-F1B42F.svg?logo=redhat)](__BASE_URL__/winload-linux-aarch64-__VERSION__.rpm) | [![binary](https://img.shields.io/badge/binary-Apple_Silicon-000000.svg?logo=apple)](__BASE_URL__/winload-macos-aarch64-__VERSION__) | [![binary](https://img.shields.io/badge/binary-ARM64-168039.svg?logo=android)](__BASE_URL__/winload-android-aarch64-__VERSION__) |
| **i686 (32-bit)** | [![msvc-no-npcap](https://img.shields.io/badge/msvc--no--npcap-i686-3B6EA5.svg?logo=windows)](__BASE_URL__/winload-windows-i686-msvc-nonpcap-__VERSION__.exe) · [![mingw](https://img.shields.io/badge/mingw-i686-3B6EA5.svg?logo=windows)](__BASE_URL__/winload-windows-i686-mingw-__VERSION__.exe) | [![binary](https://img.shields.io/badge/binary-i686-E95420.svg?logo=linux)](__BASE_URL__/winload-linux-i686-__VERSION__) | *Not provided* | *Not provided* |

> **Windows Binary Labels**: `msvc-npcap` = MSVC with Npcap loopback capture (requires Npcap installed, Windows 7+); `msvc-no-npcap` = MSVC standalone, no Npcap (Windows 7+); `mingw` = MinGW-w64 for legacy Windows (XP/Vista/7+).<br>
> **Build Toolchain**: `msvc-npcap` & `msvc-no-npcap` are built with MSVC (VS 2022, requires Windows 7+ with KB2999226). `mingw` variants use MinGW-w64 — runs on Windows XP through 11, pick this if the MSVC binary fails to start on your system.<br>


### 📥 Quick Install

**Python (pip):**
```bash
pip install winload==__PYPI_VER__
# or with uv
uv pip install winload==__PYPI_VER__
```
> 📄 [PyPI package](https://pypi.org/project/winload/__PYPI_VER__/)

**npm (cross-platform):**
```bash
npm install -g @vincentzyuapps/winload@__PLAIN_VER__
```
> 📄 [npm package](https://www.npmjs.com/package/@vincentzyuapps/winload/v/__PLAIN_VER__)

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
> 📄 [AUR package](https://aur.archlinux.org/packages/winload-rust-bin)

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
