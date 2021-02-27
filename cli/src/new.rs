use std::process::{Command};
use std::result::{Result};

pub fn main() {
    println!("Creating new Vers project...");
    Command::new("verspy")
        .arg("new")
        .spawn();
    println!("Created new Vers project")
}
