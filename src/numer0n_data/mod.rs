use super::bit_table;
use super::packed_decimal;
use super::stdin;

mod numer0n_item;

pub struct Numer0nData {
    cand: Vec<numer0n_item::Numer0nItem>,
    all_numer0n_items: Vec<numer0n_item::Numer0nItem>,
    pub call: numer0n_item::Numer0nItem,
    pub eat: usize,
    pub bite: usize,
}
impl Numer0nData {
    pub fn new() -> Self {
        let all_numer0n_items: Vec<numer0n_item::Numer0nItem> = 
            (0..10000).map(|i|
                numer0n_item::Numer0nItem::from(i)
            ).collect();
        println!("\x1b[1G\x1b[1m\x1b[92mInitialized\x1b[0m");
        Numer0nData {
            call: all_numer0n_items[9987],
            cand: all_numer0n_items.clone(),
            all_numer0n_items: all_numer0n_items,
            eat: 0,
            bite: 0,
        }
    }

    pub fn call_from_branch_table(&mut self) {
        match self.bite {
            0 => {
                match self.eat {
                    0 => {
                        self.call = self.all_numer0n_items[6654];
                    },
                    1 => {
                        self.call = self.all_numer0n_items[6584];
                    },
                    2 => {
                        self.call = self.all_numer0n_items[0912];
                    },
                    3 => {
                        self.call = self.all_numer0n_items[0127];
                    },
                    _ => panic!("unexpected error."),
                }
            },
            1 => {
                match self.eat {
                    0 => {
                        self.call = self.all_numer0n_items[8865];
                    },
                    1 => {
                        self.call = self.all_numer0n_items[9695];
                    },
                    2 => {
                        self.call = self.all_numer0n_items[9886];
                    },
                    3 => {
                        self.call = self.all_numer0n_items[0912];
                    },
                    _ => panic!("unexpected error."),
                }
            },
            2 => {
                match self.eat {
                    0 => {
                        self.call = self.all_numer0n_items[6594];
                    },
                    1 => {
                        self.call = self.all_numer0n_items[7978];
                    },
                    2 => {
                        self.call = self.all_numer0n_items[0127];
                    },
                    _ => panic!("unexpected error."),
                }
            },
            3 => {
                match self.eat {
                    0 => {
                        self.call = self.all_numer0n_items[7798];
                    },
                    1 => {
                        self.call = self.all_numer0n_items[9878];
                    },
                    _ => panic!("unexpected error."),
                }
            },
            4 => {
                match self.eat {
                    0 => {
                        self.call = self.all_numer0n_items[8879];
                    },
                    _ => panic!("unexpected error."),
                }
            },
            _ => panic!("unexpected error."),
        }
    }

    pub fn print_progress(&self, p: usize, deno: usize) {
        let progress = p/200;
        if p == 0 {
            print!(" \x1b[1m\x1b[96mThinking\x1b[0m [                                                  ] {:>3}.0%", 0);
            return;
        } else if p == deno-1 {
            println!("\x1b[2G\x1b[1m\x1b[96mCALL: \x1b[93m{}\x1b[0m                                                               ", self.call);
            return;
        } else if progress > 0 {
            print!("\x1b[{}G=>", 11 + progress);
        }
        print!("\x1b[64G{:>3}.{}", p/(deno/100), p/(deno/1000)%10);
    }

    pub fn set_next_call(&mut self) {
        stdin::hide_cursor();
        if self.cand.len() == 10000 {
            self.print_progress(1, 2);
            return;
        } else if self.call.packed_decimal == packed_decimal::from(9987, 4) {
            self.call_from_branch_table();
            self.print_progress(1, 2);
            return;
        }
        let mut min: usize = 10000;
        'search: for i in 0..10000 { // α-β pruning
            let mut mat: [[usize; 5]; 5] = [[0; 5]; 5];
            for c in &self.cand {
                let eat: usize = c.eat(&self.all_numer0n_items[i]);
                let bite: usize = c.eat_bite(&self.all_numer0n_items[i]) - eat;
                mat[eat][bite] += 1;
            }
            let mut max = 0;
            for k in 0..5 {
                for l in 0..5 {
                    max = std::cmp::max(max, mat[k][l]);
                    if min < max {
                        self.print_progress(i, 10000);
                        continue 'search; // pruning
                    }
                }
            }
            if min > max {
                self.call = self.all_numer0n_items[i];
                min = max;
            } else if min == max && self.cand.contains(&self.all_numer0n_items[i]) {
                self.call = self.all_numer0n_items[i];
            }
            self.print_progress(i, 10000);
        }
    }

    pub fn reduce_cand(&mut self) {
        for i in (0..self.cand.len()).rev() { // If candidates is not erased from behind, it will behave unintentionally.
            let et: usize = self.cand[i].eat(&self.call);
            let bt: usize = self.cand[i].eat_bite(&self.call) - et;
            if self.eat != et || self.bite != bt {
                self.cand.swap_remove(i);
            }
        }
    }
}