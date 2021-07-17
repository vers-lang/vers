use std::process::Command;

pub struct Gcc;

impl Gcc {
    pub fn new() -> Self {
        return Self;
    }

    pub fn object(&mut self) {
        Command::new("gcc")
            .args(&["-c", "~/.vers/stdlib.c"])
            .spawn();
    }
}
