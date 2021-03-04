extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/vers/asm/compile.c");
    cc::Build::new()
        .file("src/vers/asm/compile.c")
        .compile("compile");
}
