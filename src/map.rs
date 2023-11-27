use crossterm::{cursor::MoveTo, queue, style::{Print, PrintStyledContent, Stylize, Color}};

use crate::tiles::Tile::{self, *};

use std::io::{self, Write};

pub struct Map {
    map_tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            map_tiles: vec![vec![Grass, Street], vec![Rock, Grass]],
        }
    }

    pub fn draw(&self, pos: (u16, u16)) -> io::Result<()> {
        for (x, row) in self.map_tiles.iter().enumerate() {
            for (y, ref tile) in row.iter().enumerate() {
                if pos.0 as usize == x && pos.1 as usize == y { continue }
                queue!(io::stdout(), MoveTo(x as u16, y as u16), PrintStyledContent(tile.draw::<&str>()))?;
            }
        }

        queue!(io::stdout(), MoveTo(pos.0, pos.1))?;

        io::stdout().flush()?;
        Ok(())
    }
}
