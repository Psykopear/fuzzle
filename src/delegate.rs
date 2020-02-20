use druid::{
    AppDelegate, Command, DelegateCtx, Env, Event, HotKey, KeyCode, SysMods, Target, WindowId,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use walkdir::WalkDir;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ini::Ini;
use std::sync::Arc;

use crate::{AppState, SearchResult};

#[derive(Serialize, Deserialize)]
pub struct Delegate {
    #[serde(skip)]
    matcher: SkimMatcherV2,
    cache: HashMap<String, SearchResult>,
}

impl Delegate {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
            cache: match fs::File::open("/tmp/launcherrr_cache.bincode") {
                Ok(file) => match bincode::deserialize_from::<fs::File, Delegate>(file) {
                    Ok(delegate) => delegate.cache,
                    Err(_) => HashMap::new(),
                },
                Err(_) => HashMap::new(),
            },
        }
    }

    fn populate_cache(&mut self) {
        let paths: Vec<String> = fs::read_dir("/usr/share/applications/")
            .unwrap()
            // TODO: seriously?
            .map(|p| p.unwrap().path().to_str().unwrap().to_string())
            .collect();

        // Reset search results
        for path in &paths {
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

            // TODO: Ideally the user should be able to configure
            //       a default theme a some fallback themes
            //
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
            // If we can't find the icon, search any theme.
            // Even though WalkDir is quite fast, this can become slow if there are
            // a lot of themes
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
                            icon_path = String::from(entry.path().to_str().unwrap());
                            stop = true;
                            break;
                        }
                    }
                    if stop {
                        break;
                    }
                }
            };
            // Use a default icon.
            // TODO: This should not be an absolute path
            if icon_path.is_empty() {
                icon_path = "/home/docler/src/launcherrr/src/assets/default.png".to_string()
            }

            self.cache.insert(
                path.to_string(),
                SearchResult {
                    name,
                    description,
                    icon_path,
                    command,
                    selected: false,
                },
            );
        }
        if let Ok(file) = fs::File::create("/tmp/launcherrr_cache.bincode") {
            bincode::serialize_into(file, self).unwrap();
        }
    }

    fn search(&mut self, data: &AppState) -> Vec<SearchResult> {
        // Reset search results
        let mut search_results = vec![];
        for (path, search_result) in &self.cache {
            match self.matcher.fuzzy_match(path, &data.input_text) {
                Some(_) => (),
                None => continue,
            };
            let res = SearchResult {
                selected: search_results.len() == data.selected_line,
                ..search_result.clone()
            };
            search_results.push(res);
        }
        search_results
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
        let mut update_data = |data: &mut AppState| {
            data.search_results = Arc::new(self.search(&data));
        };

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
                };
                if key_event.key_code == KeyCode::ArrowDown
                    || (HotKey::new(SysMods::Cmd, "j")).matches(key_event)
                {
                    if data.selected_line < 2.min(data.search_results.len() - 1) {
                        data.selected_line += 1;
                    }
                    update_data(data);
                    return None;
                };
                if key_event.key_code == KeyCode::ArrowUp
                    || (HotKey::new(SysMods::Cmd, "k")).matches(key_event)
                {
                    if data.selected_line > 0 {
                        data.selected_line -= 1;
                    }
                    update_data(data);
                    return None;
                };
            }
            _ => (),
        };
        update_data(data);
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
    fn window_removed(&mut self, _i: WindowId, _d: &mut AppState, _e: &Env, _c: &mut DelegateCtx) {
        println!("REMOVED");
    }
}
