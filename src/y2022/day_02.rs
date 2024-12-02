use anyhow::*;

const TEST: &str = "\
A Y
B X
C Z
";

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn part_a (content: &[&str]) -> anyhow::Result<usize> {

    let score: u32 = content.iter().map(|row| {
        match row {
            r if *r == "A X" => 1 + 3,  // Rock Rock
            r if *r == "A Y" => 2 + 6,  // Rock Paper
            r if *r == "A Z" => 3 + 0,  // Rock Scissors

            r if *r == "B X" => 1 + 0,  // Paper Rock
            r if *r == "B Y" => 2 + 3,  // Paper Paper
            r if *r == "B Z" => 3 + 6,  // Paper Scissors

            r if *r == "C X" => 1 + 6,  // Scissors Rock
            r if *r == "C Y" => 2 + 0,  // Scissors Paper
            r if *r == "C Z" => 3 + 3,  // Scissors Scissors
            _ => panic!("Unexpected row: {}", row),
        }
    }).sum();

    Ok(score as usize)
}

fn part_b (content: &[&str]) -> anyhow::Result<usize> {

    let score: u32 = content.iter().map(|row| {
        match row {
            r if *r == "A X" => 3 + 0,  // Rock Loose Scissors
            r if *r == "A Y" => 1 + 3,  // Rock Draw Rock
            r if *r == "A Z" => 2 + 6,  // Rock Win Paper

            r if *r == "B X" => 1 + 0,  // Paper Loose Rock
            r if *r == "B Y" => 2 + 3,  // Paper Draw Paper
            r if *r == "B Z" => 3 + 6,  // Paper Win Scissors

            r if *r == "C X" => 2 + 0,  // Scissors Loose Paper
            r if *r == "C Y" => 3 + 3,  // Scissors Draw Scissors
            r if *r == "C Z" => 1 + 6,  // Scissors Win Rock
            _ => panic!(),
        }
    }).sum();

    Ok(score as usize)
}

pub fn day_2022_2 (content: &[&str]) -> anyhow::Result<(usize, usize)> {

    debug_assert!(part_a(&split(TEST)).unwrap_or_default() == 15);
    debug_assert!(part_b(&split(TEST)).unwrap_or_default() == 12);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    anyhow::Ok((ra, rb))
}