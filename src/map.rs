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
    pub viewleft: u16,
    pub viewtop: u16,
    pub viewwidth: u16,
    pub viewheight: u16,
    pub width: u16,
    pub height: u16,
    pub spawnpoint: (u16, u16),
    map_tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(viewleft: u16, viewtop: u16, viewwidth: u16, viewheight: u16) -> Self {
        let map_tiles = Self::generate_map();
        Self {
            viewleft,  
            viewtop,
            viewwidth,
            viewheight,
            width: 1000,
            height: 1000,
            spawnpoint: (500, 500),
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
        execute!(io::stdout(), MoveTo(x - self.viewleft, y - self.viewtop), PrintStyledContent(to_mine.draw_tile::<&str>(t)))?;
        Ok(())
    }

    pub fn draw_map(&self) -> io::Result<()> {
        for x in self.viewleft..self.viewleft + self.viewwidth {
            for y in self.viewtop..self.viewtop + self.viewheight {
                let tile = self.map_tiles[y as usize][x as usize];
                queue!(
                    io::stdout(),
                    MoveTo(x - self.viewleft, y - self.viewtop),
                    PrintStyledContent(tile.draw_tile::<&str>(false))
                )?;
            }
        }

        self.draw_map_border()?;
        Ok(())
    }

    pub fn draw_map_border(&self) -> io::Result<()> {
        queue!(io::stdout(), MoveTo(0, self.viewheight))?;
        for _ in 0..terminal::size()?.0 {
            queue!(io::stdout(), Print("─"))?;
        }

        Ok(())
    }

    pub fn draw_player(&self, current_pos: (u16, u16), player: &Player) -> io::Result<()> {
        queue!(
            io::stdout(),
            MoveTo(current_pos.0-self.viewleft, current_pos.1-self.viewtop),
            PrintStyledContent(
                self.map_tiles[current_pos.1 as usize][current_pos.0 as usize].draw_tile::<&str>(false)
            ),
            MoveTo(player.x - self.viewleft, player.y-self.viewtop),
            Print('X'),
            MoveLeft(1),
        )?;
        Ok(())
    }

    fn generate_map() -> Vec<Vec<Tile>> {
        let width = 1000;
        let height = 1000;
        let mut tiles: Vec<Vec<Tile>> = vec![vec![Tile::Empty; width]; height];
        let perl = PerlinNoise::new();
        let scale: f64 = 37.7193;

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
        // tmp: Merchants should also be generated as structures...
        // tiles[height-10][width-10] = Tile::Merchant;

        tiles 
    }
}
