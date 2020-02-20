use serde::{Serialize, Deserialize};
use druid::{Data, Lens};
use std::sync::Arc;

#[derive(Clone, Data, PartialEq, Serialize, Deserialize, Lens)]
pub struct SearchResult {
    pub icon_path: String,
    pub path: String,
    pub name: String,
    pub description: String,
    pub command: String,
    pub selected: bool,
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub input_text: String,
    pub search_results: Arc<Vec<SearchResult>>,
    pub selected_line: usize,
}
