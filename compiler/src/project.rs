use crate::{exe, lib};
use crate::messages::{compiler_message, errors::compiler_error, messages::*};
use serde_json::{from_reader, Value};
use std::fs::{File};
use std::path::Path;
use crate::messages::warnings::compiler_warning;

static mut SCORE: i32 = 0;

unsafe fn exists() {
    if Path::new("project.json").exists() {
        SCORE = SCORE + 1;
    } else {
        compiler_error(E1V);
    }
}

unsafe fn project_type() {
    if Path::new("project.json").exists() {
        let project_file = File::open("project.json")
            .expect("Couldn't open project.json file");
        let json_project_file: Value = from_reader(project_file)
            .expect("Couldn't read Json in file");
        let project_type = json_project_file.get("type")
            .expect("Couldn't read type from project.json");
        let formated_project_type = project_type.as_str().unwrap();
        if formated_project_type == "exe" {
            exe();
            SCORE = SCORE + 1;
        } else if formated_project_type == "lib" {
            lib();
            SCORE = SCORE + 1;
        } else {
            compiler_warning(W1V);
            compiler_message("Can't read ","project type ", "defaulting to exe...");
            exe();
            SCORE = SCORE + 1;
        }
    }
}

pub(crate) fn init() {
    unsafe {
        exists();
        // If it doesn't exist check again anyways
        // Just in case it somehow just shows up
        project_type();
        compiler_message("Project init score = ", SCORE.to_string().as_str(), "/3");
    }
}
