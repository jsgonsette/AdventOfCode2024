use anyhow::*;
use crate::{Solution};

const TEST: &str = "\
1
2
-3
3
-2
0
4";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}


/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let mut zero_index = None;

    let values: Result<Vec<i32>> = content.iter().enumerate ()
        .map(
            |(idx, s)| {
                if *s == "0" { zero_index = Some (idx) };
                s.parse::<i32>().map_err(|_e| anyhow!("Not valid numbers"))
            }
        ).collect();

    let zero_index = zero_index.ok_or(anyhow!("No zero position"))?;

    let values = values?;
    let mut indexes: Vec<usize> = (0..values.len()).collect();

    //println!("Values {:?}", values);

    for index in 0..values.len() {
       // println!("Indexes {:?}", indexes);
        let current_pos = indexes.iter().position(|i| *i == index).unwrap();
        let step = values [index];
        //let new_pos = (current_pos + step_abs as usize) % values.len();
        let new_pos = (current_pos as i32 + step).rem_euclid(values.len() as i32 -1) as usize;
        indexes.remove(current_pos);
        indexes.insert(new_pos, index);
    }
   //println!("Indexes {:?}", indexes);

    let zero_pos = indexes.iter ().position(|i| *i == zero_index).unwrap();
    let zero_1000 = (zero_pos + 1000) % values.len();
    let zero_2000 = (zero_pos + 2000) % values.len();
    let zero_3000 = (zero_pos + 3000) % values.len();

   // println!("Zero  {}", zero_pos);
   // println!("Zero 1000 {}", zero_1000);

    let decrypted = values [indexes [zero_1000]] + values [indexes [zero_2000]] + values [indexes [zero_3000]];
    println!("Decrypted {}", decrypted);
    Ok(0)
}

/// Solve second part of the puzzle
fn part_b (_content: &[&str]) -> Result<usize> {

    Ok(0)
}

pub fn day_20 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 0);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}