use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub(crate) fn _e(output_name: String, c_file: String) {
    Command::new("gcc")
        .args(&["-Wall", "-w", c_file.as_str(), "-o", output_name.as_str()])
        .spawn();

    sleep(Duration::new(1, 0));

    Command::new("rm")
        .args(&["-rf", c_file.as_str()])
        .spawn();
}

pub(crate) fn _l(output_name: String, c_file: String) {
    Command::new("gcc")
        .args(&["-c", "-w", c_file.as_str(), "-o", output_name.as_str()])
        .spawn();

    sleep(Duration::new(1, 0));

    Command::new("rm")
        .args(&["-rf", c_file.as_str()])
        .spawn();
}
