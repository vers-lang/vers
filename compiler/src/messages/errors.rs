use super::ERRORS;

pub(crate) fn compiler_error(message: &str) {
    red!("{}", message);
    unsafe { ERRORS = ERRORS + 1; }
}