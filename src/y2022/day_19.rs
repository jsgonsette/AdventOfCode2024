use std::cmp::Ordering;
use std::ops::{Add, Sub};
use anyhow::*;
use crate::Solution;
use crate::tools::RowReader;

const TEST: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

/// Available resources, or costs, for each kind of minerals/robots
#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

/// Cost of each robot in terms of [Resources]
#[derive(Copy, Clone, Debug, Default)]
struct Blueprint {
    robot_ore_cost: Resources,
    robot_clay_cost: Resources,
    robot_obsidian_cost: Resources,
    robot_geode_cost: Resources,
    max_cost: Resources,
}

/// Current process status, in terms of owned minerals and robots
#[derive(Copy, Clone, Debug)]
struct Process {
    minerals: Resources,
    robots: Resources,
}

/// The four kinds of minerals and associated robots
#[derive(Copy, Clone, Debug)]
enum Kind {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Blueprint {

    /// New blueprint instance
    fn new (bot_ore: Resources, bot_clay: Resources, bot_obsidian: Resources, bot_geode: Resources) -> Blueprint {
        let mut blueprint = Blueprint {
            robot_ore_cost: bot_ore,
            robot_clay_cost: bot_clay,
            robot_obsidian_cost: bot_obsidian,
            robot_geode_cost: bot_geode,
            max_cost: Default::default(),
        };

        // Compute the maximum cost of each resource for all the robots.
        blueprint.max_cost = blueprint.max_resources();
        blueprint
    }

    /// For each resource type, get the maximum cost all robot categories combined
    fn max_resources (&self) -> Resources {
        self.robot_ore_cost.max(
            &self.robot_clay_cost.max (
                &self.robot_obsidian_cost.max(&self.robot_geode_cost)
            )
        )
    }
}

impl Resources {
    fn from_ore (amount: u32) -> Resources {
        Resources { ore: amount, ..Default::default() }
    }

    fn from_clay (amount: u32) -> Resources {
        Resources { clay: amount, ..Default::default() }
    }

    fn from_obsidian (amount: u32) -> Resources {
        Resources { obsidian: amount, ..Default::default() }
    }

    fn from_geode (amount: u32) -> Resources {
        Resources { geode: amount, ..Default::default() }
    }

