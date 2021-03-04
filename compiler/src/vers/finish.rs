use crate::messages::compiler_message;
use crate::PROJECT_TYPE;
use std::fs::copy;
use std::process::Command;

fn link() {
    compiler_message("Linking ", "internal, external, ", "and imports...");
    Command::new("gcc")
    .arg("-c")
    .arg("build/external/*.o")
		.arg("build/imports/*")
		.arg("build/internal/*.o")
		.arg("-o")
		.arg("build/link")
.spawn();
}

fn exe() {
	compiler_message("Compiling,", "exe...", "");
	// Compile link to executable file (.exe)"
	Command::new("gcc")
		.arg("build/link")
		.arg("-o")
		.arg("build/link.exe")
		.spawn();
	copy("build/link.exe", "build/link");
}

fn lib() {
	compiler_message("Compiling ", "lib...", "");
	// Compile link to library (.a)
	Command::new("ar")
		.arg("rcs")
		.arg("build/libproject.a")
		.arg("build/link")
		.spawn();
}

pub(crate) unsafe fn finish_compiling() {
	link();
	if PROJECT_TYPE == "exe" {
		exe();
	} else if PROJECT_TYPE == "lib" {
		lib();
	} else { exe(); }
	compiler_message("Done ", "compiling", "");
}
