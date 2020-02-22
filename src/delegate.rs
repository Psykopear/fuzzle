use std::fs;
use std::path::Path;
use std::sync::Arc;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ini::Ini;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use druid::{
    AppDelegate, Command, DelegateCtx, Env, Event, HotKey, KeyCode, SysMods, Target, WindowId,
};

use crate::{AppState, SearchResult};

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
            cache: match fs::File::open("/tmp/launcherrr_cache.bincode") {
                Ok(file) => match bincode::deserialize_from::<fs::File, Delegate>(file) {
                    Ok(delegate) => delegate.cache,
                    Err(_) => vec![],
                },
                Err(_) => vec![],
            },
        }
    }

    fn populate_entry(&mut self, path: &Path) -> Option<SearchResult> {
        let info = Ini::load_from_file(path).unwrap();
        let section = match info.section(Some("Desktop Entry")) {
            Some(sec) => sec,
            None => return None,
        };
        let name = match section.get("Name") {
            Some(name) => name.to_string(),
            None => return None,
        };
        let description = match section.get("Comment") {
            Some(description) => description.to_string(),
            None => return None,
        };
        let icon = match section.get("Icon") {
            Some(icon) => icon,
            None => return None,
        };
        let command = match section.get("Exec") {
            Some(command) => command.to_string(),
            None => return None,
        };

        // TODO: Ideally the user should be able to configure
        //       a default theme a some fallback themes
        //
        // First search a default theme
        let icon_entry = WalkDir::new("/usr/share/icons/hicolor/48x48")
            .into_iter()
            .filter_map(|e| e.ok())
            .find(|e| e.path().file_stem().unwrap() == icon);

        // If we can't find the icon, search any theme.
        // Even though WalkDir is quite fast, this can become slow if there are
        // a lot of themes
        let icon_entry = match icon_entry {
            Some(icon_entry) => Some(icon_entry),
            None => {
                let mut res = None;
                for icon_theme in std::fs::read_dir("/usr/share/icons/").unwrap() {
                    let mut icon_theme_path = icon_theme.unwrap().path();
                    icon_theme_path.push("48x48");

                    res = WalkDir::new(icon_theme_path)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .find(|e| e.path().file_stem().unwrap() == icon);

                    if res.is_some() {
                        break;
                    }
                }
                res
            }
        };

        let icon_path = match icon_entry {
            Some(entry) => Some(String::from(entry.path().to_str().unwrap())),
            None => None,
        };
        Some(SearchResult {
            icon_path,
            path: String::from(path.to_str().unwrap()),
            name,
            description,
            command,
            score: 0,
            selected: false,
            indices: vec![],
        })
    }

    fn populate_cache(&mut self) {
        self.cache = fs::read_dir("/usr/share/applications/")
            .expect("Can't find /usr/share/applications/, I'll just die")
            .filter_map(|path| self.populate_entry(&path.unwrap().path()))
            .collect();

        // Reset search results
        if let Ok(file) = fs::File::create("/tmp/launcherrr_cache.bincode") {
            bincode::serialize_into(file, self).unwrap();
        }
    }

    fn search(&mut self, data: &AppState) -> (usize, Vec<SearchResult>) {
        // Search in all the cache so we have score for each entry
        let mut res: Vec<SearchResult> = self
            .cache
            .iter()
            .filter_map(|search_result| {
                let result = self
                    .matcher
                    .fuzzy_indices(&search_result.name, &data.input_text);

                if let Some((score, indices)) = result {
                    Some(SearchResult {
                        score,
                        indices,
                        selected: false,
                        ..search_result.clone()
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
            res[data.selected_line..(data.selected_line + 3).min(len)].to_vec(),
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
        match event {
            Event::KeyDown(key_event) => match key_event {
                ke if ke.key_code == KeyCode::Escape => std::process::exit(0),
                ke if ke.key_code == KeyCode::Return => {
                    let command = data.search_results[data.selected_line].command.clone();
                    let command = command.split_whitespace().next().unwrap();
                    match std::process::Command::new(command).spawn() {
                        Ok(_) => std::process::exit(0),
                        Err(_) => (),
                    };
                }
                ke if (HotKey::new(SysMods::Cmd, "j")).matches(ke)
                    || ke.key_code == KeyCode::ArrowDown =>
                {
                    data.selected_line = data.selected_line.min(num_results - 2) + 1
                }

                ke if (HotKey::new(SysMods::Cmd, "k")).matches(ke)
                    || ke.key_code == KeyCode::ArrowUp =>
                {
                    data.selected_line = data.selected_line.max(1) - 1;
                }
                k_e if k_e.key_code.is_printable() => {
                    data.selected_line = 0;
                }
                _ => (),
            },
            _ => (),
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
