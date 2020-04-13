use super::bit_table;
use super::packed_decimal;

mod numer0n_item;
mod numer0n_items;

pub enum DisplayMode {
    #[allow(dead_code)] On,
    #[allow(dead_code)] Off,
}

pub struct Numer0nData {
    cand: numer0n_items::Numer0nItems,
    all_numer0n_items: numer0n_items::Numer0nItems,
    pub guess: numer0n_item::Numer0nItem,
    pub eat: usize,
    pub bite: usize,
    display_mode: DisplayMode,
}
impl Numer0nData {
    pub fn new(display_mode: DisplayMode) -> Self {
        Numer0nData {
            cand: numer0n_items::Numer0nItems((0..10000).map(|i| numer0n_item::Numer0nItem::from(i)).collect()),
            all_numer0n_items: numer0n_items::Numer0nItems((0..10000).map(|i| numer0n_item::Numer0nItem::from(i)).collect()),
            guess: numer0n_item::Numer0nItem::from(0012),
            eat: 0,
            bite: 0,
            display_mode: display_mode,
        }
    }

    pub fn guess_from_branch_table(&mut self) {
        match self.bite {
            0 => {
                match self.eat {
                    0 => {
                        self.guess = numer0n_item::Numer0nItem::from(3345);
                    },
                    1 => {
                        self.guess = numer0n_item::Numer0nItem::from(3415);
                    },
                    2 => {
                        self.guess = numer0n_item::Numer0nItem::from(0345);
                    },
                    3 => {
                        self.guess = numer0n_item::Numer0nItem::from(3415);
                    },
                    _ => panic!("unexpected error."),
                }
            },
            1 => {
                match self.eat {
                    0 => {
                        self.guess = numer0n_item::Numer0nItem::from(1134);
                    },
                    1 => {
                        self.guess = numer0n_item::Numer0nItem::from(0304);
                    },
                    2 => {
                        self.guess = numer0n_item::Numer0nItem::from(0113);
                    },
                    3 => {
                        self.guess = numer0n_item::Numer0nItem::from(0345);
                    },
                    _ => panic!("unexpected error."),
                }
            },
            2 => {
                match self.eat {
                    0 => {
                        self.guess = numer0n_item::Numer0nItem::from(3405);
                    },
                    1 => {
                        self.guess = numer0n_item::Numer0nItem::from(0121);
                    },
                    2 => {
                        self.guess = numer0n_item::Numer0nItem::from(0345);
                    },
                    _ => panic!("unexpected error."),
                }
            },
            3 => {
                match self.eat {
                    0 => {
                        self.guess = numer0n_item::Numer0nItem::from(1120);
                    },
                    1 => {
                        self.guess = numer0n_item::Numer0nItem::from(0121);
                    },
                    _ => panic!("unexpected error."),
                }
            },
            4 => {
                match self.eat {
                    0 => {
                        self.guess = numer0n_item::Numer0nItem::from(1120);
                    },
                    _ => panic!("unexpected error."),
                }
            },
            _ => panic!("unexpected error."),
        }
    }

    pub fn set_next_guess(&mut self) {
        if let DisplayMode::On = self.display_mode {
            println!("{}", self.cand);
        }
        if self.cand.0.len() == 10000 {
            return;
        } else if self.cand.0.len() <= 2 {
            self.guess =  self.cand.0[0];
            return;
        } else if self.guess.packed_decimal == packed_decimal::i32_to_packed_decimal(0012) {
            self.guess_from_branch_table();
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
                    if min < max {
                        continue 'search; // pruning
                    }
                }
            }
            if min > max {
                self.guess = *guess;
                min = max;
            } else if min == max && self.cand.0.contains(guess) {
                self.guess = *guess;
            }
        }
    }

    pub fn reduce_cand(&mut self) {
        for i in (0..self.cand.0.len()).rev() { // If candidates is not erased from behind, it will behave unintentionally.
            let et: usize = self.cand.0[i].eat(&self.guess);
            let bt: usize = self.cand.0[i].eat_bite(&self.guess) - et;
            if self.eat != et || self.bite != bt {
                self.cand.0.swap_remove(i);
            }
        }
    }
}