use crossterm::style::{Color, StyledContent, Stylize};
use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone)]
pub enum Tile {
    Grass,
    Rock,
    Water,
    Street,

    Empty,
}

impl Tile {
    pub fn draw<D>(&self) -> <&str as Stylize>::Styled
    where
        D: Display,
    {
        match self {
            Tile::Grass => return "g".with(Color::DarkGreen),
            Tile::Rock => return "r".with(Color::Blue),
            Tile::Street => return "s".with(Color::White),
            _ => return "e".with(Color::DarkGrey),
        }
    }
}
