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

/// Models an operand: either a known number, or the name of the monkey we wait for
#[derive(Debug, Clone, PartialEq, Eq)]
enum Operand {
    Number (usize),
    Name (MonkeyName),
}

/// Models the different type of operations
#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Yell (usize),
    Add (Operand, Operand),
    Sub (Operand, Operand),
    Mul (Operand, Operand),
    Div (Operand, Operand),
}

type MonkeyName = String;

/// Describes a monkey, with its name and its job
type Monkey = (MonkeyName, Operation);

/// The index of a monkey in the vector, as well as all the other indexes where
/// it appears in an operand
type MonkeyLocation = (usize, Vec<usize>);


impl Operation {

    /// Extract the left and right operands of a binary operation
    fn get_operands (&self) -> Option<(&Operand, &Operand)> {
       match self {
           Operation::Add (left, right) => Some((left, right)),
           Operation::Sub (left, right) => Some((left, right)),
           Operation::Mul (left, right) => Some((left, right)),
           Operation::Div (left, right) => Some((left, right)),
           _ => None,
       }
    }

    /// Replace the left operand of a binary operation
    fn replace_left (&mut self, left: Operand) {
        let new_type = match self {
            Operation::Add (_, right) => Some (Operation::Add (left, right.clone())),
            Operation::Sub (_, right) => Some (Operation::Sub (left, right.clone())),
            Operation::Mul (_, right) => Some (Operation::Mul (left, right.clone())),
            Operation::Div (_, right) => Some (Operation::Div (left, right.clone())),
            _ => None,
        };

        if let Some (new_type) = new_type {
            *self = new_type;
        }
    }

    /// Replace the right operand of a binary operation
    fn replace_right (&mut self, right: Operand) {
        let new_type = match self {
            Operation::Add (left, _) => Some (Operation::Add (left.clone(), right)),
            Operation::Sub (left, _) => Some (Operation::Sub (left.clone(), right)),
            Operation::Mul (left, _) => Some (Operation::Mul (left.clone(), right)),
            Operation::Div (left, _) => Some (Operation::Div (left.clone(), right)),
            _ => None,
        };

        if let Some (new_type) = new_type {
            *self = new_type;
        }
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn decode_row (row: &str) -> Result<Monkey> {

    let (name, operands) = row.split_once(": ").ok_or(anyhow!("Invalid row: {}", row))?;
    if let Some (operation) = operands.chars().nth(5) {

        let (left, right) = operands.split_once(operation).ok_or(anyhow!("Invalid row: {}", row))?;
        let left = Operand::Name(left.trim().to_string());
        let right = Operand::Name(right.trim().to_string());

        let op = match operation {
            '+' => Operation::Add(left, right),
            '-' => Operation::Sub(left, right),
            '*' => Operation::Mul(left, right),
            '/' => Operation::Div(left, right),
            _ => bail!("Invalid row: {}", row),
        };

        Ok((name.to_string(), op))

    } else {
        let value = operands.trim().parse::<usize>()?;
        Ok ((name.to_string(), Operation::Yell(value)))
    }
}

fn get_monkeys (content: &[&str]) -> Result<Vec<Monkey>> {
    content.iter().map(|&row| decode_row(row)).collect ()
}

fn get_monkeys_indexes (monkeys: &[Monkey]) -> Result<HashMap<MonkeyName, MonkeyLocation>> {

    // Map the monkey names with their index in the `monkeys` vector
    let monkey_it = monkeys.iter()
        .enumerate ()
        .map(
            |(idx, (name, _))| (name.clone(), (idx, vec![]))
        );

    // and build the first part of the index
    let mut indexes: HashMap<MonkeyName, MonkeyLocation> = HashMap::from_iter(monkey_it);

    // Now index the names in the operands they appear in
    for (idx, (_name, op)) in monkeys.iter ().enumerate () {

        // Extract the monkey names in the operands
        let names = match op {
            Operation::Add(Operand::Name(left), Operand::Name(right)) => Some ((left, right)),
            Operation::Sub(Operand::Name(left), Operand::Name(right)) => Some ((left, right)),
            Operation::Mul(Operand::Name(left), Operand::Name(right)) => Some ((left, right)),
            Operation::Div(Operand::Name(left), Operand::Name(right)) => Some ((left, right)),
            _ => None,
        };

        // Record that monkey left and right both appear in the operation of the `idx`th monkey
        if let Some ((left, right)) = names {

            indexes.get_mut(left).ok_or(anyhow!("Invalid monkey name: {}", left))?.1.push(idx);
            indexes.get_mut(right).ok_or(anyhow!("Invalid monkey name: {}", right))?.1.push(idx);
        }
    }

    Ok(indexes)
}

fn update_operation (op: &mut Operation, monkey_name: &str, yell: usize) -> Option<usize>  {

    // Replace the left operand if the name matches
    if let Some ((left, _)) = op.get_operands () {
        if let Operand::Name (name) = left {
            if name == monkey_name {
                op.replace_left (Operand::Number (yell));
            }
        }
    }

    // Replace the right operand if the name matches
    if let Some ((_, right)) = op.get_operands () {
        if let Operand::Name (name) = right {
            if name == monkey_name {
                op.replace_right (Operand::Number (yell));
            }
        }
    }

    // If both operands are yelling monkeys, compute the result
    let maybe_yell = match op {
        Operation::Add(Operand::Number (yell_left), Operand::Number (yell_right)) => Some (*yell_left + *yell_right),
        Operation::Sub(Operand::Number (yell_left), Operand::Number (yell_right)) => Some (*yell_left - *yell_right),
        Operation::Div(Operand::Number (yell_left), Operand::Number (yell_right)) => Some (*yell_left / *yell_right),
        Operation::Mul(Operand::Number (yell_left), Operand::Number (yell_right)) => Some (*yell_left * *yell_right),
        _ => None,
    };

    // Make the corresponding monkey yell the result
    if let Some (yell) = maybe_yell {
        *op = Operation::Yell (yell);
    }

    maybe_yell
}

fn get_processing_queue (monkeys: &[Monkey]) -> Vec<(usize, usize)> {

    let mut queue: Vec<(usize, usize)> = vec![];
    for (idx, (_name, op)) in monkeys.iter ().enumerate () {
        if let Operation::Yell(number) = op {
            queue.push((idx, *number));
        }
    }

    queue
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let mut monkeys = get_monkeys(content)?;
    let indexes = get_monkeys_indexes(&monkeys)?;

    let mut queue = get_processing_queue(&monkeys);
    while let Some ((idx, number)) = queue.pop() {

        // Yelling monkey (name and indexes where he is involved)
        let name = &monkeys[idx].0.clone();
        for &other_idx in indexes[name].1.iter () {

            let op = &mut monkeys[other_idx].1;
            let maybe_yell = update_operation(op, &name, number);
            if let Some (yell) = maybe_yell {
                queue.push((other_idx, yell));
            }
        }
    }

    let root_index = indexes ["root"].0;
    if let Operation::Yell(number) = monkeys[root_index].1 {
        Ok(number)
    }
    else {
        bail!("Invalid root operation: {:?}", monkeys[root_index].1);
    }
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