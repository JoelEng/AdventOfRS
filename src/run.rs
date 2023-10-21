use fancy_regex::Regex;
use std::{error::Error, process::Command};

pub fn run_day(day_str: &str, example_input: bool) -> Option<(String, String, usize)> {
    let part_one: Regex = Regex::new(r"Part one: ([^\n]+)").ok()?;
    let part_two: Regex = Regex::new(r"Part two: ([^\n]+)").ok()?;
    let mut args = vec!["run", "--release", "--bin", day_str, day_str];
    if example_input {
        args.push("example");
    }

    let cmd = Command::new("cargo").args(args).output().ok()?;
    let output = String::from_utf8(cmd.stdout).ok()?;
    if output != "" {
        println!("{}", output);
    }
    let p1 = get_answer(part_one, &output)?;
    let p2 = get_answer(part_two, &output)?;
    let time = extract_microseconds(&output).ok()?;
    Some((p1, p2, time))
}

fn get_answer(r: Regex, text: &str) -> Option<String> {
    let ans = r.captures(text).ok()??.get(1)?.as_str().to_string();
    let ansi_escape = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").ok()?;
    Some(ansi_escape.replace_all(&ans, "").to_string())
}

fn extract_microseconds(output: &str) -> Result<usize, Box<dyn Error>> {
    let out = output.lines().last().expect(
        "Execution failed. Make sure all existing DAY.rs files have corresponding DAY.in files",
    );
    let ansi_escape = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
    let out = ansi_escape.replace_all(out, "").to_string();
    let time = if out.ends_with("ms") {
        out["Time: ".len()..out.len() - 2].parse::<usize>()? * 1000
    } else {
        out["Time: ".len()..out.len() - 3].parse::<usize>()?
    };
    Ok(time)
}
