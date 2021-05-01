use crate::input::Key;

#[derive(Debug, PartialEq)]
pub enum Event {
    Key(Key),
    Resize,
    OpenModeIndexComplete(Index)
}
