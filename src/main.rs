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

mod map;
mod player;
mod tiles;

use crate::map::Map;
use crate::player::{Direction, Player};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        cursor::SavePosition,
        EnterAlternateScreen,
        //cursor::Hide,
        terminal::Clear(terminal::ClearType::All),
    )?;

    let extra_height: u16 = 10;
    let map_width: u16 = terminal::size()?.0;
    let map_height: u16 = terminal::size()?.1 - extra_height;

    let map = Map::new(map_width, map_height);
    map.draw_map()?;

    let mut player = Player::new(&map);
    execute!(
        io::stdout(),
        MoveTo(map.width/2, map.height/2),
        Print('X'),
        MoveLeft(1)
    )?;

    loop {
        let old_player_pos: (u16, u16) = (player.x, player.y);
        //if event::poll(std::time::Duration::from_millis(500))? {
            match event::read()? {
                event::Event::Key(key_event) => match key_event.code {
                    event::KeyCode::Esc => break,
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
