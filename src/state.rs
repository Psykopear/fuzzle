use druid::{Data, Lens};
use std::sync::Arc;

#[derive(Clone, Data, PartialEq)]
pub struct SearchResult {
    pub icon_path: String,
    pub name: String,
    pub description: String,
    pub command: String,
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub input_text: String,
    pub search_results: Arc<Vec<SearchResult>>,
}
