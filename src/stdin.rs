use std::io::*;

pub fn read_decimal() -> usize {
    let s = stdin();
    let mut s = s.lock();
    s.by_ref().bytes().map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .skip_while(|c| c.is_whitespace())
        .fold(0, |a, x| (x as u8 - b'0') as usize + a * 10)
}