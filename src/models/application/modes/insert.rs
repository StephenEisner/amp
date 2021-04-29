use crate::models::application::modes::mode::*;

pub struct InsertMode {
    pub input: Option<char>,
}

impl InsertMode {
    pub fn new() -> InsertMode {
        InsertMode { input: None }
    }
}

impl MMode for InsertMode {
    fn get_mode_id(&self) -> ModeID {
        return ModeID{id:Some("insert")};
    }
}
