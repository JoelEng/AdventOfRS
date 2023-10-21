use std::fs;
use std::io::ErrorKind;

pub fn get(day: u8, year: u32, cookie: &str) {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    create_dir("inputs");
    create_dir("input_examples");
    create_dir("answers");
    create_dir("src/bin");

    let input = get_req(&url, cookie).expect("Failed to fetch input");
    println!("{}", input.to_string());

    create_file(
        &format!("input_examples/{}.in", day),
        "",
        "failed to create input file",
    );
    create_file(
        &format!("inputs/{}.in", day),
        &input,
        "failed to write input to file",
    );
    create_file(
        &format!("answers/{}.sol", day),
        "part one: \npart two: ",
        "failed to create answer file",
    );
    create_file(
        &format!("src/bin/{}.rs", day),
        &template(day),
        "failed to create <DAY>.rs file",
    );
}

fn create_dir(path: &str) {
    if let Err(a) = std::fs::create_dir(path) {
        if a.kind() != ErrorKind::AlreadyExists {
            eprintln!("\x1b[31m{}\x1b[0m", a);
        }
    }
}

fn get_req(url: &str, cookie: &str) -> Result<String, ureq::Error> {
    let body = ureq::get(url)
        .set("Accept", "text/plain")
        .set("Cookie", &format!("session={}", cookie))
        .send_form(&[("session", cookie)])?
        .into_string()?;
    Ok(body)
}

fn create_file(path: &str, contents: &str, error_msg: &str) {
    if let Err(_) = fs::File::open(path) {
        fs::write(path, contents).expect(error_msg);
    }
}

fn template(day: u8) -> String {
    return format!(
        "#[aoc::main({})]
fn main(input: &str) -> (i32, i32) {{
    (0, 0)
}}
",
        day
    );
}
