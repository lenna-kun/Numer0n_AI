use super::bit_table;
use crate::bit_table::BitTable;

use super::packed_decimal;
use crate::packed_decimal::PackedDecimal;

#[derive(Copy, Clone)]
pub struct Numer0nItem {
    pub packed_decimal: PackedDecimal,
    pub overlap_array: [usize; 10],
    pub bit_table: BitTable,
}
impl Numer0nItem {
    pub fn from(num: i32) -> Self {
        Numer0nItem {
            packed_decimal: packed_decimal::i32_to_packed_decimal(num),
            overlap_array: (0..4).fold([0; 10], |mut acc, x| {
                acc[num as usize / 10usize.pow(x as u32) % 10] += 1;
                acc
            }),
            bit_table: bit_table::from_multiple_digits(num, 4),
        }
    }

    pub fn eat(self, call: &Self) -> usize {
        let xor = self.packed_decimal ^ call.packed_decimal;
        (0..4).fold(0, |et, i| if (xor << (4*i)) >> 12 == 0 { et + 1 } else { et })
    }
    
    pub fn eat_bite(self, call: &Self) -> usize {
        let and = self.bit_table & call.bit_table;
        (0..and.count_ones()).fold((0usize, 0usize, and), |(num, eb, and), _| {
            let shift = and.trailing_zeros() as usize + 1;
            (num+shift, eb + call.overlap_array[num+shift-1], and >> shift)
        }).1
    }
}
impl std::cmp::PartialEq for Numer0nItem {
    fn eq(&self, other: &Self) -> bool {
        self.packed_decimal == other.packed_decimal
    }
}
impl std::fmt::Display for Numer0nItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:>04}", (0..4).fold(0, |acc, i| (self.packed_decimal << (12-4*i) >> 12) + acc*10))
    }
}