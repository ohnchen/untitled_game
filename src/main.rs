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
mod tiles;

use crate::info::Info;
use crate::map::Map;
use crate::player::{Direction, Player};

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
    let info_height: u16 = game_height/8;
    let map_height: u16 = game_height - info_height;

    let mut map = Map::new(game_width, map_height);
    map.draw_map((0, game_width.into()), (0, map_height.into()))?;

    let mut player = Player::new(&map);

    let info = Info::new(true, &player, &map); //game_width, info_height);
    info.draw_info(
        (0, game_width.into()),
        ((map_height + 1).into(), game_height.into()),
    )?;

    execute!(
        io::stdout(),
        MoveTo(map.spawnpoint.0 as u16, map.spawnpoint.1 as u16),
        Print('X'),
        MoveLeft(1)
    )?;

    loop {
        let old_player_pos: (u16, u16) = (player.x, player.y);
        //if event::poll(std::time::Duration::from_millis(500))? {
        match event::read()? {
            // event::Event::Resize(nw, nh) => {
            //     game_width = nw;
            //     game_height = nh;
            //     info_height = game_height/8;
            //     map_height = game_height - info_height;
            //     map.height = map_height;
            //     map.width = game_width;
            //     map.draw_map((0, game_width.into()), (0, map_height.into()))?;
            //     info.draw_info((0, game_width.into()),((map_height+1).into(), game_height.into()))?;
            // }
            event::Event::Key(key_event) => match key_event.code {
                event::KeyCode::Esc => break,
                event::KeyCode::F(5) => {
                    map = Map::new(game_width, map_height);
                    map.draw_map((0, game_width.into()), (0, map_height.into()))?;
                    player = Player::new(&map);
                    execute!(
                        io::stdout(),
                        MoveTo(map.spawnpoint.0 as u16, map.spawnpoint.1 as u16),
                        Print('X'),
                        MoveLeft(1)
                    )?;
                }
                event::KeyCode::Char(c) => match c {
                    'h' => player.move_direction(&map, Direction::Left, 1),
                    'l' => player.move_direction(&map, Direction::Right, 1),
                    'j' => player.move_direction(&map, Direction::Down, 1),
                    'k' => player.move_direction(&map, Direction::Up, 1),
                    'H' => player.move_direction(&map, Direction::Left, 3),
                    'L' => player.move_direction(&map, Direction::Right, 3),
                    'J' => player.move_direction(&map, Direction::Down, 3),
                    'K' => player.move_direction(&map, Direction::Up, 3),
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        //}

        map.draw_player(old_player_pos, &player)?;
        //info.draw_info((0, game_width.into()),((map_height+1).into(), game_height.into()))?;
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
