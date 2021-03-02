use std::process::{Command};

pub fn compile_asm() {
    println!("Running cd && gcc");
    Command::new("cd")
        .arg("build/internal/")
        .arg("&&")
        .args(&["gcc", "-c", "main.S"])
        .spawn();
}
