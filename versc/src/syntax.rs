pub const SYMBOLS: [&str; 2] = ["fun", "external"];

pub fn check_line(line: &str) {
    for i in 0..2 {
        if line.starts_with(SYMBOLS[i]) {
            continue;
        } else if line.ends_with(";") {
            break;
        }
    }
}
