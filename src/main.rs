mod tools;
mod y2022;
mod y2023;
mod y2024;

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::*;
use std::result::Result::Ok;
use std::time::Duration;
use crate::y2024::Y2024;
use crate::y2022::Y2022;

pub use tools::{CellArea, Cell};

/// A function solving the problem of the day.
/// * Input param is a vector of strings (input file)
/// * Output are the two problem answers (part a and b)
type FnDay = fn(&[&str]) -> Result <(Solution, Solution)>;

/// A module containing all the functions to solve the daily problems of some year.
trait Year {
    fn get_year (&self) -> u32;

    /// Get the function solving the problem of the given `day`
    fn get_day_fn (&self, day: u32) -> Option<FnDay>;
}

/// Each problem expects a final numerical or textual solution
enum Solution {
    Unsigned (usize),
    Text (String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Unsigned (n) => n.fmt(f),
            Solution::Text(s) => s.fmt(f),
        }
    }
}

fn main() -> Result<()> {

    //solve_year(Y2022);
    solve_year(Y2024);

    Ok(())
}

/// Solve for all the days of the provided `year` module.
fn solve_year<Y> (year: Y)
where Y : Year {

    println!("=========================");
    println!("Solution of year {:?}", year.get_year());

    for day in 1..= 25 {
        let Some (fn_solve) = year.get_day_fn(day) else { continue };

        match solve_day(year.get_year(), day, fn_solve) {

            Ok((a, b, duration)) => {
                println!("\n| day {}, in {:?}", day, duration);
                println!(" - Part A: {}", a);
                println!(" - Part B: {}", b);
            }
            Err(err) => {
                println!("\n| day {}, in ERROR", day);
                println!(" * {}", err.to_string());
            }
        };
    }
}

/// Solve for the given `day` of the `year`, thanks to the provided function `fn_solve`.
/// In case of success, return the two answers and the duration to compute them.
/// The corresponding input file is expected to be found at the location `input/<yyyy>/<dd>.txt`
fn solve_day (year: u32, day: u32, fn_solve: FnDay) -> Result <(Solution, Solution, Duration)> {

    // Extract the input file as a vector of strings
    let input_file = format! ("input/{}/{:02}.txt", year, day);
    let br = BufReader::new(File::open(&input_file)?);
    let content: Result<Vec<String>, std::io::Error> = br.lines().collect();

    // Measure time ...
    let start = std::time::Instant::now();
    let result = match content {

        Ok(lines) => {
            // ... to solve
            let line_refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
            fn_solve (&line_refs)?
        }
        Err(err) => { bail!("Failed to read input file: {}", err.to_string()); }
    };
    let duration = start.elapsed();

    // Return the two answers and the duration
    Ok((result.0, result.1, duration))
}