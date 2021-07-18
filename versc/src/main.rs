#[macro_use] extern crate colour;
extern crate versc_lib;

use std::env::args;
use std::fs::{read_to_string, File};
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::process::exit;

use versc_lib::syntax::{check_line, SYMBOLS};

fn check_syntax(file: &String) -> i32 {
    let mut reader = BufReader::new(File::open(file).unwrap());
    let mut line = 0;
    let mut errors = 0;

    for code in reader.lines() {
        line = line + 1;
        let mut vers_line = code.unwrap();

        errors = errors + check_line(vers_line.as_str());
    }

    return errors;
}

fn main() {
    let compiler_args: Vec<String> = args().collect();

    let option = &compiler_args[1];
    let file_name = &compiler_args[2];

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

    green_ln!("Compiling {}...", file_name);

    let mut errors = check_syntax(file_name);

    if errors == 0 {
        green_ln!("\tNo errors found");
        green_ln!("\tGenerating assembly...");
    } else {
        red_ln!("\tFound {} errors", errors);
        red_ln!("\tWill not compile {}", file_name);
    }
}
