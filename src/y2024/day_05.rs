use anyhow::*;

const TEST: &str = "\
";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {



    Ok(0)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {



    Ok(0)
}

pub fn day_5 (content: &[&str]) -> Result <(usize, usize)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 0);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = 0;//part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((ra, rb))
}