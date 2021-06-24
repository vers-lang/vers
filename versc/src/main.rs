#![feature(stmt_expr_attributes)]

#[macro_use] extern crate colour;

use std::env;
use std::fs::File;
use std::path::Path;
use std::process::{exit, Command};
use std::io::Write;

mod c;
mod types;

use c::translate_to_c;

pub static mut OUTPUT: String = String::new();

fn create_output_file(file: &String) -> File {
    let mut create_file = File::create(file.replace(".vers", "")).unwrap();
    return create_file;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let option = &args[1];
    let file = &args[2];

    if option != &String::from("-e") && option != &String::from("-l") {
        red_ln!("Error: {:?} is not an option, use -e or -l option", option);
        exit(0);
    }

    if !Path::new(file).exists() {
        red_ln!("Error: Cannot find {:?}, check if it exists or you've used the right argument", file);
    }

    let mut out_put_file = create_output_file(file);
    unsafe {
        translate_to_c(file);
        out_put_file.write_fmt(format_args!("{}", OUTPUT));
        println!("C equivalent:\n{}", OUTPUT);
    }

    exit(0);
}
