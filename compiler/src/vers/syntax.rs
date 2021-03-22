use crate::arch::*;
use crate::messages::{errors::compiler_error, messages::*};

pub(crate) fn check_for_semicolon(line_num: i32, word: &str) -> bool {
    if word.contains(";") {
        return true;
    } else {
        compiler_error(line_num, E4V, format_args!("{}", ""));
        return false;
    }
}

pub(crate) fn check_type(typen: &str) -> &str {
    if typen == "char" {
        CHAR
    } else if typen == "int" {
        INT
    } else {
        // compiler_error(line_num, E5V, format_args!("{}", typen));
        return "";
    }
}
