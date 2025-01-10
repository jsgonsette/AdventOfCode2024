use anyhow::*;
use itertools::Itertools;
use crate::{Solution};
use crate::tools::IntReader;

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

        let mut reader = IntReader::new(false);
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

    /// Executes the internal program and delivers the final output vector
    fn execute (&mut self) -> Result<Vec<Tribble>> {

        let mut outputs: Vec<Tribble> = vec![];

        loop {
            // Get the next instruction code and operand code.
            // Stop when the stack pointer is out of range
            let Some (&ins) = self.program.get(self.sp) else { break };
            let &op = self.program.get(self.sp + 1).ok_or(anyhow!("SP out of program range"))?;
            self.sp += 2;

            // Make a valid instruction with them
            let ins = Instruction::from_pair(ins, op);

            // And execute it
            let output = self.execute_instruction(ins);
            if let Some (value) = output { outputs.push(value); }
        }

        Ok(outputs)
    }

    /// Execute multiple steps until a first *Tribble* is delivered on the output,
    /// or until the program ends.
    fn output_step (&mut self) -> Option<Tribble> {
        while let Some (&ins) = self.program.get(self.sp)  {

            let &op = self.program.get(self.sp + 1)?;
            let ins = Instruction::from_pair(ins, op);
            self.sp += 2;

            let output = self.execute_instruction(ins);
            if output.is_some() { return output }
        }

        None
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

    /// Execute the provided `ins` instruction, eventually outputting a number
    fn execute_instruction (&mut self, ins: Instruction) -> Option<Tribble> {

        match ins {
            Instruction::Adv(op) => {
                let op_val = self.combo_to_value(op);
                self.a = self.a >> op_val;
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
                return Some ((op_val & 0b111) as Tribble)
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

        None
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
/// Parameter `step` indicates which program digit we try to match, starting from the end.
///
/// ## Example
/// If the program is [40, 41, 42, 43, 44, 45], calling this function with `step=3`
/// means that the Reg A value can already generate the output [43, 44, 45] and that we try
/// to find the next *tribble* that would enable to output [42, 43, 44, 45]
///
/// ## Result
/// * In case of success: The `computer` Reg A value is updated and the function returns true
/// * In case of failure: The `computer` Reg A value is left unchanged and the function returns false
fn compute_next_tribble (computer: &mut Computer, step: usize, tribble_start: Tribble) -> bool {

    // Make room from the next tribble to find
    let base = computer.a << 3;

    // Test the different possible tribbles we could add to register A
    for tribble in tribble_start..8 {

        // Execute the program until the first output is delivered
        computer.reset_with_reg_a (base | tribble as Register);
        let Some (first_output) = computer.output_step() else { continue };

        // and compare it with the program
        if first_output == computer.program [computer.program.len () -step -1] {
            computer.a = base | tribble as Register;
            return true
        }
    }

    // Reset Reg A to its original state in case of failure
    computer.a = base >> 3;
    false
}

/// Backtracking when it was not possible to find a *tribble* that would result in an output
/// matching the last digits of the program content (parameter `step`)
/// In that case, we test the other possibilities for the last *tribble* of the Reg A value.
/// If all the possibilities are exhausted, then we make a step backward by discarding
/// the last *tribble* and by incrementing the one before; and so forth.
///
/// This function stops when the backtracking is successful in finding an updated *tribble* value.
/// In that case it returns the new value of the parameter `step` to consider.
///
/// If all the possible *tribbles* have been exhausted, the function returns None
fn backtrack (computer: &mut Computer, mut step: usize) -> Option<usize> {

    // Backtracking loop
    while step > 0  {

        // make a step backward
        step -= 1;

        // Take the last tribble used, then remove it
        let last_tribble = (computer.a & 0b111) as Tribble;
        computer.a >>= 3;

        // Try computing another tribble that would give the same result for the current 'step'
        // If successful, return the new 'step' value to consider
        if compute_next_tribble (computer, step, last_tribble+1) {
            return Some (step +1);
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
    let mut step = 0;
    computer.a = 0;

    loop {
        match compute_next_tribble(&mut computer, step, 0) {
            true => {
                if step < computer.program.len () -1 { step +=1 }
                else { break Ok(computer.a) }
            },
            false => {
                if let Some (new_step) = backtrack(&mut computer, step) {
                    step = new_step;
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