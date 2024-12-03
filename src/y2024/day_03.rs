use anyhow::*;

const TEST: &str = "\

";

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}


fn part_a (content: &[&str]) -> Result<usize> {

    Ok(0)
}

fn part_b (content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_3 (content: &[&str]) -> Result <(usize, usize)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 2);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 4);

    let ra = 0;//part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((ra, rb))
}