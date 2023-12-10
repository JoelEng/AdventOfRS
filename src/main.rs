use clap::Parser;
mod commands;
mod config;
mod get;
mod init;
mod post;
mod run;

// Also needs to be changed in lib.rs
const ANSWER_FOLDER: &str = ".answers";

/// Useful rs tools for Advent of Code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// One or more specific days to run
    days: Vec<u8>,
    /// Create a new project at current location
    #[arg(long)]
    init: bool,
    /// Run with example input
    #[arg(short = 'x', long)]
    example: bool,
    /// Retrieve input data and create files for [DAYS]
    #[arg(short, long)]
    get: bool,
    /// Post solution for given days to https://adventofcode.com/<YEAR>
    #[arg(short, long)]
    post: bool,
    /// Set year to solve
    #[arg(long)]
    year: Option<u32>,
    /// Set session cookie, acquired like so: https://github.com/wimglenn/advent-of-code-wim/issues/1
    #[arg(long)]
    cookie: Option<String>,
    /// Print year and session cookie
    #[arg(short, long)]
    info: bool,
    /// Saves answers. This is done automatically when posting an answer, but can be done manually as well.
    #[arg(short, long)]
    save_answers: bool,
}

fn main() {
    let args = Args::parse();
    let mut quiet = false;

    if args.init {
        init::init().ok();
        println!("\x1b[1mCreated Advent of Code project. Use \x1b[4maors -g 01\x1b[0;1m to fetch input for day 01");
        return;
    }

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
            quiet = true;
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

    let mut total_time = 0;

    for day in days.clone() {
        let day_str = format!("{:0>2}", day);
        if args.get {
            get::get(day, year, &cookie);
            continue;
        }
        if let Some((p1, p2, time)) = run::run_day(&day_str, args.example, quiet) {
            total_time += time;
            if args.post {
                post::post(day, year, args.example, &cookie, &p1, &p2);
            }
            if args.save_answers {
                commands::save_answer(&day_str, 1, &p1);
                commands::save_answer(&day_str, 2, &p2);
            }
        }
    }

    print!("\x1b[4;1m");
    let days_completed = days.len();
    if days_completed == 25 {
        println!(
            "\n🎄 All days completed! 🎄 Total time: {}ms\x1b[0m",
            total_time / 1000
        );
    } else if days_completed > 1 {
        println!(
            "{} days completed in {}ms\x1b[0m",
            days_completed,
            total_time / 1000
        );
    }
}
