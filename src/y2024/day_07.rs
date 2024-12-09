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
        let (value, operands) = row.split_once(':').ok_or(anyhow!("Equation value not found in {row}"))?;

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

/// Merge two operands `op1` and `op2` with the provided `operator`
fn merge_pair (op1: usize, op2: usize, operator: Operator) -> usize {
    match operator {
        Operator::Add => op1 + op2,
        Operator::Mul => op1 * op2,
        Operator::Concat => {
            let num_digits = op2.ilog10()+1;
            let shift = 10usize.pow(num_digits);
            op1 * shift + op2
        },
    }
}

/// Solve the equation *recursively*, given
/// * the final equation `value`
/// * the first operand `first_op`
/// * all the other operands `other_op`
/// * the flag `allow_concat` to enable the third operation
///
/// The function returns `true` if some combination of operators could be found
fn solve_recursive (value: Value, first_op: usize, other_op: &[usize], allow_concat: bool) -> bool {

    if other_op.len() < 1 { return false }
    let op_1 = other_op [0];

    let op_01_add = merge_pair(first_op, op_1, Operator::Add);
    let op_01_mul = merge_pair(first_op, op_1, Operator::Mul);
    let op_01_concat = if allow_concat { merge_pair(first_op, op_1, Operator::Concat) } else { 0 };

    if other_op.len() == 1 {
        if op_01_add == value { true }
        else if op_01_mul == value { true }
        else if allow_concat && op_01_concat == value { true }
        else { false }
    }

    else if other_op.len() > 1 {
        solve_recursive(value, op_01_add, &other_op [1..], allow_concat) ||
        solve_recursive(value, op_01_mul, &other_op [1..], allow_concat) ||
        (allow_concat && solve_recursive(value, op_01_concat, &other_op [1..], allow_concat))
    }

    else {
        unreachable!()
    }
}

/// Solve the puzzle.
/// Flag `allow_concat` allows the third operation for the second part of the problem.
fn solve(content: &[&str], allow_concat: bool) -> Result<usize> {

    // Extract the equations to solve
    let equations = read_equations(content)?;

    // For each of them ...
    let mut sum_valid = 0;
    for (value, operands) in equations.into_iter() {
        let valid = solve_recursive(value, operands [0], &operands [1..], allow_concat);
        if valid { sum_valid += value }
    }

    Ok(sum_valid)
}

pub fn day_7 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve(&split(TEST), false).unwrap_or_default() == 3749);
    debug_assert!(solve(&split(TEST), true).unwrap_or_default() == 11387);

    let ra = solve(content, false)?;
    let rb = solve(content, true)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}