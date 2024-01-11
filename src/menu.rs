use crate::config::*;
use crossterm::{
    cursor::{MoveDown, MoveTo},
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

use crate::Item;
use crate::Map;
use crate::Merchant;
use crate::Player;

pub struct Menu {
    game_width: u16,
    game_height: u16,
}

impl Menu {
    pub fn new(game_width: u16, game_height: u16) -> Self {
        Self {
            game_width,
            game_height,
        }
    }

    pub fn draw_debug(&self, map: &Map, player: &Player, merchant: &Merchant) -> io::Result<()> {
        Self::draw_box_top(self.game_width - DEBUG_WIDTH - 1, 1, DEBUG_WIDTH)?;
        Self::draw_box_bottom(
            self.game_width - DEBUG_WIDTH - 1,
            DEBUG_HEIGHT + 1,
            DEBUG_WIDTH,
        )?;
        Self::clear_between(
            self.game_width - DEBUG_WIDTH - 1,
            1,
            DEBUG_HEIGHT + 1,
            DEBUG_WIDTH,
        )?;
        queue!(
            io::stdout(),
            MoveTo(self.game_width - DEBUG_WIDTH + 1, 2),
            Print("DEBUG>")
        )?;
        queue!(
            io::stdout(),
            MoveTo(self.game_width - DEBUG_WIDTH + 1, 3),
            Print(format!("map> {} {}", map.width, map.height))
        )?;
        queue!(
            io::stdout(),
            MoveTo(self.game_width - DEBUG_WIDTH + 1, 4),
            Print(format!("player> {} {}", player.x, player.y))
        )?;
        queue!(
            io::stdout(),
            MoveTo(self.game_width - DEBUG_WIDTH + 1, 5),
            Print(format!("player.gold> {}", player.gold))
        )?;
        queue!(
            io::stdout(),
            MoveTo(self.game_width - DEBUG_WIDTH + 1, 6),
            Print(format!("merchant.gold> {}", merchant.gold))
        )?;
        Ok(())
    }

    pub fn draw_menu(&self) -> io::Result<()> {
        Self::draw_box_top(MENU_LEFT, self.game_height - MENU_TOP, self.game_width - 2)?;
        Self::draw_box_bottom(MENU_LEFT, self.game_height - 2, self.game_width - 2)?;
        Self::clear_between(
            MENU_LEFT,
            self.game_height - MENU_TOP,
            self.game_height - MENU_TOP + MENU_HEIGHT - 1,
            self.game_width - 2,
        )?;
        Ok(())
    }

    pub fn draw_trade_menu(&self, player: &Player) -> io::Result<()> {
        Self::draw_box_top(
            TRADE_MENU_LEFT,
            self.game_height - TRADE_MENU_TOP,
            TRADE_MENU_WIDTH,
        )?;
        Self::draw_box_bottom(
            TRADE_MENU_LEFT,
            self.game_height - MENU_HEIGHT - 2,
            TRADE_MENU_WIDTH,
        )?;
        Self::clear_between(
            TRADE_MENU_LEFT,
            self.game_height - TRADE_MENU_TOP,
            self.game_height - TRADE_MENU_TOP + TRADE_MENU_HEIGHT - 1,
            TRADE_MENU_WIDTH,
        )?;
        let mut top = TRADE_MENU_TOP - 1;
        queue!(
            io::stdout(),
            MoveTo(TRADE_MENU_LEFT + 2, self.game_height - top),
            Print("MERCHANT>"),
        )?;
        for (i, (k, v)) in player.items.iter().enumerate() {
            let b_value = player.buying[k];
            top -= 1;
            queue!(
                io::stdout(),
                MoveTo(TRADE_MENU_LEFT + 2, self.game_height - top),
                Print(format!(
                    "{}:{} {:?}({}{:?})",
                    i + 1,
                    k.get_name(),
                    v,
                    {
                        if b_value > 0 {
                            '+'
                        } else {
                            '​'
                        }
                    },
                    b_value,
                ))
            )?;
        }
        Ok(())
    }

    fn draw_box_top(left: u16, row: u16, width: u16) -> io::Result<()> {
        queue!(
            io::stdout(),
            MoveTo(left, row),
            Print(TLCORNER),
            Print(HBORDER.repeat(width as usize - 2)),
            Print(TRCORNER)
        )?;
        Ok(())
    }

    fn draw_box_bottom(left: u16, row: u16, width: u16) -> io::Result<()> {
        queue!(
            io::stdout(),
            MoveTo(left, row),
            Print(BLCORNER),
            Print(HBORDER.repeat(width as usize - 2)),
            Print(BRCORNER)
        )?;
        Ok(())
    }

    fn clear_between(left: u16, top: u16, bottom: u16, width: u16) -> io::Result<()> {
        for i in 1..bottom - top {
            queue!(io::stdout(), MoveTo(left, top + i))?;
            queue!(io::stdout(), Print(" ".repeat(width.into())))?;
        }
        Ok(())
    }
}
