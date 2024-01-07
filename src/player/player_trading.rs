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
        self.items[item] >= count
    }

    pub fn trade(&mut self, item: &Item, count: i32, merchant: &mut Merchant) -> Option<i32> {
        let cost = merchant.prices[item] as i32 * count;
        let mut failed_trade = None;
        if self.gold as i32 >= cost {
            self.items.get_mut(item).map(|e| {
                if count >= 0 || (count < 0 && count.abs() <= *e) {
                    *e += count;
                } else {
                    failed_trade = Some(cost - self.gold as i32);
                }
            });
            merchant.items.get_mut(item).map(|e| {
                if count < 0 || (count > 0 && count.abs() <= *e) {
                    *e -= count;
                } else {
                    failed_trade = Some(cost - self.gold as i32);
                }
            });

            if failed_trade.is_some() {
                return failed_trade;
            }

            self.gold = (self.gold as i32 - cost) as u32;
            return None;
        }
        Some(cost - self.gold as i32)
    }
}
