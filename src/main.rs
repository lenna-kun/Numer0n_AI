mod stdin;
mod bit_table;
mod packed_decimal;
mod numer0n_data;

fn main() {
    let mut numer0n_data = numer0n_data::Numer0nData::new();  

    loop {
        numer0n_data.set_next_call();
        loop {
            match stdin::read_pair((&mut numer0n_data.eat, "eat"), (&mut numer0n_data.bite, "bite")) {
                Err(e) => {
                    if let stdin::Error::KeyboardInterrupt = e { return; } else { continue; }
                },
                Ok(()) => break,
            }
        }
        if numer0n_data.eat == 4 {
            return;
        }
        numer0n_data.reduce_cand();
    }
}