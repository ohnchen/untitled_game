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
                event::Event::FocusGained => println!("FocusGained"),
                event::Event::FocusLost => println!("FocusLost"),
                event::Event::Key(key_event) => match key_event.code {
                    event::KeyCode::Esc => break,
                    event::KeyCode::Char(c) => match c {
                        'h' => queue!(
                            stdout,
                            MoveLeft(1),
                            Print("X"),
                            MoveLeft(1)
                        )?,
                        'j' => queue!(
                            stdout,
                            MoveDown(1),
                            Print("X"),
                            MoveLeft(1)
                        )?,
                        'k' => queue!(
                            stdout,
                            MoveUp(1),
                            Print("X"),
                            MoveLeft(1)
                        )?,
                        'l' => queue!(
                            stdout,
                            MoveRight(1),
                            Print("X"),
                            MoveLeft(1)
                        )?,
                        'H' => queue!(
                            stdout,
                            MoveLeft(3),
                            Print("X"),
                            MoveLeft(1)
                        )?,
                        'J' => queue!(
                            stdout,
                            MoveDown(3),
                            Print("X"),
                            MoveLeft(1)
                        )?,
                        'K' => queue!(
                            stdout,
                            MoveUp(3),
                            Print("X"),
                            MoveLeft(1)
                        )?,
                        'L' => queue!(
                            stdout,
                            MoveRight(3),
                            Print("X"),
                            MoveLeft(1)
                        )?,
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
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
