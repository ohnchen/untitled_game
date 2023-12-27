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

mod config;
mod info;
mod map;
mod merchant;
mod player;
mod tiles;
mod utils;

use crate::config::*;
use crate::info::Info;
use crate::map::Map;
use crate::merchant::Merchant;
use crate::player::Player;
use crate::utils::*;


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

    let info = Info::new(config::DEBUG, 0, map_viewheight + 1);

    map.draw_map()?;
    info.draw_info(&map, &player, &global_merchant)?;

    execute!(
        io::stdout(),
        MoveTo(
            map.spawnpoint.0 - map.viewleft,
            map.spawnpoint.1 - map.viewtop
        ),
        Print('X'),
        MoveLeft(1)
    )?;

    loop {
        let old_player_pos: (u16, u16) = (player.x, player.y);
        //if event::poll(std::time::Duration::from_millis(500))? {
        match event::read()? {
            event::Event::Key(key_event) => match key_event.code {
                event::KeyCode::Esc => break,
                event::KeyCode::Enter => {
                    for item in player.buying.clone() {
                        if !player.is_broke() && global_merchant.has_item(&item) {
                            player.trade(item, &global_merchant);
                        }
                    }
                    player.reset_buying();
                }
                event::KeyCode::F(5) => {
                    map = Map::new(map_width, map_height, left, top, game_width, map_viewheight);
                    player = Player::new(&map);
                    map.draw_map()?;
                    execute!(
                        io::stdout(),
                        MoveTo(
                            map.spawnpoint.0 - map.viewleft,
                            map.spawnpoint.1 - map.viewtop
                        ),
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
                    '1' => {
                        if !player.is_on_merchant(&map) {
                            continue;
                        };
                        player.buying[0] = player.buying[0].add(1);
                    }
                    '2' => {
                        if !player.is_on_merchant(&map) {
                            continue;
                        };
                        player.buying[1] = player.buying[1].add(1);
                    }
                    '3' => {
                        if !player.is_on_merchant(&map) {
                            continue;
                        };
                        player.buying[0] = player.buying[0].add(-1);
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        map.draw_player(old_player_pos, &player)?;
        info.draw_info(&map, &player, &global_merchant)?;
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
