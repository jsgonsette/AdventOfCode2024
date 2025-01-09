use anyhow::*;
use crate::{Solution};
use crate::tools::RowReader;

const TEST: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add (u32),
    Mul (u32),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u32>,
    op: Operation,
    test_div: u32,
    monkey_true: u32,
    monkey_false: u32,
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn read_monkey (rows: &[&str]) -> Result<Monkey> {
    let mut reader = RowReader::new(false);
    if rows.len() < 6 { bail!("Not enough rows!") }

    let mut read_single = |row: &str| {
        reader.process_row::<u32>(row)
            .get (0)
            .copied()
            .ok_or(anyhow!("Invalid row: {}", row))
    };

    let test_div = read_single (rows [3])?;
    let monkey_true = read_single (rows [4])?;
    let monkey_false = read_single (rows [5])?;

    let op = rows [2].find("old").and_then(|idx| {
        let op_char = rows [2].as_bytes() [idx +4];
        let element = std::str::from_utf8(&rows [2].as_bytes() [idx +6..]).unwrap();

        if element == "old" { return Some (Operation::Square); }
        let element_num = element.parse::<u32>().ok ()?;
        match op_char {
            b'*' => Some (Operation::Mul(element_num)),
            b'+' => Some (Operation::Add(element_num)),
            _ => None,
        }
    }).ok_or(anyhow!("Invalid operation in {}", rows [2]))?;

    Ok (Monkey {
        items: reader.process_row::<u32>(rows [1]),
        op,
        test_div,
        monkey_true,
        monkey_false,
    })
}

fn read_monkeys (content: &[&str]) -> Result<Vec<Monkey>> {
    content.chunks(7).map(read_monkey).collect()
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let monkeys = read_monkeys(content)?;
    println!("{:?}", monkeys);
    Ok(0)
}

/// Solve second part of the puzzle
fn part_b (_content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_11 (_content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 0);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = 0;//part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}