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
                let newx = Self::saturated_sub(self.x, length);
                while !map.get_tile(Self::saturated_sub(self.x, 1), self.y).eq(&Tile::Rock) && self.x > newx {
                    self.x -= 1;
                }
            }
            Direction::Right => {
                let newx = Self::saturated_add(self.x, length, map.width);
                if !map.get_tile(Self::saturated_add(self.x, 1, map.width), self.y).eq(&Tile::Rock) && self.x < newx{
                    self.x += 1;
                }
            }
            Direction::Up => {
                let newy = Self::saturated_sub(self.y, length);
                if !map.get_tile(self.x, Self::saturated_sub(self.y, 1)).eq(&Tile::Rock) && self.y > newy {
                    self.y -= 1;
                }
            }
            Direction::Down => {
                let newy = Self::saturated_add(self.y, length, map.height);
                if !map.get_tile(self.x,  Self::saturated_add(self.y, 1, map.height)).eq(&Tile::Rock) && self.y < newy {
                    self.y += 1;
                }
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
