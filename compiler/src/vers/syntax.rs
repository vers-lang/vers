use crate::arch::*;
use crate::messages::{errors::compiler_error, messages::*};
use crate::vers::compile::LINE_NUMBER;

pub(crate) fn check_for_semicolon(word: &str) -> &str {
    if word.contains(";") {
        "\n"
    } else {
        unsafe { compiler_error(LINE_NUMBER, E4V, format_args!("{}", "")) }
        compiler_error(unsafe { LINE_NUMBER }, E4V, format_args!("{}", "Missing semicolon\n"))
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
