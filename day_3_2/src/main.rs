use std::fs;

use helpers::AocError;

fn main() -> Result<(), AocError> {
    let path = "day_3_2/data/input.txt";
    let content = fs::read_to_string(path)?;

    let regex = regex::Regex::new(r"do\(\)|don\'t\(\)|mul\((\d{1,3}),(\d{1,3})\)")?;
    let result = regex
        .captures_iter(&content)
        .fold((0, true), |(acc, enabled), captures| match &captures[0] {
            "do()" => (acc, true),
            "don't()" => (acc, false),
            _ => {
                if enabled {
                    let left_num = captures[1].parse::<i32>().unwrap();
                    let right_num = captures[2].parse::<i32>().unwrap();
                    (acc + left_num * right_num, enabled)
                } else {
                    (acc, enabled)
                }
            }
        })
        .0;

    println!("Day 3 part 2: {}", result);

    Ok(())
}
