use helpers::{read_lines, AocError};

enum Trend {
    Upwards,
    Downwards,
}

fn is_valid(values: &[i32]) -> bool {
    let first_diff = values[1] - values[0];
    if first_diff.abs() > 3 || first_diff == 0 {
        return false;
    }

    let trend = if first_diff > 0 {
        Trend::Upwards
    } else {
        Trend::Downwards
    };

    values.windows(2).all(|pair| {
        let diff = pair[1] - pair[0];
        match trend {
            Trend::Upwards => (1..4).contains(&diff),
            Trend::Downwards => (-3..0).contains(&diff),
        }
    })
}

fn is_tolerable(values: &[i32]) -> bool {
    if is_valid(values) {
        return true;
    }

    (0..values.len()).any(|index| {
        let mut parsed_values = values.to_vec();
        parsed_values.remove(index);
        is_valid(&parsed_values)
    })
}

fn main() -> Result<(), AocError> {
    let path = "day_2_2/data/input.txt";
    let lines_buffer = read_lines(path)?;

    let safe_qty = lines_buffer
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .filter_map(|value| value.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|values| is_tolerable(values))
        .count();

    println!("Day 2 part 2: {}", safe_qty);

    Ok(())
}