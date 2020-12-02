use std::fs;
use regex::Regex;

fn main() {
    let pattern = Regex::new(r#"(\d+)-(\d+) (\w): (\w*)"#).unwrap();

    let mut valid = 0;

    read_input().iter()
        .map(|line| pattern.captures_iter(line).next().unwrap())
        .for_each(|groups| {
            let min: usize = groups[1].parse().unwrap();
            let max: usize = groups[2].parse().unwrap();
            let char = &groups[3].chars().next().unwrap();
            let password = &groups[4];

            let count = password.chars().filter(|c| c.eq(char)).count();
            if count >= min && count <= max {
                valid += 1;
            }
        });

    println!("Valid passwords: {}", valid);
}

struct Password {
    min: i32,
    max: i32,
    char: char,
    password: String,
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|s| s.to_owned())
        .collect()
}
