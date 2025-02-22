use crate::errors::*;
use crate::commands::{self, Result};
use crate::input::KeyMap;
use scribe::Buffer;
use std::mem;
use crate::models::application::Application;
use crate::models::application::modes::*;
use crate::util;

pub fn handle_input(app: &mut Application) -> Result {
    // Listen for and respond to user input.
    let commands = app.view.last_key().as_ref().and_then(|key| {
        app.mode_id().and_then(|mode| {
            app.preferences.borrow().keymap().commands_for(&mode, &key)
        })
    });

    if let Some(coms) = commands {
        // Run all commands, stopping at the first error encountered, if any.
        for com in coms {
            com(app)?;
        }
    }

    Ok(())
}

pub fn switch_to_normal_mode(app: &mut Application) -> Result {
    let _ = commands::buffer::end_command_group(app);
    app.clear_stack();

    Ok(())
}

pub fn switch_to_insert_mode(app: &mut Application) -> Result {
    if app.workspace.current_buffer().is_some() {
        commands::buffer::start_command_group(app)?;
        app.enter_mode(Box::new(InsertMode::new()));
        commands::view::scroll_to_cursor(app)?;
    } else {
        bail!(BUFFER_MISSING);
    }

    Ok(())
}

//TODO fix this cuz the stack
pub fn switch_to_jump_mode(app: &mut Application) -> Result {
/*    let buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;

    // Initialize a new jump mode and swap
    // it with the current application mode.
    let jump_mode = JumpMode::new(buffer.cursor.line);
    let old_mode = app.pop_mode();
    let old_mode = mem::replace(&mut app.mode, jump_mode);

    // If we were previously in a select mode, store it
    // in the current jump mode so that we can return to
    // it after we've jumped to a location. This is how
    // we compose select and jump modes.
    match old_mode {
        Mode::Select(select_mode) => {
            if let Mode::Jump(ref mut mode) = app.mode {
                mode.select_mode = jump::SelectModeOptions::Select(select_mode);
            }
        }
        Mode::SelectLine(select_mode) => {
            if let Mode::Jump(ref mut mode) = app.mode {
                mode.select_mode = jump::SelectModeOptions::SelectLine(select_mode);
            }
        }
        _ => (),
    };
*/
    Ok(())
}

pub fn switch_to_second_stage_jump_mode(app: &mut Application) -> Result {
 /*
    switch_to_jump_mode(app)?;
    if let Mode::Jump(ref mut mode) = app.mode {
        mode.first_phase = false;
    } else {
        bail!("Failed to switch to jump mode.");
    };
*/
    Ok(())
}

pub fn switch_to_line_jump_mode(app: &mut Application) -> Result {
    if app.workspace.current_buffer().is_some() {
        app.clear_stack();
        app.enter_mode(Box::new(LineJumpMode::new()));
    } else {
        bail!(BUFFER_MISSING);
    }

    Ok(())
}

pub fn switch_to_open_mode(app: &mut Application) -> Result {
    let exclusions = app.preferences.borrow().open_mode_exclusions()?;
    let config = app.preferences.borrow().search_select_config();
    app.clear_stack();
    app.enter_mode(Box::new( OpenMode::new(app.workspace.path.clone(), exclusions, app.event_channel.clone(), config)));
    commands::search_select::search(app)?;

    Ok(())
}

pub fn switch_to_command_mode(app: &mut Application) -> Result {
    let config = app.preferences.borrow().search_select_config();
    app.clear_stack();
    app.enter_mode(Box::new(CommandMode::new(config)));
    commands::search_select::search(app)?;

    Ok(())
}

pub fn switch_to_symbol_jump_mode(app: &mut Application) -> Result {
   
    app.clear_stack();
    if let Some(buf) = app.workspace.current_buffer() {
        let token_set = buf.tokens()
            .chain_err(|| "No tokens available for the current buffer")?;
        let config = app.preferences.borrow().search_select_config();

        app.mode_stack.push_front(Box::new(SymbolJumpMode::new(&token_set, config)));
    } else {
        bail!(BUFFER_MISSING);
    }
    commands::search_select::search(app)?;

    Ok(())
    
}

