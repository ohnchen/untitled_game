use crossterm::style::{StyledContent, Stylize, Color};
use std::fmt::Display;


pub enum Tile {
    Grass,
    Rock,
    Street,
}

impl Tile {
    pub fn draw<D>(&self) -> <&str as Stylize>::Styled 
        where D: Display {

        match self {
            Tile::Grass => return "g".with(Color::DarkGreen),
            Tile::Rock => return "r".with(Color::Grey),
            Tile::Street => return "s".with(Color::White),
        } 
    } 
}
