use crate::errors::*;
use crate::input::Key;
use std::mem;
use crate::commands::Result;
use crate::models::application::modes::JumpMode;
use crate::models::application::Application;
use scribe::Workspace;

pub fn match_tag(app: &mut Application) -> Result {
    let mut curr_mode = app.mode_stack 
        .pop_front() 
        .unwrap();
    let jump_mode_maybe = curr_mode
        .as_any_mut()
        .downcast_mut::<JumpMode>();
    let result =
        match jump_mode_maybe{
            Some(jump_mode)=> {
                match jump_mode.input.len() {
                    0 => return Ok(()), // Not enough data to match to a position.
                    1 => {
                        if jump_mode.first_phase {
                            jump_to_tag(jump_mode, &mut app.workspace)
                        } else {
                            return Ok(()) // Not enough data to match to a position.
                        }
                    },
                    _ => jump_to_tag(jump_mode, &mut app.workspace),
                }
            }   
            None => {
                bail!("Can't match jump tags outside of jump mode.");
            }
        };
 //   switch_to_previous_mode(app);

    result
}

// Try to find a position for the input tag and jump to it.
fn jump_to_tag(jump_mode: &mut JumpMode, workspace: &mut Workspace) -> Result {
    let position = jump_mode
        .map_tag(&jump_mode.input)
        .ok_or("Couldn't find a position for the specified tag")?;
    let buffer = workspace.current_buffer().ok_or(BUFFER_MISSING)?;

    if !buffer.cursor.move_to(*position) {
        bail!("Couldn't move to the specified tag's position ({:?})", position)
    }

    Ok(())
}
/*
fn switch_to_previous_mode(app: &mut Application) {
    app.exit_mode();
}
*/

pub fn push_search_char(app: &mut Application) -> Result {
    let mut curr_mode = app.mode_stack 
        .pop_front() 
        .unwrap();
    let jump_mode_maybe = curr_mode
        .as_any_mut()
        .downcast_mut::<JumpMode>();

    if let Some(ref key) = *app.view.last_key() {
        match jump_mode_maybe{
                Some(mode) => {
                    match *key {
                        Key::Char('f') => {
                            if mode.first_phase {
                                mode.first_phase = false;
                            } else {
                                // Add the input to whatever we've received in jump mode so far.
                                mode.input.push('f');
                            }
                        },
                        Key::Char(c) => mode.input.push(c),
                        _ => bail!("Last key press wasn't a character")
                    }
            } 
            None => {
                bail!("Can't push jump character outside of jump mode")
            }
        }
    } else {
        bail!("View hasn't tracked a key press")
    }
    app.mode_stack.push_front(curr_mode);

    match_tag(app)
}
