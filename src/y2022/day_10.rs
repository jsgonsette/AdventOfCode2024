use anyhow::*;
use crate::{Solution};

const TEST: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

/// Models the different instructions
#[derive(Debug, Copy, Clone)]
enum Ins {
    Noop,
    Addx(i8),
}

/// Models the screen
struct Screen {
    data: [bool; 6*40],
    cycle: usize,
}

impl Screen {

    /// New screen instance
    fn new() -> Screen {
        Screen {
            data: [false; 6*40],
            cycle: 0,
        }
    }

    /// Draw one pixel and move the cycle by +1
    fn cycle(&mut self, x: isize) {
        let crt = (self.cycle % 40) as isize;
        if crt >= x -1 && crt <= x+1 {
            self.data [self.cycle] = true;
        }
        self.cycle += 1;
    }

    fn _print(&self) {
        for y in 0..6 {
            println! ();
            for x in 0..40 {
                if self.data [y*40+x] { print!("#") }
                else { print!(" ") }
            }
        }
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Return an iterator yielding [Ins], based on the puzzle file `content`
fn get_instructions<'a> (content: &'a[&'a str]) -> impl Iterator<Item=Result<Ins>> + 'a {

    content.iter().map (|&row| {
        if row.starts_with("addx") {
            let value = row.split_at(5).1.parse::<i8>()?;
            Ok(Ins::Addx(value))

        } else if row.starts_with("noop") {
            Ok(Ins::Noop)
        } else {
            Err(anyhow!("Unknown instruction: {}", row))
        }
    })
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let instructions = get_instructions(content);

    let mut x = 1;
    let mut cycle = 1;
    let mut target = 20isize;
    let mut strengths = 0;

    for ins in instructions {
        let x_prev = x;

        // Execute instruction
        match ins? {
            Ins::Noop => cycle += 1,
            Ins::Addx(v) => {
                cycle += 2;
                x += v;
            }
        }

        // When reaching the target cycle exactly, the new x value is used
        if cycle == target {
            strengths += target * x as isize;
            target += 40;
        }
        // If the target is exceeded, then we must use the value before the instruction
        else if cycle > target {
            strengths += target * x_prev as isize;
            target += 40;
        }
    }

    Ok(strengths as usize)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let instructions = get_instructions(content);
    let mut screen = Screen::new();
    let mut x = 1;

    for ins in instructions {
        match ins? {
            Ins::Noop => screen.cycle(x),
            Ins::Addx(v) => {
                screen.cycle(x);
                screen.cycle(x);
                x += v as isize;
            }
        }
    }

    //screen._print();
    Ok(0)
}

pub fn day_10 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 13140);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}