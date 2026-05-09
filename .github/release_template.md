<div align=center>

[![Downloads](https://img.shields.io/github/downloads/__REPO__/__VERSION__/total?style=flat-square&logo=github)](https://github.com/__REPO__/releases/__VERSION__)

</div>

### ⬇️ Downloads


> **For i686 (32-bit)**: On Linux, provide 32-bit binary only — no 32-bit DEB/RPM packages. On Windows & Linux, 32-bit binaries are not published to npm / Scoop / AUR.

| OS / Arch | x86_64 | ARM64 | i686 (32-bit) |
|-----------|--------|-------|---------------|
| **Windows** | [![msvc-npcap](https://img.shields.io/badge/msvc--npcap-x64-0078D4.svg?logo=windows)](__BASE_URL__/winload-windows-x86_64-msvc-npcap-__VERSION__.exe) · [![msvc-no-npcap](https://img.shields.io/badge/msvc--no--npcap-x64-0078D4.svg?logo=windows)](__BASE_URL__/winload-windows-x86_64-msvc-no-npcap-__VERSION__.exe) · [![mingw-no-npcap](https://img.shields.io/badge/mingw--no--npcap-x64-0078D4.svg?logo=windows)](__BASE_URL__/winload-windows-x86_64-mingw-no-npcap-__VERSION__.exe) | [![msvc-npcap](https://img.shields.io/badge/msvc--npcap-ARM64-0099CC.svg?logo=windows)](__BASE_URL__/winload-windows-aarch64-msvc-npcap-__VERSION__.exe) · [![msvc-no-npcap](https://img.shields.io/badge/msvc--no--npcap-ARM64-0099CC.svg?logo=windows)](__BASE_URL__/winload-windows-aarch64-msvc-no-npcap-__VERSION__.exe) | [![msvc-no-npcap](https://img.shields.io/badge/msvc--no--npcap-i686-3B6EA5.svg?logo=windows)](__BASE_URL__/winload-windows-i686-msvc-no-npcap-__VERSION__.exe) · [![mingw-no-npcap](https://img.shields.io/badge/mingw--no--npcap-i686-3B6EA5.svg?logo=windows)](__BASE_URL__/winload-windows-i686-mingw-no-npcap-__VERSION__.exe) |
| **Linux** | [![binary](https://img.shields.io/badge/binary-x64-E95420.svg?logo=linux)](__BASE_URL__/winload-linux-x86_64-__VERSION__) · [![deb](https://img.shields.io/badge/deb-x64-D70A53.svg?logo=debian)](__BASE_URL__/winload-linux-x86_64-__VERSION__.deb) · [![rpm](https://img.shields.io/badge/rpm-x64-F1B42F.svg?logo=redhat)](__BASE_URL__/winload-linux-x86_64-__VERSION__.rpm) | [![binary](https://img.shields.io/badge/binary-ARM64-E95420.svg?logo=linux)](__BASE_URL__/winload-linux-aarch64-__VERSION__) · [![deb](https://img.shields.io/badge/deb-ARM64-D70A53.svg?logo=debian)](__BASE_URL__/winload-linux-aarch64-__VERSION__.deb) · [![rpm](https://img.shields.io/badge/rpm-ARM64-F1B42F.svg?logo=redhat)](__BASE_URL__/winload-linux-aarch64-__VERSION__.rpm) | [![binary](https://img.shields.io/badge/binary-i686-E95420.svg?logo=linux)](__BASE_URL__/winload-linux-i686-__VERSION__) |
| **macOS** | [![binary](https://img.shields.io/badge/binary-Intel-000000.svg?logo=apple)](__BASE_URL__/winload-macos-x86_64-__VERSION__) | [![binary](https://img.shields.io/badge/binary-Apple_Silicon-000000.svg?logo=apple)](__BASE_URL__/winload-macos-aarch64-__VERSION__) | *Not provided* |
| **Android** | [![binary](https://img.shields.io/badge/binary-x64-96ed89.svg?logo=android)](__BASE_URL__/winload-android-x86_64-__VERSION__) | [![binary](https://img.shields.io/badge/binary-ARM64-168039.svg?logo=android)](__BASE_URL__/winload-android-aarch64-__VERSION__) | *Not provided* |

> **Windows Binary Labels**: `msvc-npcap` = MSVC with Npcap loopback capture (requires Npcap installed, **Windows 10+**); `msvc-no-npcap` = MSVC standalone, no Npcap (**Windows 10+**); `mingw-no-npcap` = MinGW-w64 standalone, no Npcap (**Windows 10+**). For older Windows (7/8/XP/Vista), maybe need use pre-1.77 Rust builds from earlier releases? (i have not tried yet. <br>
> **Build Toolchain**: Rust 1.77+ dropped Windows 7 support. All builds require **Windows 10+**.<br>


### 📥 Quick Install

**Python (pip):**
```bash
pip install winload==__PYPI_VER__
# or with uv
uv pip install winload==__PYPI_VER__
```
> 📄 [PyPI Package](https://pypi.org/project/winload/__PYPI_VER__/)

**npm (cross-platform):**
```bash
npm install -g @vincentzyuapps/winload@__PLAIN_VER__
```
> 📄 [Npm Package](https://www.npmjs.com/package/@vincentzyuapps/winload/v/__PLAIN_VER__)

**Cargo (build from source):**
```bash
cargo install winload@__PLAIN_VER__
```
> 📄 [Crates.io](https://crates.io/crates/winload/__PLAIN_VER__)

**Windows (Scoop):**
```powershell
scoop bucket add vincentzyu https://github.com/VincentZyuApps/scoop-bucket
scoop update   # optional: manually refresh bucket list before install
scoop install winload@__PLAIN_VER__
```
> 📄 [Scoop Bucket](https://github.com/VincentZyuApps/scoop-bucket)

**🇨🇳 Windows (Scoop) — 码云镜像 Gitee mirror:**
```powershell
scoop bucket add vincentzyu https://gitee.com/vincent-zyu/scoop-bucket
scoop update   # optional: manually refresh bucket list before install
scoop install winload@__PLAIN_VER__
```
> 📄 [Gitee Scoop Bucket](https://gitee.com/vincent-zyu/scoop-bucket)
> 💡 **All builds require Windows 10+** (Rust 1.77+ dropped Windows 7/8 support). Scoop provides only **MSVC + Npcap** for **x86_64** and **ARM64**. For other variants (MinGW, non-Npcap, i686), download from the table above.

**Arch Linux (AUR):**
```bash
paru -S winload-rust-bin
```
> 📄 [AUR Package](https://aur.archlinux.org/packages/winload-rust-bin)

**One-liner install For Linux (Debian/Ubuntu/RHEL/Fedora and derivatives):**
```bash
curl -fsSL https://raw.githubusercontent.com/__REPO__/main/docs/install_scripts/install.sh | bash
# or install this specific version:
WINLOAD_VERSION=__VERSION__ bash -c "$(curl -fsSL https://raw.githubusercontent.com/__REPO__/main/docs/install_scripts/install.sh)"
```
> 📄 [View install script source](https://github.com/__REPO__/blob/main/docs/install_scripts/install.sh)

**🇨🇳 一键安装脚本在码云的镜像 One-liner install for Linux On Gitee mirror (中国大陆地区更快捏，faster in China):**
```bash
curl -fsSL https://gitee.com/vincent-zyu/winload/raw/main/docs/install_scripts/install_gitee.sh | bash
# or install this specific version:
WINLOAD_VERSION=__VERSION__ bash -c "$(curl -fsSL https://gitee.com/vincent-zyu/winload/raw/main/docs/install_scripts/install_gitee.sh)"
```
> 📄 [View Gitee install script](https://gitee.com/vincent-zyu/winload/blob/main/docs/install_scripts/install_gitee.sh)

> ⚠️ These two `.sh` install scripts only support systems with **apt or dnf** on **x86_64 / aarch64**. For other platforms, use **npm** or **Cargo**.
>
> ⚠️ 以上两个`.sh`安装脚本仅适用于使用 **apt 或 dnf** 的 **x86_64 / aarch64** 系统。其他平台请使用 **npm** 或 **Cargo** 安装.

**macOS / Linux (Homebrew):**
```bash
brew tap vincentzyuapps/tap
brew update && brew install winload@__PLAIN_VER__
```
> 📄 [Homebrew Formula (GitHub)](https://github.com/VincentZyuApps/homebrew-tap/blob/main/Formula/winload.rb)

**🇨🇳 Homebrew — 码云镜像 Gitee mirror (手动克隆tap仓库，manual tap clone):**
```bash
git clone https://gitee.com/vincent-zyu/homebrew-tap.git "$(brew --prefix)/Library/Taps/vincentzyuapps/homebrew-tap"
brew update && brew install winload@__PLAIN_VER__
```
> 📄 [Homebrew Formula (Gitee)](https://gitee.com/vincent-zyu/homebrew-tap/blob/main/Formula/winload.rb)
> 💡 Homebrew supports **macOS** (Intel & Apple Silicon) and **Linux** (x86_64 & ARM64).
