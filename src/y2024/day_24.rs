use std::collections::{HashMap};
use std::fmt::Debug;
use std::hash::Hash;
use anyhow::*;
use crate::{Solution};
use crate::tools::{topo_sort, TopoSortElement};

const TEST: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

// Gates form an acyclic graph of gates. They can be topologically sorted.
impl TopoSortElement<GateName> for Gate {
    type Iter = std::vec::IntoIter<GateName>;

    fn what_next(&self) -> Self::Iter  {
        match self {
            Gate::Value(_) => vec![].into_iter(),
            Gate::OR(lhs, rhs) => vec![*lhs, *rhs].into_iter(),
            Gate::XOR(lhs, rhs) => vec![*lhs, *rhs].into_iter(),
            Gate::AND(lhs, rhs) => vec![*lhs, *rhs].into_iter(),
        }
    }
}

/// The 3-letter name of a gate
type GateName = [char; 3];

/// Models a gate as an input value or as a logical operation combining other gates
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Gate {
    Value (bool),
    OR (GateName, GateName),
    XOR (GateName, GateName),
    AND (GateName, GateName),
}

/// All the gates in the circuit
type Gates = HashMap<GateName, Gate>;

impl Gate {

    /// Return a copy of this gate with its entries swapped
    fn swap(&self) -> Gate {
        match self {
            Gate::Value(_) => *self,
            Gate::OR(a, b) => Gate::OR(*b, *a),
            Gate::XOR(a, b) => Gate::XOR(*b, *a),
            Gate::AND(a, b) => Gate::AND(*b, *a),
        }
    }

    fn input_names (&self) -> Option<(GateName, GateName)> {
        match self {
            Gate::Value(_) => None,
            Gate::OR(a, b) => Some((*a, *b)),
            Gate::XOR(a, b) => Some((*a, *b)),
            Gate::AND(a, b) => Some((*a, *b)),
        }
    }

    fn other_input_name (&self, first_input: &GateName) -> Option<GateName> {
        let Some((a, b)) =self.input_names() else { return None };

        if a == *first_input { Some(b) }
        else if b == *first_input { Some(a) }
        else { None }
    }
}


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Create a gate name from text (take the 3 first letters)
fn get_name (row: &str) -> GateName {
    let raw = row.as_bytes();
    [raw [0] as char, raw [1] as char, raw [2] as char]
}

/// Load the circuit gates from the puzzle file `content`
fn load_gates (content: &[&str]) -> Result<Gates> {

    let mut is_value = true;
    let mut gates = Gates::new();

    for row in content {

        // Input values come first and are separated from the logic gates by an empty line
        if row.is_empty() {
            is_value = false;
            continue;
        }

        // Extract the value of an input
        if is_value {
            let name = get_name(row);
            let val = row.as_bytes()[5] as char == '1';
            gates.insert(name, Gate::Value(val));
        }
        // or extract a logical operation
        else {
            let tokens: Vec<&str> = row.split_whitespace().collect();
            let name_0 = get_name (tokens [0]);
            let name_1 = get_name (tokens [2]);
            let name_out = get_name (tokens [4]);
            let op = tokens [1];
            let gate = match op {
                "XOR" => Gate::XOR (name_0, name_1),
                "AND" => Gate::AND (name_0, name_1),
                "OR" => Gate::OR (name_0, name_1),
                _ => bail!("Unknown gate: {}", op)
            };

            gates.insert(name_out, gate);
        }
    }

    Ok(gates)
}

/// Compute the output of the acyclic circuit `gates`. Parameter `topo_order` must
/// be a valid topological ordering ensuring we can do the computation in one pass through
/// all the gates of the circuit. The function returns a 64 bits value corresponding to the
/// concatenation of all the `z..` outputs, where `z00` is the lsb.
fn compute (gates: &Gates, topo_order: &Vec<GateName>) -> u64 {

    let mut values = HashMap::<GateName, bool>::new();
    let mut z = 0u64;

    // Follow the topo. ordering and compute the gate output values one by one
    for name in topo_order {

        let value = match gates [name] {
            Gate::Value(val) => { val },
            Gate::OR (name_0, name_1) => {
                let a = values.get(&name_0).unwrap();
                let b = values.get(&name_1).unwrap();
                a | b
            }
            Gate::XOR (name_0, name_1) => {
                let a = values.get(&name_0).unwrap();
                let b = values.get(&name_1).unwrap();
                a ^ b
            }
            Gate::AND (name_0, name_1) => {
                let a = values.get(&name_0).unwrap();
                let b = values.get(&name_1).unwrap();
                a & b
            }
        };

        // Save the value of this gate for those using it later in the circuit
        values.insert(*name, value);

        // Collect the bit of the final value
        if name [0] == 'z' {
            let offset = name [1].to_digit(10).unwrap() * 10 + name [2].to_digit(10).unwrap();
            if value { z |= 1 << offset; }
        }
    }

    z
}

