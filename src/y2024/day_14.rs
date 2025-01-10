use std::collections::HashSet;
use std::fmt::Display;
use anyhow::*;
use itertools::Itertools;
use crate::{Solution};
use crate::tools::{Coo, IntReader};

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

/// Model a robot position and velocity
#[derive(Debug, Copy, Clone)]
struct Robot {
    pos: Coo,
    velocity: Coo,
}

/// Models the bathroom and its swarm of robots
struct Bathroom {

    /// bathroom dimensions
    size: (usize, usize),

    /// the robots
    swarm: Vec<Robot>,
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

impl Display for Bathroom {

    /// Draw the bathroom and the location of the swarm
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let positions: HashSet<Coo> = HashSet::from_iter(
            self.swarm.iter().map(|robot | robot.pos)
        );

        for y in 0..self.size.1 {
            let row: String = (0..self.size.0).map(|x| {
                if positions.contains(&(x, y).into ()) { '#' } else { '.' }
            }).join("");

            f.write_str("\n")?;
            f.write_str(&row)?;
        }
        f.write_str("\n")
    }
}


impl Robot {

    /// Update the robot position for the given number of steps `num_steps`.
    /// Parameters `area_width` and `area_height` are used to teleport the robot at the
    /// opposite sides when it would otherwise leave the area.
    fn update (&mut self, area_width: usize, area_height: usize, num_steps: isize) {

        // Wrap in the interval [0, n[
        let unsigned_wrap = |x: isize, n: usize| -> isize {
            let xn = x % n as isize;
            if xn < 0 { xn + n as isize }
            else { xn }
        };

        self.pos.x = unsigned_wrap (self.pos.x + self.velocity.x * num_steps, area_width);
        self.pos.y = unsigned_wrap (self.pos.y + self.velocity.y * num_steps, area_height);
    }
}

impl Bathroom {

    /// New bathroom instance, based on the puzzle file `content`.
    /// Parameters `area_width` and `area_height` specify the dimensions of the area
    fn new (area_width: usize, area_height: usize, content: &[&str]) -> Result<Self> {
        Ok(Bathroom {
            size: (area_width, area_height),
            swarm: Self::collect_robots(content)?,
        })
    }

    /// Collect the positions and velocities of all the robots, based on the puzzle file `content`
    fn collect_robots (content: &[&str]) -> Result<Vec<Robot>> {

        let mut reader = IntReader::new(true);
        content.iter().map (|&row|{

            let vector: [isize; 4] = reader.process_row_fix(row)
                .ok_or(anyhow!("Could not parse row: {row}"))?;

            Ok(Robot {
                pos: (vector [0], vector [1]).into (),
                velocity: (vector [2], vector [3]).into (),
            })
        }).collect()
    }

    /// Update the position of the swarm, given a number of steps `num_steps`
    fn update (&mut self, num_steps: isize) {
        for robot in self.swarm.iter_mut() {
            robot.update(self.size.0, self.size.1, num_steps);
        }
    }

    /// Find the Christmas tree ! It's hard to know what it looks like, as there is no hint.
    /// However, a drawing can only be made if the robots are close to each others. So we compute
    /// a "per robot score" at each step, and stop when this score is high enough.
    ///
    /// * The "per-robot" score of the Christmas tree is around 24
    /// * Setting a trigger value as low as 4 enables to find it without any false positive
    ///
    /// **This means that this technique proves to be quite general and highly effective!**
    fn find_christmas_tree_accurate(&mut self, display_it: bool) -> usize{

        let mut steps = 0;
        loop {
            steps += 1;
            self.update(1);

            let score_per_robot = self.compute_density_factor () / self.swarm.len();
            if score_per_robot >= 10 {
                if display_it {
                    println!("After {} steps:", steps);
                    println!("Score {}:", score_per_robot);
                    println!("Safety factor {}", self.compute_safety_factor());
                    println!("{}", self);
                }
                break steps;
            }
        }
    }

