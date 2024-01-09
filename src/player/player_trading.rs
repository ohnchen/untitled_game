use crate::config::*;
use crate::{Item, Map, Merchant, Player};

impl Player {
    pub fn reset_buying(&mut self) {
        for key in self.buying.clone().keys() {
            self.buying.insert(*key, 0);
        }
    }

    pub fn has_money(&self, cost: i32) -> bool {
        self.gold as i32 >= cost
    }

    pub fn has_item(&mut self, item: &Item, count: i32) -> bool {
        self.items[item] >= count.abs()
    }

    pub fn trade(&mut self, item: &Item, count: i32, merchant: &mut Merchant) -> Option<i32> {
        let cost = merchant.prices[item] as i32 * count;
        if (cost < 0 && self.has_item(item, count))
            || (self.has_money(cost) && cost > 0 && merchant.has_item(item, count))
        {
            self.items.get_mut(item).map(|e| *e += count);
            merchant.items.get_mut(item).map(|e| *e -= count);

            self.gold = (self.gold as i32 - cost) as u32;
            return None;
        }
        Some(cost - self.gold as i32)
    }
}
