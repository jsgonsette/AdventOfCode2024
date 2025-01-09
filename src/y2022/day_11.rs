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

type WorryItem = usize;

/// The different worry-inspection operations
#[derive(Debug, Copy, Clone)]
enum Operation {
    Add (usize),
    Mul (usize),
    Square,
}

/// Models a single monkey
#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<WorryItem>,
    op: Operation,
    test_div: usize,
    monkey_true: usize,
    monkey_false: usize,
    activity_counter: usize,
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

impl Monkey {

    /// Drain all the objects from this monkey and return a list of `(monkey, worry level)` pairs
    fn throw_objects (&mut self, worry_decrease: bool) -> Vec<(usize, WorryItem)> {

        self.activity_counter += self.items.len();

        self.items.drain(..).map(|item| {
            let worry = match self.op {
                Operation::Add(v) => item + v,
                Operation::Mul(v) => item * v,
                Operation::Square => item * item,
            };

            let worry = match worry_decrease {
                true => worry / 3,
                false => worry,
            };

            let tested_worry = (worry % self.test_div) == 0;
            match tested_worry {
                false => (self.monkey_false, worry),
                true => (self.monkey_true, worry),
            }
        }).collect()
    }
}

/// Extract the description of a [Monkey] from a slice of 6 `rows` of the puzzle file content.
fn read_monkey (rows: &[&str]) -> Result<Monkey> {
    let mut reader = RowReader::new(false);
    if rows.len() < 6 { bail!("Not enough rows!") }

    // To read a single value from a single line
    let mut read_single = |row: &str| {
        reader.process_row::<usize>(row)
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
        let element_num = element.parse::<usize>().ok ()?;
        match op_char {
            b'*' => Some (Operation::Mul(element_num)),
            b'+' => Some (Operation::Add(element_num)),
            _ => None,
        }
    }).ok_or(anyhow!("Invalid operation in {}", rows [2]))?;

    Ok (Monkey {
        items: reader.process_row::<usize>(rows [1]),
        op,
        test_div,
        monkey_true,
        monkey_false,
        activity_counter: 0,
    })
}

/// Return the description of all the monkeys from the puzzle file content
fn read_monkeys (content: &[&str]) -> Result<Vec<Monkey>> {
    content.chunks(7).map(read_monkey).collect()
}

/// Simulate a round during which all the `monkeys`, in turn, throw the objects to each others.
/// Flag `worry_decrease` is `true` for the first question and makes the worry auto-manageable.
/// `safety_level` (see function [safety_level]) limits the worry for the second question.
fn round (monkeys: &mut [Monkey], worry_decrease: bool, safety_level: usize) {

    for idx in 0..monkeys.len() {
        let monkey = &mut monkeys[idx];
        let thrown = monkey.throw_objects(worry_decrease);

        for (monkey_idx, object) in thrown {
            monkeys [monkey_idx].items.push(object % safety_level);
        }
    }
}

/// Multiply together all the prime numbers used by the monkeys to make their inspection test.
/// This value can be used to limit the worry level without impacting their decision (*modular
/// arithmetic*)
fn safety_level (monkeys: &[Monkey]) -> usize {
    monkeys.iter().map(|monkey| monkey.test_div).product()
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let mut monkeys = read_monkeys(content)?;
    let safety_level = safety_level(&mut monkeys);
    for _ in 0.. 20 { round(&mut monkeys, true, safety_level); }

    monkeys.sort_unstable_by_key( |monkey| monkey.activity_counter);
    let activity = monkeys.last_chunk::<2>().and_then(
        |[a, b]| Some (a.activity_counter * b.activity_counter)
    );
    activity.ok_or(anyhow!("Not enough monkeys"))
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let mut monkeys = read_monkeys(content)?;
    let safety_level = safety_level(&mut monkeys);
    for _ in 0.. 10000 { round(&mut monkeys, false, safety_level); }

    monkeys.sort_unstable_by_key( |monkey| monkey.activity_counter);
    let activity = monkeys.last_chunk::<2>().and_then(
        |[a, b]| Some (a.activity_counter * b.activity_counter)
    );
    activity.ok_or(anyhow!("Not enough monkeys"))
}

pub fn day_11 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 10605);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 2713310158);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}