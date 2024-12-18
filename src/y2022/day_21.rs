use std::collections::HashMap;
use anyhow::*;
use crate::{Solution};

const TEST: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Models the different type of operations
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operation {
    Yell (usize),
    Add (MonkeyName, MonkeyName),
    Sub (MonkeyName, MonkeyName),
    Mul (MonkeyName, MonkeyName),
    Div (MonkeyName, MonkeyName),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum HumanSide { Left, Right, NA }

type MonkeyName = [char; 4];

/// Describes a monkey, with its name and its job
type Monkey = (MonkeyName, Operation);

/// Two positions (as principal and as operand) of a monkey in the vector of monkeys.
/// Also store in which subtree is the human
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
struct MonkeyLocation {
    idx: usize,
    op_idx: usize,
    human: HumanSide,
}

impl Default for HumanSide {
    fn default () -> Self {
        HumanSide::NA
    }
}

impl Operation {
    /// Extract the left and right operands of a binary operation
    fn get_names(&self) -> Option<(&MonkeyName, &MonkeyName)> {
        match self {
            Operation::Add(left, right) => Some((left, right)),
            Operation::Sub(left, right) => Some((left, right)),
            Operation::Mul(left, right) => Some((left, right)),
            Operation::Div(left, right) => Some((left, right)),
            _ => None,
        }
    }
}

fn to_monkey_name (name: &str) -> MonkeyName {
    let raw = name.as_bytes();
    [raw [0] as char, raw [1] as char, raw [2] as char, raw [3] as char]
}

fn decode_row (row: &str) -> Result<Monkey> {

    let (name, operands) = row.split_once(": ").ok_or(anyhow!("Invalid row: {}", row))?;
    if let Some (operation) = operands.chars().nth(5) {

    let (left, right) = operands.split_once(operation).ok_or(anyhow!("Invalid row: {}", row))?;
    let left = to_monkey_name (left.trim());
    let right = to_monkey_name (right.trim());

    let op = match operation {
        '+' => Operation::Add(left, right),
        '-' => Operation::Sub(left, right),
        '*' => Operation::Mul(left, right),
        '/' => Operation::Div(left, right),
        _ => bail!("Invalid row: {}", row),
    };

    Ok((to_monkey_name(name), op))

    } else {
        let value = operands.trim().parse::<usize>()?;
        Ok ((to_monkey_name (name), Operation::Yell(value)))
    }
}

fn get_monkeys (content: &[&str]) -> Result<Vec<Monkey>> {
    content.iter().map(|&row| decode_row(row)).collect ()
}

fn build_monkey_index (monkeys: &[Monkey]) -> HashMap<MonkeyName, MonkeyLocation> {
    let mut index = HashMap::new();

    for (idx, monkey) in monkeys.iter().enumerate() {

        let name = monkey.0;
        let entry = index.entry(name).or_insert(MonkeyLocation::default());
        entry.idx = idx;

        let operation = &monkey.1;
        match operation.get_names() {
            None => {}
            Some((name_left, name_right)) => {
                let entry_left = index.entry(*name_left).or_insert(MonkeyLocation::default());
                entry_left.op_idx = idx;

                let entry_right = index.entry(*name_right).or_insert(MonkeyLocation::default());
                entry_right.op_idx = idx;
            }
        }
    }

    // From the human to the root, localize the human at each parent node
    let mut current = to_monkey_name("humn");
    while current != to_monkey_name("root") {

        // Retrieve the parent of the current element
        let idx = index [&current].op_idx;
        let parent_name = monkeys [idx].0;
        let parent_idx = index [&parent_name].idx;

        // Check its operation to see if the human operator is on its left or right side
        let parent_operation = monkeys [parent_idx].1;
        if let Some((name_left, _name_right)) = parent_operation.get_names() {
            if *name_left == current {
                index.get_mut(&parent_name).unwrap().human = HumanSide::Left;
            } else {
                index.get_mut(&parent_name).unwrap().human = HumanSide::Right;
            }
        }
        current = parent_name;
    }

    index
}

fn yell_monkey(monkeys: &[Monkey], index: &HashMap<MonkeyName, MonkeyLocation>, name: MonkeyName) -> Result<usize> {

    let monkey_idx = index[&name].idx;
    let monkey = &monkeys[monkey_idx];
    let operation = &monkey.1;

    if let Operation::Yell(number) = operation { return Ok (*number); }
    let names = operation.get_names().unwrap();
    let val_left = yell_monkey(monkeys, index, *names.0)?;
    let val_right = yell_monkey(monkeys, index, *names.1)?;

    match operation {
        Operation::Add(_, _) => Ok (val_left + val_right),
        Operation::Sub(_, _) => Ok (val_left - val_right),
        Operation::Mul(_, _) => Ok (val_left * val_right),
        Operation::Div(_, _) => Ok (val_left / val_right),
        _ => unreachable!(),
    }
}


fn part_a (content: &[&str]) -> Result<usize> {
    let monkeys = get_monkeys(content)?;
    let index = build_monkey_index(&monkeys);

    let root_val = yell_monkey(&monkeys, &index, to_monkey_name("root"))?;
    Ok(root_val)
}

/// Solve second part of the puzzle
fn part_b (_content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_21 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 152);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}