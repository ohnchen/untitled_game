use crossterm::{
    cursor::MoveTo,
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::{
    io::{self, Write},
    task::Wake,
};

use crate::Item;
use crate::Map;
use crate::Merchant;
use crate::Player;

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
    left: u16,
    top: u16,
}

impl Info {
    pub fn new(debug: bool, left: u16, top: u16) -> Self {
        Self { debug, left, top }
    }

    pub fn draw_info(&self, map: &Map, player: &Player, merchant: &Merchant) -> io::Result<()> {
        let left = self.left;
        let top = self.top;
        if self.debug {
            draw_info!(
                left,
                top,
                "map: {} {}, PLAYER -> {} {}, tools: {:?}, inv: {:?}, gold: {:?}, MERCHANT -> gold: {}, inv: {:?}",
                map.width,
                map.height,
                player.x,
                player.y,
                player.tools,
                player.items,
                player.gold,
                merchant.gold,
                merchant.items,
            )?;
            return Ok(());
        }
        if player.is_on_merchant(map) {
            self.draw_trademenu(player, merchant)?;
            return Ok(());
        }
        clear_info!(left, top)?;
        Ok(())
    }

    pub fn draw_trademenu(&self, player: &Player, _merchant: &Merchant) -> io::Result<()> {
        let left = self.left;
        let mut top = self.top;

        draw_info!(left, top, "Gold: {}", player.gold)?;
        for (i, item) in player.items.iter().enumerate() {
            let buying = player.buying[i];
            top += 1;
            draw_info!(
                left,
                top,
                "{}:{} {:?}({}{:?})",
                i + 1,
                item.get_name(),
                item.get_value(),
                {
                    if buying.get_value() > 0 {
                        '+'
                    } else {
                        '​' 
                    } 
                },
                buying.get_value()
            )?;
        }
        Ok(())
    }
}
