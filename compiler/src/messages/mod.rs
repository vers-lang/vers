pub mod errors;
pub mod messages;
pub mod warnings;

pub(crate) static mut ERRORS: i32 = 0;
pub(crate) static mut WARNINGS: i32 = 0;

pub(crate) fn compiler_message(message: &str, arg1: &str, arg2: &str) {
    green!("{}", message);
    blue!("{}", arg1);
    println!("{}", arg2);
}
