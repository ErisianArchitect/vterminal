
fn main() {
    println!("cargo:rerun-if-changed=vendor/libvterm-0.3.3/src/");
    println!("cargo:rerun-if-changed=build.rs");
    cc::Build::new()
        .files([
            "vendor/libvterm-0.3.3/src/encoding.c",
            "vendor/libvterm-0.3.3/src/keyboard.c",
            "vendor/libvterm-0.3.3/src/mouse.c",
            "vendor/libvterm-0.3.3/src/parser.c",
            "vendor/libvterm-0.3.3/src/pen.c",
            "vendor/libvterm-0.3.3/src/screen.c",
            "vendor/libvterm-0.3.3/src/state.c",
            "vendor/libvterm-0.3.3/src/unicode.c",
            "vendor/libvterm-0.3.3/src/vterm.c",
        ])
        .include("vendor/libvterm-0.3.3/include")
        .compile("clibvterm");
}