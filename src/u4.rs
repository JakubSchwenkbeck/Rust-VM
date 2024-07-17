#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct U4(u8);

impl U4 {
    fn new(value: u8) -> Self {
        U4(value & 0x0F)  // Mask to ensure value is only 4 bits
    }
}

impl std::ops::Add for U4 {
    type Output = U4;

    fn add(self, other: U4) -> U4 {
        U4::new((self.0 + other.0) & 0x0F)
    }
}

impl std::ops::Sub for U4 {
    type Output = U4;

    fn sub(self, other: U4) -> U4 {
        U4::new((self.0.wrapping_sub(other.0)) & 0x0F)
    }
}

impl std::fmt::Display for U4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

use std::ops::{Index, IndexMut};

impl Index<U4> for [u16; 16] {
    type Output = u16;

    fn index(&self, index: U4) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl IndexMut<U4> for [u16; 16] {
    fn index_mut(&mut self, index: U4) -> &mut Self::Output {
        &mut self[index.0 as usize]
    }
}
