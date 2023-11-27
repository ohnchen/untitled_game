use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal,
};
use std::io::{self, Write};

use crate::tiles::Tile::{self, *};

pub struct Map {
    map_tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            map_tiles: Self::generate_map(width, height),
        }
    }

    pub fn draw_map(&self) -> io::Result<()> {
        for (x, row) in self.map_tiles.iter().enumerate() {
            for (y, ref tile) in row.iter().enumerate() {
                queue!(
                    io::stdout(),
                    MoveTo(x as u16, y as u16),
                    PrintStyledContent(tile.draw::<&str>())
                )?;
            }
        }

        io::stdout().flush()?;
        Ok(())
    }

    pub fn draw_update(&self, old_pos: (u16, u16), pos: (u16, u16)) -> io::Result<()> {
        if (pos.0 == 0 && old_pos.0 == 0) && (pos.1 == old_pos.1)
            || (pos.0 >= terminal::size()?.0 - 1 && old_pos.0 == terminal::size()?.0 - 1)
                && (pos.1 == old_pos.1)
            || (pos.1 == 0 && old_pos.1 == 0) && (pos.0 == old_pos.0)
            || (pos.1 >= terminal::size()?.1 - 1 && old_pos.1 == terminal::size()?.1 - 1)
                && (pos.0 == old_pos.0)
        {
            return Ok(());
        }
        queue!(
            io::stdout(),
            MoveTo(old_pos.0, old_pos.1),
            PrintStyledContent(
                self.map_tiles[old_pos.0 as usize][old_pos.1 as usize].draw::<&str>()
            ),
            MoveTo(pos.0, pos.1),
        )?;
        Ok(())
    }

    fn generate_map(width: usize, height: usize) -> Vec<Vec<Tile>> {
        let mut noise: Vec<Vec<Tile>> = vec![vec![Tile::Empty; width]; height];
        noise[10][10] = Tile::Rock;
        noise
    }
}
