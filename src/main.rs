use clap::Parser;
use fancy_regex::Regex;
use std::process::{Command, Stdio};
mod config;
mod get;
mod post;

/// Useful rs tools for Advent of Code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// One or more specific days to run
    days: Vec<u8>,
    /// Run with example input
    #[arg(short = 'x', long)]
    example: bool,
    /// Retrieve input data and create files for [DAYS]
    #[arg(short, long)]
    get: bool,
    /// Post solution for given days to https://adventofcode.com/<YEAR>
    #[arg(short, long)]
    post: bool,
    /// Print year and session cookie
    #[arg(short, long)]
    info: bool,
    /// Set year to solve
    #[arg(long)]
    year: Option<u32>,
    /// Set session cookie, acquired like so: https://github.com/wimglenn/advent-of-code-wim/issues/1
    #[arg(long)]
    cookie: Option<String>,
}

fn main() {
    let args = Args::parse();
    let (year, cookie) = config::config(args.year, args.cookie);

    if args.info {
        println!("\x1b[1;4myear:\x1b[0m {}", year);
        println!("\x1b[1;4msession cookie:\x1b[0m {}", &cookie);
        return;
    }

    if args.example {
        println!("\x1b[103;30m   USING EXAMPLE INPUT   \x1b[0m");
    }

    let days = match args.days.len() {
        0 => {
            let mut d: Vec<u8> = std::fs::read_dir("src/bin/")
                .unwrap()
                .filter_map(|p| p.ok()?.path().file_stem()?.to_str().map(str::to_string))
                .filter_map(|f| f.parse().ok())
                .collect();
            d.sort();
            d
        }
        _ => args.days,
    };

    for day in days {
        let day_str = format!("{:0>2}", day);
        if args.get {
            get::get(day, year, &cookie);
            continue;
        }
        if let Some((p1, p2)) = run_day(&day_str, args.example) {
            if args.post {
                post::post(day, year, args.example, &cookie, &p1, &p2);
            }
        }
    }
}

fn run_day(day_str: &str, example_input: bool) -> Option<(String, String)> {
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
