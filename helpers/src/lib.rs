use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Io Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Item not found when parsing line: {0}")]
    ItemNotFound(String),
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
