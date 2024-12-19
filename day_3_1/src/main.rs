use std::fs;

use helpers::AocError;

fn main() -> Result<(), AocError> {
    let path = "day_3_1/data/input.txt";
    let content = fs::read_to_string(path)?;

    let regex = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    let result = regex.captures_iter(&content).fold(0, |mut acc, captures| {
        let left_num = captures[1].parse::<i32>().unwrap();
        let right_num = captures[2].parse::<i32>().unwrap();
        acc += left_num * right_num;
        acc
    });

    println!("Day 3 part 1: {}", result);

    Ok(())
}
