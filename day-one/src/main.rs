use std::fs;

fn main() {
    let input = read_input();

    'outer: for num1 in &input {
        for num2 in &input {
            if num1 + num2 == 2020 {
                println!("{} * {} = {}", num1, num2, num1 * num2);
                break 'outer;
            }
        }
    }
}

fn read_input() -> Vec<i32> {
    fs::read_to_string("input.txt")
        .expect("Failed to read input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}