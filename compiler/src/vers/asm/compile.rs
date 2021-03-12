use crate::messages::{compiler_message, messages::*, warnings::compiler_warning};
use crate::{EXTERNAL_FILES, PROJECT_NAME, PROJECT_TYPE};
use libc::sync;
use std::env::set_current_dir;
use std::fs::rename;
use std::process::{Command};

pub(crate) unsafe fn compile_asm() {
    compiler_message("Compiling ", "internal ", "files...");
    compile_internal();
    println!("External files: {}", EXTERNAL_FILES);
    if EXTERNAL_FILES == true {
        compile_external();
    } else { /* nothing */ }
    link();
}

fn compile_internal() {
    set_current_dir("build/internal/");
    Command::new("pwd")
        .spawn();
    Command::new("gcc")
        .arg("-c")
        .arg("main.S")
        .spawn();
}

fn compile_external() {
    compiler_message("Compiling ", "external ", "files...");
    set_current_dir("build/external/");
    Command::new("pwd")
        .spawn();
}

fn link() {

}

pub(crate) fn compile_lib(name: &str) {

}

pub(crate) unsafe fn compile_exe(name: &str) {
    set_current_dir("build/internal/");
    println!("{}", PROJECT_NAME);
    let name = format_args!("{}{}", "../", PROJECT_NAME).to_string();
    Command::new("gcc")
        .arg("main.o")
        .arg("-o")
        .arg(name.as_str())
        .spawn();
}
