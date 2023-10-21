use std::process::exit;

use confy;
use filenamify::filenamify;
use serde::{Deserialize, Serialize};

const CONFY_NAME: &str = "aors";

#[derive(Default, Debug, Serialize, Deserialize)]
struct YearConfig {
    year: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct CookieConfig {
    cookie: String,
}

pub fn config(year: Option<u32>, cookie: Option<String>) -> (u32, String) {
    let file = std::env::current_dir().unwrap();
    let file = filenamify(file.to_str().unwrap());

    // year needs folder-scope
    let mut year_config: YearConfig = confy::load(CONFY_NAME, file.as_str()).unwrap();
    let mut cookie_config: CookieConfig = confy::load(CONFY_NAME, None).unwrap();

    if let Some(year) = year {
        year_config.year = year;
        confy::store(CONFY_NAME, file.as_str(), &year_config).unwrap();
    }

    if let Some(cookie) = cookie {
        cookie_config.cookie = cookie.to_owned();
        confy::store(CONFY_NAME, None, &cookie_config).unwrap();
    }

    missing_config_msg(year_config.year, &cookie_config.cookie);

    (year_config.year, cookie_config.cookie)
}

fn missing_config_msg(year: u32, cookie: &str) {
    if year == 0 {
        println!(
            "Please enter the year you want to solve for:\n\t\x1b[1maors --year\x1b[0m <YEAR>\n"
        );
    }
    if cookie == "" {
        println!("Please enter a session cookie:\n\t\x1b[1maors --cookie\x1b[0m <COOKIE>");
        println!("This cookie is acquired like so: https://github.com/wimglenn/advent-of-code-wim/issues/1\n");
    }
    if year == 0 || cookie == "" {
        exit(1);
    }
}
