//use std::env;
use std::fs;

fn read_input(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    return result;
}

fn part_1() {
    let result = read_input("src/input.txt");

    let mut sums = Vec::new();
    let mut line_idx = 0;
    let mut pseudonum = String::new();

    while line_idx < result.len() {
        for c in result[line_idx].chars() {
            if c.is_digit(10) {
                pseudonum.push(c);
            }
        }
        sums.push(pseudonum.clone());
        line_idx += 1;
        pseudonum.clear();
    }

    for num in &mut sums {
        let mut swap = String::new();
        swap.push(num.chars().next().unwrap());
        swap.push(num.chars().last().unwrap());
        num.replace_range(.., &swap);
    }

    let sum: u32 = sums.iter().filter_map(|s| s.parse::<u32>().ok()).sum();

    println!("{}", sum);
}

fn part_2() -> String {
    let digits = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    return digits[0].to_string();
}

fn main() {
    part_1();
    part_2();
}
