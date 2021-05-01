use crate::models::application::modes::mode::*;
//display requirements
use crate::errors::*;
use crate::presenters::current_buffer_status_line_data;
use scribe::Workspace;
use crate::view::{Colors, StatusLineData, Style, View};

pub struct InsertMode {}

impl InsertMode {

    pub fn display(workspace: &mut Workspace, view: &mut View) -> Result<()> {
        let mut presenter = view.build_presenter()?;
        let buffer_status = current_buffer_status_line_data(workspace);
        let buf = workspace.current_buffer().ok_or(BUFFER_MISSING)?;
        let data = buf.data();

        // Draw the visible set of tokens to the terminal.
        presenter.print_buffer(buf, &data, None, None)?;

        presenter.print_status_line(&[
            StatusLineData {
                content: " INSERT ".to_string(),
                style: Style::Default,
                colors: Colors::Insert,
            },
            buffer_status
        ]);

        // Render the changes to the screen.
        presenter.present();

        Ok(())
    }
}

impl Mode for InsertMode {
    fn get_mode_id() -> ModeID {
        return ModeID{id:Some("insert"), present:present_func };
    }

    fn present_func(app :&mut Application) -> Result<()>{
        display(&mut app.workspace, &mut app.view)
    }

}
