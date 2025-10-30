# FFmpeg Prebuilt Static Linking

## Overview
This project uses **prebuilt static FFmpeg libraries** via vcpkg to create independent binaries with no runtime dependencies, while avoiding the long compilation times of building FFmpeg from source.

## Approach: vcpkg for Prebuilt Static Libraries

Both Linux and Windows use **vcpkg** to download prebuilt FFmpeg static libraries. This provides:
- ✅ Static linking (no runtime dependencies)
- ✅ Fast builds (libraries are precompiled)
- ✅ Consistent cross-platform approach
- ✅ Reliable library availability

## Configuration

### xlab-core/Cargo.toml
```toml
[dependencies]
ffmpeg-sys-next = { version = "7.1.3", features = ["static"] }
```

The `static` feature tells `ffmpeg-sys-next` to use pkg-config to find static libraries (`.a` on Linux, `.lib` on Windows).

### .cargo/config.toml
```toml
[env]
VCPKG_CRT_LINKAGE = "dynamic"
VCPKG_DEFAULT_TRIPLET = "x64-windows-static-md"
```

On Windows, this ensures vcpkg uses static FFmpeg libraries with dynamic CRT (required for Rust MSVC).

## Platform Implementation

### Linux
Uses vcpkg with the `x64-linux` triplet (which builds static libraries by default on Linux).

**GitHub Actions Workflow:**
```yaml
- name: Install vcpkg
  run: |
    git clone https://github.com/Microsoft/vcpkg.git /tmp/vcpkg
    /tmp/vcpkg/bootstrap-vcpkg.sh

- name: Install prebuilt static FFmpeg via vcpkg
  run: |
    /tmp/vcpkg/vcpkg install ffmpeg[core,avcodec,avformat,avutil,avfilter,avdevice,swscale,swresample]:x64-linux
    echo "PKG_CONFIG_PATH=/tmp/vcpkg/installed/x64-linux/lib/pkgconfig:$PKG_CONFIG_PATH" >> $GITHUB_ENV
```

### Windows
Uses vcpkg with the `x64-windows-static` triplet for static libraries.

**GitHub Actions Workflow:**
```yaml
- name: Install prebuilt static FFmpeg via vcpkg (Windows)
  run: |
    vcpkg install ffmpeg[core,avcodec,avformat,avutil,avfilter,avdevice,swscale,swresample]:x64-windows-static
    echo "PKG_CONFIG_PATH=C:\vcpkg\installed\x64-windows-static\lib\pkgconfig" | Out-File -FilePath $env:GITHUB_ENV -Encoding utf8 -Append
```

## Build Times

- **First build:** ~10-15 minutes (vcpkg downloads/builds FFmpeg once)
- **Subsequent builds:** ~5-6 minutes (vcpkg caches binaries)
- **Much faster than `build` feature:** No FFmpeg compilation in Rust build

## Benefits

### ✅ Advantages
1. **True static linking** - No runtime dependencies
2. **Portable binaries** - Run anywhere without FFmpeg installed
3. **Fast builds** - Precompiled libraries (10-15 min vs 20-30+ min)
4. **Cross-platform** - Same approach on Linux and Windows
5. **Reliable** - vcpkg handles all dependencies
6. **Cacheable** - vcpkg binaries can be cached in CI

### 📊 Comparison with Alternatives

| Approach | Build Time | Static Linking | Dependencies |
|----------|-----------|----------------|--------------|
| **vcpkg (this)** | 10-15 min | ✅ Yes | None |
| `build` feature | 20-30+ min | ✅ Yes | None |
| `static` + system | ❌ Fails | Would be yes | None |
| Dynamic linking | 5-6 min | ❌ No | FFmpeg required |

## Local Development

### Linux
```bash
# Install vcpkg
git clone https://github.com/Microsoft/vcpkg.git ~/vcpkg
~/vcpkg/bootstrap-vcpkg.sh

# Install FFmpeg static libraries
~/vcpkg/vcpkg install ffmpeg[core,avcodec,avformat,avutil,avfilter,avdevice,swscale,swresample]:x64-linux

# Set environment variable
export PKG_CONFIG_PATH=~/vcpkg/installed/x64-linux/lib/pkgconfig:$PKG_CONFIG_PATH

# Build
npm run tauri build
```

### Windows
```powershell
# vcpkg is pre-installed on GitHub Actions runners
# For local development:
vcpkg install ffmpeg[core,avcodec,avformat,avutil,avfilter,avdevice,swscale,swresample]:x64-windows-static

# Set environment variable
$env:PKG_CONFIG_PATH = "C:\vcpkg\installed\x64-windows-static\lib\pkgconfig"

# Build
npm run tauri build
```

## Verifying Static Linking

### Linux
```bash
ldd target/release/xlab | grep -i ffmpeg
# Should return nothing (no FFmpeg shared libraries)
```

### Windows
```powershell
dumpbin /dependents target\release\xlab.exe | findstr /i "ffmpeg av"
# Should not show avcodec.dll, avformat.dll, etc.
```

## Troubleshooting

### Linux: "pkg-config not found"
Ensure pkg-config is installed:
```bash
sudo apt-get install pkg-config
```

### Windows: "cannot find -lavcodec"
Verify PKG_CONFIG_PATH is set correctly:
```powershell
echo $env:PKG_CONFIG_PATH
# Should show: C:\vcpkg\installed\x64-windows-static\lib\pkgconfig
```

### vcpkg installation takes too long
This is normal for the first build. Subsequent builds will be much faster as vcpkg caches the built binaries. In CI, you can cache the vcpkg directory.

## Why Not Other Approaches?

### ❌ BtbN/FFmpeg-Builds
These are excellent for binary distribution but don't include static libraries (`.a`/`.lib` files) needed for static linking.

### ❌ System Packages (apt-get)
Most Linux distributions only provide shared libraries, not static versions.

### ❌ Build from Source (`build` feature)
Takes 20-30+ minutes every time FFmpeg needs to be rebuilt. vcpkg is much faster.

## References

- [vcpkg FFmpeg package](https://github.com/microsoft/vcpkg/tree/master/ports/ffmpeg)
- [ffmpeg-sys-next documentation](https://docs.rs/ffmpeg-sys-next/)
- [vcpkg documentation](https://vcpkg.io/)
