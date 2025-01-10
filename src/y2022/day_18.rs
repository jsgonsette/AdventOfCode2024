use std::collections::HashSet;
use anyhow::*;
use crate::{Solution};
use crate::tools::IntReader;

const TEST: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

/// An axis
#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
enum Axis { X, Y, Z }

/// Droplet 3D coordinate
type Droplet = (i8, i8, i8);

/// Droplet surface coordinate
type Surface = (Axis, i8, i8, i8);

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Load all the droplets from the puzzle file `content`
fn load_droplets (content: &[&str]) -> Result<Vec<Droplet>> {

    let mut reader = IntReader::new(false);

    content.iter().map (|&row| {
        let raw: [i8; 3] = reader
            .process_row_fix(row)
            .ok_or(anyhow!("Cannot parse row {}", row))?;

        Ok((raw [0], raw [1], raw [2]))
    }).collect()
}

/// Iterate on all the `droplets` and count the number of surfaces that occur only one time
fn count_free_surfaces (droplets: &[Droplet]) -> usize {

    let mut free_surfaces: HashSet<Surface> = HashSet::new();

    // For each droplet
    for droplet in droplets {

        // and for each possible surface
        let surfaces = [
            (Axis::X, droplet.0, droplet.1, droplet.2),
            (Axis::X, droplet.0 +1, droplet.1, droplet.2),
            (Axis::Y, droplet.0, droplet.1, droplet.2),
            (Axis::Y, droplet.0, droplet.1 + 1, droplet.2),
            (Axis::Z, droplet.0, droplet.1, droplet.2),
            (Axis::Z, droplet.0, droplet.1, droplet.2 + 1),
        ];

        // Add the surface if not in the set, otherwise remove it
        for surface in surfaces {
            if free_surfaces.contains(&surface) {
                free_surfaces.remove(&surface);
            }
            else {
                free_surfaces.insert(surface);
            }
        }
    }

    free_surfaces.len ()
}

/// Flood the coordinates of air droplets around the set of lava `droplets`, knowing its maximum
/// `extend` (highest lava coordinate for the 3 axis).
///
/// If, when extending an air droplet, we bump into a lava droplet, we increase the surface by +1
fn count_free_surface_with_flood(droplets: &HashSet<Droplet>, extend: (i8, i8, i8)) -> usize {

    let directions = [(0, 0, 1), (0, 1, 0), (1, 0, 0), (0, 0, -1), (0, -1, 0), (-1, 0, 0)];

    let mut free_surfaces = 0;
    let mut out_volume: HashSet<Droplet> = HashSet::new();
    let mut queue = Vec::<Droplet>::new();

    // Start we the coordinate of an air droplet and flood ...
    queue.push((-1, -1, -1));
    while let Some(air_drop) = queue.pop() {

        // ... in all 6 directions
        for dir in &directions {
            let neighbor = (air_drop.0 + dir.0, air_drop.1 + dir.1, air_drop.2 + dir.2);

            // Bump into lava
            if droplets.contains(&neighbor) {
                free_surfaces += 1;
                continue
            }

            // Do not go too far
            if neighbor.0 < -1 || neighbor.1 < -1 || neighbor.2 < -1 { continue }
            if neighbor.0 > extend.0+1 || neighbor.1 > extend.1+1 || neighbor.2 > extend.2+1 { continue }

            // Do not repeat ourselves
            if out_volume.contains(&neighbor) { continue }

            queue.push(neighbor);
            out_volume.insert(neighbor);
        }
    }

    free_surfaces
}

/// Solve first part of the puzzle
fn part_a (_content: &[&str]) -> Result<usize> {

    let droplets = load_droplets(&_content)?;
    let count = count_free_surfaces(&droplets);

    Ok(count)
}

/// Solve second part of the puzzle
fn part_b (_content: &[&str]) -> Result<usize> {

    let droplets = load_droplets(&_content)?;

    // Max x, y, z coordinates among all the droplets
    let extend = droplets.iter().fold ((0,0,0), |acc, droplet| {
        (acc.0.max(droplet.0), acc.1.max(droplet.1), acc.2.max(droplet.2) )
    });

    let droplets = HashSet::from_iter(droplets.into_iter());
    let free_surface = count_free_surface_with_flood(&droplets, extend);

    Ok(free_surface)
}

pub fn day_18 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 64);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 58);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}