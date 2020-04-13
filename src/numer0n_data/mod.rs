use super::bit_table;
use super::packed_decimal;
use super::stdin;

mod numer0n_item;
mod numer0n_items;

pub struct Numer0nData {
    cand: numer0n_items::Numer0nItems,
    all_numer0n_items: numer0n_items::Numer0nItems,
    pub call: numer0n_item::Numer0nItem,
    pub eat: usize,
    pub bite: usize,
}
impl Numer0nData {
    pub fn new() -> Self {
        let all_numer0n_items = numer0n_items::Numer0nItems((0..10000).map(|i|  numer0n_item::Numer0nItem::from(i)).collect());
        println!("\x1b[1G\x1b[1m\x1b[92mInitialized\x1b[0m");
        Numer0nData {
            cand: all_numer0n_items.clone(),
            all_numer0n_items: all_numer0n_items,
            call: numer0n_item::Numer0nItem::from(9987),
            eat: 0,
            bite: 0,
        }
    }

    pub fn call_from_branch_table(&mut self) {
        match self.bite {
            0 => {
                match self.eat {
                    0 => {
                        self.call = numer0n_item::Numer0nItem::from(3345);
                    },
                    1 => {
                        self.call = numer0n_item::Numer0nItem::from(3415);
                    },
                    2 => {
                        self.call = numer0n_item::Numer0nItem::from(0345);
                    },
                    3 => {
                        self.call = numer0n_item::Numer0nItem::from(3415);
                    },
                    _ => panic!("unexpected error."),
                }
            },
            1 => {
                match self.eat {
                    0 => {
                        self.call = numer0n_item::Numer0nItem::from(1134);
                    },
                    1 => {
                        self.call = numer0n_item::Numer0nItem::from(0304);
                    },
                    2 => {
                        self.call = numer0n_item::Numer0nItem::from(0113);
                    },
                    3 => {
                        self.call = numer0n_item::Numer0nItem::from(0345);
                    },
                    _ => panic!("unexpected error."),
                }
            },
            2 => {
                match self.eat {
                    0 => {
                        self.call = numer0n_item::Numer0nItem::from(3405);
                    },
                    1 => {
                        self.call = numer0n_item::Numer0nItem::from(0121);
                    },
                    2 => {
                        self.call = numer0n_item::Numer0nItem::from(0345);
                    },
                    _ => panic!("unexpected error."),
                }
            },
            3 => {
                match self.eat {
                    0 => {
                        self.call = numer0n_item::Numer0nItem::from(1120);
                    },
                    1 => {
                        self.call = numer0n_item::Numer0nItem::from(0121);
                    },
                    _ => panic!("unexpected error."),
                }
            },
            4 => {
                match self.eat {
                    0 => {
                        self.call = numer0n_item::Numer0nItem::from(1120);
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
        print!("\x1b[64G{:>3}.{}", p/(deno/100), p/10);
    }

    pub fn set_next_call(&mut self) {
        stdin::hide_cursor();
        if self.cand.0.len() == 10000 {
            self.print_progress(1, 2);
            return;
        } else if self.call.packed_decimal == packed_decimal::i32_to_packed_decimal(9987) {
            self.call_from_branch_table();
            self.print_progress(1, 2);
            return;
        }
        let mut min: usize = 10000;
        'search: for i in 0..10000 {
            let mut mat: [[usize; 5]; 5] = [[0; 5]; 5];
            for c in &self.cand.0 {
                let eat: usize = c.eat(&self.all_numer0n_items.0[i]);
                let bite: usize = c.eat_bite(&self.all_numer0n_items.0[i]) - eat;
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
                self.call = self.all_numer0n_items.0[i];
                min = max;
            } else if min == max && self.cand.0.contains(&self.all_numer0n_items.0[i]) {
                self.call = self.all_numer0n_items.0[i];
            }
            self.print_progress(i, 10000);
        }
    }

    pub fn reduce_cand(&mut self) {
        for i in (0..self.cand.0.len()).rev() { // If candidates is not erased from behind, it will behave unintentionally.
            let et: usize = self.cand.0[i].eat(&self.call);
            let bt: usize = self.cand.0[i].eat_bite(&self.call) - et;
            if self.eat != et || self.bite != bt {
                self.cand.0.swap_remove(i);
            }
        }
    }
}