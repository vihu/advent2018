use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

fn lines_to_string_vector() -> Result<Vec<String>> {
    BufReader::new(File::open("src/days/input_day1.txt")?).lines().collect()
}

pub fn answer() -> Result<i32> {
    let vec = lines_to_string_vector()?;
    let sum = vec.iter().fold(0, |sum, value| sum + value.parse::<i32>().unwrap());
    Ok(sum)
}

