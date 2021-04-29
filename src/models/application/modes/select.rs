use scribe::buffer::Position;
use crate::models::application::modes::mode::*;

pub struct SelectMode {
    pub anchor: Position,
}

impl SelectMode {
    pub fn new(anchor: Position) -> SelectMode {
        SelectMode { anchor }
    }
}

impl MMode for SelectMode {
    fn get_mode_id(&self) -> ModeID {
        return ModeID{id:Some("select")};
    }
}

