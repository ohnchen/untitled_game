use crate::config::*;
use crate::utils::*;
use crate::Player;

pub struct Merchant {
    pub gold: u32,
    pub prices: Vec<u32>,
    pub items: Vec<Item>,
}

impl Merchant {
    pub fn new() -> Self {
        Self {
            gold: 1000,
            prices: vec![ROCK_PRICE, SEED_PRICE],
            items: vec![Item::Rock(1000), Item::Seed(500)],
        }
    }

    pub fn has_item(&mut self, item: &Item) -> bool {
        self.items.iter().any(|x| x.is_more(*item))
    }

    pub fn get_price(&self, item: &Item) -> i32 {
        match item {
            Item::Rock(x) => x * self.prices[0] as i32,
            Item::Seed(x) => x * self.prices[1] as i32,
        }
    }
}

    
