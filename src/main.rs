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
mod player;
mod trader;
mod tiles;
mod utils;

use crate::utils::*;
use crate::info::Info;
use crate::map::Map;
use crate::player::Player;

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
    let info_height: u16 = game_height / 6;
    let map_height: u16 = game_height - info_height;

    let mut map = Map::new(game_width, map_height);
    map.draw_map(0, game_width.into(), 0, map_height.into())?;

    let mut player = Player::new(&map);

    let info = Info::new(false);
    info.draw_info(
        &map,
        &player,
        0,
        game_width.into(),
        (map_height + 1).into(),
        game_height.into(),
    )?;

    execute!(
        io::stdout(),
        MoveTo(map.spawnpoint.0, map.spawnpoint.1),
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
                    map = Map::new(game_width, map_height);
                    player = Player::new(&map);
                    map.draw_map(0, game_width.into(), 0, map_height.into())?;
                    execute!(
                        io::stdout(),
                        MoveTo(map.spawnpoint.0, map.spawnpoint.1),
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
                    },
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
            0,
            game_width.into(),
            (map_height + 1).into(),
            game_height.into(),
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
