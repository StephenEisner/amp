//@author = StephenEisner
use std::any::Any;
use crate::models::application::Application;
use crate::errors::*;

pub trait Mode: Any{
     fn mode_str(&self) -> Option<&'static str>;
     fn mode_id(&self) -> Option<&'static str>;
     fn present(&mut self, app: &mut Application) -> Result<()>;
     fn as_any(&self) -> &dyn Any;
     fn as_any_mut(&mut self) -> &mut dyn Any;
}
