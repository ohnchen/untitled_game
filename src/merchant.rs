use crate::utils::*;
use crate::Player;

pub struct Merchant {
    pub gold: u32,
    pub items: Vec<Item>,
}

impl Merchant {
    pub fn new() -> Self {
        Self {
            gold: 1000,
            items: vec![Item::Rock(1000), Item::Seed(500)],
        }
    }

    pub fn is_broke(&self) -> bool {
        self.gold == 0
    }

    pub fn has_item(&mut self, item: &Item) -> bool {
        self.items.iter().any(|x| x.is_more(*item))
    }
}

    
