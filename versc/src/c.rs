use std::fs::File;
use std::io::{BufReader, BufRead, Write};

/* stdlib functions as external functions so you can call them easier
 * this just breakes stuff but in the future this might help
 * const STD_FUNCTIONS: &str = "extern fun pintln(var: string text);\nexternal fun square(var: int num);\n";
 */

pub fn translate_to_c(vers_file: &String, link_std: bool) {
    let mut reader = BufReader::new(File::open(vers_file).expect("Can't find Vers input file"));
    let mut lines = 0;
    let mut in_fun = false;

    let output_file_name = vers_file.replace(".vers", ".c");
    let mut output_file = File::create(output_file_name).unwrap();

    for line in reader.lines() {
        lines = lines + 1;
        let mut vers_line = line.unwrap();

        vers_line = vers_line.replace("fun", "void").replace("var: int", "int").replace("var: string ", "char *").replace("external", "extern").replace("var: char", "char");
        if vers_line.contains("{") {
            in_fun = true;
        } else if vers_line.contains("}") {
            in_fun = false;
        }


        vers_line = vers_line.replace("};", "}");

        output_file.write_fmt(format_args!("{}{}", vers_line, "\n"));
    }
}