fn patch_circuit (gates: &mut Gates, name_a: &GateName, name_b: &GateName) {

    let gate_a = *gates.get (name_a).unwrap();
    let gate_b = *gates.get (name_b).unwrap();

    gates.entry (*name_a).and_modify(|e| *e = gate_b);
    gates.entry (*name_b).and_modify(|e| *e = gate_a);
}

/// Make a name corresponding to some circuit input, given a `prefix` and a `bit_offset`
/// Example: if `prefix` = 'x' and `bit_offset` = 3, the function returns the name 'x03'
fn make_entry_name(prefix: char, bit_offset: usize) -> GateName {
    [
        prefix,
        ('0' as u8 + (bit_offset /10) as u8) as char,
        ('0' as u8 + (bit_offset %10) as u8) as char,
    ]
}

/// Change the value of the circuit inputs with the provided 64-bit values `x` and `y`.
/// Those values are spilt in individual bits that are dispatched on the corresponding
/// inputs `x01..x63` and `y01..y63`
fn set_x_y (gates: &mut Gates, mut x: u64, mut y:u64) {

    for i in 0..64 {
        let val_x = (x & 1) > 0;
        let val_y = (y & 1) > 0;
        x >>=1;
        y >>=1;
        gates.entry (make_entry_name('x', i)).and_modify(|e| *e =  Gate::Value(val_x));
        gates.entry (make_entry_name('y', i)).and_modify(|e| *e =  Gate::Value(val_y));
    }
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    // Load the circuit and compute the topological ordering
    let gates = load_gates(content)?;
    let gate_names: Vec<GateName> = topo_sort(&gates);

    // Compute the circuit output
    let z = compute(&gates, &gate_names);

    Ok(z as usize)
}

fn find_gate (gates: &Gates, gate: &Gate) -> Option<GateName> {
    let get_swap = gate.swap();
    let f0 = gates.iter ().find_map(|(output, g)| if *g == *gate { Some (*output) } else { None } );
    let f1 = gates.iter ().find_map(|(output, g)| if *g == get_swap { Some (*output) } else { None } );

    f0.or(f1)
}

fn find_gate_connected_to (gates: &Gates, name: &GateName) -> Option<GateName> {
    gates.iter ().find_map(|(output, g)| {
        let Some ((a, b)) = g.input_names() else { return None };
        if a == *name || b == *name { Some (*output) }
        else { None }
    })
}

type ErrorPair = (GateName, GateName);

/// ```
///             x ─────┬── AND ───────────────────────┬── OR ─── c_out
///                    │                              │
///                    ├── XOR ──┬── XOR ─── z (sum)  │
///             y ─────┘         │                    │
///                              │── AND ─────────────┘
///             c_in ────────────┘
/// ```
fn check_1_bit_additioner (gates: &Gates, stage: usize, carry: &mut GateName) -> Option<ErrorPair> {

    println!("Carry = {:?}", &carry);

    // The expected x and y input names for this stage
    let x = make_entry_name('x', stage);
    let y = make_entry_name('y', stage);

    // Output names for the XOR and AND operations that process the x and y inputs
    // (as they are connected to inputs only, they must exist)
    let mut xor_xy = find_gate(gates, &Gate::XOR (x, y)).unwrap();
    let mut and_xy = find_gate(gates, &Gate::AND (x, y)).unwrap();
    println!("XOR {:?}", xor_xy);
    println!("AND {:?}", and_xy);

    // Output name of the second AND gate connected to the carry input.
    // The carry is assumed to be correct. The second entry may not.
    let carry_and = match find_gate(gates, &Gate::AND (xor_xy, *carry)) {
        None => {
            let gate = find_gate_connected_to(gates, &carry).unwrap();
            let wrong_output = gates [&gate].other_input_name(&carry).unwrap();
            return Some((wrong_output, xor_xy));
        },
        Some(name) => { name }
    };

    let z = find_gate(gates, &Gate::XOR (*carry, xor_xy)).unwrap();
    let expected_z = make_entry_name('z', stage);
    if z != expected_z {
        return Some((z, expected_z));
    }
    println!("XOR Z {:?}", z);

    *carry = find_gate(gates, &Gate::OR (carry_and, and_xy)).unwrap();
    println!("carry {:?}", carry);

    None
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<usize> {

    // Load the circuit and compute the topological ordering
    let mut gates = load_gates(content)?;
    let gate_names = topo_sort(&gates);

    set_x_y(&mut gates, 1, 0);
    let z = compute(&gates, &gate_names);
    println!("Z:{}", z);

    let mut carry = ['w', 'r', 'd'];
    for offset in 1..16 {
        println!("\nOffset {}", offset);
        let error = check_1_bit_additioner (&gates, offset, &mut carry);
        if let Some((a, b)) = error {
            println!("Error between {:?}, {:?}", a, b);
            patch_circuit(&mut gates, &a, &b);
        }
        check_1_bit_additioner (&gates, offset, &mut carry);
    }

    Ok(0)
}

pub fn day_24 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 4);
    //debug_assert!(part_b (&split(TEST)).unwrap_or_default() == 0);

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Unsigned(rb)))
}