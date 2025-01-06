use anyhow::*;
use crate::{Solution};

const TEST: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
const TEST_2: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";


/// Find a marker of `marker_length` consecutive different characters in `content`.
/// The function returns the position of the last processed character when this happens.
fn find_marker (content: &str, marker_length: u32) -> Result<usize> {

    // Index where each of the 26 possible characters appeared last. Init with a distant value.
    let init = - (marker_length as isize);
    let mut last_positions = [init; 26];

    // Bitset indicating the duplicates in the last processed characters
    // Example, upon receiving 'b' in the sequence "a b c d e d f b",
    // this variable must be '0b01010000'
    let mut duplicates = 0u32;
    let duplicate_mask = 2u32.pow(marker_length) -1;

    // Process the sequence of characters
    for (idx, b) in content.as_bytes().iter().enumerate() {

        // Shift the duplicates away
        duplicates = (duplicates << 1) & duplicate_mask;

        // Compute the distance with the last seen position.
        // If less than the marker length, then we have a duplicate
        let b_idx = (*b - b'a') as usize;
        let distance = idx as isize - last_positions [b_idx];
        if distance < marker_length as isize {
            duplicates |= 1 << distance;
        }

        // Record the last position for this character
        last_positions[b_idx] = idx as isize;

        // Ok if no duplicates in the window and if we have processed enough characters
        if duplicates == 0 && idx >= marker_length as usize { return Ok (idx +1) }
    }

    Err(anyhow!("Marker not found"))
}

pub fn day_6 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(find_marker (TEST, 4).unwrap_or_default() == 11);
    debug_assert!(find_marker (TEST_2, 14).unwrap_or_default() == 19);

    let ra = find_marker(content [0], 4)?;
    let rb = find_marker(content [0], 14)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}