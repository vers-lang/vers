use git_rs::Git;
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
    Git::clone(GITHUB, author.as_str(), format_args!("{}{}", "/", name).to_string().as_str());
    Git::checkoutt(name.as_str(), version.as_str());
}
