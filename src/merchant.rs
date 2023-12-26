use crate::utils::*;

pub struct Merchant {
    gold: u32,
    sells: Vec<Items>,
    buys: Vec<Items>,
}

impl Trader for Merchant {
    fn is_broke(&self) -> bool {
        self.gold == 0
    }
    
    fn buys_item(&mut self, item: Items) -> bool {
        self.buys.contains(&item)
    }

    fn sells_item(&mut self, item: Items) -> bool {
        self.sells.contains(&item)
    }
}
