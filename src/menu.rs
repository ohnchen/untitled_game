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
        self.write_between(
            vec![
                "DEBUG>".to_string(),
                format!("map> {} {}", map.width, map.height),
                format!("player> {} {}", player.x, player.y),
                format!("player.gold> {}", player.gold),
                format!("merchant.gold> {}", merchant.gold),
            ],
            self.game_width - DEBUG_WIDTH + 1,
            1,
            DEBUG_WIDTH,
            DEBUG_HEIGHT,
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
        self.write_between(
            vec!["Dont play this game!".to_string()],
            3,
            self.game_height - MENU_HEIGHT,
            self.game_width - 2,
            MENU_HEIGHT,
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
        self.write_between(
            [vec!["MERCHANT>".to_string()], {
                player
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, (k, v))| {
                        let b_value = player.buying[k];
                        format!(
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
                        )
                    })
                    .collect::<Vec<String>>()
            }].concat(),
            3,
            self.game_height - TRADE_MENU_TOP,
            self.game_width - 2,
            TRADE_MENU_HEIGHT,
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

    fn write_between(&self, formats: Vec<String>, left: u16, top: u16, _width: u16, height: u16) -> io::Result<()> {
        for (i, fmt) in formats.iter().enumerate() {
            if i as u16 + 2 >= height {
                return Ok(());
            }
            queue!(
                io::stdout(),
                MoveTo(left, i as u16 + top),
                Print(fmt)
            )?;
        }
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
