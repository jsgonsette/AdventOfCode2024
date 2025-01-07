use anyhow::*;
use crate::Solution;

const TEST: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

/// Models the different entries of the puzzle file content
enum Entry {
    DirUp,
    DirDown,
    Dir,
    File(u32),
    Ls,
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Transform a puzzle text entry, as given by `row`, into an [Entry]
fn parse_row(row: &str) -> Result<Entry> {

    let mut command = false;
    let mut dir = false;
    let mut cd = false;

    for (idx, token) in row.split_whitespace().enumerate() {

        match (idx, token, command) {
            (0, s, _) if s == "$"   => command = true,
            (0, s, _) if s == "dir" => dir = true,
            (0, s, _) if s.as_bytes()[0].is_ascii_digit() => {
                let size = s.parse::<u32>()?;
                return Ok(Entry::File(size))
            },

            (1, s, true) if s == "ls" => return Ok(Entry::Ls),
            (1, s, true) if s == "cd" => cd = true,
            (1, _, false) if dir      => return Ok(Entry::Dir),

            (2, s, true) if cd && s == ".." => return Ok(Entry::DirUp),
            (2, _, true) if cd              => return Ok(Entry::DirDown),

            _ => return Err(anyhow!("Unexpected token {} in {}", token, row))
        }
    }

    Ok(Entry::DirUp)
}

/// Returns an iterator emitting the directory sizes collected in the puzzle file `content`
fn dir_size_it<'a> (content: &'a [&'a str]) -> impl Iterator<Item = Result<u32>> + 'a {

    // To accumulate the size of the upper directory and all its children
    let mut stack = Vec::<u32>::with_capacity(10);

    // Custom iterator
    let mut content_iter = content.iter();
    std::iter::from_fn(move || {

        // Iterate on the entries defined in the puzzle content
        while let Some (&row) = content_iter.next() {
            let entry = match parse_row(row) {
                Result::Ok(entry) => entry,
                Err(e) => return Some (Err(e))
            };

            // Emit a size for each DirUp command (because it means that we have seen all its children)
            // Otherwise, just update the stack and continue
            match entry {
                Entry::DirUp      => {
                    let dir_size = stack.drain(stack.len ()-1 ..).next().unwrap();
                    *stack.last_mut().unwrap() += dir_size;
                    return Some(Ok(dir_size))
                }
                Entry::DirDown    => { stack.push(0); }
                Entry::Dir        => {}
                Entry::File(size) => { *stack.last_mut().unwrap() += size; },
                Entry::Ls         => {}
            }
        }

        // When there is no more entry left, flush the stack until
        // we emit the size of the root (top most) directory
        if let Some (size) = stack.pop() {
            if let Some (last) = stack.last_mut() {
                *last += size;
            }
            return Some(Ok(size))
        }

        // And then we are done
        None
    })
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let mut sum = 0;
    for size in dir_size_it(content) {
        let size = size?;
        if size <= 100000 { sum += size }
    }

    Ok(sum as usize)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    // Collect all the dir size (root is last) and deduce the min space to free
    let all_dirs: Vec<u32> = dir_size_it(content).collect::<Result<Vec<u32>>>()?;
    let root_size = all_dirs.last().unwrap();
    let to_free = 30_000_000 - (70_000_000 - root_size);

    // Filter out all the directories that have the minimum required size and keep the smallest one
    let match_size = all_dirs
        .iter()
        .filter(|&&size| size >= to_free)
        .min ()
        .ok_or(anyhow!("No suitable directory found"))?;

    Ok(*match_size as usize)
}

pub fn day_7 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 95437);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 24933642);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}