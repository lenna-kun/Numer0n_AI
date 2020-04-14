pub type PackedDecimal = u16;

pub fn from(num: i32, number_of_digits: u32) -> PackedDecimal {
    (0..number_of_digits).fold(0, |acc, x| (acc << 4) | (num / 10i32.pow(x) % 10)) as PackedDecimal
}