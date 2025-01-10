use anyhow::*;
use crate::Solution;
use crate::tools::IntReader;

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// dx, dy pair
type Step = (isize, isize);

/// Model a claw machine
#[derive(Debug, Copy, Clone)]
struct ClawMachine {
    a: Step,
    b: Step,
    prize: Step,
}

impl ClawMachine {

    /// Solve the machine by returning the number of times to press the A and B buttons, if possible.
    fn solve (&self, with_correction: bool) -> Option<Step> {

        // Rename the coefficients
        let (x, y) = self.prize;
        let x = if with_correction { x + 10000000000000 } else { x };
        let y = if with_correction { y + 10000000000000 } else { y };

        let (xa, ya) = self.a;
        let (xb, yb) = self.b;

        // Solve the system of 2 equations with 2 unknowns
        let num_a = y*xb - yb*x;
        let num_b = x*ya - xa*y;
        let den = ya*xb - yb*xa;

        // Detect impossible cases
        if den == 0 { return None; }
        if num_a % den != 0 { return None; }
        if num_b % den != 0 { return None; }

        // Return the solution, if acceptable
        let a = num_a/den;
        let b = num_b/den;

        if a >= 0 && b >= 0 {
            if with_correction || (a <= 100 && b <= 100) {
                Some ((a, b))
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Load the definitions of the claw machines from the puzzle file content
fn load_machines (content: &[&str]) -> Result<Vec<ClawMachine>> {

    let mut reader = IntReader::new(false);
    let num_machines = 1 + content.len()/4;

    let machines:Result<Vec<ClawMachine>> = (0..num_machines).map(|idx| {

        let a: [usize;2] = reader.process_row_fix(
            content[idx*4]
        ).ok_or(anyhow!("button A not found"))?;

        let b: [usize;2] = reader.process_row_fix(
            content[idx*4+1]
        ).ok_or(anyhow!("button B not found"))?;

        let prize: [usize;2] = reader.process_row_fix(
            content[idx*4+2]
        ).ok_or(anyhow!("prize loc not found"))?;

        Ok(ClawMachine {
            a: (a [0] as isize, a [1] as isize),
            b: (b [0] as isize, b [1] as isize),
            prize: (prize [0] as isize, prize [1] as isize),
        })
    }).collect();

    machines
}

/// Solve the puzzle, with or without the prize location correction
fn solve (content: &[&str], with_correction: bool) -> Result<usize> {

    let machines = load_machines(content)?;

    let mut sum = 0;
    for machine in machines {
        if let Some ((a, b)) = machine.solve(with_correction) {
            sum += a*3 + b;
        }
    }

    Ok(sum as usize)
}


pub fn day_13 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve(&split(TEST), false).unwrap_or_default() == 480);

    let ra = solve(content, false)?;
    let rb = solve(content, true)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}