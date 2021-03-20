use std::fs::{read_to_string};

pub fn main() {
    let version = read_to_string("/lib/vers/vers/version").expect("Couldn't read /lib/vers/version");
    println!("Vers current version: {}", version);
}
