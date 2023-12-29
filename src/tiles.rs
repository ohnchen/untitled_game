use crossterm::style::{Color, StyledContent, Stylize};
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Grass,
    Water,

    Rock,
    Mine,

    Merchant,

    // Soil,
    // WateredSoil,
    Crop,
    
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
            Tile::Mine => return "#".with(Color::White), 
            Tile::Crop => return "v".with(Color::Yellow), 
            _ => return "e".with(Color::DarkGrey),
        }
    }
}
