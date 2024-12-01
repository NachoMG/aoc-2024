use helpers::{read_lines, AocError};

fn main() -> Result<(), AocError> {
    let path = "day_1_2/data/input.txt";
    let lines_buffer = read_lines(path)?;

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line_res in lines_buffer {
        let line = line_res?;
        let mut items = line.split_whitespace();

        let left: i32 = items
            .next()
            .ok_or(AocError::ItemNotFound(line.to_string()))?
            .parse()?;
        let right: i32 = items
            .next()
            .ok_or(AocError::ItemNotFound(line.to_string()))?
            .parse()?;

        left_list.push(left);
        right_list.push(right);
    }

    left_list.sort();
    right_list.sort();

    let distance = left_list
        .into_iter()
        .zip(right_list)
        .fold(0, |acc, (left, right)| acc + (left - right).abs());

    println!("Day 1 1: {}", distance);

    Ok(())
}
