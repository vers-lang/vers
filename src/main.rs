extern crate cli;
use cli::{help, install, new, version};

extern crate compiler;

use std::env;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let vers_arg = &arg[1];
    if vers_arg == "new" {
        new::main();
    } else if vers_arg == "--version" {
        version::main();
    } else if vers_arg == "install" {
        install::main();
    } else if vers_arg == "--help" {
        help::main();
    } else if vers_arg == "build" {
        compiler::main();
    } else {
        println!("{} not a Vers command\nTry: vers --help", vers_arg);
    }
}
