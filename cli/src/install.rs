use git_management::GitCommands;
use std::io::{Write, stdin, stdout};
use std::process::Command;

const GITHUB: &str = "https://github.com/";

fn input() -> String {
    let mut uinput = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut uinput).expect("Did not enter a correct string");
    if let Some('\n') = uinput.chars().next_back() {
        uinput.pop();
    }
    if let Some('\r') = uinput.chars().next_back() {
        uinput.pop();
    }
    return uinput;
}

pub fn main() {
    print!("Library name: ");
    let mut name = input();
    print!("Library author: ");
    let mut author = input();
    print!("Library version: ");
    let version = input();
    println!("Installing {} from {}...", name, GITHUB);
    GitCommands::clone(GITHUB, author.as_str(), name.as_str());
    println!("Sudo password is needed to add library");
    Command::new("sudo")
        .arg("mv")
        .arg(name.as_str())
        .arg("~/.vers/")
        .spawn();
}
