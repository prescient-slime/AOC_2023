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

fn part_2() -> Result<u32, std::io::Error> {
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let input = std::fs::read_to_string("src/test.txt")?;
    let sum2: u32 = input
        .lines()
        .map(|line| {
            let first_digit = line
                .split_whitespace()
                .next()
                .and_then(|word| digits.iter().position(|&d| word.starts_with(d)))
                .or_else(|| line.chars().next()?.to_digit(10).map(|num| num as usize));

            let last_digit = line
                .split_whitespace()
                .next_back()
                .and_then(|word| digits.iter().rposition(|&d| word.ends_with(d)))
                .or_else(|| line.chars().last()?.to_digit(10).map(|num| num as usize));

            match (first_digit, last_digit) {
                (Some(first), Some(last)) => Ok(first * 10 + last),
                _ => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid line format",
                )),
            }
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|&num| num as u32)
        .sum();

    Ok(sum2)
}

fn main() {
    part_1();
    println!("{:?}", part_2());
}
