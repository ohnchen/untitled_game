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

    pub fn draw_menu(&self) -> io::Result<()> {
        Self::draw_box_top(1, self.game_height - MENU_TOP, self.game_width - 2)?;
        Self::draw_box_bottom(1, self.game_height - 2, self.game_width - 2)?;
        Self::clear_between(
            1,
            self.game_height - MENU_TOP,
            self.game_height - MENU_TOP + MENU_HEIGHT - 1,
            self.game_width - 2,
        )?;
        Ok(())
    }

    pub fn draw_trade_menu(&self) -> io::Result<()> {
        Self::draw_box_top(1, self.game_height - TRADE_MENU_TOP, TRADE_MENU_WIDTH)?;
        Self::draw_box_bottom(1, self.game_height - MENU_HEIGHT - 2, TRADE_MENU_WIDTH)?;
        Self::clear_between(
            1,
            self.game_height - TRADE_MENU_TOP,
            self.game_height - TRADE_MENU_TOP + TRADE_MENU_HEIGHT - 1,
            TRADE_MENU_WIDTH,
        )?;
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
