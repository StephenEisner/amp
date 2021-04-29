use scribe::buffer::{LineRange, Position, Range};
use crate::models::application::modes::mode::*;

pub struct SelectLineMode {
    pub anchor: usize,
}

impl MMode for SelectLineMode {
    fn get_mode_id(&self) -> ModeID {
        return ModeID{id:Some("select")};
    }
}

impl SelectLineMode {
    pub fn new(anchor: usize) -> SelectLineMode {
        SelectLineMode { anchor }
    }

    pub fn to_range(&self, cursor: &Position) -> Range {
        LineRange::new(self.anchor, cursor.line).to_inclusive_range()
    }
}
