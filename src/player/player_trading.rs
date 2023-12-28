use crate::{Map, Item, Merchant, Player};
use crate::config::*;

impl Player {
    pub fn reset_buying(&mut self) {
        self.buying = self
            .buying
            .iter()
            .map(|e| match e {
                Item::Rock(_) => Item::Rock(0),
                Item::Seed(_) => Item::Seed(0),
            })
            .collect();
    }

    pub fn has_money(&self, cost: i32) -> bool {
        self.gold as i32 >= cost
    }

    pub fn has_item(&mut self, item: &Item) -> bool {
        self.items.iter().any(|x| x.is_more(*item))
    }

    pub fn trade(&mut self, item: Item, _merchant: &Merchant) -> Option<i32> {
        let mut cost: i32 = 0;
        self.items = self
            .items
            .iter()
            .map(|ele| match (ele, item) {
                (Item::Rock(x), Item::Rock(y)) => {
                    if x + y * ROCK_PRICE as i32 >= 0 {
                        cost += y * ROCK_PRICE as i32;
                        Item::Rock(x + y)
                    } else {
                        *ele
                    }
                }
                (Item::Seed(x), Item::Seed(y)) => {
                    if x + y * SEED_PRICE as i32 >= 0 {
                        cost += y * SEED_PRICE as i32;
                        Item::Seed(x + y)
                    } else {
                        *ele
                    }
                }
                _ => *ele,
            })
            .collect();
        // merchant.items = merchant.items.iter().map(|ele| match (ele, item) {
        //     (Item::Rock(x), Item::Rock(y)) => Item::Rock(x+y),
        //     (Item::Seed(x), Item::Seed(y)) => Item::Seed(x+y),
        //     _ => *ele,
        // }).collect();

        if self.gold as i32 - cost >= 0 {
            self.gold = (self.gold as i32 - cost) as u32;
            None
        } else {
            Some(cost - self.gold as i32)
        }
    }
}
