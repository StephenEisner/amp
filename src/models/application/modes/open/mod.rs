use std::any::Any;
use crate::errors::*;
mod displayable_path;
pub mod exclusions;

use crate::models::application::Application;
use std::fmt;
use std::path::PathBuf;
use std::slice::Iter;
use bloodhound::ExclusionPattern;
use crate::util::SelectableVec;
use crate::models::application::modes::{SearchSelectMode, SearchSelectConfig};
use crate::models::application::Event;
use std::sync::mpsc::Sender;
use std::thread;
pub use bloodhound::Index;
pub use self::displayable_path::DisplayablePath;

use crate::models::application::modes::mode::Mode;
use crate::presenters;

#[derive(PartialEq)]
pub enum OpenModeIndex {
    Complete(Index),
    Indexing(PathBuf)
}

pub struct OpenMode {
    pub insert: bool,
    pub input: String,
    index: OpenModeIndex,
    pub results: SelectableVec<DisplayablePath>,
    config: SearchSelectConfig,
}

impl OpenMode {
    pub fn new(path: PathBuf, exclusions: Option<Vec<ExclusionPattern>>, events: Sender<Event>, config: SearchSelectConfig) -> OpenMode {
        // Build and populate the index in a separate thread.
        let index_path = path.clone();
        thread::spawn(move || {
            let mut index = Index::new(index_path);
            index.populate(exclusions, false);
            let _ = events.send(
                Event::OpenModeIndexComplete(index)
            );
        });

        OpenMode {
            insert: true,
            input: String::new(),
            index: OpenModeIndex::Indexing(path),
            results: SelectableVec::new(Vec::new()),
            config,
        }
    }

    pub fn set_index(&mut self, index: Index) {
        self.index = OpenModeIndex::Complete(index)
    }
}

impl Mode for OpenMode {

    fn as_any(&self) -> &dyn Any{
        self
    }

    fn mode_str(&self) -> Option<&'static str> {
            if self.insert_mode() {
                Some("search_select_insert")
            } else {
                Some("search_select")
            }
    }

    fn mode_id(&self) -> Option<&'static str> {
        Some("open")
    }

    fn present(&mut self, app :&mut Application) -> Result<()>{
        presenters::modes::search_select::display(&mut app.workspace, self, &mut app.view)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any{
        self
    }
}

impl fmt::Display for OpenMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OPEN")
    }
}

impl SearchSelectMode<DisplayablePath> for OpenMode {
    fn search(&mut self) {
        let results =
            if let OpenModeIndex::Complete(ref index) = self.index {
                index.find(
                    &self.input.to_lowercase(),
                    self.config.max_results
                ).into_iter()
                .map(|path| DisplayablePath(path.to_path_buf()))
                .collect()
            } else {
                vec![]
            };

        self.results = SelectableVec::new(results);
    }

    fn query(&mut self) -> &mut String {
        &mut self.input
    }

    fn insert_mode(&self) -> bool {
        self.insert
    }

    fn set_insert_mode(&mut self, insert_mode: bool) {
        self.insert = insert_mode;
    }

    fn results(&self) -> Iter<DisplayablePath> {
        self.results.iter()
    }

    fn selection(&self) -> Option<&DisplayablePath> {
        self.results.selection()
    }

    fn selected_index(&self) -> usize {
        self.results.selected_index()
    }

    fn select_previous(&mut self) {
        self.results.select_previous();
    }

    fn select_next(&mut self) {
        self.results.select_next();
    }

    fn config(&self) -> &SearchSelectConfig {
        &self.config
    }

    fn message(&mut self) -> Option<String> {
        if let OpenModeIndex::Indexing(ref path) = self.index {
            Some(format!("Indexing {}", path.to_string_lossy()))
        } else if self.query().is_empty() {
            Some(String::from("Enter a search query to start."))
        } else if self.results().count() == 0 {
            Some(String::from("No matching entries found."))
        } else {
            None
        }
    }
}
