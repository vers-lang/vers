// Imports
#[macro_use]
extern crate colour;

extern crate serde_json;

mod messages;
mod project;

/* ----- */

// Compiler
use messages::{compiler_message, errors, warnings};

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

}

pub fn main() {
    println!("Checking if project will have any problems...");
    setup_init();
    compiler_message("Compiling", "", "");
    compiler_init();
}
