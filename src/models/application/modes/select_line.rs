use std::any::Any;
use crate::errors::*;
use scribe::buffer::{LineRange, Position, Range};
use crate::models::application::Application;
use crate::models::application::modes::mode::Mode;
use crate::presenters;

pub struct SelectLineMode {
    pub anchor: usize,
}

impl SelectLineMode {
    pub fn new(anchor: usize) -> SelectLineMode {
        SelectLineMode { anchor }
    }

    pub fn to_range(&self, cursor: &Position) -> Range {
        LineRange::new(self.anchor, cursor.line).to_inclusive_range()
    }
}

impl Mode for SelectLineMode {
    fn mode_str(&self) -> Option<&'static str> {
            Some("select_line")
    }

    fn mode_id(&self) -> Option<&'static str> {
            Some("select_line")
    }

    fn present(&mut self, app :&mut Application) -> Result<()>{
        presenters::modes::select_line::display(&mut app.workspace, self, &mut app.view)
    }

    fn as_any(&self) -> &dyn Any{
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any{
        self
    }
}
