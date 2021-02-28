use super::WARNINGS;

pub(crate) fn compiler_warning(message: &str) {
    yellow!("{}", message);
    unsafe { WARNINGS = WARNINGS + 1; }
}
