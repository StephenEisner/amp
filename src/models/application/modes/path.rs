use std::any::Any;
use crate::errors::*;
use crate::models::application::modes::mode::Mode;
use crate::models::application::Application;
use crate::presenters;
use std::fmt;

pub struct PathMode {
    pub input: String,
    pub save_on_accept: bool,
}

impl PathMode {
    pub fn new(initial_path: String) -> PathMode {
        PathMode {
            input: initial_path,
            save_on_accept: false
        }
    }
    pub fn push_char(&mut self, c: char) {
        self.input.push(c);
    }
    pub fn pop_char(&mut self) {
        self.input.pop();
    }
}

impl fmt::Display for PathMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PATH")
    }
}



impl Mode for PathMode {
    fn mode_str(&self) -> Option<&'static str> {
            Some("path")
    }

    fn mode_id(&self) -> Option<&'static str> {
            Some("path")
    }

    fn present(&mut self, app :&mut Application) -> Result<()>{
        presenters::modes::path::display(&mut app.workspace, self, &mut app.view)
    }

    fn as_any(&self) -> &dyn Any{
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any{
        self
    }
}
