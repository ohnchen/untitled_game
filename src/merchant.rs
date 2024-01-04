use std::collections::HashMap;

use crate::config::*;
use crate::utils::*;
use crate::Player;

pub struct Merchant {
    pub gold: u32,
    pub prices: HashMap<Item, u32>,
    pub items: HashMap<Item, i32>,
}

impl Merchant {
    pub fn new() -> Self {
        Self {
            gold: 1000,
            prices: HashMap::from([(Item::Rock, ROCK_PRICE), (Item::Seed, SEED_PRICE)]),
            items: HashMap::from([(Item::Rock, 1000), (Item::Seed, 500)]),
        }
    }

    pub fn has_item(&mut self, item: &Item, count: i32) -> bool {
        self.items[item] >= count
    }

    pub fn get_price(&self, item: &Item, count: i32) -> i32 {
        self.prices[item] as i32 * count 
    }
}

    
