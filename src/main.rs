use reqwest::blocking::Client;
use std::env;

pub fn get_input(day: u8) -> String {
    let url = reqwest::Url::parse(&std::format!(
        "https://adventofcode.com/2022/day/{}/input",
        day,
    ))
    .expect("Failed to parse url");

    let session = env::var("AOC_SESSION").expect("Please set the AOC_SESSION env variable");

    let client = Client::new();

    client
        .get(url)
        .header("Cookie", std::format!("session={}", session))
        .send()
        .expect("Request failed")
        .text()
        .expect("Getting reqwest body failed")
}

fn main() {
    let input = get_input(1);
    println!("{}", input);
}
