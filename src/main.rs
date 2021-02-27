extern crate cli;
use cli::{new, version};

use std::env;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let vers_arg = &arg[1];
    if vers_arg == "new" {
        new::main();
    } else if vers_arg == "--version" {
        version::main();
    }
}
