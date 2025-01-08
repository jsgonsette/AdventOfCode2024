mod day_02;
mod day_04;
mod day_05;
mod day_24;
mod day_25;
mod day_23;
mod day_22;
mod day_21;
mod day_20;
mod day_19;
mod day_18;
mod day_17;
mod day_16;
mod day_15;
mod day_01;
mod day_03;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

use crate::{FnDay, Year};

pub struct Y2022;

impl Year for Y2022 {

    fn get_year(&self) -> u32 { 2022 }

    fn get_day_fn(&self, day: u32) -> Option<FnDay> {
        match day {
            1 => Some (day_01::day_1),
            2 => Some (day_02::day_2),
            3 => Some (day_03::day_3),
            4 => Some (day_04::day_4),
            5 => Some (day_05::day_5),
            6 => Some (day_06::day_6),
            7 => Some (day_07::day_7),
            8 => Some (day_08::day_8),
            9 => Some (day_09::day_9),
            10 =>Some (day_10::day_10),
            15=> Some (day_15::day_15),
            16=> Some (day_16::day_16),
            17=> Some (day_17::day_17),
            18=> Some (day_18::day_18),
            19=> Some (day_19::day_19),
            20=> Some (day_20::day_20),
            21=> Some (day_21::day_21),
            22=> Some (day_22::day_22),
            23=> Some (day_23::day_23),
            24=> Some (day_24::day_24),
            25=> Some (day_25::day_25),
            _ => None,
        }
    }

    fn get_day_name(&self, day: u32) -> Option<&str> {
        match day {
            1 => Some ("Calorie Counting"),        13 => Some (" "),
            2 => Some ("Rock Paper Scissors"),     14 => Some (" "),
            3 => Some ("Rucksack Reorganization"), 15 => Some ("Beacon Exclusion Zone"),
            4 => Some ("Camp Cleanup"),            16 => Some ("Proboscidea Volcanium"),
            5 => Some ("Supply Stacks"),           17 => Some ("Pyroclastic Flow"),
            6 => Some ("Tuning Trouble"),          18 => Some ("Boiling Boulders"),
            7 => Some ("No Space Left On Device"), 19 => Some ("Not Enough Minerals"),
            8 => Some ("Treetop Tree House"),      20 => Some ("Grove Positioning System"),
            9 => Some ("Rope Bridge"),             21 => Some ("Monkey Math"),
            10 => Some ("Cathode-Ray Tube"),       22 => Some ("Monkey Map"),
            11 => Some (" "),                      23 => Some ("Unstable Diffusion"),
            12 => Some (" "),                      24 => Some ("Blizzard Basin"),
            25 => Some ("Full of Hot Air"),
            _ => None
        }
    }
}

