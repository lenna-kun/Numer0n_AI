extern crate rand;
use rand::Rng;

use std::io::*;
// use std::collections::{ HashSet, HashMap };

fn read_int(s: &mut StdinLock) -> usize {
    s.by_ref().bytes().map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .skip_while(|c| c.is_whitespace())
        .fold(0, |a, x| (x as u8 - b'0') as usize + a * 10)
}

struct Numer0nData {
    cand: Vec<[usize; 4]>,
    guess: [usize; 4],
    eat: usize,
    bite: usize,
}
impl Numer0nData {
    fn new() -> Numer0nData {
        Numer0nData {
            cand: (0..10000).map(|i| (0..4).fold([0, 0, 0, 0], |mut acc, x| {
                let mut num = i;
                for _ in 0..x { num /= 10; }
                acc[3-x] = num % 10;
                acc
            })).collect(),
            guess: [0, 0, 1, 2],
            eat: 0,
            bite: 0,
        }
    }
    fn next_guess(&mut self) {
        if self.cand.len() == 10000 {
            self.guess = [0, 0, 1, 2];
            return;
        } else if self.cand.len() <= 2 {
            self.guess =  self.cand[0];
            return;
        } else if self.guess == [0, 0, 1, 2] {
            match self.bite {
                0 => {
                    match self.eat {
                        0 => {
                            self.guess = [3, 3, 4, 5];
                        },
                        1 => {
                            self.guess = [3, 4, 1, 5];
                        },
                        2 => {
                            self.guess = [0, 3, 4, 5];
                        },
                        3 => {
                            self.guess = [3, 4, 1, 5];
                        },
                        _ => panic!("unexpected error."),
                    }
                },
                1 => {
                    match self.eat {
                        0 => {
                            self.guess = [1, 1, 3, 4];
                        },
                        1 => {
                            self.guess = [0, 3, 0, 4];
                        },
                        2 => {
                            self.guess = [0, 1, 1, 3];
                        },
                        3 => {
                            self.guess = [0, 3, 4, 5];
                        },
                        _ => panic!("unexpected error."),
                    }
                },
                2 => {
                    match self.eat {
                        0 => {
                            self.guess = [3, 4, 0, 5];
                        },
                        1 => {
                            self.guess = [0, 1, 2, 1];
                        },
                        2 => {
                            self.guess = [0, 3, 4, 5];
                        },
                        _ => panic!("unexpected error."),
                    }
                },
                3 => {
                    match self.eat {
                        0 => {
                            self.guess = [1, 1, 2, 0];
                        },
                        1 => {
                            self.guess = [0, 1, 2, 1];
                        },
                        _ => panic!("unexpected error."),
                    }
                },
                4 => {
                    match self.eat {
                        0 => {
                            self.guess = [1, 1, 2, 0];
                        },
                        _ => panic!("unexpected error."),
                    }
                },
                _ => panic!("unexpected error."),
            }
            return;
        }
        let mut min: usize = usize::max_value();
        for i in 0..10000 { // guess
            let mut mat: [[usize; 5]; 5] = [[0; 5]; 5];
            let guess: [usize; 4] = (0..4).fold([0, 0, 0, 0], |mut acc, x| {
                let mut num = i;
                for _ in 0..x { num /= 10; }
                acc[3-x] = num % 10;
                acc
            });
            for c in &self.cand {
                let mut eat: usize = 0;
                let mut bite: usize = 0;
                for k in 0..4 { // guessを1桁ずつみていく
                    if c[k] == guess[k] {
                        eat += 1;
                    } else {
                        for l in 0..4 {
                            if k != l && c[l] == guess[k] {
                                bite += 1;
                                break;
                            }
                        }
                    }
                }
                mat[eat][bite] += 1;
            }
            let mut max = 0;
            for k in 0..5 {
                for l in 0..5 {
                    max = std::cmp::max(max, mat[k][l]);
                }
            }
            if min > max {
                self.guess = guess;
                min = max;
            }
        }
    }
    fn reduce_cand(&mut self) {
        for i in (0..self.cand.len()).rev() { // 後ろから消さないとおかしなことになる
            let mut et: usize = 0;
            let mut bt: usize = 0;
            for k in 0..4 { // guessを1桁ずつみていく
                if self.cand[i][k] == self.guess[k] {
                    et += 1;
                } else {
                    for l in 0..4 {
                        if k != l && self.cand[i][l] == self.guess[k] {
                            bt += 1;
                            break;
                        }
                    }
                }
            }
            if self.eat != et || self.bite != bt {
                self.cand.swap_remove(i);
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
        println!("guessing...");
        numer0n_data.next_guess();
        println!("My guess is {}{}{}{}.", numer0n_data.guess[0], numer0n_data.guess[1], numer0n_data.guess[2], numer0n_data.guess[3]);
        numer0n_data.eat = read_int(&mut s);
        numer0n_data.bite = read_int(&mut s);
        if numer0n_data.eat == 4 {
            return;
        }
        numer0n_data.reduce_cand();
    }
}