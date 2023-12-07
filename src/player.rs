use crate::utils::*;
use crate::{map::Map, tiles::Tile};

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq)]
pub enum Tools {
    Pickaxe,
}

pub struct Player {
    pub x: u16,
    pub y: u16,
    pub tools: Vec<Tools>,
}

impl Player {
    pub fn new(map: &Map) -> Self {
        Self {
            x: map.spawnpoint.0 as u16,
            y: map.spawnpoint.1 as u16,
            tools: Vec::new(),
        }
    }

    pub fn move_direction(&mut self, map: &Map, direction: Direction, length: u16) {
        match direction {
            Direction::Left => {
                while self.can_go_left(&map, &Tile::Rock) && self.x > saturated_sub(self.x, length) {
                    self.x -= 1;
                }
            },
            Direction::Right => {
                while self.can_go_right(&map, &Tile::Rock) && self.x < saturated_sub(self.x, length) {
                    self.x += 1;
                }
            }, 
            Direction::Up => {
                while self.can_go_up(&map, &Tile::Rock) && self.x > saturated_sub(self.x, length) {
                    self.x -= 1;
                }
            }, 
            Direction::Down => {
                while self.can_go_down(&map, &Tile::Rock) && self.x < saturated_sub(self.x, length) {
                    self.x -= 1;
                }
            },
        }
    }

    fn can_go_left(&self, map: &Map, block_tile: &Tile) -> bool {
        !map.get_tile(saturated_sub(self.x, 1), self.y)
            .eq(block_tile)
    }
    
    fn can_go_right(&self, map: &Map, block_tile: &Tile) -> bool {
        !map.get_tile(saturated_add(self.x, 1, map.width), self.y)
            .eq(block_tile)
    }
    
    fn can_go_up(&self, map: &Map, block_tile: &Tile) -> bool {
        !map.get_tile(self.x, saturated_sub(self.y, 1))
            .eq(block_tile)
    }

    fn can_go_down(&self, map: &Map, block_tile: &Tile) -> bool {
        !map.get_tile(self.x, saturated_add(self.x, 1, map.width))
            .eq(block_tile)
    }
}
