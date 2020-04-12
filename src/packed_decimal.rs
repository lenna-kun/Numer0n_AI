pub type PackedDecimal = u16;
pub fn i32_to_packed_decimal(num: i32) -> PackedDecimal {
    (0..4).fold(0, |acc, x| (acc << 4) | (num / 10i32.pow(x) % 10)) as PackedDecimal
}