use crate::messages::{errors::compiler_error, ERRORS, messages::*, warnings::compiler_warning, WARNINGS, compiler_message};
use std::env::{consts::{ARCH, OS}};
use crate::exit_compiler;

static mut SCORE: i32 = 0;

unsafe fn check_arch() {
    if ARCH == "x86" || ARCH == "x86_64" {
        SCORE = SCORE + 1;
    } else {
        compiler_error(E1H);
        ERRORS = ERRORS + 1;
    }
}

unsafe fn check_os() {
    if OS == "linux" {
        SCORE = SCORE + 1;
    } else if OS == "macos" {
        compiler_warning(W1O);
    } else if OS == "windows" {
        compiler_error(E1O);
        exit_compiler();
    } else {
        compiler_error(E2O);
        compiler_message("If your OS isn't", "windows", "and you're seeing this message:\n    Go to https://github.com/vers-lang/vers/ and make an issue about your OS not being supported.")
    }
}

unsafe fn test() {
    let a = 1;
    let b = 1;
    let c = a + b;
    if c == 2 {
        SCORE = SCORE + 1;
    } else {
        println!("{} + {} = {}\nMath just broke and that's not good.", a, b, c);
        exit_compiler();
    }
}

pub(crate) fn init() {
    unsafe {
        // Check if architecture is supported
        check_arch();
        // Check of os/kernel is supported
        check_os();
        // A third function
        test();
        compiler_message("Compiler init score = ", SCORE.to_string().as_str(), "/3");
    }
}