    /// Second way to find the Christmas tree, using the safety score computed in part 1)
    /// as a clue. When the robots group together to form the tree, the safety factor drops
    /// drastically because they are not scattered around as before. As such,
    /// some quadrants become almost empty.
    fn find_christmas_tree_fast(&mut self, display_it: bool) -> usize{

        let base_safety_factor = self.compute_safety_factor();
        let threshold = (base_safety_factor as f32 * 0.45) as usize;
        let mut steps = 0;

        loop {
            steps += 1;
            self.update(1);
            let safety_factor = self.compute_safety_factor();

            if safety_factor < threshold {
                if display_it {
                    println!("After {} steps:", steps);
                    println!("Safety factor {}", self.compute_safety_factor());
                    println!("{}", self);
                }
                break steps;
            }
        }
    }

    /// Compute a score that relates how close are the robots from each others.
    /// The idea is that, whatever the drawing is (to form a Christmas tree), the robots
    /// must be close to each other to make something meaningful.
    fn compute_density_factor (&self) -> usize {

        // Create a map where each robot creates a +1 score on the 8 tiles around it
        let mut map_score = vec! [vec![0; self.size.1]; self.size.0];
        for robot in self.swarm.iter() {
            let (px, py) = (robot.pos.x, robot.pos.y);

            for x in px-1..=px+1 {
                for y in py-1..=py+1 {
                    if x == px && y == py { continue; }
                    if x < 0 || x >= self.size.0 as isize { continue; }
                    if y < 0 || y >= self.size.1 as isize { continue; }

                    map_score[x as usize][y as usize] += 1;
                }
            }
        }

        // Lonely robots will have a score of 0. A pair will have a score of +2. A group of
        // 3 robots will have a score of 3*4=12, etc.
        self.swarm.iter().map (|robot| {
            let score = map_score[robot.pos.x as usize][robot.pos.y as usize];
            score*score
        }).sum()
    }

    /// Compute the safety factor resulting from the current position of the robots
    fn compute_safety_factor (&self) -> usize {

        let mut scores = [0, 0, 0, 0];

        // Size of a quadrant (dimensions are odd)
        let quad_width  = (self.size.0 as isize +1) /2;
        let quad_height = (self.size.1 as isize +1) /2;

        for robot in self.swarm.iter() {

            // Skip robots that are in between the quadrants
            if (robot.pos.x+1) % quad_width == 0 { continue }
            if (robot.pos.y+1) % quad_height == 0 { continue }

            // Which quadrant ?
            let col = robot.pos.x / quad_width;
            let row = robot.pos.y / quad_height;
            let idx = row*2+col;

            // Increase the corresponding score
            scores [idx as usize] += 1;
        }

        scores [0] * scores[1] * scores[2] * scores[3]
    }
}

/// Solve first part of the puzzle
fn part_a (content: &[&str], area_width: usize, area_height: usize) -> Result<usize> {

    let mut bathroom = Bathroom::new(area_width, area_height, &content)?;

    bathroom.update(100);

    let safety_factor = bathroom.compute_safety_factor();
    Ok(safety_factor)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str], area_width: usize, area_height: usize) -> Result<usize> {

    let mut bathroom = Bathroom::new(area_width, area_height, &content)?;

    static DISPLAY_IT: bool = false;
    static METHOD_FAST_BUT_LESS_ACCURATE: bool = true;

    let num_steps = match METHOD_FAST_BUT_LESS_ACCURATE {
       true =>  bathroom.find_christmas_tree_fast(DISPLAY_IT),
       false => bathroom.find_christmas_tree_accurate(DISPLAY_IT),
    } ;

    Ok(num_steps)
}

pub fn day_14 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST), 11, 7).unwrap_or_default() == 12);

    let ra = part_a(content, 101, 103)?;
    let rb = part_b(content, 101, 103)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}