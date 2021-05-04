use std::any::Any;
use crate::errors::*;
use scribe::buffer::Position;
use crate::models::application::Application;
use crate::models::application::modes::mode::Mode;
use crate::presenters;

pub struct SelectMode {
    pub anchor: Position,
}

impl SelectMode {
    pub fn new(anchor: Position) -> SelectMode {
        SelectMode { anchor }
    }
}

impl Mode for SelectMode {
    fn mode_str(&self) -> Option<&'static str> {
            Some("select")
    }

    fn mode_id(&self) -> Option<&'static str> {
            Some("select")
    }

    fn present(&mut self, app :&mut Application) -> Result<()>{
        presenters::modes::select::display(&mut app.workspace, self, &mut app.view)
    }

    fn as_any(&self) -> &dyn Any{
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any{
        self
    }
}
