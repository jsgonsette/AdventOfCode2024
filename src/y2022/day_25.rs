use anyhow::*;
use itertools::Itertools;

const TEST: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Read a SNAFU number from the current line of the puzzle and convert it in base 10
fn read_snafu_number(row: &str) -> Result<usize> {

    let base_and_number = (1_isize, 0_isize);

    let result = row.as_bytes().iter ().rev ().try_fold(
        base_and_number,
        |(mut base, mut number), &digit| {
            match digit as char {
                '0' => {},
                '1' => {number += base; },
                '2' => {number += base*2; },
                '-' => {number -= base; },
                '=' => {number -= base*2; },
                _ => bail!("Invalid digit: {} in row {}", digit, row),
            }
            base *= 5;
            Ok((base, number))
        }
    );

    result.map(|(_base, number)| number as usize)
}

/// Convert a base 10 number into its SNAFU counterpart
fn convert_to_snafu(mut number: usize) -> String {
    let mut digits: Vec<char> = vec! [];

    let mut report = 0usize;
    while number > 0 {

        let mut digit = (report + (number % 5)) as isize;
        number /= 5;

        if digit >= 3 {
            digit -= 5;
            report = 1;
        } else {
            report = 0;
        }

        digits.push(match digit {
            0 => '0',
            1 => '1',
            2 => '2',
            -1 => '-',
            -2 => '=',
            _ => unreachable!(),
        });
    }

    if report > 0 { digits.push('1');}

    digits.iter().rev().join("")
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<String> {

    let mut sum = 0;
    for row in content.iter() {

        let number = read_snafu_number(row)?;
        debug_assert!(convert_to_snafu(number) == *row);

        sum += number;
    }

    let sum_converted = convert_to_snafu(sum);
    Ok(sum_converted)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_25 (content: &[&str]) -> Result <(usize, usize)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == "2=-1=0");
    //debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = part_a(content)?;
    let rb = 0;//part_b(content)?;

    println!("ra: {}", ra);
    Ok((0, rb))
}