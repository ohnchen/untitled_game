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
        Self::draw_menu_box_top(game_width - 2, game_height)?;
        Self::draw_menu_box_bottom(game_width - 2, game_height)?;
        Self::clear_menu_box(game_width - 2, game_height)?;
        Ok(())
    }

    fn draw_menu_box_top(width: u16, bottom: u16) -> io::Result<()> {
        queue!(io::stdout(), MoveTo(1, bottom - 5), Print(TLCORNER))?;
        for _ in 0..width - 2 {
            queue!(io::stdout(), Print(HBORDER))?;
        }
        queue!(io::stdout(), Print(TRCORNER))?;
        Ok(())
    }

    fn clear_menu_box(width: u16, bottom: u16) -> io::Result<()> {
        queue!(io::stdout(), MoveTo(1, bottom - 4))?;
        for _ in 0..width {
            queue!(io::stdout(), Print(" "))?;
        }
        queue!(io::stdout(), MoveTo(1, bottom - 3))?;
        for _ in 0..width {
            queue!(io::stdout(), Print(" "))?;
        }
        Ok(())
    }

    fn draw_menu_box_bottom(width: u16, bottom: u16) -> io::Result<()> {
        queue!(io::stdout(), MoveTo(1, bottom - 2), Print(BLCORNER))?;
        for _ in 0..width - 2 {
            queue!(io::stdout(), Print(HBORDER))?;
        }
        queue!(io::stdout(), Print(BRCORNER))?;
        Ok(())
    }
}
