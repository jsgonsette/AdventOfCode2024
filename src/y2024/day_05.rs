use anyhow::*;

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

#[derive(Debug)]
struct Rules {

    rules: Vec<(u32, u32)>
}

impl Rules {
    fn new (content: &[&str]) -> Result<Rules> {

        let rules: Vec<(u32, u32)> = content.iter ().map_while(|&row| {

            println!("Parsing row {}", row);
            if row.is_empty() {
                None
            } else {
                let first = row[0..2].parse::<u32>().ok()?;
                let second = row[3..].parse::<u32>().ok()?;
                Some((first, second))
            }
        }).collect();

        if !content [rules.len()].is_empty() {
            bail!("Rule separator not found")
        }
        else {
            Ok (Rules { rules })
        }
    }
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let rules = Rules::new(content)?;
    dbg!(&rules);

    Ok(0)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {



    Ok(0)
}

pub fn day_5 (content: &[&str]) -> Result <(usize, usize)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 0);
    //debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = 0;//part_a(content)?;
    let rb = 0;//part_b(content)?;

    Ok((ra, rb))
}