pub fn switch_to_theme_mode(app: &mut Application) -> Result {
    let config = app.preferences.borrow().search_select_config();
    app.clear_stack();
    app.enter_mode(
        Box::new( ThemeMode::new( app.view.theme_set.themes.keys().map(|k| k.to_string()).collect(), config))
    );
    commands::search_select::search(app)?;

    Ok(())
}

pub fn switch_to_select_mode(app: &mut Application) -> Result {
    app.clear_stack();
    if let Some(buffer) = app.workspace.current_buffer() {
        app.mode_stack.push_front(Box::new(SelectMode::new(*buffer.cursor.clone())));
    } else {
        bail!(BUFFER_MISSING);
    }

    Ok(())
}

pub fn switch_to_select_line_mode(app: &mut Application) -> Result {
    app.clear_stack();
    if let Some(buffer) = app.workspace.current_buffer() {
        app.mode_stack.push_front(Box::new(SelectLineMode::new(buffer.cursor.line)));
    } else {
        bail!(BUFFER_MISSING);
    }

    Ok(())
}

pub fn switch_to_search_mode(app: &mut Application) -> Result {
    if app.workspace.current_buffer().is_some() {
        app.clear_stack();
        app.enter_mode(Box::new( SearchMode::new(app.search_query.clone())));
    } else {
        bail!(BUFFER_MISSING);
    }

    Ok(())
}

pub fn switch_to_path_mode(app: &mut Application) -> Result {
    let path = app.workspace
        .current_buffer()
        .ok_or(BUFFER_MISSING)?
        .path.as_ref().map(|p|
            // The buffer has a path; use it.
            p.to_string_lossy().into_owned()
        ).unwrap_or_else(||
            // Default to the workspace directory.
            format!("{}/", app.workspace.path.to_string_lossy())
        );
    app.clear_stack();
    app.enter_mode(Box::new(PathMode::new(path)));
    

    Ok(())
}

pub fn switch_to_syntax_mode(app: &mut Application) -> Result {
    // We'll need a buffer to apply the syntax,
    // so check before entering syntax mode.
    let _ = app.workspace
        .current_buffer()
        .ok_or("Switching syntaxes requires an open buffer")?;

    let config = app.preferences.borrow().search_select_config();
    app.clear_stack();
    app.enter_mode(Box::new( SyntaxMode::new( app.workspace.
                                     syntax_set
                                     .syntaxes()
                                     .iter ()
                                     .map(|syntax| syntax.name.clone()).collect(), config)));
    commands::search_select::search(app)?;

    Ok(())
}

pub fn display_default_keymap(app: &mut Application) -> Result {
    commands::workspace::new_buffer(app)?;

    if let Some(buffer) = app.workspace.current_buffer() {
        buffer.insert(KeyMap::default_data());
    }

    Ok(())
}

pub fn display_quick_start_guide(app: &mut Application) -> Result {
    commands::workspace::new_buffer(app)?;

    if let Some(buffer) = app.workspace.current_buffer() {
        buffer.insert(include_str!("../../documentation/quick_start_guide"));
    }

    Ok(())
}

pub fn display_available_commands(app: &mut Application) -> Result {
    commands::workspace::new_buffer(app)?;

    if let Some(buffer) = app.workspace.current_buffer() {
        let command_hash = commands::hash_map();
        let mut command_keys = command_hash.keys().collect::<Vec<&&str>>();
        command_keys.sort();
        command_keys.reverse();
        for key in command_keys {
            buffer.insert(format!("{}\n", key));
        }
    }

    Ok(())
}

pub fn display_last_error(app: &mut Application) -> Result {
    let error = app.error.take().ok_or("No error to display")?;
    let scope_display_buffer = {
        let mut error_buffer = Buffer::new();
        // Add the proximate/contextual error.
        error_buffer.insert(
            format!("{}\n", error)
        );

        // Print the chain of other errors that led to the proximate error.
        for err in error.iter().skip(1) {
            error_buffer.insert(
                format!("caused by: {}", err)
            );
        }

        error_buffer
    };
    util::add_buffer(scope_display_buffer, app)
}

pub fn suspend(app: &mut Application) -> Result {
    app.view.suspend();

    Ok(())
}

pub fn exit(app: &mut Application) -> Result {
    app.mode_stack.clear();

    Ok(())
}
