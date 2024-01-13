use crossterm::event::KeyCode;
use crossterm::{cursor::MoveTo, event, execute, style::Print};
use std::io;

use crate::config::*;
use crate::utils::*;
use crate::Merchant;
use crate::Player;
use crate::{map::Map, tiles::Tile};

impl Player {
    pub fn move_direction(
        &mut self,
        map: &mut Map,
        direction: Direction,
        length: usize,
    ) -> io::Result<()> {
        match direction {
            Direction::Left => {
                let new_x = saturated_sub(self.x, length, 0);
                if !self.can_go_dir(&map, Direction::Left)
                    && map.get_tile(new_x, self.y).eq(&Tile::Rock)
                    && self.has_pickaxe()
                {
                    self.mine(map, new_x, self.y)?;
                    return Ok(());
                };
                while self.can_go_dir(&map, Direction::Left) && self.x > new_x {
                    self.x -= 1;
                    if self.x < map.viewleft {
                        map.viewleft = saturated_sub(map.viewleft, map.viewwidth, 0);
                        map.draw_map()?;
                    }
                }
                Ok(())
            }
            Direction::Right => {
                let new_x = saturated_add(self.x, length, map.width - 1);
                if !self.can_go_dir(&map, Direction::Right)
                    && map.get_tile(new_x, self.y).eq(&Tile::Rock)
                    && self.has_pickaxe()
                {
                    self.mine(map, new_x, self.y)?;
                    return Ok(());
                };
                while self.can_go_dir(&map, Direction::Right) && self.x < new_x {
                    self.x += 1;
                    if self.x >= map.viewleft + map.viewwidth {
                        map.viewleft =
                            saturated_add(map.viewleft, map.viewwidth, map.width - map.viewwidth);
                        map.draw_map()?;
                    }
                }
                Ok(())
            }
            Direction::Up => {
                let new_y = saturated_sub(self.y, length, 0);
                if !self.can_go_dir(&map, Direction::Up)
                    && map.get_tile(self.x, new_y).eq(&Tile::Rock)
                    && self.has_pickaxe()
                {
                    self.mine(map, self.x, new_y)?;
                    return Ok(());
                };
                while self.can_go_dir(&map, Direction::Up) && self.y > new_y {
                    self.y -= 1;
                    if self.y < map.viewtop {
                        map.viewtop = saturated_sub(map.viewtop, map.viewheight - MENU_TOP as usize, 0);
                        map.draw_map()?;
                    }
                }
                Ok(())
            }
            Direction::Down => {
                let new_y = saturated_add(self.y, length, map.height - 1);
                if !self.can_go_dir(&map, Direction::Down)
                    && map.get_tile(self.x, new_y).eq(&Tile::Rock)
                    && self.has_pickaxe()
                {
                    self.mine(map, self.x, new_y)?;
                    return Ok(());
                };
                while self.can_go_dir(&map, Direction::Down) && self.y < new_y {
                    self.y += 1;
                    if self.y >= map.viewtop + map.viewheight - MENU_TOP as usize {
                        map.viewtop =
                            saturated_add(map.viewtop, map.viewheight - MENU_TOP as usize, map.height - map.viewheight);
                        map.draw_map()?;
                    }
                }
                Ok(())
            }
        }
    }

    pub fn has_pickaxe(&self) -> bool {
        self.tools.contains(&Tool::Pickaxe)
    }

    pub fn is_on_merchant(&self, map: &Map) -> bool {
        if map.get_tile(self.x, self.y) == Tile::Merchant {
            return true;
        }
        return false;
    }

    pub fn left_merchant(&self, map: &Map, old_pos: (usize, usize)) -> bool {
        if map.get_tile(old_pos.0, old_pos.1) == Tile::Merchant {
            return true;
        }
        return false;
    }

    fn mine(&mut self, map: &mut Map, x: usize, y: usize) -> io::Result<bool> {
        let mut mined = false;
        map.mine_option(x, y, true)?;
        match event::read()? {
            event::Event::Key(key_event) => match key_event.code {
                KeyCode::Char(' ') => {
                    map.set_tile(x, y, Tile::Mine);
                    self.items.get_mut(&Item::Rock).map(|e| {*e += 1});
                    mined = true;
                }
                _ => {}
            },
            _ => {}
        };
        map.mine_option(x, y, false)?;
        Ok(mined)
    }

    fn can_go_dir(&self, map: &Map, dir: Direction) -> bool {
        let tile = match dir {
            Direction::Left => map.get_tile(saturated_sub(self.x, 1, 0), self.y),
            Direction::Right => map.get_tile(saturated_add(self.x, 1, map.width - 1), self.y),
            Direction::Down => map.get_tile(self.x, saturated_add(self.y, 1, map.height - 1)),
            Direction::Up => map.get_tile(self.x, saturated_sub(self.y, 1, 0)),
        };

        !tile.eq(&Tile::Rock) && !tile.eq(&Tile::Water)
    }
}
