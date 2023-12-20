use crossterm::{
    cursor::MoveTo,
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

use crate::map::Map;
use crate::player::Player;

macro_rules! clear_info {
    ($left:ident, $top:ident) => {
        queue!(
            io::stdout(),
            MoveTo($left as u16, $top as u16),
            Clear(ClearType::FromCursorDown),
        )
    };
}

macro_rules! draw_info {
    ($left:ident, $top:ident, $($arg:tt)*) => {
        queue!(
            io::stdout(),
            MoveTo($left as u16, $top as u16),
            Clear(ClearType::FromCursorDown),
            Print(format!($($arg)*)),
        )
    };
}

pub struct Info {
    debug: bool,
}

impl Info {
    pub fn new(debug: bool) -> Self {
        Self { debug }
    }

    pub fn draw_info(
        &self,
        map: &Map,
        player: &Player,
        left: usize,
        _right: usize,
        top: usize,
        _bottom: usize,
    ) -> io::Result<()> {
        if self.debug {
            draw_info!(
                left,
                top,
                "map: {} {}, player: {} {}, tools: {:?}, inv: {:?}",
                map.width,
                map.height,
                player.x,
                player.y,
                player.tools,
                player.items,
            )?;
            return Ok(());
        }
        if player.is_on_merchant(map) {
            draw_info!(left, top, "Merchant has no items left!")?;
            return Ok(());
        }
        clear_info!(left, top)?;
        Ok(())
    }
}
