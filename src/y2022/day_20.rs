use anyhow::*;
use crate::{Solution};

const TEST: &str = "\
1
2
-3
3
-2
0
4";

/// Our GPS coordinate decoding system
struct GPS {

    /// Encrypted data
    encrypted: Vec<isize>,

    /// Position of the zero value in the encrypted vector
    zero_index: usize,
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

impl GPS {

    /// Instantiate our GPS from the puzzle file `content`
    fn new (content: &[&str]) -> Result<Self> {
        let mut zero_index = None;

        let values: Result<Vec<isize>> = content.iter().enumerate ()
            .map(
                |(idx, s)| {
                    if *s == "0" { zero_index = Some (idx) };
                    s.parse::<isize>().map_err(|_e| anyhow!("Not valid numbers: {}", s))
                }
            ).collect();

        let zero_index = zero_index.ok_or(anyhow!("No zero position"));

        Ok(Self {
            encrypted: values?,
            zero_index: zero_index?,
        })
    }

    /// Extract the coordinate after having mixed the encrypted data.
    /// The procedure uses a multiplicative `key` and number of passes `n_passes`.
    /// For part 1 of the problem, both values must be 1
    fn decrypt_with_key (&self, key: isize, n_passes: u32) -> isize {

        // Indexes referencing the original encrypted data
        // (We manipulate those indexes, not the data)
        let mut indexes: Vec<usize> = (0..self.encrypted.len()).collect();

        for _ in 0..n_passes {

            // Move each encrypted data
            for index in 0..self.encrypted.len() {

                // Find the position in the scrambled vector
                let current_pos = indexes.iter().position(|i| *i == index).unwrap();

                // The displacement corresponds to the original value
                let step = self.encrypted[index] * key;

                // Compute the new position. The '-1' is important because putting a data at
                // first or last position is actually the same for something circular
                let new_pos = (current_pos as isize + step)
                    .rem_euclid(self.encrypted.len() as isize - 1) as usize;

                // Move the index at the new position
                indexes.remove(current_pos);
                indexes.insert(new_pos, index);
            }
        }

        self.extract_coordinate (&indexes) * key
    }

    /// Get the zero, then the sum of the 1000th, 2000th and 3000th numbers after it.
    fn extract_coordinate (&self, indexes: &[usize]) -> isize {

        let zero_pos = indexes.iter ().position(|i| *i == self.zero_index).unwrap();
        let zero_1000 = (zero_pos + 1000) % self.encrypted.len();
        let zero_2000 = (zero_pos + 2000) % self.encrypted.len();
        let zero_3000 = (zero_pos + 3000) % self.encrypted.len();

        self.encrypted [indexes [zero_1000]] +
            self.encrypted [indexes [zero_2000]] +
            self.encrypted [indexes [zero_3000]]
    }
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<isize> {

    let gps = GPS::new(content)?;
    let decrypted = gps.decrypt_with_key(1, 1);

    Ok(decrypted)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<isize> {

    let gps = GPS::new(content)?;
    let decrypted = gps.decrypt_with_key(811589153, 10);

    Ok(decrypted)
}

pub fn day_20 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 3);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 1623178306);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra as usize), Solution::Unsigned(rb as usize)))
}