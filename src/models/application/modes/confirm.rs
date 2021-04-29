use crate::commands::Command;
use crate::models::application::modes::mode::*;

pub struct ConfirmMode {
    pub command: Command,
}

impl ConfirmMode {
    pub fn new(command: Command) -> ConfirmMode {
        ConfirmMode { command }
    }
}
impl MMode for ConfirmMode {
    fn get_mode_id(&self) -> ModeID {
        return ModeID{id:Some("confirm")};
    }
}
