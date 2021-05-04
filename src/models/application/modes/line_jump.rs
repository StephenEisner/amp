use std::any::Any;
use crate::errors::*;
use crate::models::application::modes::mode::Mode;
use crate::models::application::Application;
use crate::presenters;

#[derive(Default)]
pub struct LineJumpMode {
    pub input: String,
}

impl LineJumpMode {
    pub fn new() -> LineJumpMode {
        LineJumpMode::default()
    }
}

impl Mode for LineJumpMode {
    fn mode_str(&self) -> Option<&'static str> {
        Some("line_jump")
    }

    fn mode_id(&self) -> Option<&'static str> {
        Some("line_jump")
    }

    fn present(&mut self, app :&mut Application) -> Result<()>{
        presenters::modes::line_jump::display(&mut app.workspace, self, &mut app.view)
    }

    fn as_any(&self) -> &dyn Any{
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any{
        self
    }
}

