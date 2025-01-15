use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl, Shr};

/// Width (in bits) of the underlying type used to encode the bits
const UNIT_WIDTH: usize = 128;

/// Underlying type used to encode the bits
type Unit = u128;

/// A vector of bits of arbitrary length
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitSet {
    set: Vec<Unit>,
    width: usize,
}

/// To display a [BitSet]
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

/// Binary And operator
impl BitAnd for &BitSet {
    type Output = BitSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        let set = self.set.iter().zip(rhs.set.iter()).map(|(x, y)| x & y).collect();
        BitSet { set, width: self.width }
    }
}

/// Binary And operator
impl BitAnd for BitSet {
    type Output = BitSet;
    fn bitand(self, rhs: Self) -> Self::Output { &self & &rhs }
}

/// Binary And operator
impl BitAnd<&BitSet> for BitSet {
    type Output = BitSet;
    fn bitand(self, rhs: &BitSet) -> Self::Output { &self & rhs }
}

/// Binary And operator
impl BitAnd<BitSet> for &BitSet {
    type Output = BitSet;
    fn bitand(self, rhs: BitSet) -> Self::Output { self & &rhs }
}

/// Binary And Assignment operator
impl BitAndAssign<&Self> for BitSet {
    fn bitand_assign(&mut self, rhs: &Self) {
        assert_eq!(self.width, rhs.width);
        for i in 0..self.set.len() {
            self.set[i] &= rhs.set[i];
        }
    }
}

/// Binary Or operator
impl BitOr for &BitSet {
    type Output = BitSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        let set = self.set.iter().zip(rhs.set.iter()).map(|(x, y)| x | y).collect();
        BitSet { set, width: self.width }
    }
}

/// Binary Or operator
impl BitOr for BitSet {
    type Output = BitSet;
    fn bitor(self, rhs: Self) -> Self::Output { &self | &rhs }
}

/// Binary Or operator
impl BitOr<&BitSet> for BitSet {
    type Output = BitSet;
    fn bitor(self, rhs: &BitSet) -> Self::Output { &self | rhs }
}

/// Binary Or operator
impl BitOr<BitSet> for &BitSet {
    type Output = BitSet;
    fn bitor(self, rhs: BitSet) -> Self::Output { self | &rhs }
}

/// Binary Or Assignment operator
impl BitOrAssign<&Self> for BitSet {
    fn bitor_assign(&mut self, rhs: &Self) {
        assert_eq!(self.width, rhs.width);
        for i in 0..self.set.len() {
            self.set[i] |= rhs.set[i];
        }
    }
}

/// Binary Xor operator
impl BitXor for &BitSet {
    type Output = BitSet;

    fn bitxor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        let set = self.set.iter().zip(rhs.set.iter()).map(|(x, y)| x ^ y).collect();
        BitSet { set, width: self.width }
    }
}

/// Binary Xor operator
impl BitXor for BitSet {
    type Output = BitSet;
    fn bitxor(self, rhs: Self) -> Self::Output { &self ^ &rhs }
}

/// Binary Xor operator
impl BitXor<&BitSet> for BitSet {
    type Output = BitSet;
    fn bitxor(self, rhs: &BitSet) -> Self::Output { &self ^ rhs }
}

/// Binary Xor operator
impl BitXor<BitSet> for &BitSet {
    type Output = BitSet;
    fn bitxor(self, rhs: BitSet) -> Self::Output { self ^ &rhs }
}

/// Binary Xor Assignment operator
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
    fn not(self) -> Self::Output { !&self }
}

impl Not for &BitSet {
    type Output = BitSet;

    fn not(self) -> Self::Output {
        let set = self.set.iter().map(|x| !x).collect();
        BitSet { set, width: self.width }
    }
}

/// Shift left operator
impl Shl<usize> for &BitSet {
    type Output = BitSet;
    fn shl(self, rhs: usize) -> Self::Output {

        let skip = rhs / UNIT_WIDTH;
        let shift = rhs % UNIT_WIDTH;
        let mask_left = if shift > 0 {Unit::MAX << (UNIT_WIDTH - shift) } else { 0 };
        let mask_right = Unit::MAX >> shift;

        // Work from MSB to LSB
        let set = (0..self.set.len()).rev ().map (
            |idx| {

                let right = if idx >= 1 && skip <= idx -1 && shift > 0 {
                    (self.set[idx-1-skip] & mask_left) >> (UNIT_WIDTH - shift)
                } else {
                    0
                };

                let left = match skip {
                    x if x <= idx => (self.set[idx-skip] & mask_right) << shift,
                    _             => 0,
                };

                left | right
            }
        ).rev().collect();

        let mut s = BitSet { set, width: self.width };
        s.clear_unused();
        s
    }
}

