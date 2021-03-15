#![feature(fmt_as_str)]
#![feature(option_result_contains)]
// Imports
#[macro_use]
extern crate colour;

extern crate libc;
extern crate serde_json;

mod arch;
mod compiler;
mod functions;
mod messages;
mod project;
mod vers;

/* ----- */

// Compiler
use messages::{compiler_message, errors, ERRORS, messages::*, warnings, WARNINGS};
use crate::messages::errors::{compiler_error, error};
use crate::messages::warnings::compiler_warning;
use core::fmt::Arguments;
use vers::{asm, compile, finish::*};
use std::process::exit;

static mut PROJECT_NAME: &'static str = "";
static mut PROJECT_TYPE: &'static str = "exe";
static mut EXTERNAL_FILES: bool = false;

unsafe fn set_project_name(name: &'static str) {
    PROJECT_NAME = name;
    // println!("PROJECT_NAME = {}", PROJECT_NAME);
}

unsafe fn exe() {
    println!("Project type is exe");
    PROJECT_TYPE = "exe";
}

unsafe fn lib() {
    println!("Project type is lib");
    PROJECT_TYPE = "lib";
}

fn setup_init() {
    project::init();
}

fn compiler_init() {
    compiler::init();
    compiler_message("Compiling...", "", "");
    unsafe { compile::compile_vers(); }
    unsafe { compile_asm(); }
    compiler_message("Compiling ", "external files...", "");
}

pub(crate) unsafe fn exit_compiler() {
    println!("\nExit with:");
    error(format_args!("{}{}{}", ERRORS, " Errors", "\n").to_string().as_str());
    compiler_warning(format_args!("{}{}{}", WARNINGS, " Warnings", "\n").to_string().as_str());
    print!("\n");
    exit(0);
}

pub fn main() {
    println!("Checking if project will have any problems...");
    setup_init();
    compiler_init();
    println!("compiling project");
    unsafe {
        println!("Project name: {}\nProject type: {}\n", PROJECT_NAME, PROJECT_TYPE);
        if PROJECT_TYPE == "exe" {
            compiler_message("Compiling project ", "as ", "executable");
            compile_exe(PROJECT_NAME);
        } else if PROJECT_NAME == "lib" {
            compiler_message("Compiling project ", "as ", "library");
            compile_lib(PROJECT_NAME);
        } else {
            compiler_warning(W2V);
            println!("Defaulting to executable");
            compiler_message("Compiling project ", "as ", "executable");
            compile_lib(PROJECT_NAME);
        }
    }
    unsafe { exit_compiler(); }
}
