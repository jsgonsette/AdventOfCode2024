use std::collections::{HashMap, HashSet};
use anyhow::*;
use crate::Solution;

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

/// A single page number
type Page = u32;

/// A sequence of page to update
type Update = Vec<Page>;

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Loads and checks the rules of precedence
#[derive(Debug)]
struct Rules {

    /// The rules of precedence
    rules: HashMap<Page, Vec<Page>>,

    /// Number of such rules
    num_rules: usize,
}

impl Rules {

    /// New instance based on the puzzle file `content`
    fn new (content: &[&str]) -> Result<Rules> {

        // Load the list of rules until we detect the empty line
        let list_rules: Vec<(u32, u32)> = content.iter ().map_while(|&row| {
            if row.is_empty() {
                None
            } else {
                let first = row[0..2].parse::<u32>().ok()?;
                let second = row[3..].parse::<u32>().ok()?;
                Some((first, second))
            }
        }).collect();

        if !content [list_rules.len()].is_empty() {
            bail!("Rule separator not found")
        }
        else {
            let num_rules = list_rules.len();

            // Group the rules that have the same first page number
            let mut rules = HashMap::new();
            for (first, second) in list_rules.into_iter() {
                rules.entry(first).or_insert_with(Vec::new).push(second);
            }

            Ok (Rules { num_rules, rules })
        }
    }

    /// Check if a page update sequence is correct, according to the rules
    fn check_update (&self, update: &Update) -> bool {

        // To collect all the pages we have already seen in this update
        let mut pages_seen = HashSet::<u32>::new();

        // Check each page in sequence
        for page in update.iter () {

            // If we have rules specifying what pages should come after
            if let Some (late_pages) = self.rules.get(page) {

                // Check we have not seen it
                let already_seen = late_pages.iter().any(|late_page| pages_seen.contains (late_page));
                if already_seen { return false }
            }

            // We have seen this page
            pages_seen.insert(*page);
        }

        true
    }

    /// Find and return the correct `update` ordering that respects the rules
    fn correct_update (&self, update: Update) -> Result<Update> {

        let mut indexes_ok: Vec<bool> = vec![false; update.len()];
        let mut correct_update: Update = vec![];
        correct_update.reserve(update.len());

        type Constraint = Vec<Page>;

        // Build a constraint for each page in the update. That is, for each page,
        // collect all the other pages of the update that must come first
        let mut constraints: Vec<Constraint> = update.iter().map(
            |page| self.get_constraints(*page, &update)
        ).collect();

        // Build the correct update ordering ...
        for _ in 0..update.len() {

            // Get the index of the next empty constraint (there should be one if no cycle)
            let next_idx = constraints.iter().enumerate ().find_map(
                | (idx, constraint) | {
                    if !indexes_ok[idx] && constraint.is_empty() { Some (idx) } else { None }
                }
            ).ok_or(anyhow!("No empty constraint found. Cycle ?"))?;

            // The page with no constraint can be added to the solution
            let next_page = update [next_idx];
            correct_update.push(next_page);

            // Remove the page we used
            indexes_ok[next_idx] = true;

            // Remove the page we used from the constraints of the other pages
            for constraint in constraints.iter_mut() {
                if let Some (idx) = constraint.iter().position(|p| *p == next_page) {
                    constraint.swap_remove(idx);
                }
            }
        }

        Ok(correct_update)
    }

    /// Given a `page` number belonging to some `update` sequence,
    /// return all the rules that apply to both of them
    fn get_constraints (&self, page: u32, update: &Update) -> Vec<u32> {
        if self.rules.contains_key(&page) == false { vec![] }
        else {
            let constraints = &self.rules [&page];
            constraints.iter().filter(|late_page| {
                update.contains(late_page)
            }).copied ().collect()
        }
    }

    fn num_rules (&self) -> usize {
        self.num_rules
    }
}

/// Read the updates from the puzzle file content
fn read_updates (content: &[&str]) -> Vec<Update> {

    // Read the updates, row by row
    let updates: Vec<Update> = content.iter().map(|row| {

        // Each number is two digits, so we can take some shortcuts
        let len = row.as_bytes().iter().len();
        let num_numbers = (len+1) / 3;
        let update: Update = (0..num_numbers).map(
            |idx| row [idx*3..idx*3+2].parse::<u32>().unwrap()
        ).collect();

        update
    }).collect();

    updates
}


/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    // Extract the rules and the list of updates
    let rules = Rules::new(content)?;
    let updates = read_updates(&content [rules.num_rules()+1 ..]);

    // Sum the middle number of all the correct updates
    let sum: u32 = updates.iter().map (|update| {
            if rules.check_update(update) {
                let middle = (update.len() - 1) / 2;
                update[middle]
            } else {
                0
            }
        }
    ).sum ();

    Ok(sum as usize)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    // Extract the rules and the list of updates
    let rules = Rules::new(content)?;
    let updates = read_updates(&content [rules.num_rules()+1 ..]);

    // Sum the middle number of all the wrong updates, after correction
    let mut sum = 0;
    for update in updates.into_iter() {

        if !rules.check_update(&update) {
            let corrected_update = rules.correct_update (update)?;
            let middle = (corrected_update.len() - 1) / 2;
            sum += corrected_update[middle];
        }
    }

    Ok(sum as usize)
}

pub fn day_5 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 143);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 123);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}
