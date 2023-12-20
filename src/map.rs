use crossterm::{
    cursor::{MoveLeft, MoveTo, MoveToRow},
    queue,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal, execute,
};
use perlin_noise::PerlinNoise;
use std::io::{self, Write};

use crate::player::Player;
use crate::tiles::Tile::{self, *};

pub struct Map {
    pub width: u16,
    pub height: u16,
    pub spawnpoint: (u16, u16),
    map_tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(width: u16, height: u16) -> Self {
        let (map_tiles, spawnpoint) = Self::generate_map(width.into(), height.into());
        Self {
            width,
            height,
            spawnpoint: (spawnpoint.0 as u16, spawnpoint.1 as u16),
            map_tiles,
        }
    }

    pub fn get_tile(&self, x: u16, y: u16) -> Tile {
        self.map_tiles[y as usize][x as usize]
    }

    pub fn set_tile(&mut self, x: u16, y: u16, tile: Tile) {
        self.map_tiles[y as usize][x as usize] = tile;
    }

    pub fn mine_option(&self, x: u16, y: u16, t: bool) -> io::Result<()> {
        let to_mine = self.get_tile(x,y);
        execute!(io::stdout(), MoveTo(x,y), PrintStyledContent(to_mine.draw_tile::<&str>(t)))?;
        Ok(())
    }

    pub fn draw_map(&self, xmin: usize, xmax: usize, ymin: usize, ymax: usize) -> io::Result<()> {
        for x in xmin..xmax {
            for y in ymin..ymax {
                let tile = self.map_tiles[y][x];
                queue!(
                    io::stdout(),
                    MoveTo(x as u16, y as u16),
                    PrintStyledContent(tile.draw_tile::<&str>(false))
                )?;
            }
        }

        self.draw_map_border()?;
        Ok(())
    }

    pub fn draw_map_border(&self) -> io::Result<()> {
        let border_row = self.map_tiles.len() as u16;
        queue!(io::stdout(), MoveTo(0, border_row))?;
        for _ in 0..self.map_tiles[0].len() {
            queue!(io::stdout(), Print("─"))?;
        }

        Ok(())
    }

    pub fn draw_player(&self, current_pos: (u16, u16), player: &Player) -> io::Result<()> {
        queue!(
            io::stdout(),
            MoveTo(current_pos.0, current_pos.1),
            PrintStyledContent(
                self.map_tiles[current_pos.1 as usize][current_pos.0 as usize].draw_tile::<&str>(false)
            ),
            MoveTo(player.x, player.y),
            Print('X'),
            MoveLeft(1),
        )?;
        Ok(())
    }

    fn generate_map(width: usize, height: usize) -> (Vec<Vec<Tile>>, (usize, usize)) {
        let mut spawnpoint_set: bool = false;
        let mut spawnpoint: (usize, usize) = (height / 2, width / 2);
        let mut tiles: Vec<Vec<Tile>> = vec![vec![Tile::Empty; width]; height];
        let perl = PerlinNoise::new();
        let scale: f64 = 2.7193;

        for x in 0..width {
            for y in 0..height {
                let n = x as f64 / width as f64 * scale;
                let m = y as f64 / height as f64 * scale;

                let perl = perl.get2d([n, m]);
                if perl > 0.5 {
                    tiles[y][x] = Tile::Rock;
                } else if perl > 0.4 {
                    tiles[y][x] = Tile::Grass;
                    if !spawnpoint_set && x >= width / 3 && y >= height / 2 {
                        spawnpoint = (x, y);
                        spawnpoint_set = true;
                    }
                } else {
                    tiles[y][x] = Tile::Water;
                }
            }
        }
        // tmp: Merchants should also be generated as structures...
        tiles[height-10][width-10] = Tile::Merchant;

        (tiles, spawnpoint)
    }
}
