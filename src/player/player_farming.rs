use crate::config::*;
use crate::tiles::Tile;
use crate::{Item, Map, Merchant, Player};

impl Player {
    pub fn plant_seeds(&mut self, map: &mut Map, pos: (usize, usize)) {
        if !map.is_near_water(pos.0, pos.1) && self.has_item(&Item::Seed(1)){
            return;
        }
        map.set_tile(pos.0, pos.1, Tile::Crop);
    }
}
