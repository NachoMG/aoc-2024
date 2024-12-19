use std::collections::HashMap;

use helpers::{read_lines, AocError};

fn main() -> Result<(), AocError> {
    let path = "day_4_1/data/input.txt";
    let lines_buffer = read_lines(path)?;

    let (relevant_char_positions, x_positions) =
        lines_buffer.map_while(Result::ok).enumerate().fold(
            (HashMap::new(), Vec::new()),
            |(mut relevant_char_positions, mut x_positions), (i, line)| {
                line.chars().enumerate().for_each(|(j, char)| match char {
                    'X' => {
                        x_positions.push((i, j));
                    }
                    'M' | 'A' | 'S' => {
                        relevant_char_positions.insert((i, j), char);
                    }
                    _ => {}
                });
                (relevant_char_positions, x_positions)
            },
        );

    let directions = [
        (1, 0),   // right
        (-1, 0),  // left
        (0, 1),   // down
        (0, -1),  // up
        (1, 1),   // down-right
        (-1, -1), // up-left
        (-1, 1),  // up-right
        (1, -1),  // down-left
    ];

    let xmas_qty = x_positions.into_iter().fold(0, |mut acc, x_position| {
        let i = x_position.0;
        let j = x_position.1;

        for direction in directions {
            let positions = [
                (i as i32 + direction.0, j as i32 + direction.1),
                (i as i32 + 2 * direction.0, j as i32 + 2 * direction.1),
                (i as i32 + 3 * direction.0, j as i32 + 3 * direction.1),
            ];

            if positions.iter().any(|&(x, y)| x < 0 || y < 0) {
                continue;
            }

            let pos_usize: Vec<(usize, usize)> = positions
                .into_iter()
                .map(|(x, y)| (x as usize, y as usize))
                .collect();

            if relevant_char_positions.get(&pos_usize[0]) == Some(&'M')
                && relevant_char_positions.get(&pos_usize[1]) == Some(&'A')
                && relevant_char_positions.get(&pos_usize[2]) == Some(&'S')
            {
                acc += 1;
            }
        }

        acc
    });

    println!("Day 4 part 1: {}", xmas_qty);

    Ok(())
}
