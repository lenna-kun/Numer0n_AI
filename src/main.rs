use std::io::*;

mod bit_table;
mod packed_decimal;
mod numer0n_data;

fn read_int(s: &mut StdinLock) -> usize {
    s.by_ref().bytes().map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .skip_while(|c| c.is_whitespace())
        .fold(0, |a, x| (x as u8 - b'0') as usize + a * 10)
}

fn main() {
    let s = stdin();
    let mut s = s.lock();
    println!("initializing...");
    let mut numer0n_data = numer0n_data::Numer0nData::new(numer0n_data::DisplayMode::Off);  

    loop {
        println!("guessing...");
        numer0n_data.set_next_guess();
        println!("My guess is {}.", numer0n_data.guess);
        numer0n_data.eat = read_int(&mut s);
        numer0n_data.bite = read_int(&mut s);
        if numer0n_data.eat == 4 {
            return;
        }
        numer0n_data.reduce_cand();
    }
}