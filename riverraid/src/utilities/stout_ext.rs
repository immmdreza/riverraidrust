//! This module contains extension to [`Stdout`] object.

use std::{fmt::Display, io::Stdout};

use crossterm::{cursor::MoveTo, style::Print, terminal::Clear, QueueableCommand};

use crate::entities::Location;

pub type StdoutResult<'a> = Result<&'a mut Stdout, std::io::Error>;

pub trait Located {
    fn location(&self) -> &Location;
}

impl<T: Located> AsLocationTuple for T {
    fn as_loc_tuple(&self) -> (u16, u16) {
        (self.location().column, self.location().line)
    }
}

pub trait AsLocationTuple {
    fn as_loc_tuple(&self) -> (u16, u16);
}

impl AsLocationTuple for (u16, u16) {
    fn as_loc_tuple(&self) -> (u16, u16) {
        (self.0, self.1)
    }
}

impl AsLocationTuple for u16 {
    fn as_loc_tuple(&self) -> (u16, u16) {
        (*self, *self)
    }
}

pub trait StdoutExt {
    fn clear_all(&'_ mut self) -> StdoutResult<'_>;

    fn move_cursor(&'_ mut self, loc: impl AsLocationTuple) -> StdoutResult<'_>;

    fn print(&'_ mut self, display: impl Display) -> StdoutResult<'_>;

    fn draw(&'_ mut self, loc: impl AsLocationTuple, display: impl Display) -> StdoutResult<'_>;
}

impl StdoutExt for Stdout {
    fn move_cursor(&'_ mut self, loc: impl AsLocationTuple) -> StdoutResult<'_> {
        let (c, l) = loc.as_loc_tuple();
        self.queue(MoveTo(c, l))
    }

    fn draw(&'_ mut self, loc: impl AsLocationTuple, display: impl Display) -> StdoutResult<'_> {
        self.move_cursor(loc)?.print(display)
    }

    fn clear_all(&'_ mut self) -> StdoutResult<'_> {
        self.queue(Clear(crossterm::terminal::ClearType::All))
    }

    fn print(&'_ mut self, display: impl Display) -> StdoutResult<'_> {
        self.queue(Print(display))
    }
}
