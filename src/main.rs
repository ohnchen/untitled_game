#![allow(unused_imports)]
use crossterm::{
    cursor::{self, MoveTo, MoveLeft, MoveRight, MoveUp, MoveDown, SetCursorStyle},
    event::{self, KeyCode},
    execute, queue,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        SetCursorStyle::SteadyBlock,
        terminal::Clear(terminal::ClearType::All),
        MoveTo(0,0),
    )?;

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            match event::read()? {
                event::Event::FocusGained => println!("FocusGained"),
                event::Event::FocusLost => println!("FocusLost"),
                event::Event::Key(key_event) => match key_event.code { 
                    event::KeyCode::Esc => break,
                    event::KeyCode::Char(c) => match c {
                        'h' => queue!(stdout, MoveLeft(1))?,
                        'j' => queue!(stdout, MoveDown(1))?,
                        'k' => queue!(stdout, MoveUp(1))?,
                        'l' => queue!(stdout, MoveRight(1))?,
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

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
