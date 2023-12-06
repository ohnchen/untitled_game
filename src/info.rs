use std::io::{self, Write};
use crossterm::{queue, cursor::MoveTo, style::Print};

use crate::map::Map;
use crate::player::Player;

pub struct Info<'a> {
    debug: bool,
    player: &'a Player,
    map: &'a Map,
}

impl<'a> Info<'a> {
    pub fn new(debug: bool, player: &'a Player, map: &'a Map) -> Self {
        Self { debug, player, map }
    }
    
     pub fn draw_info(&self, xbounds: (usize, usize), ybounds: (usize, usize)) -> io::Result<()> {
        if self.debug {
            queue!(io::stdout(), Print(format!("{} {}", self.map.width, self.map.height)))?;
            return Ok(());
        }
        for x in xbounds.0..xbounds.1 {
            for y in ybounds.0..ybounds.1 {
                queue!(
                    io::stdout(),
                    MoveTo(x as u16, y as u16),
                    Print("i"),
                )?;
            }
        }
    
        io::stdout().flush()?;
        Ok(())
    }

}
