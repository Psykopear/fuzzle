use druid::{Data, Lens};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Data, PartialEq, Serialize, Deserialize, Lens)]
pub struct SearchResult {
    pub icon_path: Option<String>,
    pub path: String,
    pub name: String,
    pub description: String,
    pub command: String,

    #[serde(skip)]
    pub selected: bool,
    #[serde(skip)]
    pub score: i64,
    #[serde(skip)]
    #[druid(ignore)]
    pub indices: Vec<usize>,
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub input_text: String,
    pub search_results: Arc<Vec<SearchResult>>,
    pub selected_line: usize,
}
