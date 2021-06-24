use std::process::Command;

pub(crate) fn _e(output_name: String, c_file: String) {
    Command::new("gcc")
        .args(&["-Wall", c_file.as_str(), "-o", output_name.as_str()])
        .spawn();
}

pub(crate) fn _l(output_name: String, c_file: String) {

}