use fancy_regex::Regex;
use std::process::{Command, Stdio};

pub fn run_day(day_str: &str, example_input: bool) -> Option<(String, String)> {
    let part_one: Regex = Regex::new(r"Part one: ([^\n]+)").ok()?;
    let part_two: Regex = Regex::new(r"Part two: ([^\n]+)").ok()?;
    let mut args = vec!["run", "--release", "--bin", day_str];
    if example_input {
        args.push("example");
    }

    let cmd = Command::new("cargo")
        .args(args)
        .stdout(Stdio::piped())
        .output()
        .ok()?;
    let output = String::from_utf8(cmd.stdout).ok()?;
    if output != "" {
        println!("{}", output);
    }
    let p1 = get_answer(part_one, &output)?;
    let p2 = get_answer(part_two, &output)?;
    Some((p1, p2))
}

fn get_answer(r: Regex, text: &str) -> Option<String> {
    let ans = r.captures(text).ok()??.get(1)?.as_str().to_string();
    let ansi_escape = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").ok()?;
    Some(ansi_escape.replace_all(&ans, "").to_string())
}
