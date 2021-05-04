use std::any::Any;
use crate::errors::*;
use crate::models::application::modes::mode::Mode;
use crate::models::application::Application;
use crate::presenters;

pub struct InsertMode {
    pub input: Option<char>,
}

impl InsertMode {
    pub fn new() -> InsertMode {
        InsertMode { input: None }
    }
}

impl Mode for InsertMode {
    fn mode_str(&self) -> Option<&'static str> {
            Some("insert")
    }

    fn mode_id(&self) -> Option<&'static str> {
            Some("insert")
    }

    fn present(&mut self, app :&mut Application) -> Result<()>{
        presenters::modes::insert::display(&mut app.workspace, &mut app.view)
    }

    fn as_any(&self) -> &dyn Any{
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any{
        self
    }
}
