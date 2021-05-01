//display
use crate::models::application::modes::mode::*;
use crate::errors::*;
use scribe::Workspace;
use crate::view::{Colors, StatusLineData, Style, View};

pub struct ConfirmMode {}

impl ConfirmMode {
    pub fn display(workspace: &mut Workspace, view: &mut View) -> Result<()> {
        let mut presenter = view.build_presenter()?;
        let buf = workspace.current_buffer().ok_or(BUFFER_MISSING)?;
        let data = buf.data();

        // Draw the visible set of tokens to the terminal.
        presenter.print_buffer(buf, &data, None, None)?;

        // Draw the status line as a search prompt.
        let confirmation = "Are you sure? (y/n)".to_string();
        presenter.print_status_line(&[
            StatusLineData {
                content: confirmation,
                style: Style::Bold,
                colors: Colors::Warning,
            }
        ]);

        // Render the changes to the screen.
        presenter.present();

        Ok(())
    }
}

impl Mode for ConfirmMode {
    fn get_mode_id() -> ModeID {
        return ModeID{id:Some("confirm"), present:present_func };
    }

    fn present_func(app :&mut Application) -> Result<()>{
        display(&mut app.workspace, &mut app.view)
    }

}

























