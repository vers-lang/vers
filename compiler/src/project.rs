use crate::{exe, exit_compiler, lib, PROJECT_TYPE, PROJECT_NAME, set_project_name};
use crate::messages::{compiler_message, errors::{compiler_error, error}, messages::*};
use serde_json::{from_reader, Value};
use std::fs::{File};
use std::path::Path;
use crate::messages::warnings::compiler_warning;
use std::borrow::Borrow;
use std::io::Write;

static mut SCORE: i32 = 0;

unsafe fn exists() {
    if Path::new("project.json").exists() {
        SCORE = SCORE + 1;
    } else {
        error(E1V);
    }
}

unsafe fn project_type() {
    let project_file = File::open("project.json")
        .expect("Couldn't open project.json file");
    let json_project_file: Value = from_reader(project_file)
        .expect("Couldn't read Json in file");
    let project_type = json_project_file.get("type")
        .expect("Couldn't read type from project.json");


    if project_type.to_owned().as_str().unwrap() == "exe" {
        exe();
        SCORE = SCORE + 1;
    } else if project_type.to_owned().as_str().unwrap() == "lib" {
        lib();
        SCORE = SCORE + 1;
    } else {
        compiler_warning(W1V);
        compiler_message("Can't read ","project type ", "defaulting to exe...");
        exe();
        SCORE = SCORE + 1;
    }
}

pub(crate) unsafe fn project_name() {
    let project_file = File::open("project.json")
        .expect("Couldn't open project.json file");
    let json_project_file: Value = from_reader(project_file)
        .expect("Couldn't read Json in file");
    let project_name = json_project_file.get("name")
        .expect("Couldn't read project name from project.json");
    if project_name.to_owned().as_str().unwrap() != " " {
        println!("project_name = {}", project_name);
    }
}

unsafe fn project_info() {
    if Path::new("project.json").exists() {
        project_type();
    } else {
        error(E1V);
    }
}

unsafe fn files_exist() {
    if Path::new("src/main.vers").exists() {
        SCORE = SCORE + 1;
    } else {
        error(E2V);
        exit_compiler();
    }
}

pub(crate) fn init() {
    unsafe {
        exists();
        // If it doesn't exist check again anyways
        // Just in case it somehow just shows up
        project_type();
        // see if main file[s] exist
        files_exist();
        compiler_message("Project init score = ", SCORE.to_string().as_str(), "/4");
    }
}
