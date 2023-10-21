use clap::Parser;
mod config;

/// Useful rs tools for Advent of Code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// One or more specific days to run
    days: Vec<u8>,
    /// Run with example input
    #[arg(short = 'x', long)]
    example: bool,
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

    for d in args.days {
        println!("DAY: {}", d);
    }
}
