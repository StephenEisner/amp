use crate::errors::*;
use crate::input::Key;
use crate::commands::{self, Result};
use crate::models::application::Application;
use crate::models::application::modes::SearchMode;

pub fn move_to_previous_result(app: &mut Application) -> Result {
    if Some("search") == app.mode_id() {
        app.unwrap_mode_mutable::<SearchMode>().unwrap().results.as_mut().ok_or(NO_SEARCH_RESULTS)?.select_previous();
    } else {
        bail!("Can't move to search result outside of search mode");
    }

    commands::view::scroll_cursor_to_center(app)
        .chain_err(|| SCROLL_TO_CURSOR_FAILED)?;
    move_to_current_result(app)
}

pub fn move_to_next_result(app: &mut Application) -> Result {
    if Some("search") == app.mode_id() {
        app.unwrap_mode_mutable::<SearchMode>().unwrap().results.as_mut().ok_or(NO_SEARCH_RESULTS)?.select_next();
    } else {
        bail!("Can't move to search result outside of search mode");
    }

    commands::view::scroll_cursor_to_center(app)
        .chain_err(|| SCROLL_TO_CURSOR_FAILED)?;
    move_to_current_result(app)
}

pub fn move_to_current_result(app: &mut Application) -> Result {
    let mut curr_mode = app.mode_stack 
        .pop_front() 
        .unwrap();
    let search_mode_maybe = curr_mode
        .as_any_mut()
        .downcast_mut::<SearchMode>();
    if let Some(mode) = search_mode_maybe {
        let buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;
        let query = mode.input.as_ref().ok_or(SEARCH_QUERY_MISSING)?;
        let result = mode.results
            .as_mut()
            .ok_or(NO_SEARCH_RESULTS)?
            .selection()
            .ok_or_else(|| format!("No matches found for \"{}\"", query))?;
        buffer.cursor.move_to(result.start());
    } else {
        bail!("Can't move to search result outside of search mode");
    }
    app.mode_stack.push_front(curr_mode);

    commands::view::scroll_cursor_to_center(app)
        .chain_err(|| SCROLL_TO_CURSOR_FAILED)?;

    Ok(())
}

pub fn accept_query(app: &mut Application) -> Result {
    if Some("search") == app.mode_id() {
        // Disable insert sub-mode.
        app.unwrap_mode_mutable::<SearchMode>().unwrap().insert = false;
    } else {
        bail!("Can't accept search query outside of search mode");
    }
    run(app)?;

    Ok(())
}

pub fn clear_query(app: &mut Application) -> Result {
    if Some("search") == app.mode_id() {
        app.unwrap_mode_mutable::<SearchMode>().unwrap().input = None;
        app.search_query = None;
    } else {
        bail!("Can't clear search outside of search mode");
    };

    Ok(())
}

pub fn push_search_char(app: &mut Application) -> Result {
    let key = app.view.last_key().as_ref().ok_or("View hasn't tracked a key press")?;

    if let Key::Char(c) = *key {
        if Some("search") == app.mode_id() {
            let mode = app.unwrap_mode_mutable::<SearchMode>().unwrap();
            let query = mode.input.get_or_insert(String::new());
            query.push(c);
            app.search_query = Some(query.clone());
        } else {
            bail!("Can't push search character outside of search mode");
        }
    } else {
        bail!("Last key press wasn't a character")
    }

    Ok(())
}

pub fn pop_search_char(app: &mut Application) -> Result {
    if Some("search") == app.mode_id() {
        let mode = app.unwrap_mode_mutable::<SearchMode>().unwrap();
        let query = mode.input.as_mut().ok_or(SEARCH_QUERY_MISSING)?;

        query.pop();
        app.search_query = Some(query.clone());
    } else {
        bail!("Can't pop search character outside of search mode");
    };

    Ok(())
}

pub fn run(app: &mut Application) -> Result {
    let mut curr_mode = app.mode_stack 
        .pop_front() 
        .unwrap();
    let search_mode_maybe = curr_mode
        .as_any_mut()
        .downcast_mut::<SearchMode>();

    if let Some(mode) = search_mode_maybe {
        // Search the buffer.
        let buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;
        mode.search(&buffer)?;
    } else {
        bail!("Can't run search outside of search mode");
    }
    app.mode_stack.push_front(curr_mode);

    select_closest_result(app)?;
    move_to_current_result(app)
}

fn select_closest_result(app: &mut Application) -> Result {
    let mut curr_mode = app.mode_stack 
        .pop_front() 
        .unwrap();
    let search_mode_maybe = curr_mode
        .as_any_mut()
        .downcast_mut::<SearchMode>();


    if let Some(mode) = search_mode_maybe {
        let buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;
        let results = mode.results.as_mut().ok_or(NO_SEARCH_RESULTS)?;

        // Skip over previous entries.
        let skip_count = results
            .iter()
            .filter(|r| r.start() <= *buffer.cursor)
            .count();
        for _ in 0..skip_count {
            results.select_next();
        }
    }

    Ok(())
}

//TODO go back and re add tests
