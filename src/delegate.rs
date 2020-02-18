use druid::{
    AppDelegate, Command, DelegateCtx, Env, Event, HotKey, KeyCode, SysMods, Target, WindowId,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// use glob::glob;
use walkdir::WalkDir;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ini::Ini;
use std::sync::Arc;

use crate::{AppState, SearchResult};

pub struct Delegate {
    matcher: SkimMatcherV2,
    cache: HashMap<String, SearchResult>,
    search_results: Vec<SearchResult>,
    paths: Vec<String>,
}

impl Delegate {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
            cache: HashMap::new(),
            search_results: vec![],
            paths: fs::read_dir("/usr/share/applications/")
                .unwrap()
                // TODO: seriously?
                .map(|p| p.unwrap().path().to_str().unwrap().to_string())
                .collect(),
        }
    }

    fn search(&mut self, data: &AppState) {
        // Reset search results
        self.search_results = vec![];
        let mut go = true;
        let mut paths = self.paths.iter();
        while go {
            if let Some(path) = paths.next() {
                if self.search_results.len() < 3 {
                    match self.matcher.fuzzy_match(path, &data.input_text) {
                        Some(_) => (),
                        None => continue,
                    };
                    let res = match self.cache.get(path) {
                        Some(search_result) => SearchResult {
                            selected: self.search_results.len() == data.selected_line,
                            ..search_result.clone()
                        },
                        None => {
                            let info = Ini::load_from_file(path).unwrap();
                            let section = match info.section(Some("Desktop Entry")) {
                                Some(sec) => sec,
                                None => continue,
                            };
                            let name = match section.get("Name") {
                                Some(name) => name.to_string(),
                                None => continue,
                            };
                            let description = match section.get("Comment") {
                                Some(description) => description.to_string(),
                                None => continue,
                            };
                            let icon = match section.get("Icon") {
                                Some(icon) => icon,
                                None => continue,
                            };
                            let command = match section.get("Exec") {
                                Some(command) => command.to_string(),
                                None => continue,
                            };

                            // First search a default theme
                            let mut icon_path = String::new();
                            for entry in WalkDir::new("/usr/share/icons/hicolor/48x48")
                                .into_iter()
                                .filter_map(|e| e.ok())
                            {
                                if entry.path().file_stem().unwrap() == icon {
                                    icon_path = String::from(entry.path().to_str().unwrap());
                                }
                            }

                            // If we couldn't find the icon, search any theme.
                            // This should be really slow, but it's almost immediate with walkdir.
                            // Still, we can do this better
                            if icon_path.is_empty() {
                                let mut stop = false;
                                for icon_theme in std::fs::read_dir("/usr/share/icons/").unwrap() {
                                    let mut icon_theme_path = icon_theme.unwrap().path();
                                    icon_theme_path.push("48x48");

                                    for entry in WalkDir::new(icon_theme_path)
                                        .into_iter()
                                        .filter_map(|e| e.ok())
                                    {
                                        if entry.path().file_stem().unwrap() == icon {
                                            icon_path =
                                                String::from(entry.path().to_str().unwrap());
                                            stop = true;
                                            break;
                                        }
                                    }
                                    if stop {
                                        break;
                                    }
                                }
                            }

                            SearchResult {
                                name,
                                description,
                                icon_path,
                                command,
                                selected: self.search_results.len() == data.selected_line,
                            }
                        }
                    };
                    self.search_results.push(res);
                }
            } else {
                go = false
            };
        }
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
        match event {
            Event::KeyDown(key_event) => {
                if key_event.key_code == KeyCode::Escape {
                    // TODO: Ok, maybe find a nicer way to exit here, but this works
                    panic!();
                }
                if key_event.key_code == KeyCode::Return {
                    let command = data.search_results[data.selected_line].command.clone();
                    let command = command.split_whitespace().next().unwrap();
                    match std::process::Command::new(command).spawn() {
                        // TODO: Ok, maybe find a nicer way to exit here, but this works
                        Ok(_) => panic!(),
                        Err(_) => return None,
                    };
                }

                match key_event {
                    k_e if (HotKey::new(SysMods::Cmd, "j")).matches(k_e) => {
                        if data.selected_line < 2.min(data.search_results.len() - 1) {
                            data.selected_line += 1;
                        }
                        return None;
                    }
                    k_e if (HotKey::new(SysMods::Cmd, "k")).matches(k_e) => {
                        if data.selected_line > 0 {
                            data.selected_line -= 1;
                        }
                        return None;
                    }
                    _ => {}
                }
            }
            _ => (),
        };

        self.search(&data);
        data.search_results = Arc::new(self.search_results.clone());
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

    fn window_added(&mut self, _i: WindowId, _d: &mut AppState, _e: &Env, _c: &mut DelegateCtx) {}
    fn window_removed(&mut self, _i: WindowId, _d: &mut AppState, _e: &Env, _c: &mut DelegateCtx) {}
}
