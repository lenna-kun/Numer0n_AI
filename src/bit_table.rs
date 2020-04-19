pub type BitTable = u16;

pub fn from(num: i32, number_of_digits: u32) -> BitTable {
    (0..number_of_digits).fold(0u16, |acc, x| acc | (1 << (num / 10i32.pow(x) % 10)))
}