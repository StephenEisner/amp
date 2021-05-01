//@author = StephenEisner
use crate::models::application::Application;

pub trait Mode {
     fn get_mode_id() -> ModeID;
     fn present_func(app: &mut Application) -> Result<()>;
}

pub struct ModeID {
    pub id: Option<&'static str>,
    pub present: fn(&mut Application) -> Result<()>,
}

impl ModeID {
    pub fn get_id(&self) -> Option<&'static str>{
         return self.id;
    }
    pub fn set_id(&mut self, id: Option<&'static str>){
        self.id = id;
    }
}
