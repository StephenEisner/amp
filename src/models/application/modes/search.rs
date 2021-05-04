use std::any::Any;
use crate::errors::*;
use crate::models::application::Application;
use crate::util::SelectableVec;
use std::fmt;
use scribe::buffer::{Buffer, Distance, Range};

use crate::models::application::modes::mode::Mode;
use crate::presenters;
pub struct SearchMode {
    pub insert: bool,
    pub input: Option<String>,
    pub results: Option<SelectableVec<Range>>,
}

impl SearchMode {
    pub fn new(query: Option<String>) -> SearchMode {
        SearchMode {
            insert: true,
            input: query,
            results: None,
        }
    }

    pub fn insert_mode(&self) -> bool {
        self.insert
    }

    // Searches the specified buffer for the input string
    // and stores the result as a collection of ranges.
    pub fn search(&mut self, buffer: &Buffer) -> Result<()> {
        let query = self.input.as_ref().ok_or(SEARCH_QUERY_MISSING)?;
        let distance = Distance::of_str(&query);

        // Buffer search returns match starting positions, but we'd like ranges.
        // This maps the positions to ranges using the search query distance
        // before storing them.
        self.results = Some(
            SelectableVec::new(
                buffer.search(&query)
                    .into_iter()
                    .map(|start| Range::new(start, start + distance))
                    .collect()
            )
        );

        Ok(())
    }
}

impl fmt::Display for SearchMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SEARCH")
    }
}

impl Mode for SearchMode {
    fn mode_str(&self) -> Option<&'static str> {
            if self.insert_mode() {
                Some("search_insert")
            } else {
                Some("search")
            }
    }

    fn mode_id(&self) -> Option<&'static str> {
        Some("search")
    }

    fn present(&mut self, app :&mut Application) -> Result<()>{
            presenters::modes::search::display(&mut app.workspace, self, &mut app.view)
    }

    fn as_any(&self) -> &dyn Any{
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any{
        self
    }
}

#[cfg(test)]
mod tests {
    use scribe::buffer::{Buffer, Position, Range};
    use super::SearchMode;

    #[test]
    fn search_populates_results_with_correct_ranges() {
        let mut buffer = Buffer::new();
        buffer.insert("test\ntest");

        let mut mode = SearchMode::new(Some(String::from("test")));
        mode.search(&buffer).unwrap();

        assert_eq!(
            *mode.results.unwrap(),
            vec![
                Range::new(
                    Position{ line: 0, offset: 0 },
                    Position{ line: 0, offset: 4 },
                ),
                Range::new(
                    Position{ line: 1, offset: 0 },
                    Position{ line: 1, offset: 4 },
                ),
            ]
        );
    }
}
