mod day_01;
mod day_02;
mod day_03;

use crate::{FnDay, Year};

pub struct Y2024;

impl Year for Y2024 {

    fn get_year(&self) -> u32 { 2024 }

    fn get_day_fn(&self, day: u32) -> Option<FnDay> {
        match day {
            1 => Some (day_01::day_1),
            2 => Some (day_02::day_2),
            _ => None,
        }
    }
}
