use crossterm::{cursor::MoveTo, queue, style::Print};
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

    pub fn draw_info(&self, map: &Map, player: &Player, xmin: usize, xmax: usize, ymin: usize, ymax: usize) -> io::Result<()> {
        if self.debug {
            queue!(
                io::stdout(),
                MoveTo(xmin as u16, ymin as u16),
                Print(format!(
                    "map: {} {}, player: {} {}, inv: {:?}",
                    map.width, map.height, player.x, player.y, player.tools
                ))
            )?;
            return Ok(());
        }

        for x in xmin..xmax {
            for y in ymin..ymax {
                queue!(io::stdout(), MoveTo(x as u16, y as u16), Print("i"),)?;
            }
        }

        Ok(())
    }
}
