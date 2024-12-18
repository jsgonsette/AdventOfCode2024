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

/// To localize in which subtree is the Human at each node
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum HumanSide { Left, Right, NA }

type MonkeyName = [char; 4];

/// Indexes to find back a monkey in the vector of monkeys, given its name
type MonkeyIndex = HashMap<MonkeyName, MonkeyLocation>;

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

/// Save the 4-letters name of the monkey
fn to_monkey_name (name: &str) -> MonkeyName {
    let raw = name.as_bytes();
    [raw [0] as char, raw [1] as char, raw [2] as char, raw [3] as char]
}

/// Decode a row of the puzzle file content and return the corresponding [Monkey]
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

/// Create all the monkeys from the puzzle file content
fn get_monkeys (content: &[&str]) -> Result<Vec<Monkey>> {
    content.iter().map(|&row| decode_row(row)).collect ()
}

/// Given a vector of `monkeys`, create an index based on the names that enables to retrieve
/// * the position of the monkey in the vector
/// * the position of the parent monkey in the vector (the one waiting for the value)
fn build_monkey_index (monkeys: &[Monkey]) -> MonkeyIndex {
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

    index
}

/// Augment the provided `index` with the localization of the human at each parent node (when applicable)
fn update_index_with_human_loc (monkeys: &[Monkey], index: &mut MonkeyIndex) {

    // From the human to the root, localize the human at each parent node
    let mut current = to_monkey_name("humn");
    while current != to_monkey_name("root") {

        // Retrieve the parent of the current element
        let idx = index [&current].op_idx;
        let parent_name = monkeys [idx].0;
        let parent_idx = index [&parent_name].idx;

        // Check its operation to see if the human operator is on its left or right side,
        // then save this information
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
}

/// Solve the value the monkey `name` will yell, given the vector of `monkeys` and the `index`
fn yell_monkey(monkeys: &[Monkey], index: &MonkeyIndex, name: MonkeyName) -> Result<usize> {

    // Get the target monkey and its operation
    let monkey_idx = index[&name].idx;
    let monkey = &monkeys[monkey_idx];
    let operation = &monkey.1;

    // If operation is Yell, return the value
    if let Operation::Yell(number) = operation { return Ok (*number); }

    // Otherwise get the names of the monkeys on the left and on the right, and solve them recursively
    let names = operation.get_names().unwrap();
    let val_left = yell_monkey(monkeys, index, *names.0)?;
    let val_right = yell_monkey(monkeys, index, *names.1)?;

    // Combine both values given the monkey operation
    match operation {
        Operation::Add(_, _) => Ok (val_left + val_right),
        Operation::Sub(_, _) => Ok (val_left - val_right),
        Operation::Mul(_, _) => Ok (val_left * val_right),
        Operation::Div(_, _) => Ok (val_left / val_right),
        _ => unreachable!(),
    }
}

/// Solve the value the human should yell to have an equality at the root monkey,
/// given the vector of `monkeys` and the `index`.
fn solve_human (monkeys: &[Monkey], index: &MonkeyIndex) -> Result<usize> {

    // Identify the child monkey sitting above the human, and the value it must yell
    let (monkey_on_human_side, value_to_yell) = get_human_side(monkeys, index, to_monkey_name("root"), None)?;

    // Solve the other subtree knowing this value
    solve_human_at(monkeys, index, monkey_on_human_side, value_to_yell)
}

/// Given a monkey `name`, identify the child monkey on the side the human belongs to.
/// Then, return
/// * the name of this monkey sitting above the human
/// * the value that it should yell to match the `expected_operation_value`.
/// If this later is `None`, we are at the root; we seek at satisfying the equality
fn get_human_side(
    monkeys: &[Monkey],
    index: &MonkeyIndex,
    name: MonkeyName,
    expected_operation_value: Option<usize>
) -> Result<(MonkeyName, usize)> {

    // Get the top monkey characteristics
    let monkey_idx = index[&name];
    let monkey = &monkeys[monkey_idx.idx];
    let human_side = monkey_idx.human;

    // Get the names of the monkeys on the left and the right
    let operation = &monkey.1;
    let (name_left,name_right) = operation.get_names().ok_or(anyhow!("Expecting binary op"))?;

    // Identify the subtree with the human (to solve) and the other one with the known value
    let (known, to_solve) = match human_side {
        HumanSide::Left => (name_right, name_left),
        HumanSide::Right => (name_left, name_right),
        HumanSide::NA => bail!("Expecting human side"),
    };

    // Get the value we can know from the corresponding subtree
    let value = yell_monkey(monkeys, index, *known)?;

    // Knowing the result and the value at one side, solve the other side
    let val_other_side = if let Some (expected_value) = expected_operation_value {
        match operation {
            Operation::Add(_, _) => expected_value - value,
            Operation::Sub(_, _) => if human_side == HumanSide::Left { expected_value + value } else { value - expected_value },
            Operation::Mul(_, _) => expected_value / value,
            Operation::Div(_, _) => if human_side == HumanSide::Left { expected_value * value } else { value / expected_value },
            _ => unreachable!(),
        }
    } else {
        value
    };

    Ok ((*to_solve, val_other_side))
}

/// Solve the value the human should yell to have an equality at the root monkey,
/// given the vector of `monkeys` and the `index`.
/// Parameter `name` designates a monkey on top of the human, while parameter `expected_value`
/// indicates what this monkey should yell.
fn solve_human_at (monkeys: &[Monkey], index: &HashMap<MonkeyName, MonkeyLocation>, name: MonkeyName, expected_value: usize) -> Result<usize> {

    // Identify the child monkey sitting above the human, and the value it must yell
    let (monkey_on_human_side, value_to_yell) = get_human_side(monkeys, index, name, Some (expected_value))?;

    // If we reach the human, we finally know what to yell. Otherwise, continue digging down.
    if monkey_on_human_side == to_monkey_name("humn") { Ok (value_to_yell) }
    else { solve_human_at(monkeys, index, monkey_on_human_side, value_to_yell) }
}

/// Solve both parts of the puzzle
fn solve (content: &[&str]) -> Result<(usize, usize)> {

    // Collect the monkeys
    let monkeys = get_monkeys(content)?;

    // Create the index
    let mut index = build_monkey_index(&monkeys);
    update_index_with_human_loc(&monkeys, &mut index);

    // Solve both problems
    let root_val = yell_monkey(&monkeys, &index, to_monkey_name("root"))?;
    let human_val = solve_human(&monkeys, &index)?;

    Ok((root_val, human_val))
}

pub fn day_21 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve (&split(TEST)).unwrap_or_default() == (152, 301));

    let (ra, rb) = solve(content)?;
    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}