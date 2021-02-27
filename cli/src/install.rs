use std::process::{Command};

pub fn main() {
    Command::new("verspy")
        .arg("install")
        .spawn();
}
