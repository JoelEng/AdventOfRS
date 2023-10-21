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

    if year_config.year == 0 {
        println!("Please enter a year. aors -h for more help.")
    }

    if cookie_config.cookie == "" {
        println!("Please enter an aoc session cookie. aors -h for more help.")
    }

    (year_config.year, cookie_config.cookie)
}
