use anyhow::*;
use itertools::Itertools;
use crate::{Solution};
use crate::tools::RowReader;

const TEST_1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

const TEST_2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

type Register = usize;

/// A fancy name for a 3-bit value
type Tribble = u8;

/// The different types of combo operands
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComboOperand {
    Literal(Tribble),
    RegA,
    RegB,
    RegC,
    Invalid,
}

/// The different types of instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(Tribble),
    Bst(ComboOperand),
    Jnz(Tribble),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}


/// Convert a *tribble* into an operand
impl From<u8> for ComboOperand {
    fn from (val: u8) -> Self {

        match val {
            0..=3 => ComboOperand::Literal(val),
            4 => ComboOperand::RegA,
            5 => ComboOperand::RegB,
            6 => ComboOperand::RegC,
            7 => ComboOperand::Invalid,
            _ => unreachable!(),
        }
    }
}

impl Instruction {

    /// Generate an instruction from a pair of values: (instruction code, operande code)
    fn from_pair (ins_code: u8, op_code: u8) -> Self {

        match ins_code {
            0 => Instruction::Adv(ComboOperand::from(op_code)),
            1 => Instruction::Bxl(op_code),
            2 => Instruction::Bst(ComboOperand::from(op_code)),
            3 => Instruction::Jnz(op_code),
            4 => Instruction::Bxc,
            5 => Instruction::Out(ComboOperand::from(op_code)),
            6 => Instruction::Bdv(ComboOperand::from(op_code)),
            7 => Instruction::Cdv(ComboOperand::from(op_code)),
            _ => unreachable!(),
        }
    }
}

/// Models our computer
#[derive(Debug, Clone)]
struct Computer {

    /// Register A
    a: Register,

    /// Register B
    b: Register,

    /// Register C
    c: Register,

    /// Stack Pointer
    sp: Register,

    /// The program
    program: Vec<Tribble>,
}

impl Computer {

    /// New computer with program and registers set from the puzzle file content
    fn new (content: &[&str]) -> Result<Self> {

        let mut reader = RowReader::new(false);
        let reg_a: [usize;1] = reader.process_row_fix(content [0]).ok_or(anyhow!("Reg A not found"))?;
        let reg_b: [usize;1] = reader.process_row_fix(content [1]).ok_or(anyhow!("Reg B not found"))?;
        let reg_c: [usize;1] = reader.process_row_fix(content [2]).ok_or(anyhow!("Reg C not found"))?;

        let program: Vec<u8> = reader.process_row(content [4]);

        Ok(Computer {
            a: reg_a [0],
            b: reg_b [0],
            c: reg_c [0],
            sp: 0,
            program,
        })
    }

    /// Executes the internal program and delivers the output vector
    fn execute (&mut self) -> Result<Vec<Tribble>> {

        let mut output: Vec<Tribble> = vec![];

        loop {
            // Get the next instruction code and operand code.
            // Stop when the stack pointer is out of range
            let Some (&ins) = self.program.get(self.sp) else { break };
            let &op = self.program.get(self.sp + 1).ok_or(anyhow!("SP out of program range"))?;
            self.sp += 2;

            // Make a valid instruction with them
            let ins = Instruction::from_pair(ins, op);

            // And execute it
            self.execute_instruction(ins, &mut output);
        }

        Ok(output)
    }

    /// Print a human-readable version of the program
    fn _decompile (&self) -> String {
        let mut program = "Program:".to_string();

        for idx in 0..self.program.len() {
            if idx % 2 == 0 {
                let ins = self.program[idx];
                let op = self.program[idx+1];
                let ins = Instruction::from_pair(ins, op);
                program += "\n - ";
                program += format!("{:?}", ins).as_str();
            }
        }
        program
    }

    /// Execute the provided `ins` instruction, eventually update the `output` vector
    fn execute_instruction (&mut self, ins: Instruction, output: &mut Vec<Tribble>) {

        match ins {
            Instruction::Adv(op) => {
                let op_val = self.combo_to_value(op);
                self.a = self.a / (2u32.pow(op_val as u32) as usize);
            },

            Instruction::Bxl(n)  => {
                self.b = self.b ^ (n as usize);
            },

            Instruction::Bst(op) => {
                let op_val = self.combo_to_value(op);
                self.b = op_val & 0b111;
            },

            Instruction::Jnz(n)  => {
                if self.a != 0 { self.sp = n as usize; }
            }

            Instruction::Bxc     => {
                self.b = self.b ^ self.c;
            },

            Instruction::Out(op) => {
                let op_val = self.combo_to_value(op);
                output.push((op_val & 0b111) as Tribble);
            },

            Instruction::Bdv(op) => {
                let op_val = self.combo_to_value(op);
                self.b = self.a / (2u32.pow(op_val as u32) as usize);
            },

            Instruction::Cdv(op) => {
                let op_val = self.combo_to_value(op);
                self.c = self.a / (2u32.pow(op_val as u32) as usize);
            },
        }
    }

    /// Transform a combo operand into a value
    fn combo_to_value (&self, op: ComboOperand) -> usize {
        match op {
            ComboOperand::Literal(n) => n as usize,
            ComboOperand::RegA => self.a,
            ComboOperand::RegB => self.b,
            ComboOperand::RegC => self.c,
            _ => unreachable!(),
        }
    }