/// Shift right operator
impl Shr<usize> for BitSet {
    type Output = BitSet;
    fn shr(self, rhs: usize) -> Self::Output { &self >> rhs }
}

/// Shift left operator
impl Shl<usize> for BitSet {
    type Output = BitSet;
    fn shl(self, rhs: usize) -> Self::Output { &self << rhs }
}

/// Shift right operator
impl Shr<usize> for &BitSet {
    type Output = BitSet;

    fn shr(self, rhs: usize) -> Self::Output {
        let skip = rhs / UNIT_WIDTH;
        let shift = rhs % UNIT_WIDTH;
        let mask_left = Unit::MAX << shift;
        let mask_right = if shift > 0 { Unit::MAX >> (UNIT_WIDTH - shift) } else { 0 };

        // Work from LSB to MSB
        let set = (0..self.set.len()).map(
            |idx| {
                let left = if idx +skip +1 < self.set.len() && shift > 0 {
                    (self.set[idx +skip +1] & mask_right) << (UNIT_WIDTH - shift)
                } else {
                    0
                };

                let right = if idx + skip < self.set.len() {
                    (self.set[idx + skip] & mask_left) >> shift
                } else {
                    0
                };

                left | right
            }
        ).collect();

        BitSet { set, width: self.width }
    }
}

/// To return a bit at some index
impl Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {

        assert!(index < self.width);
        let unit_idx = index / UNIT_WIDTH;
        let rem = index % UNIT_WIDTH;
        match self.set [unit_idx] & (1 << rem) {
            0 => &false,
            _ => &true,
        }
    }
}

impl BitSet {

    /// Instantiate a new set of `width` bits, all at `0`
    pub fn zeros(width: usize) -> BitSet {
        let unit_width = 1 + width / UNIT_WIDTH;
        BitSet { width, set: vec![0; unit_width], }
    }

    /// Instantiate a new set of `width` bits, all at `1`
    pub fn ones(width: usize) -> BitSet {
        let unit_width = 1 + width / UNIT_WIDTH;
        let mut s = BitSet { width, set: vec![Unit::MAX; unit_width], };

        s.clear_unused();
        s
    }

    /// Set a `bit` value at some `index`
    pub fn set_bit (&mut self, index: usize, bit: bool) {
        assert!(index < self.width);
        let unit_idx = index / UNIT_WIDTH;
        let rem = index % UNIT_WIDTH;
        match bit {
            false => self.set[unit_idx] &= !(1 << rem),
            true => self.set[unit_idx] |= 1 << rem,
        }
    }
    /// Return the number of bits in this set
    pub fn width (&self) -> usize { self.width }

    /// Return `true` if all the bits are 0
    pub fn all_zeros(&self) -> bool {
        self.set.iter().all(|x| *x == 0)
    }

    /// Returns the number of ones in this set
    pub fn count_ones(&self) -> u32 {
        self.set.iter().map (|&x| x.count_ones()).sum()
    }

    /// Returns the number of zeros in this set
    pub fn count_zeros(&self) -> u32 {
        self.width as u32 - self.count_ones()
    }

    /// Returns the number of leading zeros in this set
    pub fn leading_zeros(&self) -> u32 {
        let Some (start) = self.set.iter ().rev ().position(|&x| x != 0) else { return self.width as u32 };
        let n = self.set.len();
        let unused = (UNIT_WIDTH * n - self.width) as u32;
        (start * UNIT_WIDTH) as u32 + self.set [n -1 -start].leading_zeros() - unused
    }

    /// Returns the number of trailing zeros in this set
    pub fn trailing_zeros(&self) -> u32 {
        let Some (start) = self.set.iter ().position(|&x| x != 0) else { return self.width as u32 };
        (start * UNIT_WIDTH) as u32 + self.set [start].trailing_zeros()
    }

    /// Force the un-used bits to 0
    fn clear_unused (&mut self) {
        let mask = !(Unit::MAX << (self.width % UNIT_WIDTH));
        let n = self.set.len() -1;
        self.set [n] &= mask;
    }
}