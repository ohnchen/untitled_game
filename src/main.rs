#![allow(unused_imports)]
use crossterm::{
    cursor::{self, MoveTo, MoveLeft, MoveRight, MoveUp, MoveDown, SetCursorStyle},
    event::{self, KeyCode},
    execute, queue,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, ClearType, Clear}, style::Print,
};
use std::io::{self, Write};

mod tiles;
mod map;

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
        MoveTo(terminal::size()?.0/2,terminal::size()?.1/2),
        Print("X"),
        MoveLeft(1),
    )?;

    let map = Map::new();

    loop {
        map.draw(cursor::position().unwrap())?;

        if event::poll(std::time::Duration::from_millis(500))? {
            match event::read()? {
                event::Event::FocusGained => println!("FocusGained"),
                event::Event::FocusLost => println!("FocusLost"),
                event::Event::Key(key_event) => match key_event.code { 
                    event::KeyCode::Esc => break,
                    event::KeyCode::Char(c) => match c {
                        'h' => queue!(stdout, Clear(ClearType::All), MoveLeft(1), Print("X"), MoveLeft(1))?,
                        'j' => queue!(stdout, Clear(ClearType::All), MoveDown(1), Print("X"), MoveLeft(1))?,
                        'k' => queue!(stdout, Clear(ClearType::All), MoveUp(1), Print("X"), MoveLeft(1))?,
                        'l' => queue!(stdout, Clear(ClearType::All), MoveRight(1), Print("X"), MoveLeft(1))?,
                        'H' => queue!(stdout, Clear(ClearType::All), MoveLeft(3), Print("X"), MoveLeft(1))?,
                        'J' => queue!(stdout, Clear(ClearType::All), MoveDown(3), Print("X"), MoveLeft(1))?,
                        'K' => queue!(stdout, Clear(ClearType::All), MoveUp(3), Print("X"), MoveLeft(1))?,
                        'L' => queue!(stdout, Clear(ClearType::All), MoveRight(3), Print("X"), MoveLeft(1))?,
                        _ => {},
                    },
                    _ => {},
                }
                event::Event::Mouse(event) => println!("{:?}", event),
                event::Event::Paste(data) => println!("Pasted {:?}", data),
                event::Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
        }


        stdout.flush()?;
    }

    execute!(stdout, LeaveAlternateScreen, cursor::Show, cursor::RestorePosition)?;
    disable_raw_mode()?;
    Ok(())
}
