use std::process::{Command};

pub fn main() {
    println!("Creating new Vers project...");
    Command::new("vers_new")
        .spawn();
}