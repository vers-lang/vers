// Symbols and punctuation
const NUM_SYMBOLS: usize = 5;
const NUM_PUNCTUATION: usize = 5;
pub const SYMBOLS: [&str; NUM_SYMBOLS] = ["fun", "external", "\"", "\"", "var:"];
pub const PUNCTUATION: [&str; NUM_PUNCTUATION] = ["{", "}", ";", "(", ")"];

// Is the symbol or punctuation recognized? (These statics were normal variables but kept getting reset.)
static mut KNOWN_SYMBOL: bool = false;
static mut KNOWN_PUNCTUATION: bool = false;

fn check_punctuation(line: &str) {
    for i in 0..NUM_PUNCTUATION {
        if line.ends_with(PUNCTUATION[i]) {
            unsafe { KNOWN_PUNCTUATION = true }
        }
    }
}

pub fn check_line(line: &str) -> i32 {
    let mut known_symbol = false;
    let mut errors = 0;

    for i in 0..NUM_SYMBOLS {
        if line.starts_with(SYMBOLS[i]) {
            unsafe { KNOWN_SYMBOL = true; }
        }
    }

    unsafe {
        if !KNOWN_SYMBOL {
            yellow_ln!("WARNING: Unknown symbol, this might cause an error later");
            green!("// TODO: ");
            print!("Make syntax checker better so this warning doesn't come up\n");
        }

        check_punctuation(line);

        if !KNOWN_PUNCTUATION {
            errors = errors + 1;
            red_ln!("ERROR: Unknown punctuation at the end of the line:");
            println!("\t{}\n", line);
        }
    }

    unsafe {
        KNOWN_SYMBOL = false;
        KNOWN_PUNCTUATION = false;
    }
    return errors;
}
