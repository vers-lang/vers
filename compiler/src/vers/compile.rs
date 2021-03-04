use super::asm::{asm::*};
use crate::PROJECT_TYPE;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::ptr::replace;
use std::fmt::Debug;
use crate::messages::errors::compiler_error;
use crate::messages::messages::E3V;

static mut LAST_KNOWN_WORD: &str = "";
static mut ASM_LINE: &str = "";
static mut WRITE_ASM: bool = false;

fn project_main_file() -> &'static str {
    unsafe {
        if PROJECT_TYPE == "exe" {
            "src/main.vers"
        } else if PROJECT_TYPE == "lib" {
            "src/lib.vers"
        } else { "src/main.vers" }
    }
}

unsafe fn t_asm_file() -> &'static str {
    if PROJECT_TYPE == "exe" {
        "build/internals/main.S"
    } else if PROJECT_TYPE == "lib" {
        "build/internals/lib.S"
    } else {
        "build/internals/main.S"
    }
}

unsafe fn add_asm(asm: &str) -> i32 {
    if asm.contains("'") && WRITE_ASM == false {
        ASM_LINE.to_string().push_str("\n   ");
        ASM_LINE.to_string().push_str(asm.replace("'", "").as_str());
        WRITE_ASM = true;
        return 0
    } else if asm.contains("';") {
        ASM_LINE.to_string().push_str(asm.replace("'", "").as_str());
        WRITE_ASM = false;
        return 1
    } else {
        ASM_LINE.to_string().push_str(asm);
        return 0;
    }
}

pub(crate) unsafe fn compile_vers() {
    let mut main_file_name = project_main_file();
    let reader = BufReader::new(File::open(main_file_name).expect("Cannot open main project file"));
    let mut asm_file = File::create("build/internal/main.S").unwrap();

    asm_file.write(b".globl main\n\n");

    for line in reader.lines() {
        for mut word in line.unwrap().split_whitespace() {
            println!("word = {}\nLAST_UKNOWN_WORD = {}", word, LAST_KNOWN_WORD);
            // word
            if word == "fun" {
                LAST_KNOWN_WORD = "fun";
            } else if word == "extern" {
                LAST_KNOWN_WORD = "extern";
            } else if word == "asm" {
                if add_asm(word) == 0 {
                    LAST_KNOWN_WORD = "asm";
                }
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
            } else if LAST_KNOWN_WORD == "asm" {
                if add_asm(word) == 1 {
                    asm_file.write_fmt(format_args!("{}", ASM_LINE));
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
