#![allow(unused_imports)]
use crossterm::{
    cursor::{self, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, SetCursorStyle},
    event::{self, KeyCode},
    execute, queue,
    style::Print,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io::{self, Write};

mod info;
mod map;
mod merchant;
mod player;
mod tiles;
mod utils;

use crate::info::Info;
use crate::map::Map;
use crate::merchant::Merchant;
use crate::player::Player;
use crate::utils::*;

static SEED_PRICE: u32 = 1;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        cursor::SavePosition,
        EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All),
    )?;

    let game_width: u16 = terminal::size()?.0;
    let game_height: u16 = terminal::size()?.1;
    let info_viewheight: u16 = game_height / 6;
    let map_viewheight: u16 = game_height - info_viewheight;

    let left = 220;
    let top = 220;
    let map_width = 500;
    let map_height = 500;

    let mut map = Map::new(map_width, map_height, left, top, game_width, map_viewheight);
    let mut player = Player::new(&map);
    
    let mut global_merchant = Merchant::new();

    let info = Info::new(true);

    map.draw_map()?;
    info.draw_info(
        &map,
        &player,
        &global_merchant,
    )?;

    execute!(
        io::stdout(),
        MoveTo(map.spawnpoint.0 - map.viewleft, map.spawnpoint.1 - map.viewtop),
        Print('X'),
        MoveLeft(1)
    )?;

    loop {
        let old_player_pos: (u16, u16) = (player.x, player.y);
        //if event::poll(std::time::Duration::from_millis(500))? {
        match event::read()? {
            event::Event::Key(key_event) => match key_event.code {
                event::KeyCode::Esc => break,
                event::KeyCode::F(5) => {
                    map = Map::new(map_width, map_height, left, top, game_width, map_viewheight);
                    player = Player::new(&map);
                    map.draw_map()?;
                    execute!(
                        io::stdout(),
                        MoveTo(map.spawnpoint.0 - map.viewleft, map.spawnpoint.1 - map.viewtop),
                        Print('X'),
                        MoveLeft(1)
                    )?;
                }
                event::KeyCode::Char(c) => match c {
                    'h' => player.move_direction(&mut map, Direction::Left, 1)?,
                    'l' => player.move_direction(&mut map, Direction::Right, 1)?,
                    'j' => player.move_direction(&mut map, Direction::Down, 1)?,
                    'k' => player.move_direction(&mut map, Direction::Up, 1)?,
                    'p' => {
                        if player.has_pickaxe() {
                            player.tools.clear();
                        } else {
                            player.tools.push(Tools::Pickaxe);
                        }
                    }
                    'b' => {
                        if !player.is_on_merchant(&map) { continue }
                        if global_merchant.has_item(&Items::Seed(1)) {
                        global_merchant.sells(Items::Seed(1), SEED_PRICE);
                        }
                    },
                    's' => {
                        if !player.is_on_merchant(&map) { continue };
                        if !global_merchant.is_broke() {
                            global_merchant.buys(Items::Seed(1), SEED_PRICE);
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        map.draw_player(old_player_pos, &player)?;
        info.draw_info(
            &map,
            &player,
            &global_merchant,
        )?;
        stdout.flush()?;
    }

    execute!(
        stdout,
        LeaveAlternateScreen,
        cursor::Show,
        cursor::RestorePosition
    )?;
    disable_raw_mode()?;
    Ok(())
}
