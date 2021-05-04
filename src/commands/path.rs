use crate::errors::*;
use crate::commands::{self, Result};
use crate::input::Key;
use crate::models::application::{Application};
use crate::models::application::modes::PathMode;
use std::path::PathBuf;

pub fn push_char(app: &mut Application) -> Result {
    let last_key = app.view.last_key().as_ref().ok_or("View hasn't tracked a key press")?;
    if let Key::Char(c) = *last_key {
        match app.unwrap_mode_mutable::<PathMode>(){
            Some(ref mut mode) => mode.push_char(c),
            None => bail!("Cannot push char outside of path mode"),
        };
    } else {
        bail!("Last key press wasn't a character");
    }
    Ok(())
}

pub fn pop_char(app: &mut Application) -> Result {
    match app.unwrap_mode_mutable::<PathMode>(){
        Some(ref mut mode) => mode.pop_char(),
        None => bail!("Cannot pop char outside of path mode"),
    };
    Ok(())
}

pub fn accept_path(app: &mut Application) -> Result {
    let mut curr_mode = app.mode_stack 
        .pop_front() 
        .unwrap();
    let path_mode_maybe = curr_mode
        .as_any_mut()
        .downcast_mut::<PathMode>();

    let save_on_accept =
        if let Some(mode) = path_mode_maybe {
            let current_buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;
            let path_name = mode.input.clone();
            if path_name.is_empty() {
                bail!("Please provide a non-empty path")
            }
            current_buffer.path = Some(PathBuf::from(path_name));
            mode.save_on_accept
        } else {
            bail!("Cannot accept path outside of path mode");
        };

    app.workspace.update_current_syntax().chain_err(||
        "Failed to update buffer's syntax definition"
    )?;
    app.clear_stack();

    if save_on_accept {
        commands::buffer::save(app)
    } else {
        Ok(())
    }
}

