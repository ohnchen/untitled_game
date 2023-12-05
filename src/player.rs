use crate::map::Map;

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
                if self.x as i32 - length as i32 >= 0 {
                    self.x -= length;
                } else {
                    self.x = 0
                }
            }
            Direction::Right => {
                if self.x + length < map.width {
                    self.x += length;
                } else {
                    self.x = map.width - 1
                }
            }
            Direction::Down => {
                if self.y + length < map.height {
                    self.y += length;
                } else {
                    self.y = map.height - 1
                }
            }
            Direction::Up => {
                if self.y as i32 - length as i32 >= 0 {
                    self.y -= length;
                } else {
                    self.y = 0
                }
            }
        }
    }
}
