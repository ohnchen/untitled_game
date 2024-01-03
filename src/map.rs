use crossterm::{
    cursor::{MoveLeft, MoveTo, MoveToRow},
    execute, queue,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal,
};
use perlin_noise::PerlinNoise;
use std::io::{self, Write};

use crate::player::Player;
use crate::tiles::Tile::{self, *};
use crate::utils::*;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub viewleft: usize,
    pub viewtop: usize,
    pub viewwidth: usize,
    pub viewheight: usize,
    pub spawnpoint: (usize, usize),
    map_tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(
        width: usize,
        height: usize,
        viewleft: usize,
        viewtop: usize,
        viewwidth: usize,
        viewheight: usize,
    ) -> Self {
        let map_tiles = Self::generate_map(width.into(), height.into());
        Self {
            width,
            height,
            viewleft,
            viewtop,
            viewwidth,
            viewheight,
            spawnpoint: (viewleft as usize + 20, viewtop as usize + 20),
            map_tiles,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.map_tiles[y][x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.map_tiles[y][x] = tile;
    }

    pub fn is_near_water(&self, x: usize, y: usize) -> bool {
        // saturated
        for i in x-2..x+2 {
            for j in y-2..y+2 {
                if self.get_tile(i, j) == Tile::Water {
                    return true;
                }
            }
        }
        false
    }

    pub fn mine_option(&self, x: usize, y: usize, t: bool) -> io::Result<()> {
        let to_mine = self.get_tile(x, y);
        execute!(
            io::stdout(),
            MoveTo((x - self.viewleft as usize) as u16, (y - self.viewtop as usize) as u16),
            PrintStyledContent(to_mine.draw_tile::<&str>(t))
        )?;
        Ok(())
    }

    pub fn draw_map(&self) -> io::Result<()> {
        for x in self.viewleft..self.viewleft + self.viewwidth as usize {
            for y in self.viewtop..self.viewtop + self.viewheight as usize {
                let tile = self.map_tiles[y][x];
                queue!(
                    io::stdout(),
                    MoveTo((x - self.viewleft) as u16, (y - self.viewtop) as u16),
                    PrintStyledContent(tile.draw_tile::<&str>(false))
                )?;
            }
        }

        self.draw_map_border()?;
        Ok(())
    }

    pub fn draw_map_border(&self) -> io::Result<()> {
        queue!(io::stdout(), MoveTo(0, self.viewheight as u16))?;
        for _ in 0..terminal::size()?.0 {
            queue!(io::stdout(), Print("─"))?;
        }

        Ok(())
    }

    pub fn draw_player(&self, current_pos: (usize, usize), player: &Player) -> io::Result<()> {
        queue!(
            io::stdout(),
            MoveTo((saturated_sub(current_pos.0, self.viewleft, 0)) as u16, {
                if (saturated_sub(current_pos.1, self.viewtop, 0) as u16) < self.viewheight as u16 {
                    saturated_sub(current_pos.1, self.viewtop, 0) as u16
                } else {
                    self.viewheight as u16 - 1 
                }
            }),
            PrintStyledContent(
                self.map_tiles[current_pos.1 as usize][current_pos.0 as usize]
                    .draw_tile::<&str>(false)
            ),
            MoveTo((player.x - self.viewleft) as u16, (player.y - self.viewtop) as u16),
            Print('X'),
            MoveLeft(1),
        )?;
        Ok(())
    }

    fn generate_map(width: usize, height: usize) -> Vec<Vec<Tile>> {
        let mut tiles: Vec<Vec<Tile>> = vec![vec![Tile::Empty; width]; height];
        let perl = PerlinNoise::new();
        let scale: f64 = width as f64 / 42.7193;

        for x in 0..width {
            for y in 0..height {
                let n = x as f64 / width as f64 * scale;
                let m = y as f64 / height as f64 * scale;

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

        //generate_merchants(width, height);
        tiles[250][250] = Tile::Merchant;

        tiles
    }
}
