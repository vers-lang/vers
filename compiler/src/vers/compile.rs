use super::asm::*;
use crate::functions::*;
use crate::messages::errors::compiler_error;
use crate::messages::messages::*;
use crate::PROJECT_TYPE;
use std::fmt::Arguments;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::ptr::replace;
use crate::vers::syntax::check_type;
use crate::vers::vers;

static mut LAST_KNOWN_WORD: &str = "";
static mut LINE_NUMBER: i32 = 0;

fn create_function(fun: &str) -> &str {
    println!("Function is {}", fun);
    if fun.contains("()") {
        fun.replace("()", "");
    } else if fun.contains("(var") {
        fun.replace("(var", "");
    }
    println!("Function is {}", fun);
    // Function names that aren't allowed
    // This needs to be written in a better way
    // if fun.contains("(") || fun.contains(")") || fun.contains("!") || fun.contains("@") || fun.contains("#") || fun.contains("$") || fun.contains("%") || fun.contains("^") || fun.contains("&") || fun.contains("*") || fun.contains("-") {
        // unsafe { compiler_error(LINE_NUMBER, E6V, format_args!("{}{}", fun, " isn't a proper way of declaring a function\n")); }
    // }
    fun
}

pub(crate) unsafe fn compile_vers() {
    let mut asm_file = File::create("build/internal/main.S").unwrap();
    let mut line_number = 0;
    let reader = BufReader::new(File::open("src/main.vers").expect("Cannot open main project file"));

    asm_file.write_fmt(format_args!("{}", ".globl main\n"));

    for line in reader.lines() {
        line_number = line_number + 1;
        LINE_NUMBER = line_number;
        for mut word in line.unwrap().split_whitespace() {
            if word == "fun" {
                LAST_KNOWN_WORD = "fun";
            } else if word == "extern" {
                LAST_KNOWN_WORD = "extern";
            } else if word == "asm" {
                LAST_KNOWN_WORD = "asm";
            } else if word == "const" {
                LAST_KNOWN_WORD = "constantname";
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
                asm_file.write_fmt(format_args!("{} {}{}", EXTERN, word.replace(";", "").as_str(), "\n"));
                LAST_KNOWN_WORD = "";
            } else if LAST_KNOWN_WORD == "il_asm" {
                if word.contains("');") {
                    let asm = il_asm(word);
                    asm_file.write_fmt(format_args!("{}{}{}", " ", asm, "\n"));
                    LAST_KNOWN_WORD = " ";
                } else {
                    let asm = il_asm(word);
                    asm_file.write_fmt(format_args!("{}{}", " ", asm));
                }
            } else if LAST_KNOWN_WORD == "constantname" {
                asm_file.write_fmt(format_args!("{}{}{}", "\n", word, "\n"));
                LAST_KNOWN_WORD = "constname";
            } else if LAST_KNOWN_WORD == "constname" {
                asm_file.write_fmt(format_args!("{}", check_type(word)));
            }
            // Ignore
            else if word == "{" || word == "}" {
                // Nothing
            } else {
                println!("\n{}", word);
                compiler_error(LINE_NUMBER, E3V, format_args!("{}", ""));
            }
        }
    }
}
