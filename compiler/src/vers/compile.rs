use super::asm::{asm::*};
use crate::functions::*;
use crate::messages::errors::compiler_error;
use crate::messages::messages::*;
use crate::PROJECT_TYPE;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::ptr::replace;

static mut LAST_KNOWN_WORD: &str = "";
static mut ASM_LINE: &str = "";

fn project_main_file() -> &'static str {
    "src/main.vers"
}

unsafe fn t_asm_file() -> &'static str {
    "build/internal/main.S"
}

pub(crate) unsafe fn compile_vers() {
    let mut main_file_name = project_main_file();
    let reader = BufReader::new(File::open(main_file_name).expect("Cannot open main project file"));
    let mut asm_file = File::create("build/internal/main.S").unwrap();
    asm_file.write(b".globl main\n\n");

    for line in reader.lines() {
        for mut word in line.unwrap().split_whitespace() {
            // For debugging
            println!("word = {}\nLAST_UKNOWN_WORD = {}", word, LAST_KNOWN_WORD);
            // word
            if word == "fun" {
                LAST_KNOWN_WORD = "fun";
            } else if word == "extern" {
                LAST_KNOWN_WORD = "extern";
            } else if word == "asm" {
                LAST_KNOWN_WORD = "asm";
            }
            // Compiler functions
            else if word.contains("il_asm") {
                let asm = il_asm(word);
                asm_file.write_fmt(format_args!("{}", "    "));
                asm_file.write_fmt(format_args!("{}", asm));
                LAST_KNOWN_WORD = "il_asm";
            }
            // LAST_KNOWN_WORD
            else if LAST_KNOWN_WORD == "fun" {
                if word == "main".replace("()", "") {
                    asm_file.write_fmt(format_args!("{}{}{}{}", ".globl main\n\n", "main", FUN, "\n"));
                } else {
                    asm_file.write_fmt(format_args!("{}{}{}", word.replace("()", ""), FUN, "\n"));
                }
                LAST_KNOWN_WORD = "";
            } else if LAST_KNOWN_WORD == "extern" {
                asm_file.write_fmt(format_args!("{} {}", EXTERN, word.replace(";", "").as_str()));
                LAST_KNOWN_WORD = "";
            } else if LAST_KNOWN_WORD == "il_asm" {
                if word.contains("');") {
                    let asm = il_asm(word);
                    asm_file.write_fmt(format_args!("{}{}", asm, "\n"));
                    LAST_KNOWN_WORD = " ";
                } else {
                    let asm = il_asm(word);
                    asm_file.write_fmt(format_args!("{}{}", " ", asm));
                }
            }
            // Ignore
            else if word == "{" || word == "}" {
                // Nothing
            } else {
                    println!("\n{}", word);
                    compiler_error(E3V);
            }
        }
    }
}