    /// Return the maximum of each resource's category
    fn max (&self, other: &Resources) -> Resources {
        Resources {
            ore: self.ore.max(other.ore),
            clay: self.clay.max(other.clay),
            obsidian: self.obsidian.max(other.obsidian),
            geode: self.geode.max(other.geode),
        }
    }
}

/// To add resources together
impl Add for Resources {
    type Output = Resources;
    fn add(self, rhs: Resources) -> Resources {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

/// To subtract resources from each others
impl Sub for Resources {
    type Output = Resources;
    fn sub(self, rhs: Resources) -> Resources {
        Resources {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

/// Partial ordering required to check if we have enough resources to build such robot
impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        if *self== *other {
            Some(Ordering::Equal)
        }
        else if self.geode >= other.geode &&
            self.clay >= other.clay &&
            self.ore >= other.ore &&
            self.obsidian >= other.obsidian {
            Some(Ordering::Greater)
        }
        else if self.geode <= other.geode &&
            self.clay <= other.clay &&
            self.ore <= other.ore &&
            self.obsidian <= other.obsidian {
            Some(Ordering::Less)
        }
        else { None }
    }

    fn ge(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => true,
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}

impl Process {

    /// Make the resources (minerals and robots) evolve by one time step.
    /// Parameter `build` indicates which robot we build this turn.
    fn step (&self, blueprint: &Blueprint, build: Option<Kind>) -> Process {

        // Robot we build this turn, and its cost
        let (cost, new_robot) = match build {
            None => (Resources::default(), Resources::default()),
            Some(Kind::Ore)      => (blueprint.robot_ore_cost, Resources::from_ore(1)),
            Some(Kind::Clay)     => (blueprint.robot_clay_cost, Resources::from_clay(1)),
            Some(Kind::Obsidian) => (blueprint.robot_obsidian_cost, Resources::from_obsidian(1)),
            Some(Kind::Geode)    => (blueprint.robot_geode_cost, Resources::from_geode(1)),
        };

        Process {
            minerals: self.minerals + self.robots - cost,
            robots: self.robots + new_robot,
        }
    }
}

/// Load all the [Blueprint] from the puzzle file `content`
fn load_blueprints (content: &[&str]) -> Result<Vec<Blueprint>> {

    let mut reader = RowReader::new(false);

    content.iter ().map (|&row| {
        let raw_blueprint: [u32; 7] = reader
            .process_row_fix(row)
            .ok_or(anyhow!("Cannot parse blueprint row: {}", row))?;

        let bot_ore = Resources::from_ore (raw_blueprint[1]);
        let bot_clay = Resources::from_ore (raw_blueprint[2]);
        let bot_obsidian = Resources::from_ore (raw_blueprint[3]) + Resources::from_clay (raw_blueprint[4]);
        let bot_geode = Resources::from_ore (raw_blueprint[5]) + Resources::from_obsidian (raw_blueprint[6]);
        Ok (Blueprint::new(
            bot_ore, bot_clay, bot_obsidian, bot_geode
        ))
    }).collect ()
}

/// Heuristic that checks if it would be possible to build an additional *Geode* robot.
fn heuristic (_blueprint: &Blueprint, mut _process: Process, mut _time: u32) -> bool {
   true
}

/// Move time ahead, step by step, until we have enough resources to build `buy_robot` according
/// to the `blueprint`, the current `process` state and `time` step.
/// The function then returns the maximum number of geodes that we can get after this action.
fn do_next (blueprint: &Blueprint, mut process: Process, mut time: u32, buy_robot: Kind) -> u32 {

    // Cost of the robot to build
    let cost = match buy_robot {
        Kind::Ore => blueprint.robot_ore_cost,
        Kind::Clay => blueprint.robot_clay_cost,
        Kind::Obsidian => blueprint.robot_obsidian_cost,
        Kind::Geode => blueprint.robot_geode_cost,
    };

    // Move time forward until we can buy the robot
    while time > 0 {
        time -= 1;

        // Buy it as soon as possible and recurse to find what to do next
        if process.minerals >= cost {
            process = process.step(blueprint, Some (buy_robot));
            return solve_max_geodes(blueprint, process, time);
        }
        // Otherwise, let produce more resources one step more
        else {
            process = process.step(blueprint, None);
        }
    }

    // We get here if we could not perform the requested action before the end
    process.minerals.geode
}

/// Find out the *maximum number of geodes* its is possible to collect, given the `blueprint`,
/// the current `process` state and the number of `time` steps left.
///
/// * For any resource kind, having more robots than the maximum price we can pay during a turn
/// is unproductive. Indeed, we can buy only one robot per turn, so the surplus would be lost
/// whatever we do. Therefore, some actions are disabled when we have enough robots of the
/// corresponding category.
///
/// * This function is recursive. However, we try to move forward in time as much as possible
/// without using recursion (see function [do_next])
///
/// * Actions are disabled when we don't have enough time for them to have an impact on the
/// final number of geodes.
fn solve_max_geodes (blueprint: &Blueprint, process: Process, time: u32) -> u32 {

    let mut options = [0u32; 4]; // Different results depending on the different options we have

    // Minimum number of geodes we are sure to get without doing anything
    let min_geodes = process.minerals.geode + process.robots.geode * time;

    // Use an upper bound heuristic to check we can do better
    let maybe_improvable = heuristic (blueprint, process, time);
    if maybe_improvable {
        // Try to build an Ore robot next. But only if it makes sense regarding the time left,
        // and only if we have not reached the limit where an additional robot does not help.
        if process.robots.ore < blueprint.max_cost.ore && time > 3 {
            options[0] = do_next(blueprint, process, time, Kind::Ore);
        }

        // Same for the Clay robot,
        if process.robots.clay < blueprint.max_cost.clay && time > 5 {
            options[1] = do_next(blueprint, process, time, Kind::Clay);
        }

        // the obsidian robot,
        if process.robots.obsidian < blueprint.max_cost.obsidian && time > 3 {
            options[2] = do_next(blueprint, process, time, Kind::Obsidian);
        }

        // and the geode robot
        if time > 1 {
            options[3] = do_next(blueprint, process, time, Kind::Geode);
        }
    }

    // Return the maximum of all those strategies
    min_geodes.max ( options.into_iter().max().unwrap() )
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let blueprints = load_blueprints(content)?;
    let process = Process {
        minerals: Default::default(),
        robots: Resources::from_ore(1),
    };

    let quality_level = blueprints.iter ().enumerate ().map (|(idx, blueprint)| {
        let quality = solve_max_geodes(blueprint, process, 24);
        (idx+1) * quality as usize
    }).sum();

    Ok(quality_level)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let blueprints = load_blueprints(content)?;
    let process = Process {
        minerals: Default::default(),
        robots: Resources::from_ore(1),
    };

    let value = blueprints.iter ().take (3).map ( |blueprint| {
        let quality = solve_max_geodes(blueprint, process, 32);
        quality as usize
    }).product();

    Ok(value)
}

pub fn day_19 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 33);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 56*62);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}