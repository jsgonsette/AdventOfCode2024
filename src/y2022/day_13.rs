use std::cmp::Ordering;
use anyhow::*;
use itertools::Itertools;
use crate::{Solution};

const TEST: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

/// Models the semantic elements of a sequence
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SequenceItem {
    Open,
    Close,
    Number (u8),
}

/// Special mode used to promote a single number as a list
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Nesting {
    None,

    /// Must emit a number, then some amount of closing braces
    Number(u8, u8),

    /// Must emit some amount of closing braces
    Close (u8)
}

/// Raw sequence reader emitting [SequenceItem]
struct SequenceReader<'a> {
    raw: &'a [u8],
    index: usize,
    nesting: Nesting,
}

fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}


impl<'a> SequenceReader<'a> {

    /// New sequence reader based on the provided `raw` sequence
    fn new(raw: &str) -> SequenceReader {
        SequenceReader {
            raw: raw.as_bytes(),
            index: 0,
            nesting: Nesting::None,
        }
    }

    /// Return the next item of the sequence, if any
    fn next_item(&mut self) -> Option<SequenceItem> {

        match self.nesting {

            // Special mode: we emit the number registered in `self.nesting`
            Nesting::Number(n, nesting) => {
                self.nesting = Nesting::Close (nesting);
                Some(SequenceItem::Number(n))
            },

            // Special mode: we emit the closing brace
            Nesting::Close (nesting) => {
                self.nesting = if nesting > 1 { Nesting::Close (nesting-1) } else { Nesting::None };
                Some(SequenceItem::Close)
            }

            // Normal decoding mode
            Nesting::None => {
                while let Some(b) = self.raw.get(self.index) {
                    self.index += 1;

                    match b {
                        b'0'..= b'9' => {
                            let mut number = b - b'0';
                            let peek_next = *self.raw.get(self.index)?;
                            if peek_next >= b'0' && peek_next <= b'9' {
                                self.index += 1;
                                number = number *10 + (peek_next - b'0');
                            }
                            return Some (SequenceItem::Number(number))
                        },
                        b'['         => return Some (SequenceItem::Open),
                        b']'         => return Some (SequenceItem::Close),
                        b','         => continue,
                        _            => unreachable!(),
                    }
                }
                None
            }
        }
    }

    /// Activate the nesting mechanism where `number` is emitted as a list at next iteration.
    fn nest_number_in_list(&mut self, number: u8) {

        self.nesting = match self.nesting {
            Nesting::Close(nesting) => {
                Nesting::Number(number, nesting +1)
            },
            Nesting::None => {
                Nesting::Number(number, 1)
            }
            _ => unreachable!()
        };
    }
}

/// Processes two sequences and provides the comparison result
fn compare (left: &mut SequenceReader, right: &mut SequenceReader) -> Result<bool> {

    while let (Some (a), Some (b)) = (left.next_item(), right.next_item()) {
        match (a, b) {

            // Same number or opening/closing item, continue
            (_, _) if a == b => continue,

            // One list is terminated before the other one
            (SequenceItem::Close, _) => return Ok(true),
            (_, SequenceItem::Close) => return Ok(false),

            // Two different numbers
            (SequenceItem::Number(na), SequenceItem::Number(nb)) => {
                return Ok(na < nb)
            },

            // One is a number, the other one a list. Use the promotion mechanism to yield
            // the number a second time
            (SequenceItem::Number(na), SequenceItem::Open) => {
                left.nest_number_in_list(na);
            },

            (SequenceItem::Open, SequenceItem::Number(nb)) => {
                right.nest_number_in_list(nb);
            }

            _ => unreachable!(),
        }
    }

    bail!("Invalid sequences");
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    let mut sum = 0;
    for (idx, packet_pair) in content.chunks(3).enumerate() {
        let mut first = SequenceReader::new (packet_pair[0]);
        let mut second = SequenceReader::new (packet_pair[1]);

        let comparison = compare(&mut first, &mut second)?;
        if comparison { sum += idx+1; }
    };

    Ok(sum)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let mut raw_sequences: Vec<&str> =
        content.iter().filter( |&&s| !s.is_empty()).copied ().collect();

    raw_sequences.push("[[2]]");
    raw_sequences.push("[[6]]");

    let fn_compare = | s1: &&str, s2: &&str | -> Ordering {
        let mut reader_1 = SequenceReader::new(s1);
        let mut reader_2 = SequenceReader::new(s2);
        match compare (&mut reader_1, &mut reader_2).unwrap() {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    };

    raw_sequences.sort_unstable_by(fn_compare);

    let key_2 = 1 + raw_sequences.iter().find_position(|&&s| s == "[[2]]").unwrap().0;
    let key_6 = 1 + raw_sequences.iter().find_position(|&&s| s == "[[6]]").unwrap().0;

    Ok(key_2 * key_6)
}

pub fn day_13 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 13);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 140);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}