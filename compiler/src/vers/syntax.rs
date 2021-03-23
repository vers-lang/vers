use crate::arch::*;
use crate::messages::{errors::compiler_error, messages::*};
use crate::vers::compile::LINE_NUMBER;

pub(crate) fn check_for_semicolon(word: &str) -> bool {
    if word.contains(";") {
        return true;
    } else {
        unsafe { compiler_error(LINE_NUMBER, E4V, format_args!("{}", "")) }
        return false;
    }
}

pub(crate) fn check_type(typen: &str) -> &str {
    if typen == "char" {
        CHAR
    } else if typen == "int" {
        INT
    } else {
        unsafe { compiler_error(LINE_NUMBER, E5V, format_args!("{}", typen)) }
        return "";
    }
}
