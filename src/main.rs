mod days;

use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::*;
use std::result::Result::Ok;
use crate::days::*;

fn main() -> Result<()> {

    for day in 1..2 {

        let input_file = format! ("input/{:02}.txt", day);
        let br = BufReader::new(File::open(&input_file)?);
        let content: Result<Vec<String>, std::io::Error> = br.lines().collect();

        let start = std::time::Instant::now();
        let result = match content {
            Ok(lines) => {
                let line_refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
                match day {
                    1 => day_1(&line_refs)?,
                    _ => { (0, 0) }
                }
            }
            Err(_err) => { bail!("d") }
        };
        let duration = start.elapsed();

        println!("\nSolution of day {}, in {:?}", day, duration);
        println!(" - Part A: {}", result.0);
        println!(" - Part B: {}", result.1);
    }

    Ok(())
}