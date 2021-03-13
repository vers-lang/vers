use crate::arch::*;
use crate::messages::{errors::compiler_error, messages::*};

pub(crate) fn check_for_semicolon(word: &str) -> bool {
    if word.contains(";") {
        return true;
    } else {
        compiler_error(E4V);
        return false;
    }
}

pub(crate) fn check_type(typen: &str) -> &str {
    if typen == "char" {
        CHAR
    } else if typen == "int" {
        INT
    } else {
        println!("\n{}", typen);
        compiler_error(E5V);
        return "";
    }
}
