use crate::utils::*;

pub trait Trader {
    fn buys_item(&mut self, item: Items);
    fn sells_item(&mut self, item: Items);
    fn is_broke(&self);
}
