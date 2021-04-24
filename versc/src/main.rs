use std::env;
use std::fmt::Arguments;
use std::fs::{remove_file, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{exit, Command};

static mut ASM: String = String::new();

fn make_globl_functions(vers: File) -> String {
    let reader = BufReader::new(vers);
    let mut globl_functions = String::new();
    for (mut line, mut vers_line) in reader.lines().enumerate() {
        let mut vers_line = vers_line.unwrap();
        if vers_line.contains("fun") {
            let fun_len = vers_line.len();
            // Remove "() {"
            vers_line.truncate(fun_len - 4);
            vers_line = vers_line.replace("fun ", "");
            globl_functions.push_str(format_args!(".globl {}\n.type {} @function\n", vers_line, vers_line).to_string().as_str());
        }
        line = line + 1;
    }
    return globl_functions;
}

fn make_function(vers: String) -> String {
    let mut function = String::new();
    let fun_len = function.len();
    function.truncate(fun_len - 4);
    function = vers.replace("fun", "");
    return function;
}

fn generate_assembly(mut vers_file: &String) {
    println!("Generating assembly...");
    // Setup
    let file = File::open(vers_file).unwrap();
    let asm_file_name = vers_file.replace(".vers", ".S");
    let mut asm_file = File::create(asm_file_name).unwrap();
    let functions = make_globl_functions(file.try_clone().unwrap());
    unsafe { ASM.push_str(&*functions); }
    // Main loop
    let mut vers_reader = BufReader::new(file);
    for (mut line_num, mut vers_line) in vers_reader.lines().enumerate() {
        let vers_line = vers_line.unwrap();
        if vers_line.contains("fun") {
            let fun = make_function(vers_line);
            println!("Function name: {}", fun);
            unsafe {
                ASM.push_str("\n");
                ASM.push_str(&*fun);
                ASM.push_str(":\n");
            }
        } else {
            println!("Unknown instruction: {}", vers_line);
        }
    }

    unsafe { println!("Writing:\n{}", ASM); }
    unsafe { asm_file.write_fmt(format_args!("{}", ASM)); }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let option = &args[2];
    if !Path::exists(file.as_ref()) {
        println!("{} - File doesn't exist", file);
        exit(0);
    }

    let mut asm_file_name = file.replace(".vers", ".S");
    File::create(asm_file_name);
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
