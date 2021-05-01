use std::fmt;
use crate::models::application::modes::mode::*;
//display
use crate::errors::*;
use scribe::Workspace;
use scribe::buffer::Position;
use unicode_segmentation::UnicodeSegmentation;
use crate::view::{Colors, StatusLineData, Style, View};

pub struct PathMode {

}

impl PathMode {
    pub fn display(workspace: &mut Workspace, input_buffer: &Option<String>, view: &mut View) -> Result<()> {
        let mut presenter = view.build_presenter()?;

        // Draw the visible set of tokens to the terminal.
        let buffer = workspace.current_buffer().ok_or(BUFFER_MISSING)?;
        let data = buffer.data();
        presenter.print_buffer(buffer, &data, None, None)?;

        let mode_display = format!(" {} ","PATH");
        let search_input = format!(
            " {:?}",
            input_buffer 
        );

        let cursor_offset =
            mode_display.graphemes(true).count() +
            search_input.graphemes(true).count();

        presenter.print_status_line(&[
            StatusLineData {
                content: mode_display,
                style: Style::Default,
                colors: Colors::PathMode,
            },
            StatusLineData {
                content: search_input,
                style: Style::Default,
                colors: Colors::Focused,
            },
        ]);

        // Move the cursor to the end of the search query input.
        {
            let cursor_line = presenter.height() - 1;
            presenter.set_cursor(Some(Position {
                line: cursor_line,
                offset: cursor_offset
            }));
        }

        // Render the changes to the screen.
        presenter.present();

        Ok(())
    }
}

impl Mode for PathMode {
    fn get_mode_id() -> ModeID {
        return ModeID{id:Some("path"), present:present_func };
    }

    fn present_func(app :&mut Application) -> Result<()>{
        display(&mut app.workspace, &app.string_buffer, &mut app.view)
    }
}
