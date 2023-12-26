use crate::utils::*;

pub struct Merchant {
    pub gold: u32,
    pub items: Vec<Items>,
}

impl Merchant {
    pub fn new() -> Self {
        Self {
            gold: 1000,
            items: vec![Items::Rock(1000), Items::Seed(500)],
        }
    }

    pub fn is_broke(&self) -> bool {
        self.gold == 0
    }

    pub fn has_item(&mut self, item: &Items) -> bool {
        self.items.iter().any(|x| x.same(*item))
    }

    pub fn buys(&mut self, item: Items, cost: u32) {
        self.items = self.items.iter().map(|ele| match (ele, item) {
            (Items::Rock(x), Items::Rock(y)) => Items::Rock(x+y),
            (Items::Seed(x), Items::Seed(y)) => Items::Seed(x+y),
            _ => *ele,
        }).collect();
        self.gold -= cost;
    }

    pub fn sells(&mut self, item: Items, cost: u32) {
        self.items = self.items.iter().map(|ele| match (ele, item) {
            (Items::Rock(x), Items::Rock(y)) => Items::Rock(x-y),
            (Items::Seed(x), Items::Seed(y)) => Items::Seed(x-y),
            _ => *ele,
        }).collect();
        self.gold += cost;
    }
}

    
