use crossterm::event::KeyCode;
use crossterm::{cursor::MoveTo, event, execute, style::Print};
use std::io;

use crate::utils::*;
use crate::{map::Map, tiles::Tile};

pub struct Player {
    pub x: u16,
    pub y: u16,
    pub tools: Vec<Tools>,
    pub items: Vec<Item>,
    pub gold: u32,
}

impl Player {
    pub fn new(map: &Map) -> Self {
        let (x, y): (u16, u16) = map.spawnpoint;
        Self {
            x,
            y,
            tools: vec![],
            items: vec![
                Item::Rock(0),
                Item::Seed(0),
            ],
            gold: 100,
        }
    }

    pub fn has_pickaxe(&self) -> bool {
        self.tools.contains(&Tools::Pickaxe) 
    }

    pub fn move_direction(
        &mut self,
        map: &mut Map,
        direction: Direction,
        length: u16,
    ) -> io::Result<()> {
        match direction {
            Direction::Left => {
                let new_x = saturated_sub(self.x, length, 0);
                if !self.can_go_left(&map, &Tile::Rock) && self.has_pickaxe() {
                    self.mine(map, new_x, self.y)?;
                    return Ok(());
                };
                while (self.can_go_left(&map, &Tile::Rock)) && self.x > new_x {
                    self.x -= 1;
                    if self.x < map.viewleft {
                        map.viewleft = saturated_sub(map.viewleft, map.viewwidth, 0);    
                        map.draw_map()?;
                    }
                }
                Ok(())
            }
            Direction::Right => {
                let new_x = saturated_add(self.x, length, map.width-1);
                if !self.can_go_right(&map, &Tile::Rock) && self.has_pickaxe() {
                    self.mine(map, new_x, self.y)?;
                    return Ok(());
                };
                while (self.can_go_right(&map, &Tile::Rock)) && self.x < new_x {
                    self.x += 1;
                    if self.x >= map.viewleft + map.viewwidth {
                        map.viewleft = saturated_add(map.viewleft, map.viewwidth, map.width-map.viewwidth);    
                        map.draw_map()?;
                    }
                }
                Ok(())
            }
            Direction::Up => {
                let new_y = saturated_sub(self.y, length, 0);
                if !self.can_go_up(&map, &Tile::Rock) && self.has_pickaxe() {
                    self.mine(map, self.x, new_y)?;
                    return Ok(());
                };
                while (self.can_go_up(&map, &Tile::Rock)) && self.y > new_y {
                    self.y -= 1;
                    if self.y < map.viewtop {
                        map.viewtop = saturated_sub(map.viewtop, map.viewheight, 0);    
                        map.draw_map()?;
                    }
                }
                Ok(())
            }
            Direction::Down => {
                let new_y = saturated_add(self.y, length, map.height-1);
                if !self.can_go_down(&map, &Tile::Rock) && self.has_pickaxe() {
                    self.mine(map, self.x, new_y)?;
                    return Ok(());
                };
                while (self.can_go_down(&map, &Tile::Rock)) && self.y < new_y {
                    self.y += 1;
                    if self.y >= map.viewtop + map.viewheight {
                        map.viewtop = saturated_add(map.viewtop, map.viewheight, map.height-map.viewheight);    
                        map.draw_map()?;
                    }
                }
                Ok(())
            }
        }
    }

    pub fn is_on_merchant(&self, map: &Map) -> bool {
        if map.get_tile(self.x, self.y) == Tile::Merchant {
            return true;
        }
        return false;
    }

    fn mine(&mut self, map: &mut Map, x: u16, y: u16) -> io::Result<bool> {
        let mut mined = false;
        map.mine_option(x, y, true)?;
        match event::read()? {
            event::Event::Key(key_event) => match key_event.code {
                KeyCode::Char(' ') => {
                    map.set_tile(x, y, Tile::Mine);
                    self.items = self.items.iter().map(|r| {match r {
                        Item::Rock(num) => Item::Rock(num + 1),
                        Item::Seed(i) => Item::Seed(*i),
                    }}).collect();
                    mined = true;
                }
                _ => {}
            },
            _ => {}
        };
        map.mine_option(x, y, false)?;
        Ok(mined)
    }

    fn can_go_left(&self, map: &Map, block_tile: &Tile) -> bool {
        !map.get_tile(saturated_sub(self.x, 1, 0), self.y)
            .eq(block_tile)
    }

    fn can_go_right(&self, map: &Map, block_tile: &Tile) -> bool {
        !map.get_tile(saturated_add(self.x, 1, map.width-1), self.y)
            .eq(block_tile)
    }

    fn can_go_up(&self, map: &Map, block_tile: &Tile) -> bool {
        !map.get_tile(self.x, saturated_sub(self.y, 1, 0))
            .eq(block_tile)
    }

    fn can_go_down(&self, map: &Map, block_tile: &Tile) -> bool {
        !map.get_tile(self.x, saturated_add(self.y, 1, map.height-1))
            .eq(block_tile)
    }

    pub fn is_broke(&self) -> bool {
        self.gold == 0
    }

    pub fn has_item(&mut self, item: &Item) -> bool {
        self.items.iter().any(|x| x.is_more(*item))
    }

    pub fn buys(&mut self, item: Item, cost: u32) {
        self.items = self.items.iter().map(|ele| match (ele, item) {
            (Item::Rock(x), Item::Rock(y)) => Item::Rock(x+y),
            (Item::Seed(x), Item::Seed(y)) => Item::Seed(x+y),
            _ => *ele,
        }).collect();
        self.gold -= cost;
    }

    pub fn sells(&mut self, item: Item, cost: u32) {
        self.items = self.items.iter().map(|ele| match (ele, item) {
            (Item::Rock(x), Item::Rock(y)) => Item::Rock(x-y),
            (Item::Seed(x), Item::Seed(y)) => Item::Seed(x-y),
            _ => *ele,
        }).collect();
        self.gold += cost;
    }
}
