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

pub struct Menu();
impl Menu {
    pub fn draw_menu(&self, game_width: u16, game_height: u16) -> io::Result<()> {
        Self::draw_menu_box_top(game_width - 2, game_height-5)?;
        Self::draw_menu_box_bottom(game_width - 2, game_height-2)?;
        Self::clear_between(1, game_height-5, game_height-2, game_width-2)?;
        Ok(())
    }

    pub fn draw_trade_menu(&self, game_width: u16, game_height: u16) -> io::Result<()> {
        Ok(())
    }

    fn draw_menu_box_top(width: u16, bottom: u16) -> io::Result<()> {
        queue!(io::stdout(), MoveTo(1, bottom), Print(TLCORNER))?;
        for _ in 0..width - 2 {
            queue!(io::stdout(), Print(HBORDER))?;
        }
        queue!(io::stdout(), Print(TRCORNER))?;
        Ok(())
    }

    fn draw_menu_box_bottom(width: u16, bottom: u16) -> io::Result<()> {
        queue!(io::stdout(), MoveTo(1, bottom), Print(BLCORNER))?;
        for _ in 0..width - 2 {
            queue!(io::stdout(), Print(HBORDER))?;
        }
        queue!(io::stdout(), Print(BRCORNER))?;
        Ok(())
    }

    fn clear_between(left: u16, top: u16, bottom: u16, width: u16) -> io::Result<()> {
        for i in 1..bottom-top {
            queue!(io::stdout(), MoveTo(left, top+i))?;
            queue!(io::stdout(), Print(" ".repeat(width.into())))?;
        }
        Ok(())
    }
}
