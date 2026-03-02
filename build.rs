use std::path::PathBuf;


fn main() {
    println!("cargo:rerun-if-changed=vendor/libvterm/src/");
    println!("cargo:rerun-if-changed=build.rs");
    let bindings = bindgen::builder()
        .header("vendor/libvterm/include/vterm.h")
        .allowlist_function("vterm_.*")
        .allowlist_type("VTerm.*")
        .allowlist_var("VTERM_.*")
        .generate_inline_functions(true)
        .generate()
        .expect("Generating Rust bindings for libvterm failed.");
    
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(out_path).unwrap();
    let mut build = cc::Build::new();
    // println!("cargo:rustc-link-arg=/LTCG");
    // if build.get_compiler().is_like_msvc() {
    //     build.flag("/GL");
    // } else {
    //     build.flag("-flto");
    // }
    build
        .files([
            "vendor/libvterm/src/encoding.c",
            "vendor/libvterm/src/keyboard.c",
            "vendor/libvterm/src/mouse.c",
            "vendor/libvterm/src/parser.c",
            "vendor/libvterm/src/pen.c",
            "vendor/libvterm/src/screen.c",
            "vendor/libvterm/src/state.c",
            "vendor/libvterm/src/unicode.c",
            "vendor/libvterm/src/vterm.c",
        ])
        .include("vendor/libvterm/include")
        .compile("clibvterm");
    println!("cargo:rustc-link-lib=static=clibvterm");
}