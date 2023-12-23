use crossterm::style::{Color, StyledContent, Stylize};
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Grass,
    Water,

    Rock,
    Mine,

    Merchant,
    
    Empty,
}

impl Tile {
    pub fn draw_tile<D>(&self, red: bool) -> <&str as Stylize>::Styled
    where
        D: Display,
    {
        if red { return "\"".with(Color::Red) }
        match self {
            Tile::Grass => return "g".with(Color::DarkGreen),
            Tile::Water => return "w".with(Color::Blue),
            Tile::Merchant => return "M".with(Color::DarkYellow),
            Tile::Rock => return "r".with(Color::Grey),
            Tile::Mine => return "o".with(Color::DarkGrey), 
            _ => return "e".with(Color::DarkGrey),
        }
    }
}
