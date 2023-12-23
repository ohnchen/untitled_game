use crate::utils::*;

pub trait Trader {
    fn buys_item(&mut self, item: Items);
    fn sells_item(&mut self, item: Items);
    fn is_broke(&self);
}

pub struct Merchant {
    pos: (usize, usize),
    items: Vec<Items>,
    gold: u32,
}

impl Trader for Merchant {

}
