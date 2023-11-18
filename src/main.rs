#![allow(unused_imports)]
use crossterm::{
    cursor::{self, MoveTo},
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
        terminal::Clear(terminal::ClearType::All),
        MoveTo(0,0),
    )?;

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            match event::read()? {
                event::Event::FocusGained => println!("FocusGained"),
                event::Event::FocusLost => println!("FocusLost"),
                event::Event::Key(event) => {
                    match event.code {
                        event::KeyCode::Char('q') => break,
                        _ => {},
                    }
                }
                event::Event::Mouse(event) => println!("{:?}", event),
                event::Event::Paste(data) => println!("Pasted {:?}", data),
                event::Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
        }
    }

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
