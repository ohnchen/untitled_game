use crate::map::Map;

pub enum Direction {
    Left,
    Right,
    Up,
    Down
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

    pub fn move_direction(&mut self, direction: Direction, length: u16) {
        // [TODO] put in checks
        match direction {
            Direction::Left => self.x -= length,
            Direction::Right => self.x += length,
            Direction::Down => self.y += length,
            Direction::Up => self.y -= length,
        } 
    }
}
