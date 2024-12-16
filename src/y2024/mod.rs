mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

use crate::{FnDay, Year};

pub struct Y2024;

impl Year for Y2024 {

    fn get_year(&self) -> u32 { 2024 }

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
            10 => Some (day_10::day_10),
            11 => Some (day_11::day_11),
            12 => Some (day_12::day_12),
            13 => Some (day_13::day_13),
            14 => Some (day_14::day_14),
            15 => Some (day_15::day_15),
            16 => Some (day_16::day_16),
            17 => Some (day_17::day_17),
            _ => None,
        }
    }
}

