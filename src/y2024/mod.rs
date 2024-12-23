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
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;

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
            18 => Some (day_18::day_18),
            19 => Some (day_19::day_19),
            20 => Some (day_20::day_20),
            21 => Some (day_21::day_21),
            22 => Some (day_22::day_22),
            23 => Some (day_23::day_23),
            24 => Some (day_24::day_24),
            _ => None,
        }
    }
}

