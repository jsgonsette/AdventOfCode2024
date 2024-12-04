mod day_02;
mod day_04;
mod day_05;

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
            _ => None,
        }
    }
}

