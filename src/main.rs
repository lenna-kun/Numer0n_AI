mod stdin;
mod bit_table;
mod packed_decimal;
mod numer0n_data;

fn main() {
    println!("initializing...");
    let mut numer0n_data = numer0n_data::Numer0nData::new(numer0n_data::DisplayMode::Off);  

    loop {
        println!("guessing...");
        numer0n_data.set_next_guess();
        println!("My guess is {}.", numer0n_data.guess);
        numer0n_data.eat = stdin::read_decimal();
        numer0n_data.bite = stdin::read_decimal();
        if numer0n_data.eat == 4 {
            return;
        }
        numer0n_data.reduce_cand();
    }
}