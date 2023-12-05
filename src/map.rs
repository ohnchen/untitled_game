use crossterm::{
    cursor::{MoveLeft, MoveTo, MoveToRow},
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

    pub fn get_tile(&self, x: u16, y: u16) -> Tile {
        self.map_tiles[y as usize][x as usize]
    }

    pub fn get_tiles(&self, x: u16, y: u16, xto: u16, yto: u16) -> Vec<Tile> {
        let tiles = Vec::new();
        for row in &self.map_tiles[y as usize..yto as usize+1] {
            [tiles.clone(), row[x as usize..xto as usize+1].to_vec()].concat(); 
        }

        dbg!(&tiles);

        tiles
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
    
        self.draw_map_border()?;
        io::stdout().flush()?;
        Ok(())
    }

    pub fn draw_map_border(&self) -> io::Result<()> {
        let row = self.map_tiles.len();
        queue!(io::stdout(), MoveTo(0, row as u16))?;
        for _ in 0..self.map_tiles[0].len() {
            queue!(io::stdout(), Print("─"))?;
        }

        Ok(())
    }

    pub fn draw_player(&self, current_pos: (u16, u16), player: &Player) -> io::Result<()> {
        queue!(
            io::stdout(),
            MoveTo(current_pos.0, current_pos.1),
            PrintStyledContent(self.map_tiles[current_pos.1 as usize][current_pos.0 as usize].draw::<&str>()),
            MoveTo(player.x, player.y),
            Print('X'),
            MoveLeft(1),
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
