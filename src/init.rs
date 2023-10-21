use std::io::ErrorKind;
use std::{error::Error, process::Command};

pub fn init() -> Result<(), Box<dyn Error>> {
    Command::new("cargo").args(&["init"]).output()?;
    std::fs::remove_file("src/main.rs")?;
    Command::new("cargo").args(&["add", "aors"]).output()?;

    mkdir("inputs");
    mkdir("input_examples");
    mkdir("answers");
    mkdir("src/bin/helpers");

    Ok(())
}

fn mkdir(path: &str) {
    if let Err(a) = std::fs::create_dir_all(path) {
        if a.kind() != ErrorKind::AlreadyExists {
            eprintln!("\x1b[31m{}\x1b[0m", a);
        }
    }
}
