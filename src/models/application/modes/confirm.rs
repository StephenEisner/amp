use std::any::Any;
use crate::errors::*;
use crate::commands::Command;
use crate::presenters;
use crate::models::application::modes::mode::Mode;
use crate::models::application::Application;

pub struct ConfirmMode {
    pub command: Command,
}

impl ConfirmMode {
    pub fn new(command: Command) -> ConfirmMode {
        ConfirmMode { command }
    }
}

impl Mode for ConfirmMode {
    fn mode_str(&self) -> Option<&'static str> {
        Some("confirm")
    }

    fn mode_id(&self) -> Option<&'static str> {
        Some("confirm")
    }

    fn present(&mut self, app :&mut Application) -> Result<()>{
        presenters::modes::confirm::display(&mut app.workspace, &mut app.view)
    }

    fn as_any(&self) -> &dyn Any{
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any{
        self
    }
}
