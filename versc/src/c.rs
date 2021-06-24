use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::{ERRORS, OUTPUT};

pub(crate) unsafe fn translate_to_c(vers_file: &String) {
    let mut reader = BufReader::new(File::open(vers_file).expect("Can't find Vers input file"));
    let mut lines = 0;
    let mut in_fun = false;

    for line in reader.lines() {
        lines = lines + 1;
        let mut vers_line = line.unwrap();

        vers_line = vers_line.replace("fun", "void").replace("var: int", "int").replace("var: string", "char[99999]").replace("external", "extern");
        if vers_line.contains("{") {
            in_fun = true;
        } else if vers_line.contains("}") {
            in_fun = false;
        }

        if !vers_line.contains("{") {
            if !vers_line.trim().is_empty() {
                if !vers_line.contains(";") {
                    ERRORS = true;
                    red_ln!("Line: {} needs a semicolon", lines);
                }
            }
        }

        OUTPUT.push_str(vers_line.as_str());
        OUTPUT.push_str("\n");
    }
}
