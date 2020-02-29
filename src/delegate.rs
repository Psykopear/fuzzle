use std::fs;
use std::path::Path;
use std::sync::Arc;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Serialize};

use druid::{
    AppDelegate, Command, DelegateCtx, Env, Event, HotKey, KeyCode, SysMods, Target, WindowId,
};

use crate::{AppState, SearchResult};

use crate::dirutils::build_cache;

#[derive(Serialize, Deserialize)]
pub struct Delegate {
    #[serde(skip)]
    matcher: SkimMatcherV2,
    cache: Vec<SearchResult>,
}

impl Delegate {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
            cache: match fs::File::open("/tmp/fuzzle_cache.bincode") {
                Ok(file) => match bincode::deserialize_from::<fs::File, Delegate>(file) {
                    Ok(delegate) => delegate.cache,
                    Err(_) => vec![],
                },
                Err(_) => vec![],
            },
        }
    }

    fn populate_cache(&mut self) {
        self.cache = build_cache();
        // Reset search results
        if let Ok(file) = fs::File::create("/tmp/fuzzle_cache.bincode") {
            bincode::serialize_into(file, self).unwrap();
        }
    }

    fn search(&mut self, data: &AppState) -> (usize, Vec<SearchResult>) {
        // Search in all the cache so we have score for each entry
        let mut res: Vec<SearchResult> = self
            .cache
            .iter()
            .filter_map(|sr| {
                let mut search_name = String::from(&sr.name);
                if let Some(file_name) =
                    Path::new(sr.desktop_entry_path.as_ref().unwrap_or(&"".to_string())).file_stem()
                {
                    search_name = search_name + " " + file_name.to_str().unwrap_or("");
                };
                let result = self.matcher.fuzzy_indices(&search_name, &data.input_text);

                if let Some((score, indices)) = result {
                    Some(SearchResult {
                        // Always put desktop entry files first
                        score: if sr.desktop_entry_path.is_some() {
                            score + 1000
                        } else {
                            score
                        },
                        indices: Arc::new(indices),
                        selected: false,
                        ..sr.clone()
                    })
                } else {
                    None
                }
            })
            .collect();

        // Now order by score, descending
        res.sort_unstable_by_key(|a| -a.score);

        // Select the line
        let len = res.len();
        if len > data.selected_line {
            res[data.selected_line].selected = true;
        }

        (
            len,
            res[data.selected_line.max(1) - 1..(data.selected_line + 3).min(len)].to_vec(),
        )
    }
}

impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        _ctx: &mut DelegateCtx,
        _window_id: WindowId,
        event: Event,
        data: &mut AppState,
        _env: &Env,
    ) -> Option<Event> {
        let (num_results, results) = self.search(&data);
        if let Event::KeyDown(key_event) = event {
            match key_event {
                ke if ke.key_code == KeyCode::Escape => std::process::exit(0),
                ke if ke.key_code == KeyCode::Return => {
                    // TODO: use data.selected_line. The unwrap should never panic here,
                    // but it's not nice anyway
                    let command = results.iter().find(|r| r.selected).unwrap().command.clone();
                    let command = command.split_whitespace().next().unwrap();
                    if std::process::Command::new(command).spawn().is_ok() {
                        std::process::exit(0)
                    }
                }
                ke if (HotKey::new(SysMods::Cmd, "j")).matches(ke)
                    || ke.key_code == KeyCode::ArrowDown =>
                {
                    data.selected_line = data.selected_line.min(num_results - 2) + 1;
                }

                ke if (HotKey::new(SysMods::Cmd, "k")).matches(ke)
                    || ke.key_code == KeyCode::ArrowUp =>
                {
                    data.selected_line = data.selected_line.max(1) - 1;
                }
                k_e if k_e.key_code.is_printable()
                    || (HotKey::new(None, KeyCode::Backspace)).matches(k_e) =>
                {
                    // Reset selected line if new text comes
                    data.selected_line = 0;
                }
                _ => (),
            }
        };
        data.search_results = Arc::new(results);
        Some(event)
    }

    fn command(
        &mut self,
        _d: &mut DelegateCtx,
        _t: &Target,
        _c: &Command,
        _a: &mut AppState,
        _e: &Env,
    ) -> bool {
        false
    }

    fn window_added(&mut self, _i: WindowId, _d: &mut AppState, _e: &Env, _c: &mut DelegateCtx) {
        if self.cache.is_empty() {
            self.populate_cache();
        }
    }
    fn window_removed(&mut self, _i: WindowId, _d: &mut AppState, _e: &Env, _c: &mut DelegateCtx) {}
}
