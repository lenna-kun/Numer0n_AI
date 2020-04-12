use super::bit_table;
use crate::bit_table::BitTable;

use super::packed_decimal;
use crate::packed_decimal::PackedDecimal;

#[derive(Copy, Clone)]
pub struct Numer0nItem {
    pub packed_decimal: PackedDecimal,
    pub call_bit: [BitTable; 4],
    pub bit_table: BitTable,
}
impl Numer0nItem {
    pub fn from(num: i32) -> Self {
        Numer0nItem {
            packed_decimal: packed_decimal::i32_to_packed_decimal(num),
            call_bit: (0..4).fold([0 as BitTable; 4], |mut acc, x| {
                acc[3-x] = bit_table::from_one_digit(num / 10i32.pow(x as u32) % 10);
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
        (0..4).fold(0, |eb, i| if (self.bit_table & call.call_bit[i]) > 0 { eb + 1 } else { eb }) 
    }
}
impl std::fmt::Display for Numer0nItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}{}", self.call_bit[0].trailing_zeros(), self.call_bit[1].trailing_zeros(), self.call_bit[2].trailing_zeros(), self.call_bit[3].trailing_zeros())
    }
}