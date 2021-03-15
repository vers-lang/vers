use super::ERRORS;
use std::fmt::Arguments;

pub(crate) fn error(message: &str) {
    red!("{}", message);
    unsafe { ERRORS = ERRORS + 1; }
}
pub fn compiler_error(line: i32, message: &str, extra_arg: Arguments) {
    red!("{}: {} {}", line, message, extra_arg);
    unsafe { ERRORS = ERRORS + 1; }
}
