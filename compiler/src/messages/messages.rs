// Vers errors
pub const E1V: &'static str = "E1V: Cannot find project.json file\n";
pub const E2V: &'static str = "E2V: Cannot find project main file (main.vers/lib.vers)\n";
pub const E3V: &'static str = "E2V: Unknown instruction\n";

// Vers warnings
pub const W1V: &'static str = "W1V: Cannot read project.json file\n";

// Hardware errors
pub const E1H: &'static str = "E1H: CPU architecture not supported\n";

// OS errors
pub const E1O: &'static str = "E1O: OS not supported\n";
pub const E2O: &'static str = "E2O: OS not recognised by Vers\n";

// OS warnings
pub const W1O: &'static str = "W1O: Unix/GNU like OS may not run everything Vers can on Linux\n";