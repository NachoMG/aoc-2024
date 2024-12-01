use std::collections::HashMap;

use helpers::read_lines;
use thiserror::Error;

#[derive(Error, Debug)]
enum Day1Error {
    #[error("Io Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Item not found when parsing line: {0}")]
    ItemNotFound(String),
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

fn main() -> Result<(), Day1Error> {
    let path = "day_1_2/data/input.txt";
    let lines_buffer = read_lines(path)?;

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line_res in lines_buffer {
        let line = line_res?;
        let mut items = line.split_whitespace();

        let left: i32 = items
            .next()
            .ok_or(Day1Error::ItemNotFound(line.to_string()))?
            .parse()?;
        let right: i32 = items
            .next()
            .ok_or(Day1Error::ItemNotFound(line.to_string()))?
            .parse()?;

        left_list.push(left);
        right_list.push(right);
    }

    let occurrences = right_list
        .into_iter()
        .fold(HashMap::new(), |mut acc, value| {
            *acc.entry(value).or_insert(0) += 1;
            acc
        });

    let similarity = left_list.into_iter().fold(0, |acc, value| {
        acc + (value * *occurrences.get(&value).unwrap_or(&0))
    });

    println!("Day 1 part 2: {}", similarity);

    Ok(())
}
