use crate::commands::save_answer;
use fancy_regex::Regex;
use std::path::Path;
use std::process::exit;

const TOO_FAST: &str = "(You gave an answer too recently.*to wait.)";
const INCORRECT: &str = r"(That's not the right answer[^\.]*.)";
const ALREADY_DONE: &str = r"(You don't seem to be solving.*\.)";
const CORRECT: &str = "(That's the right answer!)";

pub fn post(day: u8, year: u32, example_input: bool, cookie: &str, p1: &str, p2: &str) {
    let day_str = format!("{:0>2}", day);

    if day < 1 || day > 25 {
        eprintln!("\x1b[31;1mIncorrect day. Should be between 1 and 25\x1b[0m");
        exit(1);
    }

    if !Path::new(&format!("src/bin/{}.rs", day_str)).exists() {
        eprintln!("\x1b[31;1mYou do not have a solution for this day\x1b[0m");
        exit(1);
    }

    let ans1 = std::fs::read_to_string(format!("answers/{}p1.sol", day_str))
        .expect("unable to find answer file");
    let ans2 = std::fs::read_to_string(format!("answers/{}p2.sol", day_str))
        .expect("unable to find answer file");

    let part = if ans1 == "" { 1 } else { 2 };

    if part == 2 && ans2 != "" {
        println!(
            "⭐ \x1b[103;30mYou've already solved day {}!\x1b[0m ⭐",
            day_str
        );
        return;
    }

    if example_input {
        eprintln!("\x1b[41;30mTried to submit with example input\x1b[0m");
        exit(1);
    }

    let answer = match part {
        1 => p1,
        _ => p2,
    };

    println!(
        "\x1b[4;1mPosting {} to day {} part {} ({})\x1b[0m\n",
        answer, day_str, part, year
    );

    let html = post_req(year, day, answer, part, cookie);

    for err in [TOO_FAST, INCORRECT, ALREADY_DONE] {
        let err_re = Regex::new(err).unwrap();
        if let Ok(c) = err_re.captures(&html) {
            if let Some(c) = c {
                eprintln!("\x1b[41;30m{}\x1b[0m", c.get(1).unwrap().as_str());
            }
        }
    }

    let corr_re = Regex::new(CORRECT).unwrap();
    if let Ok(c) = corr_re.captures(&html) {
        if let Some(c) = c {
            println!("\x1b[102;30m{}\x1b[0m", c.get(1).unwrap().as_str());
            save_answer(&day_str, part, answer);
        }
    }
}

fn post_req(year: u32, day: u8, answer: &str, part: i32, cookie: &str) -> String {
    ureq::post(&format!(
        "https://adventofcode.com/{}/day/{}/answer",
        year, day
    ))
    .set("Cookie", &format!("session={}", cookie))
    .send_form(&[("answer", answer), ("level", &part.to_string())])
    .unwrap()
    .into_string()
    .unwrap()
}
