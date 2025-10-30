# FFmpeg Build Configuration Guide

This document explains how FFmpeg static linking is configured for this project and how to prevent common build errors.

## Current Configuration

### Cargo Dependency
```toml
ffmpeg-sys-next = { version = "7.1.3", features = ["static"] }
```

### How It Works

The `ffmpeg-sys-next` crate with `static` feature automatically handles platform-specific FFmpeg discovery:

#### Linux (via pkg-config)
- Uses `pkg-config` to find system FFmpeg development libraries
- Requires: `libavcodec-dev`, `libavformat-dev`, `libavutil-dev`, etc.
- Links statically against these libraries
- Build time: ~1-2 minutes (no source compilation needed)

#### Windows MSVC (via vcpkg)
- Automatically detects and uses vcpkg-installed FFmpeg
- Requires: `ffmpeg:x64-windows-static-md` installed via vcpkg
- Links statically with `/MD` runtime (matches Rust default)
- Build time: ~8-10 minutes (vcpkg install + Rust compile)

## Feature Comparison

### `static` Feature (Current - ✅ Recommended)
**What it does**: Links against pre-built static libraries

**Pros**:
- ✅ Fast builds (no FFmpeg compilation)
- ✅ Reliable on all platforms
- ✅ Automatic platform detection (pkg-config/vcpkg)
- ✅ Still produces statically-linked binaries

**Cons**:
- ⚠️ Requires system FFmpeg dev packages on Linux
- ⚠️ Requires vcpkg FFmpeg on Windows

**When to use**: Default choice for most projects

---

### `build` Feature (Previous - ❌ Not Recommended)
**What it does**: Clones and compiles FFmpeg from source

**Pros**:
- ✅ No system dependencies needed
- ✅ Complete control over FFmpeg configuration

**Cons**:
- ❌ Slow (10-15+ minute FFmpeg compilation)
- ❌ Windows: Requires sh.exe and Unix-like environment
- ❌ Windows: Unreliable even with MSYS2
- ❌ Complex troubleshooting

**When to use**: Only when you need custom FFmpeg configuration or minimal dependencies

---

### No Feature (Default - ❌ Not Suitable)
**What it does**: Links against system shared libraries dynamically

**Pros**:
- ✅ Fast builds
- ✅ Small binary size

**Cons**:
- ❌ Runtime dependency on system FFmpeg
- ❌ Version compatibility issues
- ❌ Not portable

**When to use**: Development only, not for distribution

## Common Errors and Solutions

### Error: "Failed to find 'sh.exe'"
**Cause**: Using `build` feature on Windows without proper Unix environment

**Solution**: Switch to `static` feature with vcpkg (current configuration)

### Error: "Could not find ffmpeg with vcpkg" (Windows)
**Cause**: FFmpeg not installed via vcpkg

**Solution**:
```powershell
vcpkg install ffmpeg:x64-windows-static-md
```

### Error: "Package 'libavcodec' not found" (Linux)
**Cause**: FFmpeg development packages not installed

**Solution**:
```bash
sudo apt-get install -y \
  libavcodec-dev \
  libavformat-dev \
  libavutil-dev \
  libavfilter-dev \
  libavdevice-dev \
  libswscale-dev \
  libswresample-dev
```

### Error: "undefined reference to..." (Linux)
**Cause**: Missing optional FFmpeg dependencies

**Solution**: Install additional libraries as needed:
```bash
sudo apt-get install -y \
  libx264-dev \
  libx265-dev \
  libvpx-dev \
  libopus-dev
```

## Workflow Configuration

### Linux Build
```yaml
- name: Install system dependencies
  run: |
    sudo apt-get update
    sudo apt-get install -y \
      pkg-config \
      libavcodec-dev \
      libavformat-dev \
      libavutil-dev \
      libavfilter-dev \
      libavdevice-dev \
      libswscale-dev \
      libswresample-dev
```

### Windows Build
```yaml
- name: Install FFmpeg (Windows)
  run: |
    vcpkg install ffmpeg:x64-windows-static-md
  shell: pwsh
```

## Verification

### Check Static Linking (Linux)
```bash
ldd target/release/xlab-core | grep -i ffmpeg
# Should output: "not a dynamic executable" or no ffmpeg lines
```

### Check Static Linking (Windows)
```powershell
dumpbin /dependents target\release\xlab-core.exe | findstr /i "av"
# Should not list avcodec.dll, avformat.dll, etc.
```

## References

- [ffmpeg-sys-next documentation](https://docs.rs/ffmpeg-sys-next)
- [ffmpeg-sys-next source code](https://github.com/zmwangx/rust-ffmpeg-sys)
- [vcpkg documentation](https://vcpkg.io/)
- [FFmpeg official site](https://ffmpeg.org/)

## Change History

- **2025-10-30**: Switched from `build` to `static` feature for simpler, faster builds
- **2025-10-30**: Initial attempt with `build` feature (failed on Windows)
