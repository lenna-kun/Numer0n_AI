pub type BitTable = u16;

pub fn from_multiple_digits(num: i32, number_of_digits: u32) -> BitTable {
    (0..number_of_digits).fold(0u16, |acc, x| acc | (2u16.pow((num / 10i32.pow(x) % 10) as u32)))
}