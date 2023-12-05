use crate::{map::Map, tiles::Tile};

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Player {
    pub x: u16,
    pub y: u16,
}

impl Player {
    pub fn new(map: &Map) -> Self {
        Self {
            x: map.width / 2,
            y: map.height / 2,
        }
    }

    pub fn move_direction(&mut self, map: &Map, direction: Direction, length: u16) {
        match direction {
            Direction::Left => {
                let newx = Self::saturated_sub(self.x, length);
                if !map.get_tile(newx, self.y).eq(&Tile::Rock) {
                    self.x = newx
                }
                //if !map.get_tiles(self.x, self.y, newx, self.y).contains(&Tile::Rock) { self.x = newx }
            }
            Direction::Right => {
                let newx = Self::saturated_add(self.x, length, map.width);
                if !map.get_tile(newx, self.y).eq(&Tile::Rock) {
                    self.x = newx
                }
                //if !map.get_tiles(self.x, self.y, newx, self.y).contains(&Tile::Rock) { self.x = newx }
            }
            Direction::Up => {
                let newy = Self::saturated_sub(self.y, length);
                if !map.get_tile(self.x, newy).eq(&Tile::Rock) {
                    self.y = newy
                }
                //if !map.get_tiles(self.x, self.y, newx, self.y).contains(&Tile::Rock) { self.x = newx }
            }
            Direction::Down => {
                let newy = Self::saturated_add(self.y, length, map.height);
                if !map.get_tile(self.x, newy).eq(&Tile::Rock) {
                    self.y = newy
                }
                //if !map.get_tiles(self.x, self.y, newx, self.y).contains(&Tile::Rock) { self.x = newx }
            }
        }
    }

    fn saturated_sub(op1: u16, op2: u16) -> u16 {
        let diff: i32 = op1 as i32 - op2 as i32;
        if diff < 0 {
            return 0;
        } else {
            return diff as u16;
        }
    }

    fn saturated_add(op1: u16, op2: u16, max: u16) -> u16 {
        let sum: u16 = op1 + op2;
        if sum >= max {
            return max - 1;
        } else {
            return sum as u16;
        }
    }
}
