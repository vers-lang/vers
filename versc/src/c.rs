use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::types::Action;
use crate::OUTPUT;

pub(crate) unsafe fn translate_to_c(vers_file: &String) {
    let mut reader = BufReader::new(File::open(vers_file).expect("Can't find Vers input file"));

    for line in reader.lines() {
        let mut vers_line = line.unwrap();
        let line_len = vers_line.len();

        vers_line = vers_line.replace("fun", "int").replace("var: int", "int").replace("var: string", "char[99999]");
        OUTPUT.push_str(vers_line.as_str());
    }
}
