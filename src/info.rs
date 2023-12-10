use crossterm::{cursor::MoveTo, queue, style::Print, terminal::{Clear, ClearType}};
use std::io::{self, Write};

use crate::map::Map;
use crate::player::Player;

pub struct Info {
    debug: bool,
}

impl Info {
    pub fn new(debug: bool) -> Self {
        Self { debug }
    }

    pub fn draw_info(&self, map: &Map, player: &Player, xmin: usize, _xmax: usize, ymin: usize, _ymax: usize) -> io::Result<()> {
        if self.debug {
            queue!(
                io::stdout(),
                MoveTo(xmin as u16, ymin as u16),
                Clear(ClearType::FromCursorDown),
                Print(format!(
                    "map: {} {}, player: {} {}, tools: {:?}, inv: {:?}",
                    map.width, map.height, player.x, player.y, player.tools, player.items,
                ))
            )?;
            return Ok(());
        }
        Ok(())
    }
}
