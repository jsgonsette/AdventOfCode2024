use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl};

const UNIT_WIDTH: usize = 128;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitSet {
    set: Vec<u128>,
    width: usize,
}

impl fmt::Display for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut unit_idx = self.set.len()-1;
        let mut bit_idx = self.width % UNIT_WIDTH;
        while unit_idx != 0 || bit_idx != 0 {
            bit_idx = if bit_idx == 0 { unit_idx -= 1; UNIT_WIDTH-1 } else { bit_idx -1 };
            match self.set[unit_idx] & (1 << bit_idx) {
                0 => write!(f, "0")?,
                _ => write!(f, "1")?,
            }
        }

        Ok(())
    }
}

impl BitAnd for &BitSet {
    type Output = BitSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        let set = self.set.iter().zip(rhs.set.iter()).map(|(x, y)| x & y).collect();
        BitSet { set, width: self.width }
    }
}

impl BitAndAssign<&Self> for BitSet {
    fn bitand_assign(&mut self, rhs: &Self) {
        assert_eq!(self.width, rhs.width);
        for i in 0..self.set.len() {
            self.set[i] &= rhs.set[i];
        }
    }
}

impl BitOr for &BitSet {
    type Output = BitSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        let set = self.set.iter().zip(rhs.set.iter()).map(|(x, y)| x | y).collect();
        BitSet { set, width: self.width }
    }
}

impl BitOrAssign<&Self> for BitSet {
    fn bitor_assign(&mut self, rhs: &Self) {
        assert_eq!(self.width, rhs.width);
        for i in 0..self.set.len() {
            self.set[i] |= rhs.set[i];
        }
    }
}

impl BitXor for &BitSet {
    type Output = BitSet;

    fn bitxor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        let set = self.set.iter().zip(rhs.set.iter()).map(|(x, y)| x ^ y).collect();
        BitSet { set, width: self.width }
    }
}

impl BitXorAssign<&Self> for BitSet {
    fn bitxor_assign(&mut self, rhs: &Self) {
        assert_eq!(self.width, rhs.width);
        for i in 0..self.set.len() {
            self.set[i] ^= rhs.set[i];
        }
    }
}

impl Not for BitSet {
    type Output = BitSet;

    fn not(self) -> Self::Output {
        let set = self.set.iter().map(|x| !x).collect();
        BitSet { set, width: self.width }
    }
}

impl Shl<usize> for &BitSet {
    type Output = BitSet;
    fn shl(self, rhs: usize) -> Self::Output {
        let skip = rhs / UNIT_WIDTH;
        let shift = rhs % UNIT_WIDTH;
        let mask_left = u128::MAX << (UNIT_WIDTH - shift);
        let mask_right = u128::MAX >> shift;

        let set = (0..self.set.len()).rev ().map (
            |idx| {
                let right = match skip {
                    x if x <= idx-1 => (self.set[idx-1-skip] & mask_left) >> (UNIT_WIDTH - shift),
                    _               => 0,
                };
                let left = match skip {
                    x if x <= idx => (self.set[idx-skip] & mask_right) << shift,
                    _             => 0,
                };
                left | right
            }
        ).rev().collect();
        BitSet { set, width: self.width }
    }
}

impl Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {

        assert!(index < self.width);
        let unit_idx = index / UNIT_WIDTH;
        let rem = index % UNIT_WIDTH;
        match self.set [unit_idx] & (1 << rem) {
            0 => &false,
            1 => &true,
            _ => unreachable!(),
        }
    }
}

impl BitSet {
    pub fn zeros(width: usize) -> BitSet {
        let unit_width = 1 + width / UNIT_WIDTH;
        BitSet { width, set: vec![0; unit_width], }
    }

    pub fn ones(width: usize) -> BitSet {
        let unit_width = 1 + width / UNIT_WIDTH;
        BitSet { width, set: vec![u128::MAX; unit_width], }
    }
}