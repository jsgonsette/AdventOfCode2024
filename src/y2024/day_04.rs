use anyhow::*;

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

#[derive(Debug, Copy, Clone)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal1,
    Diagonal2,
}

#[derive(Debug, Copy, Clone)]
enum FB {
    Forward,
    Backward,
}

static DIRECTIONS: &[Direction] = &[
    Direction::Horizontal,
    Direction::Vertical,
    Direction::Diagonal1,
    Direction::Diagonal2
];

static FBS: &[FB] = &[
    FB::Backward,
    FB::Forward
];

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Return the character at some position `(x, y)`, or None if the position is out limit.
fn sample_content (content: &[&str], x: isize, y: isize) -> Option<char> {

    if x < 0 || y < 0 {
        None
    } else {
        content.get(y as usize).and_then(
            |&row| row.as_bytes().get(x as usize)
        ).map(|&c| c as char)
    }
}

/// Compute the coordinate adjacent to `(x, y)` for some orientation `direction / fb`.
/// This new coordinate can be out limit !
fn next_coordinate (x: isize, y: isize, direction: Direction, fb: FB) -> (isize, isize) {

    let v = match direction {
        Direction::Horizontal => (1, 0),
        Direction::Vertical => (0, 1),
        Direction::Diagonal1 => (1, 1),
        Direction::Diagonal2 => (-1, 1),
    };

    let v_signed = match fb {
        FB::Forward => v,
        FB::Backward => (-v.0, -v.1),
    };

    (x+v_signed.0, y+v_signed.1)
}

/// Check if the XMAS pattern can be found at some location `(x, y)`
/// and orientation `direction / fb`.
fn look_at (content: &[&str], x: isize, y: isize, direction: Direction, fb: FB) -> bool {

    static PATTERN: [char; 4] = ['X', 'M', 'A', 'S'];

    let mut xy = (x, y);
    for step in 0..4 {
        if sample_content(content, xy.0, xy.1) != Some(PATTERN[step]) { return false; }
        xy = next_coordinate(xy.0, xy.1, direction, fb);
    }

    true
}

/// Count how many XMAS patterns are found horizontally, vertically and diagonally, starting
/// at the coordinate `(x, y)`
fn look_around (content: &[&str], x: isize, y: isize) -> usize {

    let mut count = 0;
    for dir in DIRECTIONS.iter() {
        for fb in FBS.iter() {
            if look_at(content, x, y, *dir, *fb) { count += 1; }
        }
    }

    count
}

/// Check if the X-MAS pattern is found at location `(x, y)`
fn look_around_x (content: &[&str], x: isize, y: isize) -> bool {

    if sample_content(content, x, y) != Some('A') { return false; }
    let c1 = sample_content(content, x-1, y-1);
    let c2 = sample_content(content, x+1, y+1);
    let c3 = sample_content(content, x+1, y-1);
    let c4 = sample_content(content, x-1, y+1);

    match (c1, c2, c3, c4) {
        (Some('M'), Some('S'), Some('M'), Some('S')) => true,
        (Some('M'), Some('S'), Some('S'), Some('M')) => true,
        (Some('S'), Some('M'), Some('M'), Some('S')) => true,
        (Some('S'), Some('M'), Some('S'), Some('M')) => true,
        _ => false,
    }
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let width = content[0].as_bytes().len();
    let height = content.len();
    let mut sum = 0;

    for x in 0..width {
        for y in 0..height {
            sum += look_around(content, x as isize, y as isize);
        }
    }

    Ok(sum)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let width = content[0].as_bytes().len();
    let height = content.len();
    let mut sum = 0;

    for x in 0..width {
        for y in 0..height {
            if look_around_x(content, x as isize, y as isize) { sum += 1; }
        }
    }

    Ok(sum)
}

pub fn day_4 (content: &[&str]) -> Result <(usize, usize)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 18);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 9);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((ra, rb))
}