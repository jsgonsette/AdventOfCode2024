use anyhow::*;
use crate::{Cell, CellArea, Solution};

const TEST: &str = "\
30373
25512
65332
33549
35390";

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Return an iterator yielding tree height for each character in the provided `row`
fn convert_input_height_<'a> (row: &'a str) -> impl Iterator<Item=Result<u8>> + 'a {
    row.as_bytes().iter ().map (
        |b| if b.is_ascii_digit() {
            Ok(*b - b'0')
        } else {
            bail!("")
        }
    )
}

/// Convert the puzzle file `content` into a unique vector representing the forest.
fn convert_content_to_forest (content: &[&str]) -> Result<Vec<u8>> {
    content.iter().flat_map(
        |row| convert_input_height_(row)
    ).collect()
}

fn convert_input_height (row: &str) -> Result<Vec<i8>> {

    row.as_bytes().iter ().map (
        |b| if b.is_ascii_digit() {
            Ok((*b - b'0') as i8)
        } else {
            bail!("")
        }
    ).collect()
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let forest_width = content [0].len();
    let mut top_vertical = vec! [-1; forest_width];
    let mut tree_visibility = vec! [false; forest_width * content.len()];

    let mut make_visible = |idx_h: usize, idx_v: usize| {
        (&mut tree_visibility) [idx_v * forest_width + idx_h] = true;
    };

    for (idx_v, row) in content.iter().enumerate() {
        let heights = convert_input_height(&row)?;

        heights
            .iter()
            .enumerate ()
            .fold((-1, 0), |(mut top, mut num_visible), (idx_h,  &height)| {

                if height > top {
                    top = height;
                    make_visible (idx_h, idx_v);
                }

                if height > top_vertical [idx_h] {
                    top_vertical [idx_h] = height;
                    make_visible (idx_h, idx_v);
                }

                (top, num_visible)
            });
    }

    top_vertical.fill(-1);
    for (idx_v, row) in content.iter().rev ().enumerate() {
        let heights = convert_input_height(&row)?;

        heights
            .iter().rev ()
            .enumerate ()
            .fold((-1, 0), |(mut top, mut num_visible), (idx_h,  &height)| {

                let idx_h = forest_width - idx_h -1;
                let idx_v = content.len() - idx_v -1;

                if height > top {
                    top = height;
                    make_visible (idx_h, idx_v);
                }

                if height > top_vertical [idx_h] {
                    top_vertical [idx_h] = height;
                    make_visible (idx_h, idx_v);
                }

                (top, num_visible)
            });
    }

    Ok(tree_visibility.iter().filter(|&&x| x).count())
}

/// Solve second part of the puzzle
///
/// We solve the scenic score in one pass through the forest, top-down and left to right.
/// * Each time we consider a tree at (x, y), we have seen everything above and on the left,
/// which means that we can do the maths for the left and top components.
/// * In addition, we can update the score components of all the tree that are smaller on the
/// left and on top.
fn part_b (content: &[&str]) -> Result<usize> {

    let forest_width = content [0].len();
    let forest_height = content.len();
    let forest = convert_content_to_forest(content)?;

    // Score for each tree in the forest
    let mut score = vec! [1; forest_width * forest_height];

    // When scanning a row, keep track of the x position of the last tree whose size is taller.
    // Example: 'last_taller_than [4]' gives the last position of a tree whose height is 4 or above.
    let mut last_taller_than = vec! [0; 10];

    // Same, but for each column in the forest
    let mut last_taller_than_vert = vec![vec! [0; 10]; forest_width];

    let index = |x: usize, y: usize| { y * forest_width + x };

    // Scan top down and left to right
    for y in 0..forest_height {
        last_taller_than.fill(0);

        for x in 0..forest_width {

            // Trees on the border have a scenic score of 0 and must not be considered.
            // However, we artificially change their height for our logic to work.
            let on_border = x == 0 || y == 0 || x == forest_width - 1 || y == forest_height - 1;
            let h = match on_border {
                false => forest [index (x, y)] as usize,
                true => 9,
            };

            // Look left and up and determine the left and up score at (x, y)
            let distance_left = x - last_taller_than[h];
            let distance_up = y - last_taller_than_vert[x][h];
            score [index (x, y)] *= distance_left * distance_up;
            if on_border { score [index (x, y)] = 0 }

            // Consider smaller trees on our left and above us to determine their right and down score.
            // There are multiple candidates to consider (a tall tree can set a bound for many smaller ones)
            for height in 0..= h {

                // get the positions of the closest trees (left and up) of given 'height' or more
                let x_left = last_taller_than [height];
                let y_up = last_taller_than_vert[x][height];

                // and update its score if it has the height we look for
                let idx = index(x_left, y);
                if forest [idx] as usize == height {
                    let distance = x - x_left;
                    score[idx] *= distance;
                }

                let idx = index(x, y_up);
                if forest [idx] as usize == height {
                    let distance = y - y_up;
                    score[idx] *= distance;
                }
            }

            // Update our height tracking variables
            for height in 0..= h {
                last_taller_than [height] = x;
                last_taller_than_vert[x][height] = y;
            }

        }
    }

    let highest_score = *score.iter ().max().unwrap();
    Ok(highest_score)
}

pub fn day_8 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 21);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 8);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}