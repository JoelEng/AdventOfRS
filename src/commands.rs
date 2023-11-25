use std::ffi::OsStr;
use std::fs;
use std::io::{BufRead, BufReader, ErrorKind};
use std::process::{Command, Stdio};

/// Save an answer to the specified day & part
pub fn save_answer(day_str: &str, part: i32, answer: &str) {
    let ans_path = format!("answers/{}p{}.sol", day_str, part);
    write(&ans_path, answer, "could not save answer to answers file");
}

/// Run a terminal command with the provided arguments, returning the output
pub fn cmd<I, S>(cmd: &str, args: I) -> Option<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = command.stdout.take().unwrap();

    let mut bufread = BufReader::new(stdout);
    let mut buf = String::new();
    let mut output = String::new();

    while let Ok(n) = bufread.read_line(&mut buf) {
        if n > 0 {
            println!("{}", buf.trim_end());
            output.push_str(&buf);
            buf.clear();
        } else {
            break;
        }
    }
    Some(output)
}

/// Write to a file if it does not already exist
pub fn touch(path: &str, contents: &str, error_msg: &str) {
    if let Err(_) = fs::File::open(path) {
        fs::write(path, contents).expect(error_msg);
    }
}

/// Write to a file, overwriting current contents and creating a new file if it does not exist
pub fn write(path: &str, contents: &str, error_msg: &str) {
    fs::write(path, contents).expect(error_msg);
}

/// Create a directory if it does not already exist
pub fn mkdir(path: &str) {
    if let Err(a) = fs::create_dir_all(path) {
        if a.kind() != ErrorKind::AlreadyExists {
            eprintln!("\x1b[31m{}\x1b[0m", a);
        }
    }
}
