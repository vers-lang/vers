#![feature(stmt_expr_attributes)]

#[macro_use] extern crate colour;

use std::env;
use std::fs::{File, remove_file};
use std::path::Path;
use std::process::{exit, Command};
use std::io::Write;

mod c;
mod compile;

use c::translate_to_c;
use compile::{_e, _l};

pub static mut OUTPUT: String = String::new();
pub static mut ERRORS: bool = false;

fn create_c_output_file(file: &String) -> File {
    let mut create_file = File::create(file.replace(".vers", ".c")).unwrap();
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
        exit(0);
    }

    let mut out_put_file = create_c_output_file(file);
    unsafe {
        translate_to_c(file);
        out_put_file.write_fmt(format_args!("{}", OUTPUT));

        if ERRORS == true {
            red_ln!("Did not compile successfully");
            remove_file(file.replace(".vers", ""));
        }
    }



    if option == &String::from("-e") {
        let c_file_name = file.to_string().replace(".vers", ".c");
        let output = file.to_string().replace(".vers", "");
        _e(output, c_file_name);
    } else if option == &String::from("-l") {
        let c_file_name = file.to_string().replace(".vers", ".c");
        let output = file.to_string().replace(".vers", "");
        _l(output, c_file_name);
    }

    exit(0);
}
