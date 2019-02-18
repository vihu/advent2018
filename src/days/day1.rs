use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn answer() -> Result<()> {
    let file = File::open("src/days/input_day1.txt")?;
    for line in BufReader::new(file).lines() {
        println!("{}", line?);
    }

    Ok(())
}

