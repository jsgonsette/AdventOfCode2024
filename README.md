# My solutions for the Advent of Code 2024 puzzles, in Rust

![AoC in Rust](./AoCRust.jpg)

## Usage

1. The application expects your input files to be in the [./input/2024](./input/2024) directory. Each file must be named according to the day number: `01.txt`, .. `25.txt`.
2. There is only one binary to compile, that invokes all the daily puzzles in sequence (see [./src/main.rs](./src/main.rs) )

## Performance

I focus on code *readability* and *usability* first, instead of seeking for performance at all cost. As such, this code is well documented and should be very understandable.
Here are the results I get to solve the two parts of the problems:

> All benchmarks are run on a **AMD Ryzen 9 5900X**, *single thread*

### [2024](https://adventofcode.com/2024)

| Day | Puzzle                                                       | Code                               | Perf. (ms) |
|-----|--------------------------------------------------------------|------------------------------------|------------|
| 01  | [Historian Hysteria](https://adventofcode.com/2024/day/1)    | [day_01.rs](./src/y2024/day_01.rs) | 0.154      |
| 02  | [Red-Nosed Reports](https://adventofcode.com/2024/day/2)     | [day_02.rs](./src/y2024/day_02.rs) | 0.272      |
| 03  | [Mull It Over](https://adventofcode.com/2024/day/3)          | [day_03.rs](./src/y2024/day_03.rs) | 0.080      |
| 04  | [Ceres Search Hysteria](https://adventofcode.com/2024/day/4) | [day_04.rs](./src/y2024/day_04.rs) | 0.282      |
| 05  | [Print Queue](https://adventofcode.com/2024/day/5)           | [day_05.rs](./src/y2024/day_05.rs) | 2.820      |
| 06  | [Guard Gallivant](https://adventofcode.com/2024/day/6)       | [day_06.rs](./src/y2024/day_06.rs) | 160.1      |
| 07  | [Bridge Repair](https://adventofcode.com/2024/day/7)         | [day_07.rs](./src/y2024/day_07.rs) | 19.71      |
| 08  | [Resonant Collinearity](https://adventofcode.com/2024/day/8) | [day_08.rs](./src/y2024/day_08.rs) | 0.112      |
| 09  | [Disk Fragmenter](https://adventofcode.com/2024/day/9)       | [day_09.rs](./src/y2024/day_09.rs) | 6.850      |
| 10  | [Hoof It](https://adventofcode.com/2024/day/10)              | [day_10.rs](./src/y2024/day_10.rs) | 1.101      |
| 11  | [Plutonian Pebbles](https://adventofcode.com/2024/day/11)    | [day_11.rs](./src/y2024/day_11.rs) | 14.79      |
| 12  | [Garden Groups](https://adventofcode.com/2024/day/12)        | [day_12.rs](./src/y2024/day_12.rs) | 2.150      |
    