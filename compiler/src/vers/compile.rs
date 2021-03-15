use super::asm::*;
use crate::functions::*;
use crate::messages::errors::compiler_error;
use crate::messages::messages::*;
use crate::PROJECT_TYPE;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::ptr::replace;
use crate::vers::syntax::check_type;

pub(crate) unsafe fn compile_vers() {
    let mut asm_file = File::create("build/internal/main.S").unwrap();
    let mut line_number = 0;
    let reader = BufReader::new(File::open("src/main.vers").expect("Cannot open main project file"));

    for line in reader.lines() {
        line_number = line_number + 1;
        println!("{}: {}", line_number, line.unwrap());
    }
}
