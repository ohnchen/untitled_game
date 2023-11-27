
pub enum Tile {
    Grass,
    Rock,
    Street,
}

impl Tile {
    pub fn draw(&self) -> &str {
        match self {
            Tile::Grass => return "g",
            Tile::Rock => return "r",
            Tile::Street => return "s",
        } 
    } 
}
