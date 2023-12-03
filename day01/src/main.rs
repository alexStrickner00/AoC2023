use std::fs::read_to_string;

fn main() {
    let input = get_input("input.txt");
    let numbers = input
        .iter()
        .map(|row| first_and_last_digit(row))
        .map(digits_to_num);
    let sum: u32 = numbers.sum();
    println!("sum: {}", sum);
}

fn digits_to_num(digits: (u32, u32)) -> u32 {
    digits.0 * 10 + digits.1
}

fn first_and_last_digit(seq: &str) -> (u32, u32) {
    let digits: Vec<u32> = seq.chars().flat_map(|d| d.to_digit(10)).collect();
    return (
        *digits.first().expect("no digit found"),
        *digits.last().expect("no digit found"),
    );
}

fn get_input(path: &str) -> Vec<String> {
    let content = read_to_string(path).expect("failed to read input file!");
    content
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<String>>()
}
