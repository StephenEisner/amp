//@author = StephenEisner

use crate::models::application::modes::mode::*;


pub struct NormalMode {}

impl MMode for NormalMode {
    fn get_mode_id(&self) -> ModeID {
        return ModeID{id:Some("normal")};
    }
}
