use std::collections::HashMap;

use helpers::{read_lines, AocError};

fn main() -> Result<(), AocError> {
    let path = "day_4_2/data/input.txt";
    let lines_buffer = read_lines(path)?;

    let (relevant_char_positions, a_positions) =
        lines_buffer.map_while(Result::ok).enumerate().fold(
            (HashMap::new(), Vec::new()),
            |(mut relevant_char_positions, mut x_positions), (i, line)| {
                line.chars().enumerate().for_each(|(j, char)| match char {
                    'A' => {
                        x_positions.push((i, j));
                    }
                    'M' | 'S' => {
                        relevant_char_positions.insert((i, j), char);
                    }
                    _ => {}
                });
                (relevant_char_positions, x_positions)
            },
        );

    let xmas_qty = a_positions
        .into_iter()
        .filter(|a_position| {
            let i = a_position.0;
            let j = a_position.1;

            if i < 1 || j < 1 {
                return false;
            }

            let diagonals = [
                ((i - 1, j - 1), (i + 1, j + 1)),
                ((i - 1, j + 1), (i + 1, j - 1)),
            ];

            diagonals.into_iter().all(|diagonal| {
                relevant_char_positions.get(&diagonal.0) == Some(&'M')
                    && relevant_char_positions.get(&diagonal.1) == Some(&'S')
                    || relevant_char_positions.get(&diagonal.0) == Some(&'S')
                        && relevant_char_positions.get(&diagonal.1) == Some(&'M')
            })
        })
        .count();

    println!("Day 4 part 2: {}", xmas_qty);

    Ok(())
}
