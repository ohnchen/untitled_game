use crossterm::style::{Color, StyledContent, Stylize};
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Grass,
    Rock,
    Water,

    Empty,
}

impl Tile {
    pub fn draw<D>(&self) -> <&str as Stylize>::Styled
    where
        D: Display,
    {
        match self {
            Tile::Grass => return "g".with(Color::DarkGreen),
            Tile::Rock => return "r".with(Color::Grey),
            Tile::Water => return "w".with(Color::Blue),
            _ => return "e".with(Color::DarkGrey),
        }
    }
}
