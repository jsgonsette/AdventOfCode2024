mod coordinates;
mod threads;
mod topo_sort;
mod cycle_detector;
mod all_pair_distances;
mod int_intervals;
mod grid_cell;
mod bit_set;
mod array_set;

use num::Integer;

pub use coordinates::{Direction, Coo, find_coo_extents};
pub use topo_sort::{TopoSortElement, topo_sort};
pub use all_pair_distances::*;
pub use int_intervals::{IntInterval, IntIntervals};
pub use grid_cell::{Cell, GridCell};
pub use array_set::ArraySet;
pub use bit_set::BitSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Sign { Positive, Negative }

/// Reads rows made of numbers
pub struct IntReader {
    built_number: Option<usize>,
    sign: Sign,
    allow_negative: bool,
}

impl IntReader {

    pub fn new(allow_negative: bool) -> IntReader {
        IntReader { built_number: None, sign: Sign::Positive, allow_negative }
    }

    /// Iterate on all the [Integer] numbers contained in a row, ignoring non-digit characters.
    pub fn iter_row<'a, T> (&'a mut self, row:&'a str) -> impl Iterator<Item=T> + 'a
    where T: Integer + TryFrom<isize> + 'a
    {
        let row_it = row.as_bytes().iter().chain(std::iter::once(&0));
        row_it.flat_map(|&b| {
            self.process_byte(b).map(|value| {
                let value = if self.allow_negative { value } else { value.abs () };
                T::try_from(value).ok().expect("Value to big to be converted")
            })
        })
    }

    /// Return a vector containing all the [Integer] numbers detected in the provided `row`.
    /// All the non-digit characters are ignored.
    pub fn process_row<T> (&mut self, row: &str) -> Vec<T>
    where T: Integer + TryFrom<isize> {
        self.iter_row(row).collect()
    }

    /// Return a fixed-size vector containing all the [Integer] numbers detected in the provided `row`.
    /// All the non-digit characters are ignored.
    /// ## Panic
    /// **The function fails if the exact number of numbers is not found.**
    pub fn process_row_fix<T, const N: usize> (&mut self, row: &str) -> Option<[T; N]>
    where T: Integer + TryFrom<isize> + Copy {
        let mut v = [T::zero(); N];
        let mut idx = 0;

        let row_it = row.as_bytes().iter().chain(std::iter::once(&0));
        for &b in row_it {

            // Process the next byte and eventually collect a number
            match self.process_byte(b) {
                Some (number) => {
                    if idx < N {
                        let number = if self.allow_negative { number } else { number.abs () };
                        let converted_number = T::try_from(number).ok().expect("Value to big to be converted");
                        v[idx] = converted_number;
                        idx += 1;
                    }
                    else { return None; }
                },
                None => {},
            }
        }

        match idx {
            _ if idx == N => Some (v),
            _ => None
        }
    }

    /// Process the next ASCII character `byte` and optionally yield a number.
    fn process_byte(&mut self, byte: u8) -> Option<isize> {
        match byte {
            b'0' ..= b'9' => {
                let current = self.built_number.unwrap_or_default();
                self.built_number = Some(current*10 + (byte - b'0') as usize);
                None
            },
            _ => {
                let out = self.built_number.map (|number|
                    match self.sign {
                        Sign::Positive => number as isize,
                        Sign::Negative => - (number as isize),
                    }
                );
                self.built_number = None;

                self.sign = match byte {
                    b'-' => Sign::Negative,
                    _    => Sign::Positive,
                };

                out
            },
        }
    }

}
