use std::{error::Error, process::Command};

pub fn init() -> Result<(), Box<dyn Error>> {
    Command::new("cargo").args(&["init"]).output()?;
    std::fs::remove_file("src/main.rs")?;
    Command::new("cargo").args(&["add", "aors"]).output()?;
    Ok(())
}
