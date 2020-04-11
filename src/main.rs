extern crate rand;
use rand::Rng;

use std::io::*;

fn read_int(s: &mut StdinLock) -> usize {
    s.by_ref().bytes().map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .skip_while(|c| c.is_whitespace())
        .fold(0, |a, x| (x as u8 - b'0') as usize + a * 10)
}
fn i32_to_packed_decimal(num: i32) -> u16 {
    (0..4).fold(0, |acc, x| (acc << 4) | (num / 10i32.pow(x) % 10)) as u16
}

#[derive(Copy, Clone)]
struct Numer0nItem {
    packed_decimal: u16,
    call_bit: [u16; 4],
    bit_table: u16,
}
impl Numer0nItem {
    fn from(num: i32) -> Self {
        Numer0nItem {
            packed_decimal: i32_to_packed_decimal(num),
            call_bit: (0..4).fold([0u16; 4], |mut acc, x| {
                acc[3-x] = 2u16.pow((num / 10i32.pow(x as u32) % 10) as u32);
                acc
            }),
            bit_table: (0..4).fold(0u16, |acc, x| acc | (2u16.pow((num / 10i32.pow(x) % 10) as u32))),
        }
    }
    fn eat(self, call: &Self) -> usize {
        let xor = self.packed_decimal ^ call.packed_decimal;
        (0..4).fold(0, |et, i| if (xor << (4*i)) >> 12 == 0 { et + 1 } else { et })
    }
    fn eat_bite(self, call: &Self) -> usize {
        (0..4).fold(0, |eb, i| if (self.bit_table & call.call_bit[i]) > 0 { eb + 1 } else { eb }) 
    }
}
impl std::fmt::Display for Numer0nItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}{}", self.call_bit[0].trailing_zeros(), self.call_bit[1].trailing_zeros(), self.call_bit[2].trailing_zeros(), self.call_bit[3].trailing_zeros())
    }
}

struct Numer0nItems(Vec<Numer0nItem>);
impl std::fmt::Display for Numer0nItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut comma_separated = String::new();

        for numer0n_item in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&*format!("{}", numer0n_item));
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&*format!("{}", &self.0[self.0.len() - 1]));
        write!(f, "[{}]", comma_separated)
    }
}

struct Numer0nData {
    cand: Numer0nItems,
    all_numer0n_items: Numer0nItems,
    guess: Numer0nItem,
    eat: usize,
    bite: usize,
}
impl Numer0nData {
    fn new() -> Self {
        Numer0nData {
            cand: Numer0nItems((0..10000).map(|i| Numer0nItem::from(i)).collect()),
            all_numer0n_items: Numer0nItems((0..10000).map(|i| Numer0nItem::from(i)).collect()),
            guess: Numer0nItem::from(0012),
            eat: 0,
            bite: 0,
        }
    }
    fn set_next_guess(&mut self) {
        if self.cand.0.len() == 10000 {
            return;
        } else if self.cand.0.len() <= 2 {
            self.guess =  self.cand.0[0];
            return;
        } else if self.guess.packed_decimal == i32_to_packed_decimal(0012) {
            match self.bite {
                0 => {
                    match self.eat {
                        0 => {
                            self.guess = Numer0nItem::from(3345);
                        },
                        1 => {
                            self.guess = Numer0nItem::from(3415);
                        },
                        2 => {
                            self.guess = Numer0nItem::from(0345);
                        },
                        3 => {
                            self.guess = Numer0nItem::from(3415);
                        },
                        _ => panic!("unexpected error."),
                    }
                },
                1 => {
                    match self.eat {
                        0 => {
                            self.guess = Numer0nItem::from(1134);
                        },
                        1 => {
                            self.guess = Numer0nItem::from(0304);
                        },
                        2 => {
                            self.guess = Numer0nItem::from(0113);
                        },
                        3 => {
                            self.guess = Numer0nItem::from(0345);
                        },
                        _ => panic!("unexpected error."),
                    }
                },
                2 => {
                    match self.eat {
                        0 => {
                            self.guess = Numer0nItem::from(3405);
                        },
                        1 => {
                            self.guess = Numer0nItem::from(0121);
                        },
                        2 => {
                            self.guess = Numer0nItem::from(0345);
                        },
                        _ => panic!("unexpected error."),
                    }
                },
                3 => {
                    match self.eat {
                        0 => {
                            self.guess = Numer0nItem::from(1120);
                        },
                        1 => {
                            self.guess = Numer0nItem::from(0121);
                        },
                        _ => panic!("unexpected error."),
                    }
                },
                4 => {
                    match self.eat {
                        0 => {
                            self.guess = Numer0nItem::from(1120);
                        },
                        _ => panic!("unexpected error."),
                    }
                },
                _ => panic!("unexpected error."),
            }
            return;
        }
        let mut min: usize = usize::max_value();
        'search: for guess in &self.all_numer0n_items.0 {
            let mut mat: [[usize; 5]; 5] = [[0; 5]; 5];
            for c in &self.cand.0 {
                let eat: usize = c.eat(guess);
                let bite: usize = c.eat_bite(guess) - eat;
                mat[eat][bite] += 1;
            }
            let mut max = 0;
            for k in 0..5 {
                for l in 0..5 {
                    max = std::cmp::max(max, mat[k][l]);
                    if min <= max {
                        continue 'search; // pruning
                    }
                }
            }
            if min > max {
                self.guess = *guess;
                min = max;
            }
        }
    }
    fn reduce_cand(&mut self) {
        for i in (0..self.cand.0.len()).rev() { // If candidates is not erased from behind, it will behave unintentionally.
            let et: usize = self.cand.0[i].eat(&self.guess);
            let bt: usize = self.cand.0[i].eat_bite(&self.guess) - et;
            if self.eat != et || self.bite != bt {
                self.cand.0.swap_remove(i);
            }
        }
    }
}

fn main() {
    let s = stdin();
    let mut s = s.lock();
    println!("initializing...");
    let mut numer0n_data = Numer0nData::new();
    let mut rng = rand::thread_rng();
    let mut my_number: [usize; 4] = [0, 0, 0, 0];
    for i in 0..4 {
        loop {
            let n = rng.gen::<usize>() % 10;
            if my_number[0..i].iter().filter(|&&x| x == n).count() <= 1 {
                my_number[i] = n;
                break;
            }
        }
    }
    println!("My number is {}{}{}{}.", my_number[0], my_number[1], my_number[2], my_number[3]);    

    loop {
        println!("{}", numer0n_data.cand);
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