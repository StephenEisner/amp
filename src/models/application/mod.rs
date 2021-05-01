mod clipboard;
mod event;
pub mod modes;
mod preferences;

// Published API
pub use self::clipboard::ClipboardContent;
pub use self::event::Event;
pub use self::preferences::Preferences;

use self::clipboard::Clipboard;
use self::modes::*;
use crate::commands;
use crate::errors::*;
use git2::Repository;
use crate::presenters;
use scribe::{Buffer, Workspace};
use std::cell::RefCell;
use std::env;
use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, Sender};
use crate::view::View;



pub struct Application {
    pub mode: ModeID,
    pub workspace: Workspace,
    pub string_buffer: Option<String>,
    pub view: View,
    pub clipboard: Clipboard,
    pub repository: Option<Repository>,
    pub error: Option<Error>,
    pub preferences: Rc<RefCell<Preferences>>,
    pub event_channel: Sender<Event>,
    events: Receiver<Event>,
    pub command_buffer: Command,
    pub save_on_accept: bool,
}

impl Application {
    pub fn new(args: &Vec<String>) -> Result<Application> {
        let preferences = initialize_preferences();

        let (event_channel, events) = mpsc::channel();
        let mut view = View::new(preferences.clone(), event_channel.clone())?;
        let clipboard = Clipboard::new();

        // Set up a workspace in the current directory.
        let workspace = create_workspace(&mut view, &preferences.borrow(), args)?;

        Ok(Application {
            mode: NormalMode::get_mode_id(),
            workspace,
            string_buffer: None,
            view,
            clipboard,
            repository: Repository::discover(&env::current_dir()?).ok(),
            error: None,
            preferences,
            event_channel,
            events,
            save_on_accept: true,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.render();
            self.wait_for_event()?;

            if let Mode::Exit = self.mode {
                break;
            }
        }

        Ok(())
    }

    fn render(&mut self) {
        if let Err(error) = self.present() {
            presenters::error::display(&mut self.workspace, &mut self.view, &error);
        } else if let Some(ref error) = self.error {
            // Display an error from previous command invocation, if one exists.
            presenters::error::display(&mut self.workspace, &mut self.view, error);
        }
    }

    fn present(&mut self) -> Result<()> {
        self.mode.present(&self)
    }

    fn wait_for_event(&mut self) -> Result<()> {
        let event = self
            .events
            .recv()
            .chain_err(|| "Error receiving application event")?;
        match event {
            Event::Key(key) => {
                self.view.last_key = Some(key);
                self.error = commands::application::handle_input(self).err();
            }
            Event::Resize => {}
            Event::OpenModeIndexComplete(index) => {
                if let Mode::Open(ref mut open_mode) = self.mode {
                    open_mode.set_index(index);

                    // Trigger a search, in case a query was
                    // entered while we were indexing.
                    open_mode.search();
                }
            }
        }

        Ok(())
    }

    pub fn mode_str(&self) -> Option<&'static str> {
        self.mode.get_id()
    }

}

fn initialize_preferences() -> Rc<RefCell<Preferences>> {
    Rc::new(RefCell::new(
        Preferences::load().unwrap_or_else(|_| Preferences::new(None)),
    ))
}

fn create_workspace(view: &mut View, preferences: &Preferences, args: &Vec<String>) -> Result<Workspace> {
    // Discard the executable portion of the argument list.
    let mut path_args = args.iter().skip(1).peekable();

    // Move into an argument-specified directory, if present.
    let initial_dir = env::current_dir()?;
    if let Some(arg) = path_args.peek() {
        let path = Path::new(&arg);

        if path.is_dir() {
            env::set_current_dir(path.canonicalize()?)?;
        }
    }

    let workspace_dir = env::current_dir()?;
    let mut workspace = Workspace::new(&workspace_dir)?;

    // Load user syntax definitions.
    //
    // It's important to do this before opening buffers, as that's when syntax
    // definitions are associated; we want the complete set before that happens.
    let syntax_path = Preferences::syntax_path()?;
    if let Err(e) = workspace.syntax_set.load_syntaxes(syntax_path, true) {
        bail!("Failed to load user syntaxes: {:?}", e);
    }
    workspace.syntax_set.link_syntaxes();

    // If the first argument was a directory, we've navigated into
    // it; skip it before evaluating file args, lest we interpret
    // it again as a non-existent file and create a buffer for it.
    if workspace_dir != initial_dir { path_args.next(); }

    // Try to open specified files.
    for path_arg in path_args {
        let path = Path::new(&path_arg);

        if path.is_dir() { continue; }

        // Check if the user has provided any syntax preference for this file.
        // If not, a default one will be applied on calling workspace.add_buffer()
        let syntax_definition =
            preferences.syntax_definition_name(&path).and_then(|name| {
                workspace.syntax_set.find_syntax_by_name(&name).cloned()
            });

        // Open the specified path if it exists, or
        // create a new buffer pointing to it if it doesn't.
        let argument_buffer = if path.exists() {
            let mut buffer = Buffer::from_file(path)?;
            buffer.syntax_definition = syntax_definition;

            buffer
        } else {
            let mut buffer = Buffer::new();
            buffer.syntax_definition = syntax_definition;

            // Point the buffer to the path, ensuring that it's absolute.
            if path.is_absolute() {
                buffer.path = Some(path.to_path_buf());
            } else {
                buffer.path = Some(workspace.path.join(path));
            }

            buffer
        };

        workspace.add_buffer(argument_buffer);
        view.initialize_buffer(workspace.current_buffer().unwrap())?;
    }

    Ok(workspace)
}

#[cfg(test)]
mod tests {
    use super::Application;
    use crate::view::View;
    use super::preferences::Preferences;

    use yaml::YamlLoader;
    use scribe::Buffer;
    use std::cell::RefCell;
    use std::env;
    use std::path::Path;
    use std::rc::Rc;
    use std::sync::mpsc;

    #[test]
    fn application_uses_file_arguments_to_load_contents_into_buffers_when_files_exist() {
        let mut application =
            Application::new(&vec![String::new(), String::from("Cargo.lock")]).unwrap();
        let buffer = Buffer::from_file(Path::new("Cargo.lock")).unwrap();

        assert_eq!(
            application.workspace.current_buffer().unwrap().path,
            buffer.path
        );
        assert_eq!(
            application.workspace.current_buffer().unwrap().data(),
            buffer.data()
        );
    }

    #[test]
    fn application_uses_file_arguments_to_create_new_buffers_when_files_do_not_exist() {
        let mut application =
            Application::new(&vec![String::new(), String::from("non_existent_file")]).unwrap();

        assert_eq!(
            application.workspace.current_buffer().unwrap().path,
            Some(env::current_dir().unwrap().join("non_existent_file"))
        );
        assert_eq!(application.workspace.current_buffer().unwrap().data(), "");
    }

    #[test]
    fn create_workspace_correctly_applies_user_defined_syntax_when_opening_buffer_from_command_line() {
        let data = YamlLoader::load_from_str("types:\n  xyz:\n    syntax: Rust").unwrap();
        let preferences = Rc::new(RefCell::new(Preferences::new(data.into_iter().nth(0))));
        let (event_channel, _) = mpsc::channel();
        let mut view = View::new(preferences.clone(), event_channel.clone()).unwrap();

        let args = vec![String::new(), String::from("src/test.xyz")];
        let mut workspace = super::create_workspace(&mut view, &preferences.borrow(), &args).unwrap();

        assert_eq!(
            workspace.current_buffer().unwrap().syntax_definition.as_ref().unwrap().name,
            "Rust"
        );
    }
}
