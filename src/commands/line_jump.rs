use crate::errors::*;
use crate::input::Key;
use crate::commands::{self, Result};
use crate::models::application::Application;
use crate::models::application::modes::LineJumpMode;
use scribe::buffer::Position;

pub fn accept_input(app: &mut Application) -> Result {
    match app.unwrap_mode::<LineJumpMode>(){
        Some(ref mode)  => {
            // Try parsing an integer from the input.
            let line_number = mode
                .input
                .parse::<usize>()
                .chain_err(|| "Couldn't parse a line number from the provided input.")?;

            // Ignore zero-value line numbers.
            if line_number > 0 {
                let buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;

                // Input values won't be zero-indexed; map the value so
                // that we can use it for a zero-indexed buffer position.
                let target_line = line_number - 1;

                // Build an ideal target position to which we'll try moving.
                let mut target_position = Position {
                    line: target_line,
                    offset: buffer.cursor.offset,
                };

                if !buffer.cursor.move_to(target_position) {
                    // Moving to that position failed. It may be because the
                    // current offset doesn't exist there. Try falling back
                    // to the end of the target line.
                    let line_content = buffer
                        .data()
                        .lines()
                        .nth(target_line)
                        .map(|line| line.to_string())
                        .ok_or("Couldn't find the specified line")?;

                    target_position.offset = line_content.len();
                    buffer.cursor.move_to(target_position);
                }
            }
        }
        None => {
            bail!("Can't accept line jump input outside of line jump mode.");
        }
    };

    commands::application::switch_to_normal_mode(app)?;
    commands::view::scroll_cursor_to_center(app)?;

    Ok(())
}

pub fn push_search_char(app: &mut Application) -> Result {
    let key = app.view.last_key().as_ref().ok_or("View hasn't tracked a key press")?;

    if let Key::Char(c) = *key {
        match app.unwrap_mode_mutable::<LineJumpMode>(){
            Some(ref mut mode) => {
                mode.input.push(c);
            }
            None => {
                bail!("Can't push search character outside of search insert mode")
            }
        };
    } else {
        bail!("Last key press wasn't a character")
    }

    Ok(())
}

pub fn pop_search_char(app: &mut Application) -> Result {

    match app.unwrap_mode_mutable::<LineJumpMode>(){
        Some(ref mut mode) => {
            mode.input.pop();
        }
        None => {
            bail!("Can't pop search character outside of search insert mode")
        }
    };

    Ok(())
}

