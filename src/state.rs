use druid::{Data, Lens};
use std::sync::Arc;

#[derive(Clone, Data, PartialEq)]
pub struct SearchResult {
    pub icon_path: String,
    pub name: String,
    pub description: String,
    pub command: String,
}

#[derive(Clone, Lens)]
pub struct AppState {
    pub input_text: String,
    pub search_results: Arc<Vec<SearchResult>>,
}

// I need to implement Data here because there is no Data trait
// implementation for Vec<SearchResult> and i cannot implement it
// here since Vec is not defined in the current crate.
// But I can compare the two Vecs easily if I derive PartialEq in
// the SearchResult struct
impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.input_text == other.input_text && self.search_results == other.search_results
    }
}

