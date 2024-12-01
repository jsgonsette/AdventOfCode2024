use std::fmt::Error;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

// ================================================================================================

fn extract_num_cubes (game: &str) -> Option<(usize, usize, usize)> {

    let cube_it = game.trim().split(',');
    let (mut r, mut g, mut b) = (0, 0, 0);
    for cube in cube_it {

        let (num, color) = {
            let mut it = cube.trim().split(' ');
            (
                it.next()?,
                it.next()?,
            )
        };

        match color {
            "red" => r += num.parse::<usize> ().ok()?,
            "green" => g += num.parse::<usize> ().ok()?,
            "blue" => b += num.parse::<usize> ().ok()?,
            _ => return None,
        }
    }

    Some ((r, g, b))
}

fn get_game_number_from_head (head: &str) -> Option<usize> {
    let mut it = head.trim().split(' ');
    it.next()?;
    it.next()?.parse::<usize>().ok()
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {

    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    let mut game_sum = 0;
    let lines_it = reader.lines().flatten();
    for line in lines_it {

        let (head, games) = {
            let mut it = line.split(':');
            (
                it.next().ok_or(anyhow!("invalid line"))?,
                it.next().ok_or(anyhow!("invalid line"))?,
            )
        };

        let game_number: usize = get_game_number_from_head(head).ok_or(anyhow!("invalid head"))?;

        let mut game_it = games.trim().split(';');

        let possible = game_it.try_fold(true, |possible, game| {
            match extract_num_cubes(game) {
                Some((r, g, b)) => {
                    let game_possible = r <= MAX_RED && g <= MAX_GREEN && b <= MAX_BLUE;
                    Some (possible && game_possible)
                },
                _ => None,
            }
        });

        if let Some (possible) = possible {
            if possible { game_sum += game_number; }
        } else {
            bail!("invalid game number");
        }
    }
    Ok(game_sum)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {

    let mut game_sum = 0;
    let lines_it = reader.lines().flatten();
    for line in lines_it {

        let (_head, games) = {
            let mut it = line.split(':');
            (
                it.next().ok_or(anyhow!("invalid line"))?,
                it.next().ok_or(anyhow!("invalid line"))?,
            )
        };

        let mut game_it = games.trim().split(';');

        let (min_r, min_g, min_b) = game_it.fold(
            (0, 0, 0),
            | accumulator, game | {
                match extract_num_cubes(game) {
                    Some((r, g, b)) => {
                        (accumulator.0.max(r),
                         accumulator.1.max(g),
                         accumulator.2.max(b))
                    }
                    None => accumulator,
                }
            }
        );

        game_sum += min_r * min_g * min_b;
    }
    Ok(game_sum)
}

// ================================================================================================

pub fn day_2() -> Result<()> {

    println!("=== Part 1 ===");
    assert_eq!(8, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result for part 1 = {}", result);

    println!("\n=== Part 2 ===");
    assert_eq!(2286, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result for part 2 = {}", result);

    Ok(())
}
