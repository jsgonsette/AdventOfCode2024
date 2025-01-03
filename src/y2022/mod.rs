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

use crate::{FnDay, Year};

pub struct Y2022;

impl Year for Y2022 {

    fn get_year(&self) -> u32 { 2022 }

    fn get_day_fn(&self, day: u32) -> Option<FnDay> {
        match day {
            1 => None,
            2 => Some (day_02::day_2),
            4 => Some (day_04::day_4),
            5 => Some (day_05::day_5),
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
}

