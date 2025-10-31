fn main() {
    // Link Windows Media Foundation libraries when building for Windows
    // These libraries provide the COM interface IDs that FFmpeg's Windows encoders need
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=mfplat");
        println!("cargo:rustc-link-lib=mfuuid");
        println!("cargo:rustc-link-lib=mfreadwrite");
        println!("cargo:rustc-link-lib=strmiids");
        println!("cargo:rustc-link-lib=wmcodecdspuuid");
    }
}
