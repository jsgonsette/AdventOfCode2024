use anyhow::*;

const TEST: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

struct Range (u32, u32);
enum RangeState {
    LeftA, LeftB, RightA, RightB
}
struct RangesParser {
    left_a: u32,
    left_b: u32,
    right_a: u32,
    right_b: u32,
    state: RangeState,
}

impl Range {
    fn is_contained_in (&self, other: &Self) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }
}

impl RangesParser {
    fn new() -> RangesParser {
        RangesParser {
            left_a: 0,
            left_b: 0,
            right_a: 0,
            right_b: 0,
            state: RangeState::LeftA,
        }
    }

    fn process(&mut self, c: char) -> Result<()> {
        let digit = c.to_digit(10);
        match (&self.state, digit) {
            (RangeState::LeftA, Some (d)) => { self.left_a = self.left_a*10 + d; Ok(()) },
            (RangeState::LeftB, Some (d)) => { self.left_b = self.left_b*10 + d; Ok(()) },
            (RangeState::RightA, Some (d)) => { self.right_a = self.right_a*10 + d; Ok(()) },
            (RangeState::RightB, Some (d)) => { self.right_b = self.right_b*10 + d; Ok(()) },
            (RangeState::LeftA, None) if c == '-' => { self.state = RangeState::LeftB; Ok(()) },
            _ => bail!("Invalid character '{}'", c),
        }
    }

    fn left_range (&self) -> Range {
        Range(self.left_a, self.left_b)
    }

    fn right_range (&self) -> Range {
        Range(self.right_a, self.right_b)
    }
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn read_ranges (row: &str) -> Result<(Range, Range)> {
    let mut parser= RangesParser::new();

    for &b in row.as_bytes() {
        let c = b as char;
        parser.process(c).map_err(|_e| anyhow!("Invalid row: {}", row))?;
    }

    Ok((parser.left_range(), parser.right_range()))
}

fn part_a (content: &[&str]) -> anyhow::Result<usize> {

    let mut total_overlap_count = 0;

    for row in content.iter() {
        let (left_range, right_range) = read_ranges(row)?;
        if left_range.is_contained_in(&right_range) || right_range.is_contained_in(&left_range) {
            total_overlap_count += 1;
        }
    }

    Ok(total_overlap_count)
}

fn part_b (content: &[&str]) -> anyhow::Result<usize> {


    Ok(0)
}

pub fn day_4(content: &[&str]) -> anyhow::Result<(usize, usize)> {

    debug_assert!(part_a(&split(TEST)).unwrap_or_default() == 2);
    debug_assert!(part_b(&split(TEST)).unwrap_or_default() == 0);

    let ra = part_a(content)?;
    let rb = 0;//part_b(content)?;

    anyhow::Ok((ra, rb))
}