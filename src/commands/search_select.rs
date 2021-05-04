use crate::errors::*;
use crate::commands::{self, application, Result};
use crate::input::Key;
use std::mem;
use crate::models::application::modes::open::DisplayablePath;
use crate::models::application::Application;
use crate::models::application::modes::*;

pub fn accept(app: &mut Application) -> Result {
/*    // Consume the application mode. This is necessary because the selection in
    // command mode needs to run against the application, but we can't hold the
    // reference to the selection and lend the app mutably to it at the time.
    let mut app_mode = mem::replace(&mut app.mode, Mode::Normal);*/

    //this one is a mess cuz there is no shared behavior
    
    let mut curr_mode = app.mode_stack 
        .pop_front() 
        .unwrap();
            
    match curr_mode.mode_id() {
        Some("command") => {
            let mode = curr_mode
                .as_any_mut()
                .downcast_mut::<CommandMode>()
                .unwrap();
            let selection = mode.selection().ok_or("No command selected")?;

            // Run the selected command.
            (selection.command)(app)?;
        },
        Some("open") => {
            let mode = curr_mode
                .as_any_mut()
                .downcast_mut::<OpenMode>()
                .unwrap();
            let &DisplayablePath(ref path) = mode 
                .selection()
                .ok_or("Couldn't find a selected path to open")?;

            let syntax_definition =
                app.preferences.borrow().syntax_definition_name(&path).and_then(|name| {
                    app.workspace.syntax_set.find_syntax_by_name(&name).cloned()
                });

            app.workspace
                .open_buffer(&path)
                .chain_err(|| "Couldn't open a buffer for the specified path.")?;

            let buffer = app.workspace.current_buffer().unwrap();

            // Only override the default syntax definition if the user provided
            // a valid one in their preferences.
            if syntax_definition.is_some() {
                buffer.syntax_definition = syntax_definition;
            }

            app.view.initialize_buffer(buffer)?;

        },
        Some("theme") => {
            let mode = curr_mode
                .as_any_mut()
                .downcast_mut::<ThemeMode>()
                .unwrap();
            let theme_key = mode.selection().ok_or("No theme selected")?;
            app.preferences.borrow_mut().set_theme(theme_key.as_str());
        },
        Some("symbol_jump") => {
            let mode = curr_mode
                .as_any_mut()
                .downcast_mut::<SymbolJumpMode>()
                .unwrap();
            let buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;
            let position = mode 
                .selection()
                .ok_or("Couldn't find a position for the selected symbol")?
                .position;

            if !buffer.cursor.move_to(position) {
                bail!("Couldn't move to the selected symbol's position");
            }
        },
        Some("syntax") => {
            let mode = curr_mode
                .as_any_mut()
                .downcast_mut::<SyntaxMode>()
                .unwrap();
            let name = mode.selection().ok_or("No syntax selected")?;
            let syntax =
                app.workspace.syntax_set.find_syntax_by_name(name).and_then(|s|
                    Some(s.clone())
                );
            let mut buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;
            buffer.syntax_definition = syntax;
        },
        _ => bail!("Can't accept selection outside of search select mode."),
    }

    commands::view::scroll_cursor_to_center(app).ok();
//might need an app.clear_stack here cuz that stuff at the begining of this function that i
//commented out, the mem replace in particular
    Ok(())
}

pub fn search(app:&mut Application) -> Result {


    match app.mode_id(){
        Some("open") => app.unwrap_mode_mutable::<OpenMode>().unwrap().search(),
        Some("command") => app.unwrap_mode_mutable::<CommandMode>().unwrap().search(),
        Some("theme") =>app.unwrap_mode_mutable::<ThemeMode>().unwrap().search(),
        Some("symbol_jump") =>app.unwrap_mode_mutable::<SymbolJumpMode>().unwrap().search(),
        Some("syntax") =>app.unwrap_mode_mutable::<SyntaxMode>().unwrap().search(),
        _ =>  bail!("Can't search outside of search select mode."),
    };



    Ok(())
}

pub fn select_next(app: &mut Application) -> Result {
    match app.mode_id(){
        Some("open") => app.unwrap_mode_mutable::<OpenMode>().unwrap().select_next(),
        Some("command") => app.unwrap_mode_mutable::<CommandMode>().unwrap().select_next(),
        Some("theme") =>app.unwrap_mode_mutable::<ThemeMode>().unwrap().select_next(),
        Some("symbol_jump") =>app.unwrap_mode_mutable::<SymbolJumpMode>().unwrap().select_next(),
        Some("syntax") =>app.unwrap_mode_mutable::<SyntaxMode>().unwrap().select_next(),
        _ =>  bail!("Can't select_next outside of search select mode."),
    };

    Ok(())
}

