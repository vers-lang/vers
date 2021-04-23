use std::env;
use std::fmt::Arguments;
use std::fs::{remove_file, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{exit, Command};

static mut ASM_FILE: String = String::new();

fn write_asm_file(write: Arguments) {
    let mut asm_file = unsafe { File::open(ASM_FILE.clone()).unwrap() };
    asm_file.write_fmt(write);
}

fn look_for_functions(vers: File) -> String {
    let reader = BufReader::new(vers);
    let mut globl_functions = String::new();
    for (mut line, mut vers_line) in reader.lines().enumerate() {
        let mut vers_line = vers_line.unwrap();
        if vers_line.contains("fun") {
            let fun_len = vers_line.len();
            // Remove "() {"
            vers_line.truncate(fun_len - 4);
            vers_line = vers_line.replace("fun ", "");
            globl_functions.push_str(format_args!(".globl {}\n.type {} @function", vers_line, vers_line).to_string().as_str());
        }
        line = line + 1;
    }
    // println!("ASM globl functions:\n{}", globl_functions);
    write_asm_file(format_args!("{}", globl_functions));
    return globl_functions;
}

fn make_functions(vers: File) -> String {
    let reader = BufReader::new(vers);
    let mut functions = String::new();
    for (mut line, mut vers_line) in reader.lines().enumerate() {
        let mut vers_line = vers_line.unwrap();
        if vers_line.contains("fun") {

        }
    }
    return functions;
}

fn generate_assembly(mut vers_file: &String) {
    println!("Generating assembly...");
    let file = File::open(vers_file).unwrap();
    let asm_file_name = vers_file.replace(".vers", ".S");
    unsafe { ASM_FILE = asm_file_name }
    look_for_functions(file.try_clone().unwrap()).as_str();
    make_functions(file.try_clone().unwrap()).as_str();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let option = &args[2];
    if !Path::exists(file.as_ref()) {
        println!("{} - File doesn't exist", file);
        exit(0);
    }

    generate_assembly(&file.clone());

    let output_asm_file = file.replace(".vers", ".S");
    let output_file_name = file.replace(".vers", "");

    Command::new("gcc")
        .arg(output_asm_file)
        .arg("-o")
        .arg(output_file_name)
        .spawn();

    exit(0);
}
