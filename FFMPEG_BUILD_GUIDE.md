# FFmpeg Build Configuration Guide

## Overview
This document explains how FFmpeg is configured for building the xlab project on different platforms.

## Background
The xlab project uses FFmpeg for video encoding through the `ffmpeg-sys-next` Rust crate. This crate provides Rust bindings to FFmpeg C libraries.

## Platform-Specific Configuration

### Linux
**Approach:** Dynamic linking with system packages

**Why:** 
- Linux distributions provide well-maintained FFmpeg packages
- Dynamic linking is the standard practice on Linux
- Significantly faster build times
- Users can update FFmpeg independently

**Setup:**
1. System packages are installed via apt-get (libav* packages)
2. `ffmpeg-sys-next` uses pkg-config to find the libraries
3. The binary links dynamically against system FFmpeg libraries

**GitHub Actions:**
```yaml
- name: Install system dependencies
  run: |
    sudo apt-get install -y \
      libavcodec-dev \
      libavformat-dev \
      libavutil-dev \
      libavfilter-dev \
      libavdevice-dev \
      libswscale-dev \
      libswresample-dev
```

### Windows
**Approach:** Dynamic linking with vcpkg prebuilt libraries

**Why:**
- vcpkg provides prebuilt FFmpeg libraries for Windows
- Avoids long compilation times (20+ minutes)
- pkg-config support through pkgconf package
- Proper integration with Rust build system

**Setup:**
1. vcpkg installs pkgconf (provides pkg-config on Windows)
2. vcpkg installs FFmpeg with all required components
3. Environment variables guide the build system:
   - `PKG_CONFIG_PATH`: Points to vcpkg's pkgconfig directory
   - `VCPKG_ROOT`: vcpkg installation directory
   - `PATH`: Includes pkgconf binary location

**GitHub Actions:**
```yaml
- name: Install FFmpeg (Windows)
  run: |
    vcpkg install pkgconf:x64-windows
    vcpkg install ffmpeg[core,avcodec,avformat,avutil,avfilter,avdevice,swscale,swresample]:x64-windows
    echo "PKG_CONFIG_PATH=C:\vcpkg\installed\x64-windows\lib\pkgconfig" | Out-File -FilePath $env:GITHUB_ENV -Encoding utf8 -Append
    echo "VCPKG_ROOT=C:\vcpkg" | Out-File -FilePath $env:GITHUB_ENV -Encoding utf8 -Append
    echo "C:\vcpkg\installed\x64-windows\tools\pkgconf" | Out-File -FilePath $env:GITHUB_PATH -Encoding utf8 -Append
```

## Cargo Configuration

### xlab-core/Cargo.toml
```toml
[dependencies]
ffmpeg-sys-next = "7.1.3"
```

**Note:** No `static` feature is specified. This tells `ffmpeg-sys-next` to use pkg-config to find system/vcpkg libraries and link dynamically.

### .cargo/config.toml
```toml
[env]
VCPKG_CRT_LINKAGE = "dynamic"
VCPKG_DEFAULT_TRIPLET = "x64-windows"
```

This configuration ensures vcpkg uses the correct triplet (x64-windows for dynamic libraries) on Windows.

## Alternatives Considered

### 1. Static Linking (features = ["static"])
**Pros:**
- Single self-contained binary
- No runtime dependencies

**Cons:**
- Requires static library files (.a on Linux, .lib on Windows)
- Most Linux distributions don't provide static FFmpeg libraries
- Longer build times
- Larger binary size
- Security updates require rebuilding

**Verdict:** Not practical for cross-platform development

### 2. Build from Source (features = ["build"])
**Pros:**
- Complete control over FFmpeg configuration
- True static linking possible
- Works on all platforms

**Cons:**
- Extremely long build times (20-30 minutes)
- Requires many build tools (yasm, nasm, make, perl, etc.)
- Complex to configure correctly
- CI/CD pipeline slowdown

**Verdict:** Too slow for regular development

### 3. vcpkg Static Libraries (x64-windows-static-md)
**Pros:**
- Static linking on Windows
- Prebuilt, so faster than building from source

**Cons:**
- Still requires building on Linux from source
- Inconsistent across platforms
- More complex configuration

**Verdict:** Doesn't solve Linux problem

## Local Development

### Linux
```bash
# Install FFmpeg development packages
sudo apt-get install -y libavcodec-dev libavformat-dev libavutil-dev \
  libavfilter-dev libavdevice-dev libswscale-dev libswresample-dev

# Build the project
npm run tauri build
```

### Windows
```powershell
# Install FFmpeg via vcpkg
vcpkg install pkgconf:x64-windows
vcpkg install ffmpeg[core,avcodec,avformat,avutil,avfilter,avdevice,swscale,swresample]:x64-windows

# Set environment variables (in PowerShell session)
$env:PKG_CONFIG_PATH = "C:\vcpkg\installed\x64-windows\lib\pkgconfig"
$env:VCPKG_ROOT = "C:\vcpkg"
$env:PATH = "C:\vcpkg\installed\x64-windows\tools\pkgconf;" + $env:PATH

# Build the project
npm run tauri build
```

## Troubleshooting

### Linux: "cannot find -lavcodec"
- Ensure FFmpeg development packages are installed
- Check that pkg-config can find FFmpeg: `pkg-config --libs libavcodec`

### Windows: "Could not find libavcodec"
- Verify vcpkg installed successfully: `vcpkg list | findstr ffmpeg`
- Check PKG_CONFIG_PATH is set correctly
- Ensure pkgconf is in PATH: `where pkgconf`

### Build Times
- Linux: ~5-6 minutes (mostly Rust compilation)
- Windows: ~8-10 minutes (includes vcpkg package installation)

## References
- [ffmpeg-sys-next documentation](https://docs.rs/ffmpeg-sys-next/)
- [vcpkg documentation](https://vcpkg.io/)
- [FFmpeg documentation](https://ffmpeg.org/documentation.html)
