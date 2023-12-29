pub mod player_movement;
pub mod player_trading;
pub mod player_farming;

use crate::Map;
use crate::Item;
use crate::Tool;

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub tools: Vec<Tool>,
    pub items: Vec<Item>,
    pub gold: u32,
    pub buying: Vec<Item>, 
}

impl Player {
    pub fn new(map: &Map) -> Self {
        let (x, y) = map.spawnpoint;
        Self {
            x,
            y,
            tools: vec![
                Tool::Pickaxe,
                Tool::Hoe,
                Tool::FishingRod,
            ],
            items: vec![
                Item::Rock(0),
                Item::Seed(0),
            ],
            gold: 100,
            buying: vec![
                Item::Rock(0),
                Item::Seed(0),
            ],
        }
    }
}
