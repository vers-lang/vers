#[macro_use] extern crate colour;
extern crate versc_lib;

use std::env::args;
use std::fs::{read_to_string, File};
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::process::{exit, Command};

use versc_lib::c::translate_to_c;
use versc_lib::compile::{_e, _l};
use versc_lib::syntax::{check_line, SYMBOLS};

static mut FILE: &'static str = "";

fn check_syntax(file: &String) -> i32 {
    let mut reader = BufReader::new(File::open(file).unwrap());
    let mut line = 0;
    let mut errors = 0;

    for code in reader.lines() {
        line = line + 1;
        let mut vers_line = code.unwrap();
        if !vers_line.trim().is_empty() {
            errors = errors + check_line(vers_line.as_str());
        }
    }

    return errors;
}

fn main() {
    let compiler_args: Vec<String> = args().collect();

    let option = &compiler_args[1];
    let file_name = &compiler_args[2];
    let add_std = &compiler_args[3];

    let mut link = "exe";

    if option == &String::from("-e") {
        link = "exe";
    } else if option == &String::from("-l") {
        link = "lib";
    } else {
        red_ln!("{} is not a valid option, use \"-e\" or \"-l\"", option);
        exit(0);
    }

    if !Path::new(&file_name).exists() {
        red_ln!("{} cannot be found, check file path", file_name);
        exit(0);
    }

    let mut link_std = true;
    let mut std_file = "";

    if add_std == &String::from("std") {
        link_std = true;
        std_file = "~/.vers/vstd";
    } else if add_std == &String::from("no-std") {
        link_std = false;
        std_file = "~/.vers/nvstd";
    } else {
        red_ln!("{} is not a valid option, use \"std\" or \"no-std\"");
    }

    green_ln!("Compiling {}...", file_name);

    let errors = check_syntax(file_name);

    if errors == 0 {
        green_ln!("No errors were found");
    } else {
        red_ln!("Found: {} errors, will not compile", errors);
        exit(0);
    }

    translate_to_c(&file_name);

    if link == "exe" {
        _e(file_name.replace(".vers", ""), file_name.replace(".vers", ".c"));
    } else if link == "lib" {
        _l(file_name.replace(".vers", ""), file_name.replace(".vers", ".c"));
    } else {
        red_ln!("Unknown link option, can't compile");
    }
}
