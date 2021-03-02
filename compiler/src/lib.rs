#![feature(fmt_as_str)]
// Imports
#[macro_use]
extern crate colour;

extern crate serde_json;

mod arch;
mod compiler;
mod messages;
mod project;
mod vers;

/* ----- */

// Compiler
use messages::{compiler_message, errors, ERRORS, warnings, WARNINGS};
use crate::messages::errors::compiler_error;
use crate::messages::warnings::compiler_warning;
use vers::compile;
use std::process::exit;

static mut PROJECT_TYPE: &'static str = "exe";

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
    unsafe { compile::compile_vers(); }
}

unsafe fn exit_compiler() {
    println!("\nExit with:");
    compiler_error(format_args!("{}{}{}", ERRORS, " Errors", "\n").to_string().as_str());
    compiler_warning(format_args!("{}{}{}", WARNINGS, " Warnings", "\n").to_string().as_str());
    print!("\n");
    exit(0);
}

pub fn main() {
    println!("Checking if project will have any problems...");
    setup_init();
    compiler_message("Compiling...", "", "");
    compiler_init();
    unsafe { exit_compiler(); }
}
