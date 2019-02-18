use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn answer() -> Result<i32> {
    let res = BufReader::new(File::open("src/days/input_day1.txt")?)
                        .lines()
                        .fold(0, |sum, value| sum + value.unwrap().parse::<i32>().unwrap());
    Ok(res)
}