pub fn select_previous(app: &mut Application) -> Result {
    match app.mode_id(){
        Some("open") => app.unwrap_mode_mutable::<OpenMode>().unwrap().select_previous(),
        Some("command") => app.unwrap_mode_mutable::<CommandMode>().unwrap().select_previous(),
        Some("theme") =>app.unwrap_mode_mutable::<ThemeMode>().unwrap().select_previous(),
        Some("symbol_jump") =>app.unwrap_mode_mutable::<SymbolJumpMode>().unwrap().select_previous(),
        Some("syntax") =>app.unwrap_mode_mutable::<SyntaxMode>().unwrap().select_previous(),
        _ =>  bail!("Can't select_previous() outside of search select mode."),
    };

    Ok(())
}

pub fn enable_insert(app: &mut Application) -> Result {

    match app.mode_id(){
        Some("open") => app.unwrap_mode_mutable::<OpenMode>().unwrap().set_insert_mode(true),
        Some("command") => app.unwrap_mode_mutable::<CommandMode>().unwrap().set_insert_mode(true),
        Some("theme") =>app.unwrap_mode_mutable::<ThemeMode>().unwrap().set_insert_mode(true),
        Some("symbol_jump") =>app.unwrap_mode_mutable::<SymbolJumpMode>().unwrap().set_insert_mode(true),
        Some("syntax") =>app.unwrap_mode_mutable::<SyntaxMode>().unwrap().set_insert_mode(true),
        _ =>  bail!("Can't set_insert_mode(true)() outside of search select mode."),
    };

    Ok(())
}

pub fn disable_insert(app: &mut Application) -> Result {

    match app.mode_id(){
        Some("open") => app.unwrap_mode_mutable::<OpenMode>().unwrap().set_insert_mode(false),
        Some("command") => app.unwrap_mode_mutable::<CommandMode>().unwrap().set_insert_mode(false),
        Some("theme") =>app.unwrap_mode_mutable::<ThemeMode>().unwrap().set_insert_mode(false),
        Some("symbol_jump") =>app.unwrap_mode_mutable::<SymbolJumpMode>().unwrap().set_insert_mode(false),
        Some("syntax") =>app.unwrap_mode_mutable::<SyntaxMode>().unwrap().set_insert_mode(false),
        _ =>  bail!("Can't set_insert_mode(false)() outside of search select mode."),
    };



    Ok(())
}

pub fn push_search_char(app: &mut Application) -> Result {

    if let Some(Key::Char(c)) = *app.view.last_key() {
        match app.mode_id() {
             Some("open") => app.unwrap_mode_mutable::<OpenMode>().unwrap().push_search_char(c),
             Some("command") => app.unwrap_mode_mutable::<CommandMode>().unwrap().push_search_char(c),
             Some("theme") =>app.unwrap_mode_mutable::<ThemeMode>().unwrap().push_search_char(c),
             Some("symbol_jump") =>app.unwrap_mode_mutable::<SymbolJumpMode>().unwrap().push_search_char(c),
             Some("syntax") =>app.unwrap_mode_mutable::<SyntaxMode>().unwrap().push_search_char(c),
            _ => bail!("Can't push search character outside of search select mode"),
        }
    }


    // Re-run the search.
    search(app)
}

pub fn pop_search_token(app: &mut Application) -> Result {

    match app.mode_id() {
         Some("open") => app.unwrap_mode_mutable::<OpenMode>().unwrap().pop_search_token(),
         Some("command") => app.unwrap_mode_mutable::<CommandMode>().unwrap().pop_search_token(),
         Some("theme") =>app.unwrap_mode_mutable::<ThemeMode>().unwrap().pop_search_token(),
         Some("symbol_jump") =>app.unwrap_mode_mutable::<SymbolJumpMode>().unwrap().pop_search_token(),
         Some("syntax") =>app.unwrap_mode_mutable::<SyntaxMode>().unwrap().pop_search_token(),
        _ => bail!("Can't push search character outside of search select mode"),
    }



    search(app)?;
    Ok(())
}

pub fn step_back(app: &mut Application) -> Result {

    let result_count =
        match app.mode_id() {
             Some("open") => app.unwrap_mode_mutable::<OpenMode>().unwrap().results().count(),
             Some("command") => app.unwrap_mode_mutable::<CommandMode>().unwrap().results().count(),
             Some("theme") =>app.unwrap_mode_mutable::<ThemeMode>().unwrap().results().count(),
             Some("symbol_jump") =>app.unwrap_mode_mutable::<SymbolJumpMode>().unwrap().results().count(),
             Some("syntax") =>app.unwrap_mode_mutable::<SyntaxMode>().unwrap().results().count(),
            _ => bail!("Can't push search character outside of search select mode"),
        };


    if result_count == 0 {
        application::switch_to_normal_mode(app)
    } else {
        disable_insert(app)
    }
}

