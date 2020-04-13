mod stdin;
mod bit_table;
mod packed_decimal;
mod numer0n_data;

fn main() {
    let mut numer0n_data = numer0n_data::Numer0nData::new();  

    loop {
        // println!("   \x1b[1m\x1b[96mThinklng\x1b[0m"); // 92 = green
        numer0n_data.set_next_call();
        // println!("My call is {}.", numer0n_data.call);
        numer0n_data.eat = stdin::read_decimal();
        numer0n_data.bite = stdin::read_decimal();
        if numer0n_data.eat == 4 {
            return;
        }
        numer0n_data.reduce_cand();
    }
}