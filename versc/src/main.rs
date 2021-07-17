#[macro_use] extern crate colour;
extern crate versc_lib;

use std::env::args;
use std::fs::{File};
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::process::exit;

use versc_lib::syntax::{check_line, SYMBOLS};

fn check_syntax(file: File) -> i32 {
    let mut reader = BufReader::new(file);
    let mut line = 0;
    let mut errors = 0;

    for code in reader.lines() {
        line = line + 1;
        let mut vers_line = code.unwrap();

        for i in 0..2 {
            if vers_line.starts_with(SYMBOLS[i]) {
                println!("\tVers line starts with {}", SYMBOLS[i]);

                if vers_line.contains("()") {

                }
            }

            if vers_line.ends_with("{") || vers_line.ends_with("}") || vers_line.ends_with(";") {
                println!("\t{}: Vers line ends with the right character! {}", line, vers_line.chars().last().unwrap());
            } else {
                errors = errors + 1;
            }
        }
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

    let mut errors = check_syntax(File::open(&file_name).unwrap());

    if errors == 0 {
        green_ln!("\tNo errors found");
        green_ln!("\tGenerating assembly...");
    } else {
        red_ln!("\tFound {} errors", errors);
        red_ln!("\tWill not compile {}", file_name);
    }
}
