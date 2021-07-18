const NUM_SYMBOLS: usize = 4;
const NUM_PUNCTUATION: usize = 5;
pub const SYMBOLS: [&str; NUM_SYMBOLS] = ["fun", "external", "\"", "\""];
pub const PUNCTUATION: [&str; NUM_PUNCTUATION] = ["{", "}", ";", "(", ")"];


pub fn check_line(line: &str) -> i32 {
    let mut print_warning = true;
    let mut print_error = true;
    let mut errors = 0;

    for i in 0..NUM_SYMBOLS {
        if line.starts_with(SYMBOLS[i]) {
            print_warning = false;
        } else {
            for i in 0..NUM_PUNCTUATION {
                if line.ends_with(PUNCTUATION[i]) {
                    print_error = false;
                    errors = errors + 1;
                }
            }
        }
    }

    if print_warning {
        yellow_ln!("WARNING: Unknown symbol");
        green!("// TODO:");
        print!(" Fix this part of the syntax checker");
    } else if print_error {
        red_ln!("Missing punctuation");
    }

    return errors;
}
