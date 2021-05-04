use crate::commands::{self, Result};
use crate::models::application::Application;
use crate::models::application::modes::ConfirmMode;

pub fn confirm_command(app: &mut Application) -> Result {
    let command =
      if Some("confirm") == app.mode_id() {
          app.unwrap_mode::<ConfirmMode>().unwrap().command
      } else {
          bail!("Can't confirm command outside of confirm mode");
      };

    command(app)?;
    commands::application::switch_to_normal_mode(app)
}
