use std::fs;
use std::io::ErrorKind;

pub fn get(day: u8, year: u32, cookie: &str) {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let day = format!("{:0>2}", day);

    mkdir("inputs");
    mkdir("input_examples");
    mkdir("answers");
    mkdir("src/bin");

    let input = get_req(&url, cookie).expect("Failed to fetch input");
    println!("{}", input);

    touch(
        &format!("input_examples/{}.in", day),
        "",
        "failed to create input file",
    );
    touch(
        &format!("inputs/{}.in", day),
        &input,
        "failed to write input to file",
    );
    touch(
        &format!("answers/{}p1.sol", day),
        "",
        "failed to create answer file",
    );
    touch(
        &format!("answers/{}p2.sol", day),
        "",
        "failed to create answer file",
    );
    touch(
        &format!("src/bin/{}.rs", day),
        &template(&day),
        "failed to create <DAY>.rs file",
    );
}

fn mkdir(path: &str) {
    if let Err(a) = std::fs::create_dir_all(path) {
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

fn touch(path: &str, contents: &str, error_msg: &str) {
    if let Err(_) = fs::File::open(path) {
        fs::write(path, contents).expect(error_msg);
    }
}

fn template(day: &str) -> String {
    return format!(
        "#[aors::main({})]
fn main(input: &str) -> (i32, i32) {{
    (0, 0)
}}
",
        day
    );
}
