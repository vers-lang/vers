use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::path::Path;

pub fn _e(output_name: String, c_file: String, std: String) {

    Command::new("gcc")
        .args(&["-Wall", "-w", c_file.as_str(), std.as_str(), "-o", output_name.as_str()])
        .spawn();

    // Wait one second so there is time to compile before removing the C file
    sleep(Duration::new(1, 0));

    Command::new("rm")
        .args(&["-rf", c_file.as_str()])
        .spawn();
}

pub fn _l(output_name: String, c_file: String, std: String) {
    Command::new("gcc")
        .args(&["-c", "-w", c_file.as_str(), "-o", output_name.as_str()])
        .spawn();

    sleep(Duration::new(1, 0));

    Command::new("rm")
        .args(&["-rf", c_file.as_str()])
        .spawn();
}