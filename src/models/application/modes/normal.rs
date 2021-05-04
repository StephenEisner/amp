use std::any::Any;
use crate::errors::*;
//@author = StephenEisner
//done?
use crate::models::application::modes::mode::Mode;
use crate::models::application::Application;
//Display requirements
use crate::presenters;


pub struct NormalMode {}

impl NormalMode {
    pub fn new() -> NormalMode {
        NormalMode{}
    }
}

impl Mode for NormalMode {

    fn mode_str(&self) -> Option<&'static str> {
        Some("normal")
    }

    fn mode_id(&self) -> Option<&'static str> {
        Some("normal")
    }

    fn present(&mut self, app :&mut Application) -> Result<()>{
     presenters::modes::normal::display( &mut app.workspace, &mut app.view, &app.repository,)
    }

    fn as_any(&self) -> &dyn Any{
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any{
        self
    }
}
