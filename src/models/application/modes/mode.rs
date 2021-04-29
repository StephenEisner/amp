//@author = StephenEisner

pub trait MMode {
     fn get_mode_id(&self) -> ModeID;
}

pub struct ModeID {
    pub id: Option<&'static str>,
}

impl ModeID {
    pub fn get_id(&self) -> Option<&'static str>{
         return self.id;
    }
    pub fn set_id(&mut self, id: Option<&'static str>){
        self.id = id;
    }
}
