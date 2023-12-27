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

    pub fn buys(&mut self, item: Item, cost: u32) {
        self.items = self.items.iter().map(|ele| match (ele, item) {
            (Item::Rock(x), Item::Rock(y)) => Item::Rock(x+y),
            (Item::Seed(x), Item::Seed(y)) => Item::Seed(x+y),
            _ => *ele,
        }).collect();
        self.gold -= cost;
    }

    pub fn sells(&mut self, item: Item, cost: u32) {
        self.items = self.items.iter().map(|ele| match (ele, item) {
            (Item::Rock(x), Item::Rock(y)) => Item::Rock(x-y),
            (Item::Seed(x), Item::Seed(y)) => Item::Seed(x-y),
            _ => *ele,
        }).collect();
        self.gold += cost;
    }
}

    
