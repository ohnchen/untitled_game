pub mod player_farming;
pub mod player_movement;
pub mod player_trading;

use crate::Item;
use crate::Map;
use crate::Tool;

use std::collections::HashMap;

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub tools: Vec<Tool>,
    pub items: HashMap<Item, i32>,
    pub gold: u32,
    pub buying: HashMap<Item, i32>,
}

impl Player {
    pub fn new(map: &Map) -> Self {
        let (x, y) = map.spawnpoint;
        Self {
            x,
            y,
            tools: vec![Tool::Pickaxe, Tool::Hoe, Tool::FishingRod],
            items: HashMap::from([(Item::Rock, 0), (Item::Seed, 0)]),
            gold: 100,
            buying: HashMap::from([(Item::Rock, 0), (Item::Seed, 0)]),
        }
    }
}
