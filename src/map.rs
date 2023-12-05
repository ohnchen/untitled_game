use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal,
};
use perlin_noise::PerlinNoise;
use std::io::{self, Write};

use crate::tiles::Tile::{self, *};
use crate::player::Player;

pub struct Map {
    pub width: u16,
    pub height: u16,
    map_tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            map_tiles: Self::generate_map(width.into(), height.into()),
        }
    }

    pub fn draw_map(&self) -> io::Result<()> {
        for (x, row) in self.map_tiles.iter().enumerate() {
            for (y, ref tile) in row.iter().enumerate() {
                queue!(
                    io::stdout(),
                    MoveTo(y as u16, x as u16),
                    PrintStyledContent(tile.draw::<&str>())
                )?;
            }
        }

        io::stdout().flush()?;
        Ok(())
    }

    pub fn draw_player(&self, player: &Player) -> io::Result<()> {
        queue!(
            io::stdout(),
            //MoveTo(current_pos.0, current_pos.1),
            //PrintStyledContent(self.map_tiles[current_pos.0 as usize][current_pos.1 as usize].draw::<&str>()),
            MoveTo(player.x, player.y),
            Print('X'),
        )?;
        Ok(())
    }

    fn generate_map(width: usize, height: usize) -> Vec<Vec<Tile>> {
        let mut tiles: Vec<Vec<Tile>> = vec![vec![Tile::Empty; width]; height];
        let perl = PerlinNoise::new();
        let scale: f64 = 2.7193;

        for x in 0..width {
            for y in 0..height {
                let n = x as f64/width as f64 * scale;
                let m = y as f64/height as f64 * scale;

                let perl = perl.get2d([n, m]);
                if perl > 0.5 {
                    tiles[y][x] = Tile::Rock;
                } else if perl > 0.4 {
                    tiles[y][x] = Tile::Grass;
                } else {
                    tiles[y][x] = Tile::Water;
                }
            }
        }

        println!("{}", width);

        tiles
    }
}
