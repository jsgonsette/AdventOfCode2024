use anyhow::*;
use crate::Solution;

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

type Value = usize;
type Operands = Vec<usize>;

/// The different type of operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Mul,
    Concat,
}

/// A value to match and a list of operands (but not the operators)
type Equation = (Value, Operands);

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Extract a list of [Equation] from the puzzle file content
fn read_equations (content: &[&str]) -> Result<Vec<Equation>> {

    let mut equations: Vec<Equation> = vec! [];
    equations.reserve(content.len());

    for &row in content.iter () {

        // Separate the value from the operands
        let mut tokens = row.split (':');
        let value = tokens.next ().ok_or(anyhow!("Equation value not found in {row}"))?;
        let operands = tokens.next ().ok_or(anyhow!("Operands not found in {row}"))?;

        // Read the value
        let value = value.parse::<usize>()?;

        // Read the operands
        let operands = operands.trim ().split(' ');
        let operands: Result<Vec<Value>> = operands.map(
            |v| {
                v.parse::<usize> ().map_err(|_e| anyhow!("Invalid value {v}"))
            }
        ).collect();
        let operands = operands?;

        equations.push((value, operands) );
    };

    Ok (equations)
}

/// Return an iterator on all the possible combinations that it is possible to arrange in a vector
/// of [Operator] of length `num_operators`. If `allow_concatenation` is false,
/// the operation [Operator::Concat] is excluded.
fn make_operators_iterator(num_operators: u32, allow_concatenation: bool) -> impl Iterator<Item = Vec<Operator>> {

    assert!(num_operators < 20);

    // Number of possible combinations to produce
    let num_combi = 3u32.pow(num_operators);

    // Change if we work with 2 or 3 operations
    let base = match allow_concatenation {
        false => 2,
        true => 3,
    };

    // Iterator on the combinations
    (0..num_combi).map (move |mut combi| {

        // Create the current combination
        let operators: Vec<Operator> = (0..num_operators).map (|_| {
            let op_num = combi % base;
            combi /= base;

            match op_num {
                0 => Operator::Add,
                1 => Operator::Mul,
                _ => Operator::Concat,
            }
        }).collect();

        operators
    })
}

/// Given a list of `operands` and a list of `operators` to put in between, return
/// the resulting computed value.
fn compute_value (operands: &Operands, operators: &[Operator]) -> Value {

    assert_eq!(operands.len(), operators.len () +1);

    let it_0 = operands.iter().skip(1);
    let it_1 = operators.iter();

    let init = operands [0];
    it_0.zip(it_1).fold(init, |acc, (&op, operation)| {
        match operation {
            Operator::Add => acc + op,
            Operator::Mul => acc * op,
            Operator::Concat => {
                let num_digits = op.ilog10()+1;
                let shift = 10usize.pow(num_digits);
                acc * shift + op
            },
        }
    })
}

/// Solve the puzzle
fn solve(content: &[&str], allow_concat: bool) -> Result<usize> {

    // Extract the equations to solve
    let equations = read_equations(content)?;

    // For each of them ...
    let mut sum_valid = 0;
    for (value, operands) in equations.into_iter() {

        // ... iterate on the possible operators to put in between the operands
        let num_operators = operands.len() -1;
        for operators in make_operators_iterator(num_operators as u32, allow_concat) {

            // Test the current combination
            let test_value = compute_value(&operands, &operators);
            if value == test_value {
                sum_valid += value;
                break;
            }
        }
    }

    Ok(sum_valid)
}

pub fn day_7 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve(&split(TEST), false).unwrap_or_default() == 3749);
    debug_assert!(solve (&split(TEST), true).unwrap_or_default() == 11387);

    let ra = solve(content, false)?;
    let rb = solve(content, true)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}