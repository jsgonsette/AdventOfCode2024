use std::cmp::Ordering;
use std::ops::{Add, Sub};
use anyhow::*;
use crate::Solution;
use crate::tools::IntReader;

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
    time_left: u32,
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
            time_left: self.time_left -1,
        }
    }
}

/// Load all the [Blueprint] from the puzzle file `content`
fn load_blueprints (content: &[&str]) -> Result<Vec<Blueprint>> {

    let mut reader = IntReader::new(false);

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

/// Move time ahead, step by step, until we have enough resources to build `buy_robot` according
/// to the `blueprint` and the current `process` state. The function then returns the resulting state.
/// If buying the robot is never possible given the time left, it returns `None`.
fn try_next (blueprint: &Blueprint, process: &Process, buy_robot: Kind) -> Option<Process> {

    // Cost of the robot to build
    let cost = match buy_robot {
        Kind::Ore => blueprint.robot_ore_cost,
        Kind::Clay => blueprint.robot_clay_cost,
        Kind::Obsidian => blueprint.robot_obsidian_cost,
        Kind::Geode => blueprint.robot_geode_cost,
    };

    // Move time forward until we can buy the robot
    let mut next_process = *process;
    while next_process.time_left > 0 {

        // Buy it as soon as possible
        if next_process.minerals >= cost {
            next_process = next_process.step(blueprint, Some (buy_robot));
            return Some (next_process)
        }
        // Otherwise, let produce more resources one step more
        else {
            next_process = next_process.step(blueprint, None);
        }
    }

    // We get here if we could not perform the requested action before the end
    None
}

/// Optimistic heuristic returning an upper bound of the maximum number of geodes we could
/// produce given the `blueprint` and current `process` state.
fn heuristic(blueprint: &Blueprint, process: &Process) -> u32 {

    let mut current = *process;
    while current.time_left > 0 {

        // Infinite ore, yeah !
        current.minerals.ore = blueprint.max_cost.ore;

        // Buy geode robot ASAP as they are the most important.
        if current.minerals >= blueprint.robot_geode_cost {
            current = current.step(blueprint, Some (Kind::Geode));
        }

        // Otherwise build Obsidian or Clay robots. Because we have infinite ore, this does
        // not prevent us to buy Geode robots later
        else if current.minerals >= blueprint.robot_obsidian_cost {
            current = current.step(blueprint, Some (Kind::Obsidian));
        }
        else {
            current = current.step(blueprint, Some (Kind::Clay));
        }
    }

    current.minerals.geode
}

/// Find out the *maximum number of geodes* its is possible to collect, given the `blueprint` and
/// `process_start` state.
///
/// * For any resource kind, having more robots than the maximum price we can pay during a turn
/// is unproductive. Indeed, we can buy only one robot per turn, so the surplus would be lost
/// whatever we do. Therefore, some actions are disabled when we have enough robots of the
/// corresponding category.
///
/// * This function work with a DFS queue. For each possible process state, it envisions
/// the different possible next actions. Those actions correspond to the four
/// possible robots to buy (see function [try_next])
///
/// * Actions are disabled when we don't have enough time for them to have an impact on the
/// final number of geodes.
///
/// * An optimistic heuristic gives an upper bound that enables to drop bad solutions early.
fn solve_max_geodes (blueprint: &Blueprint, process_start: Process) -> u32 {

    let mut max_geodes = 0;
    let mut dfs_queue = vec! [process_start];
    while let Some (process) = dfs_queue.pop() {

        // Minimum number of geodes we are sure to get without doing anything else.
        // Record the best solution.
        let min_geodes = process.minerals.geode + process.robots.geode * process.time_left;
        max_geodes = max_geodes.max(min_geodes);

        // Skip this process state if it is not optimistically possible to do better
        if heuristic(blueprint, &process) <= max_geodes { continue }

        // Try to build an Ore robot next. But only if it makes sense regarding the time left,
        // and only if we have not reached the limit where an additional robot does not help.
        if process.robots.ore < blueprint.max_cost.ore && process.time_left > 3 {
            if let Some (next_process) = try_next(blueprint, &process, Kind::Ore) {
                dfs_queue.push(next_process);
            }
        }

        // Same for the Clay robot,
        if process.robots.clay < blueprint.max_cost.clay && process.time_left > 5 {
            if let Some (next_process) = try_next(blueprint, &process, Kind::Clay) {
                dfs_queue.push(next_process);
            }
        }

        // the obsidian robot,
        if process.robots.obsidian < blueprint.max_cost.obsidian && process.time_left > 3 {
            if let Some (next_process) = try_next(blueprint, &process, Kind::Obsidian) {
                dfs_queue.push(next_process);
            }
        }

        // and the geode robot
        if process.time_left > 1 {
            if let Some (next_process) = try_next(blueprint, &process, Kind::Geode) {
                dfs_queue.push(next_process);
            }
        }
    }

    max_geodes
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
        time_left: 24,
    };

    let quality_level = blueprints.iter ().enumerate ().map (|(idx, blueprint)| {
        let quality = solve_max_geodes(blueprint, process);
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
        time_left: 32,
    };

    let value = blueprints.iter ().take (3).map ( |blueprint| {
        let quality = solve_max_geodes(blueprint, process);
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