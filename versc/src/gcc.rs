use std::process::Command;

pub struct Gcc {
    pub vers_std: bool,
    pub file: String,
}

impl Gcc {
    pub fn new(&mut self) -> &mut Self {
        return self;
    }

    pub fn object(&mut self) {
        let mut vers_std_file = "";

        if self.vers_std {
            vers_std_file = "~/.vers/vstd";
        } else {
            vers_std_file = "~/.vers/nvstd";
        }

        Command::new("gcc")
            .args(&["-c", "~/.vers/stdlib.c", vers_std_file, self.file.replace(".vers", ".c").as_str(), "-o", self.file.replace(".vers", "").as_str()])
            .spawn();
    }

    pub fn bin(&mut self) {
        let mut vers_std_file = "";

        if self.vers_std {
            vers_std_file = "~/.vers/vstd";
        } else {
            vers_std_file = "~/.vers/nvstd";
        }

        Command::new("gcc")
            .args(&["-Wall", "~/.vers/stdlib.c", vers_std_file, self.file.replace(".vers", ".c").as_str(), "-o", self.file.replace(".vers", "").as_str()])
            .spawn();
    }
}
