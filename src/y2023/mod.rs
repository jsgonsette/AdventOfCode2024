mod day_02;
mod day_03;
mod day_10;

use crate::{FnDay, Year};

pub struct Y2023;

impl Year for Y2023 {

    fn get_year(&self) -> u32 { 2023 }

    fn get_day_name(&self, day: u32) -> Option<&str> {
        match day {
            1 => Some (""),
            _ => None
        }
    }

    fn get_day_fn(&self, day: u32) -> Option<FnDay> {
        match day {
            10 => Some (day_10::day_10),
            _ => None,
        }
    }
}

