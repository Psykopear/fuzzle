use druid::{AppDelegate, Command, DelegateCtx, Env, Event, KeyCode, Target, WindowId};

// use glob::glob;
use walkdir::WalkDir;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ini::Ini;
use std::sync::Arc;

use crate::{AppState, SearchResult};

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        _c: &mut DelegateCtx,
        _w: WindowId,
        event: Event,
        data: &mut AppState,
        _e: &Env,
    ) -> Option<Event> {
        match event {
            Event::KeyDown(key_event) => {
                if key_event.key_code == KeyCode::Return {
                    let command = data.search_results.first().unwrap().command.clone();
                    let command = command.split_whitespace().next().unwrap();
                    match std::process::Command::new(command).spawn() {
                        // TODO: Ok, maybe find a nicer way to exit here, but this works
                        Ok(_) => panic!(),
                        Err(_) => return None
                    };
                }
            }
            _ => (),
        };
        let matcher = SkimMatcherV2::default();
        let mut search_results = vec![];
        let paths = std::fs::read_dir("/usr/share/applications/").unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if search_results.len() < 4 && !path.is_dir() {
                match matcher.fuzzy_match(path.to_str().unwrap(), &data.input_text) {
                    Some(_) => (),
                    None => continue,
                };
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
                                icon_path = String::from(entry.path().to_str().unwrap());
                                stop = true;
                                break;
                            }
                        }
                        if stop {
                            break;
                        }
                    }
                }

                let res = SearchResult {
                    name,
                    description,
                    icon_path,
                    command,
                };
                search_results.push(res);
            };
        }
        data.search_results = Arc::new(search_results);
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
