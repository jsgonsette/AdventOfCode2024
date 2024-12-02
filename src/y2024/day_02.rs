use anyhow::*;
use num::Signed;

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn make_levels (row: &str) -> Vec<u32> {
    let items = row.split(' ');
    items.map(|x| x.parse::<u32>().unwrap()).collect()
}

fn is_safe (levels: &[u32]) -> bool {

    if levels.len() <= 1 { return true }
    let is_increasing = levels [1] > levels[0];

    let r = levels.iter().skip(1).fold ((true, levels [0]), |(safe, level), next_level| {

        let diff = (*next_level as i32 - level as i32).abs() as u32;
        let next_increasing = *next_level > level;
        let next_safe = diff <= 3 && diff >= 1;
        (safe && next_safe && (next_increasing == is_increasing), *next_level)
    });

    r.0
}

fn part_a (content: &[&str]) -> Result<usize> {

    let mut sum = 0;
    for row in content {
        let levels = make_levels(row);
        let safe = is_safe(&levels);
        if safe { sum += 1 }
    }
    Ok(sum)
}

fn part_b (content: &[&str]) -> Result<usize> {

    let mut sum = 0;
    for row in content {

        let levels = make_levels(row);
        let n = levels.len();;
        let safe = is_safe(&levels);

        let shrinked = (0usize..levels.len()).map (|idx| {
            match idx {
                0 => levels.iter().skip(1).copied ().collect::<Vec<_>>(),
                x => levels.iter().take (x).chain (
                    levels.iter ().skip(x+1)).copied().collect::<Vec<_>>(),
            }
        });

       // dbg!(shrinked.clone ().collect::<Vec<_>>());

        let any_safe = shrinked.into_iter().any (|x| is_safe(x.as_slice()));

        if safe || any_safe { sum += 1 }
    }
    Ok(sum)
}

pub fn day_2 (content: &[&str]) -> Result <(usize, usize)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 2);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 4);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((ra, rb))
}