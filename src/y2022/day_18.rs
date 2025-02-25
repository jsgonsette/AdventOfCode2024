use std::collections::HashSet;
use anyhow::*;
use crate::{Solution};
use crate::tools::{ArraySet, IntReader};

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
        let mut it = reader.iter_row::<i8>(row);
        let a = it.next().ok_or_else(|| anyhow!("No enough value in row {}", row))?;
        let b = it.next().ok_or_else(|| anyhow!("No enough value in row {}", row))?;
        let c = it.next().ok_or_else(|| anyhow!("No enough value in row {}", row))?;
        Ok ((a, b, c))
    }).collect()
}

/// Iterate on all the `droplets` and count the number of surfaces that occur only one time.
/// `extend` must provide the highest lava coordinate for the 3 axis.
fn count_free_surfaces (droplets: &[Droplet], extend: (i8, i8, i8)) -> usize {

    // The set of all possible surface coordinates is small. Use an array set to accelerate things.
    let mut surface_set = ArraySet::new(
        [0; 4],
        [2, extend.0 as isize +1, extend.1 as isize +1, extend.2 as isize +1]
    );

    let mut add_or_remove_surface = | surface: &Surface | {
        let item = [surface.0 as isize, surface.1 as isize, surface.2 as isize, surface.3 as isize];
        surface_set.toggle (&item);
    };

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
            add_or_remove_surface (&surface);
        }
    }

    surface_set.count ()
}

/// Flood the coordinates of air droplets around the set of lava `droplets`, knowing its maximum
/// `extend` (highest lava coordinate for the 3 axis).
///
/// If, when extending an air droplet, we bump into a lava droplet, we increase the surface by +1
fn count_free_surface_with_flood(droplets: &HashSet<Droplet>, extend: (i8, i8, i8)) -> usize {

    // 6 moving directions around a cube
    let directions = [(0, 0, 1), (0, 1, 0), (1, 0, 0), (0, 0, -1), (0, -1, 0), (-1, 0, 0)];

        // The set of all possible 3D coordinates is small. Use an array set to accelerate things.
    let mut out_volume = ArraySet::new(
        [-1; 3],
        [extend.0 as isize +1, extend.1 as isize +1, extend.2 as isize +1]
    );

    let mut free_surfaces = 0;
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
            let neighbor_item = [neighbor.0 as isize, neighbor.1 as isize, neighbor.2 as isize];
            if out_volume.test(&neighbor_item) { continue }

            queue.push(neighbor);
            out_volume.set(&neighbor_item);
        }
    }

    free_surfaces
}

/// Return the max x, y, z coordinates among all the droplets
fn get_lava_extend (droplets: &[Droplet]) -> (i8, i8, i8) {

    droplets.iter().fold ((0,0,0), |acc, droplet| {
        (acc.0.max(droplet.0), acc.1.max(droplet.1), acc.2.max(droplet.2) )
    })
}

/// Solve first part of the puzzle
fn part_a (_content: &[&str]) -> Result<usize> {

    let droplets = load_droplets(&_content)?;
    let extend = get_lava_extend(&droplets);

    let count = count_free_surfaces(&droplets, extend);

    Ok(count)
}

/// Solve second part of the puzzle
fn part_b (_content: &[&str]) -> Result<usize> {

    let droplets = load_droplets(&_content)?;
    let extend = get_lava_extend(&droplets);

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