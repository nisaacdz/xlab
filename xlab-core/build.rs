use std::path::PathBuf;
use bindgen::Builder;

fn main() {
    println!("cargo:rustc-link-search=native=ffmpeg_static/lib"); // Path to static libraries
    println!("cargo:rustc-link-lib=static=avcodec"); // Link to libavcodec.a
    println!("cargo:rustc-link-lib=static=avformat"); // Link to libavformat.a
    println!("cargo:rustc-link-lib=static=avutil"); // Link to libavutil.a
    // Add more libraries as needed

    // Get the path to the FFmpeg include directory
    let ffmpeg_include_dir = PathBuf::from("ffmpeg_static/include");

    // Configure the builder
    let bindings = Builder::default()
        .header(ffmpeg_include_dir.join("libavcodec/avcodec.h").to_string_lossy().to_string())
        .header(ffmpeg_include_dir.join("libavformat/avformat.h").to_string_lossy().to_string())
        .header(ffmpeg_include_dir.join("libswscale/swscale.h").to_string_lossy().to_string())
        .header(ffmpeg_include_dir.join("libavutil/rational.h").to_string_lossy().to_string())
        .allowlist_function("av.*")
        .allowlist_type("AV.*")
        .allowlist_var("AV.*")
        .allowlist_type("SwsContext")
        .allowlist_type("AVRational")
        .clang_arg(format!("-I{}", ffmpeg_include_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to a file
    bindings
        .write_to_file("src/video/ffmpeg_bindings.rs")
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=build.rs"); // Rebuild if build script changes
}