    /// Reset the computer with Reg A value forced to `a`
    fn reset_with_reg_a (&mut self, a: Register) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.sp = 0;
    }
}

/// Assuming the `computer` Reg A value can already generate an output that matches the 'n' last digits of the
/// program, search for the next *tribble* that would result in 'n+1' matching digits.
/// This function tests the 8 possible *tribble* values, except if `tribble_start` is > 0. This
/// parameter can be used when backtracking to restart after the last known good *tribble*.
/// `matching_output_idx` is relative to the whole computer program and indicates the digit
/// we try to match.
///
/// ## Example
/// If the program is [40, 41, 42, 43, 44, 45], calling this function with `matching_output_idx=2`
/// means that the Reg A value can already generate the output [43, 44, 45] and that we try
/// to find the next *tribble* that would enable to output [42, 43, 44, 45]
///
/// ## Result
/// * In case of success: The `computer` Reg A value is updated and the function returns true
/// * In case of failure: The `computer` Reg A value is left unchanged and the function returns false
fn compute_next_tribble (computer: &mut Computer, matching_output_idx: usize, tribble_start: Tribble) -> bool {

    // The search procedure works right to left. If the program is [1, 2, 3, 4, 5] and if
    // we check the digit at position 3, the generated output must have a length
    // of 2 minimum (e.g [4, 5])
    let expected_output_len = computer.program.len () - matching_output_idx;

    // Make room from the next tribble to find
    let base = computer.a << 3;

    // Test the different possible tribbles we could add to register A
    for tribble in tribble_start..8 {
        computer.reset_with_reg_a (base | tribble as Register);

        // Get the resulting output. Check we have enough digits
        let Some(output) = computer.execute().ok () else { return false };
        if output.len() < expected_output_len { continue };

        // Extract the digit to check from the digit. If it matches the program digit
        let Some(value) = output.get (output.len () - expected_output_len) else { continue };
        if *value == computer.program [matching_output_idx] {
            computer.a = base | tribble as Register;
            return true
        }
    }

    // Reset Reg A to its original state in case of failure
    computer.a = base >> 3;
    false
}

/// Backtracking when it was not possible to find a *tribble* that would result in an output
/// matching the last digits of the program content (from the index `matching_output_idx`)
/// In that case, we test the other possibilities for the last *tribble* of the Reg A value.
/// If all the possibilities are exhausted, then we make a step backward by discarding
/// the last *tribble* and by incrementing the one before; and so forth.
///
/// This function stops when the backtracking is successful in finding an updated *tribble* value.
/// In that case it returns the new value of the parameter `matching_output_idx` to consider.
///
/// If all the possible tribbles have been exhausted, the function returns None
fn backtrack (computer: &mut Computer, mut matching_output_idx: usize) -> Option<usize> {

    // Backtracking loop
    while matching_output_idx < computer.program.len () -1 {

        // make a step backward
        matching_output_idx += 1;

        // Take the last tribble used, then remove it
        let last_tribble = (computer.a & 0b111) as Tribble;
        computer.a >>= 3;

        // Try computing another tribble that would give the same result for the current 'matching_output_idx'
        // If successful, return the new 'matching_output_idx' to consider
        if compute_next_tribble (computer, matching_output_idx, last_tribble+1) {
            return Some (matching_output_idx -1);
        }
    }

    // Fail!
    None
}

/// Find the value to put in the register A in order to get an output that replicates the
/// computer program. This function does that iteratively, *tribble* by *tribble*,
/// and make steps backward when stuck in dead-ends.
///
/// In other words, we first try to find a single *tribble* that results in a program
/// outputting a single digit matching the last program digit (step 0).
/// Then we shift the register and try to find another *tribble* so that the output
/// matches the two last digits of the program (step 1), and so forth.
///
/// This procedure works because of the nature of the instructions and the program structure
/// ```
/// while A > 0
///    B = A & 0b111;
///    ...
///    C = A >> B
///    A = A >> 3
///    ...
///    B = B xor C
///    out [B & 0b111]
/// ```
fn compute_reg_a(content: &[&str]) -> Result<Register> {

    let mut computer = Computer::new(content)?;
    let mut matching_output_idx = computer.program.len() - 1;
    computer.a = 0;

    loop {
        match compute_next_tribble(&mut computer, matching_output_idx, 0) {
            true => {
                if matching_output_idx > 0 {
                    matching_output_idx -= 1;
                } else {
                    break Ok(computer.a)
                }
            },
            false => {
                if let Some (idx) = backtrack(&mut computer, matching_output_idx) {
                    matching_output_idx = idx;
                }
                else { bail!("Could not compute register A")}
            },
        }
    }
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<String> {

    let mut computer = Computer::new(content)?;
    let output = computer.execute()?;

    let output_string = output.iter ().map(|&x| x.to_string()).join(",");
    Ok(output_string)
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    let reg_a = compute_reg_a(content)?;
    Ok(reg_a)
}

pub fn day_17 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST_1)).unwrap_or_default() == "4,6,3,5,6,3,5,2,1,0");
    debug_assert!(part_b (&split(TEST_2)).unwrap_or_default() == 117440);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Text(ra), Solution::Unsigned(rb)))
}