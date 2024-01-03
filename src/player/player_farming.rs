use crate::config::*;
use crate::tiles::Tile;
use crate::{Item, Map, Merchant, Player};

impl Player {
    pub fn plant_seeds(&mut self, map: &mut Map) {
        if map.is_near_water(self.x, self.y) && self.has_item(&Item::Seed(1)){
            // remove seed from inv
            map.set_tile(self.x, self.y, Tile::Crop);
        }
    }
}
