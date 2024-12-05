# My solutions for the Advent of Code 2024 puzzles, in Rust with RustRover

## Usage

1. The application expects your input files to be in the [./input/2024]() directory. Each file must be named according to the day number: `01.txt`, .. `25.txt`.
2. There is only one binary to compile, that invokes all the daily puzzles in sequence (see [./src/main.rs]() )

## Performance

I focus on code *readability* and *usability* first, instead of seeking for performance at all cost. Here are the results I get on my computer (**AMD Ryzen 9 5900X**) to solve the two parts of the problems:

### 2024 
| Day | Time (µs) |   | Day | Time (µs) |
|-----|-----------|---|-----|-----------| 
| 01  | *178*     |   | 02  | *507*     |
| 03  | *80*      |   | 04  | *282*     |
| 05  | *2820*    |   | 06  | *?*       |
| 07  | *?*       |   | 08  | *?*       |
| 09  | *?*       |   | 10  | *?*       |
| 11  | *?*       |   | 12  | *?*       |
| 13  | *?*       |   | 14  | *?*       |
| 15  | *?*       |   | 16  | *?*       |
| 17  | *?*       |   | 18  | *?*       |