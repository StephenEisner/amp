use crate::models::application::modes::mode::*;

#[derive(Default)]
pub struct LineJumpMode {
    pub input: String,
}

impl LineJumpMode {
    pub fn new() -> LineJumpMode {
        LineJumpMode::default()
    }
}

impl MMode for LineJumpMode{
    fn get_mode_id(&self) -> ModeID {
        return ModeID{id:Some("line_jump")};
    }
}

