/*
use std::fmt::Error;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

// ================================================================================================

#[derive(Debug)]
struct Part {
    pub width: u16,
    pub pos:  u16,
    pub number: u32,
}

impl Part {
    pub fn new(pos: u16) -> Part {
        Part {
            width: 0,
            pos,
            number: 0,
        }
    }
}

fn extract_gears (row: &str) -> Vec<u16> {

    let acc = Vec::<u16>::new ();
    let gears = row
        .as_bytes()
        .iter ()
        .enumerate()
        .fold (acc, |mut acc, (idx, b)| {
            if *b as char == '*' {
                acc.push(idx as u16);
            }
            acc
        });

    gears
}

fn extract_part_numbers (row: &str) -> Vec<Part> {

    struct Accumulator {
        pub parts: Vec<Part>,
        pub partial: Option<Part>,
    }

    let init = Accumulator {
        parts: vec![],
        partial: None,
    };

    let mut acc = row.as_bytes().iter().enumerate ().fold(init, |mut acc, (pos, &c)| {
        let c = c as char;

        // Update the part number under construction (start a new one eventually)
        if let Some (x) = c.to_digit(10) {
            let current = acc.partial.get_or_insert(Part::new(pos as u16));
            current.number = current.number * 10 + x;
            current.width += 1;
        }
        // Current part finished
        else if let Some (part) = acc.partial {
            acc.parts.push (part);
            acc.partial = None;
        }

        acc
    });

    // Last one
    if let Some (part) = acc.partial {
        acc.parts.push(part);
    }

    acc.parts
}

fn is_touching (part: &Part, top: &str, current: &str, bottom: &str) -> bool {

    let width = current.len() as u16;
    let left = part.pos.saturating_sub(1);
    let right = (part.pos + part.width).min (width-1);

    let mut touching = false;
    for row in [top, current, bottom] {
        for x in left..=right {
            let c = row.as_bytes() [x as usize] as char;
            if c != '.' && !c.is_digit(10) {
                touching = true;
            }
        }
    }

    touching
}

fn touching_parts<'a, I> (gear: u16, parts: I) -> Vec<&'a Part>
where I: IntoIterator<Item = &'a Part> {

    parts.into_iter().filter(|part| {
        part.pos <= gear+1 && (part.pos + part.width) >= gear
    }).collect()
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {

    let mut part_sum = 0;

    let mut lines_it = reader.lines().flatten().peekable();
    let width = lines_it.peek().ok_or(anyhow! ("Empty input!"))?.len();
    let empty_first = ".".repeat(width);
    let empty_last = ".".repeat(width);

    let mut row_top = empty_first;
    while let Some (row) = &lines_it.next() {

        let parts = extract_part_numbers(&row);

        let row_bottom = match lines_it.peek() {
            Some (line) => line,
            None        => &empty_last,
        };

        for part in parts {
            if is_touching(&part, &row_top, row, row_bottom) {
                part_sum += part.number;
            }
        }

        row_top = row.clone();
    }

    Ok(part_sum as usize)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut part_sum = 0;

    let mut lines_it = reader.lines().flatten().peekable();
    let width = lines_it.peek().ok_or(anyhow! ("Empty input!"))?.len();
    let empty_first = ".".repeat(width);
    let empty_last = ".".repeat(width);

    let mut row_top = empty_first;
    while let Some (row) = &lines_it.next() {

        let row_bottom = match lines_it.peek() {
            Some (line) => line,
            None        => &empty_last,
        };

        let parts_top = extract_part_numbers(&row_top);
        let parts_current = extract_part_numbers(&row);
        let parts_bottom = extract_part_numbers(&row_bottom);

        let parts = parts_top.iter ()
            .chain(parts_current.iter())
            .chain(parts_bottom.iter());

        let gears = extract_gears(&row);
        for gear in gears {
            let touching_parts = touching_parts(gear, parts.clone());
            if touching_parts.len() == 2 {
                part_sum += touching_parts[0].number * touching_parts[1].number;
            }
        }

        row_top = row.clone();
    }

    Ok(part_sum as usize)
}

// ================================================================================================

pub fn day_3() -> Result<()> {

    println!("=== Part 1 ===");
    assert_eq!(4361, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result for part 1 = {}", result);

    println!("\n=== Part 2 ===");
    assert_eq!(467835, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result for part 2 = {}", result);

    Ok(())
}

 */