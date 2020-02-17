use druid::{AppDelegate, Command, DelegateCtx, Env, Event, Target, WindowId};

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
        let matcher = SkimMatcherV2::default();
        let mut search_results = vec![];
        let paths = std::fs::read_dir("/usr/share/applications/").unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if search_results.len() < 5 && !path.is_dir() {
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
                let icon_path = format!("/home/docler/src/launcherrr/src/assets/{}.png", icon);
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
