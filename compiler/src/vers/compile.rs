use super::asm::{*};
use crate::PROJECT_TYPE;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};

static mut LAST_KNOWN_WORD: &str = "";

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

pub(crate) unsafe fn compile_vers() -> std::io::Result<()> {
    let mut main_file_name = project_main_file();
    let reader = BufReader::new(File::open(main_file_name).expect("Cannot open main project file"));
    let mut asm_file = File::create("build/internal/main.S")?;

    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            if word == "fun" {
                LAST_KNOWN_WORD = "fun";
            } else if word == "extern" {
                LAST_KNOWN_WORD = "extern";
            }
            else {
                if LAST_KNOWN_WORD == "fun" {
                    asm_file.write_fmt(format_args!("{}{}", word, FUN));
                } else if LAST_KNOWN_WORD == "extern" {
                    asm_file.write_fmt(format_args!("{} {}", EXTERN, word));
                }
            }
        }
    }
    asm_file.write_fmt(format_args!("{}", "\n"))
}
