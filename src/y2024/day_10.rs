use anyhow::*;
use crate::{Solution};
use crate::tools::{Coo, Direction};

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

/// Sample the `map` altitude at some given `coo`
fn try_sample_map(map: &[&str], coo: Coo) -> Option<u8> {

    if coo.x < 0 || coo.x >= map.len() as isize { return None }
    if coo.y < 0 || coo.y >= map[0].len() as isize { return None }

    let row = map [coo.y as usize].as_bytes();
    Some (row [coo.x as usize])
}

/// Draw a trail on the `path_map` by following it back to the start,
/// incrementing the counter along the way.
/// `trail_coo` gives the end of the trail
fn create_path (path_map: &mut PathMap, trail_coo: Coo) {

    // Start from the end of the trail
    let mut coo = trail_coo;

    // Step back to the head
    for altitude in ('0'..= '9' as char).rev() {

        // Increment the trail counter for this path location
        let path_item = &mut path_map [coo.x as usize][coo.y as usize];
        path_item.num_trails += 1;

        if altitude != '0' { coo = path_item.back.unwrap() }
        else { assert!(path_item.back.is_none()); }
    }
}

/// Score a trail's head starting at `head_coo`. If `with_rating` is true, the score counts
/// all the possibly different trails leading to height `9`
fn score_head (map: &[&str], path_map: &mut PathMap, head_coo: Coo, with_rating: bool) -> u32 {

    let height = map.len();
    let width = map[0].len();

    // DFS queue
    let mut jobs = Vec::<Job>::with_capacity(height*width);
    let first_job = Job { coo: head_coo };
    jobs.push(first_job);

    // Process the jobs
    while let Some (job) = jobs.pop() {

        // Check if we have reached a goal. In this case, trace the path
        let Some (altitude) = try_sample_map(&map, job.coo) else { unreachable!() };
        if altitude == '9' as u8 {
            create_path(path_map, job.coo);
        }

        // Test all the directions around
        for dir in Direction::iter() {
            let Some(n_coo) = job.coo.try_next(dir, width, height) else { continue };

            // If we don't care about rating, ignore the locations that have already been visited
            // (they are already explored)
            if !with_rating {
                if path_map[n_coo.x as usize][n_coo.y as usize].back.is_some() { continue }
            }

            // If we have an unexplored location with the required elevation step ...
            if try_sample_map(map, n_coo.into ()) == Some (altitude+1) {

                // ... we must explore it
                let next_job = Job { coo: n_coo };
                jobs.push(next_job);

                // and update our path map
                let path_item = PathItem { num_trails: 0, back: Some(job.coo) };
                path_map[n_coo.x as usize][n_coo.y as usize] = path_item;
            }
        }
    }

    // The score is equal to the number of trails starting from the trail's head
    path_map[head_coo.x as usize][head_coo.y as usize].num_trails
}

/// Element to process next while searching for trails
struct Job {
    /// Coordinate to process
    pub coo: Coo,
}

/// Location in a [PathMap], keeping track of the number of trails going through
#[derive(Default, Debug, Copy, Clone)]
struct PathItem {

    /// Number of trails
    pub num_trails: u32,

    /// Direction to the trail's head
    pub back: Option<Coo>,
}

/// Enables to trace paths along the map
type PathMap = Vec<Vec<PathItem>>;

/// Solve the puzzle. Parameter `with_rating` enables the rated score of part b.
fn solve (map: &[&str], with_rating: bool) -> Result<usize> {

    let height = map.len();
    let width = map[0].len();

    // Look for trail heads
    let mut sum_score = 0;
    for y in 0..map.len() {
        let row = map [y].as_bytes();
        for x in 0..row.len() {

            // For each trail head ...
            if row [x] as char == '0' {

                let mut path_map: Vec<Vec<PathItem>> = vec![vec![Default::default(); height]; width];

                // ... compute score and collect sum
                let score = score_head(map, &mut path_map,(x, y).into (), with_rating);
                sum_score += score;
            }
        }
    }

    Ok (sum_score as usize)
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

pub fn day_10 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(solve (&split (TEST), false).unwrap_or_default() == 36);
    debug_assert!(solve (&split (TEST), true).unwrap_or_default() == 81);

    let ra = solve(content, false)?;
    let rb = solve(content, true)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}