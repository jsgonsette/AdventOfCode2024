mod tools;
mod y2022;
mod y2023;
mod y2024;

use crate::y2022::Y2022;
use crate::y2024::Y2024;
use anyhow::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};
use std::ops::RangeBounds;
use std::result::Result::Ok;
use std::time::Duration;

pub use tools::{Cell, CellArea};
use crate::y2023::Y2023;

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

    solve_year(Y2022, 21..21);
    solve_year(Y2023, 10..10);
    solve_year(Y2024, 21..22);

    //let result = benchmark_year(&Y2024);
    //dbg!(&result);

    Ok(())
}

/// Solve for all the days of the provided `year` module.
fn solve_year<Y> (year: Y, day_range: impl RangeBounds<u32>)
where Y : Year {

    println!("=========================");
    println!("Solutions for year {:?}", year.get_year());
    println!("WARNING: execution time may be noisy!");

    for day in (1..= 25).filter(|day| day_range.contains(day)) {

        // Get the function related to the current day, or skip the test
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

/// The result of performance benchmarking, indexed on the day numbers.
type BenchmarkResult = HashMap<u32, Result<Duration>>;

fn benchmark_year<Y> (year: &Y) -> BenchmarkResult
where Y : Year {

    println!("Benchmark year {:?}: ", year.get_year());

    let mut durations = BenchmarkResult::new();
    const REPETITIONS: u32 = 100;

    for _ in 0..REPETITIONS {
        print!(".");
        stdout().flush().expect("TODO: panic message");
        
        for day in 1..=25 {

            // Get the function related to the current day, or skip the test
            let Some(fn_solve) = year.get_day_fn(day) else { continue };

            // Also skip if failed in previous iteration
            let day_entry = durations.entry(day).or_insert_with(|| Ok(Duration::default()));
            let Ok(day_duration) = day_entry else { continue };

            // Solve and collect the solving time, or the error
            match solve_day(year.get_year(), day, fn_solve) {
                Ok((_a, _b, duration)) => { *day_duration += duration; }
                Err(err) => { *day_entry = Err(err) }
            };
        }
    }

    for duration in durations.values_mut() {
        if let Ok(duration) = duration {
            *duration /= REPETITIONS;
        }
    }

    durations
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