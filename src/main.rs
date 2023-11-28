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
mod tiles;

use map::Map;

macro_rules! movePlayer {
    ($direction:tt, $len:tt) => {
        match $direction {
            "left" => queue!(io::stdout(), MoveLeft($len), Print("X"), MoveLeft(1))?,
            "down" => queue!(io::stdout(), MoveDown($len), Print("X"), MoveLeft(1))?,
            "up" => queue!(io::stdout(), MoveUp($len), Print("X"), MoveLeft(1))?,
            "right" => queue!(io::stdout(), MoveRight($len), Print("X"), MoveLeft(1))?,
            _ => {}
        }
    };
    () => {
        queue!(io::stdout(), Print("X"), MoveLeft(1))?
    };
}

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

    let map = Map::new(terminal::size()?.1.into(), terminal::size()?.0.into());
    map.draw_map()?;

    execute!(
        stdout,
        MoveTo(terminal::size()?.0 / 2, terminal::size()?.1 / 2),
        Print("X"),
        MoveLeft(1),
    )?;

    loop {
        let last_pos = cursor::position().unwrap();

        // if event::poll(std::time::Duration::from_millis(500))? {
        match event::read()? {
            event::Event::Key(key_event) => match key_event.code {
                event::KeyCode::Esc => break,
                event::KeyCode::Char(c) => match c {
                    'h' => movePlayer!("left", 1),
                    'j' => movePlayer!("down", 1),
                    'k' => movePlayer!("up", 1),
                    'l' => movePlayer!("right", 1),
                    'H' => movePlayer!("left", 3),
                    'J' => movePlayer!("down", 3),
                    'K' => movePlayer!("up", 3),
                    'L' => movePlayer!("right", 3),
                    _ => movePlayer!(),
                },
                _ => movePlayer!(),
            },
            _ => movePlayer!(),
        }
        //}

        map.draw_update(last_pos, cursor::position().unwrap())?;
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
