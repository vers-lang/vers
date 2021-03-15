use crate::messages::{compiler_message, errors::{compiler_error, error}, ERRORS, messages::*, warnings::compiler_warning, WARNINGS};
use crate::project::{project_name};
use std::fs::{create_dir, File, remove_dir};
use std::env::{consts::{ARCH, OS}};
use std::path::{Path};
use std::process::{Command};
use crate::{exit_compiler, EXTERNAL_FILES, PROJECT_TYPE};

static mut SCORE: i32 = 0;

unsafe fn check_arch() {
    if ARCH == "x86" || ARCH == "x86_64" {
        SCORE = SCORE + 1;
    } else {
        error(E1H);
        ERRORS = ERRORS + 1;
    }
}

unsafe fn check_os() {
    if OS == "linux" {
        SCORE = SCORE + 1;
    } else if OS == "macos" {
        compiler_warning(W1O);
    } else if OS == "windows" {
        error(E1O);
        exit_compiler();
    } else {
        error(E2O);
        compiler_message("If your OS isn't", "windows", "and you're seeing this message:\n    Go to https://github.com/vers-lang/vers/ and make an issue about your OS not being supported.")
    }
}

unsafe fn build_dir() {
    if Path::new("build/").exists() {
        compiler_message("Cleaning ", "precompiled ", "code...");
        remove_dir("build/");
    }
    create_dir("build/");
    create_dir("build/externals/");
    create_dir("build/internal/");
    create_dir("build/imports/");
    project_name();
    SCORE = SCORE + 1;
}

fn run_build_script() {
    if Path::new("build.sh").exists() {
        compiler_message("Running ", "build ", "script...");

        Command::new("sh")
            .arg("build.sh")
            .spawn();
    unsafe { EXTERNAL_FILES = true }
    } else { /* Nothing */ }
}

pub(crate) fn init() {
    unsafe {
        // Check if architecture is supported
        check_arch();
        // Check of os/kernel is supported
        check_os();

        build_dir();
        run_build_script();
        compiler_message("Compiler init score = ", SCORE.to_string().as_str(), "/3");
    }
}
