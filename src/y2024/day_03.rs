use anyhow::*;

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\
";

/// Steps to detect the mul pattern
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum StateMul {
    Mul(u8),
    LeftBrace,
    LeftNumber(u32),
    RightNumber(u32),
    Done(u32),
}

/// Detects a simple string pattern
#[derive(Clone, Debug, PartialEq, Eq)]
struct SimplePattern {
    pattern: String,
    index: usize,
}

/// Detects the multiplication pattern
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct MulPattern {
    state: StateMul,
    left: Option<u32>,
}

impl SimplePattern {
    fn new(pattern: String) -> SimplePattern {
        SimplePattern {
            pattern,
            index: 0,
        }
    }

    fn process (&mut self, c: char) -> bool {

        let first = self.pattern.as_bytes()[0] as char;
        let current = self.pattern.as_bytes() [self.index] as char;

        self.index = match c {
            c if c == current => self.index + 1,
            c if c == first   => 1,
            _                 => 0,
        };

        match self.index {
            i if i == self.pattern.len() => {
                self.index = 0;
                true
            },
            _ => false
        }
    }
}

impl MulPattern {
    fn new() -> Self {
        MulPattern {
            state: StateMul::Mul(0),
            left: None,
        }
    }

    fn reset (&mut self, c: char) {
        self.state = if c == 'm' { StateMul::Mul(1) } else { StateMul::Mul(0) };
        self.left = None;
    }

    fn process (&mut self, c: char) -> Option<u32> {

        self.state = match (self.state, c) {

            (StateMul::Mul(0), 'm') => StateMul::Mul(1),
            (StateMul::Mul(1), 'u') => StateMul::Mul(2),
            (StateMul::Mul(2), 'l') => StateMul::LeftBrace,

            (StateMul::LeftBrace, '(') => StateMul::LeftNumber(0),

            (StateMul::LeftNumber(n), ',') => {
                self.left = Some(n);
                StateMul::RightNumber(0)
            },
            (StateMul::LeftNumber(n), _) if c.is_digit(10) => {
                let new_n = n * 10 + c.to_digit(10).unwrap();
                StateMul::LeftNumber(new_n)
            },

            (StateMul::RightNumber(n), _) if c.is_digit(10) => {
                let new_n = n * 10 + c.to_digit(10).unwrap();
                StateMul::RightNumber(new_n)
            },
            (StateMul::RightNumber(right), ')') => {
                let Some (left) = self.left else { panic!() };
                StateMul::Done (left*right)
            },

            _ => {
                self.reset(c);
                self.state
            },
        };

        match self.state {
            StateMul::Done(n) => Some (n),
            _ => None,
        }
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn part_a (content: &[&str]) -> Result<usize> {

    let mut mul_matcher = MulPattern::new();
    let mut mul_sum = 0;

    for row in content.iter() {
        for &b in row.as_bytes() {
            let c = b as char;
            if let Some(value) = mul_matcher.process(c) {
                mul_sum += value;
            }
        }
    }

    Ok(mul_sum as usize)
}

fn part_b (content: &[&str]) -> Result<usize> {

    let mut active = true;
    let mut activate_matcher = SimplePattern::new("do()".to_string());
    let mut deactivate_matcher = SimplePattern::new("don't()".to_string());
    let mut mul_matcher = MulPattern::new();
    let mut mul_sum = 0;

    for row in content.iter() {
        for &b in row.as_bytes() {
            let c = b as char;

            if activate_matcher.process(c) { active = true; }
            if deactivate_matcher.process(c) { active = false; }
            if active {
                if let Some(value) = mul_matcher.process(c) {
                    mul_sum += value;
                }
            }
        }
    }

    Ok(mul_sum as usize)
}

pub fn day_3 (content: &[&str]) -> Result <(usize, usize)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 161);
    debug_assert!(part_b (&split(TEST2)).unwrap_or_default() == 48);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((ra, rb))
}