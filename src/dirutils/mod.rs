use ini::Ini;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;
use xdg::BaseDirectories;

use crate::SearchResult;

fn search_dirs() -> Vec<PathBuf> {
    let base_dirs = BaseDirectories::new()
        .expect("Can't find xdg directories! Good luck and thanks for all the fish");
    let mut data_dirs: Vec<PathBuf> = Vec::new();
    data_dirs.push(base_dirs.get_data_home());
    data_dirs.append(&mut base_dirs.get_data_dirs());
    data_dirs
}

/// Given an icon name, search for the icon file.
fn search_icon(icon: &str) -> Option<String> {
    // Get data dirs, add "icons" (/usr/share/icons ecc...)
    let mut data_dirs: Vec<String> = search_dirs()
        .iter()
        .map(|dd| dd.join("icons").to_str().unwrap().to_string())
        .collect();
    // Add $HOME/.icons
    data_dirs.insert(0, "/home/docler/.icons".to_string());

    for dir in data_dirs {
        if let Ok(dirs) = fs::read_dir(dir) {
            for dir in dirs.map(|res| res.map(|e| e.path())) {
                if let Ok(path) = dir {
                    let theme_dir = path.join("48x48");
                    let res = WalkDir::new(theme_dir).into_iter().find(|e| match e {
                        Ok(entry) => entry.path().file_stem().unwrap() == icon,
                        Err(_) => false,
                    });
                    match res {
                        Some(Ok(icon_path)) => {
                            return Some(icon_path.path().to_str().unwrap().to_string())
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    None
}

/// Given a desktop file path, try to build a SearchResult
fn searchresult_from_desktopentry(desktop_file_path: &Path) -> Option<SearchResult> {
    // If anything we need can't be found, return None
    let info = match Ini::load_from_file(desktop_file_path) {
        Ok(info) => info,
        Err(_) => return None,
    };
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

    let desktop_entry_path = match desktop_file_path.to_str() {
        Some(path) => Some(path.to_string()),
        None => return None,
    };

    Some(SearchResult {
        icon_path: search_icon(icon),
        desktop_entry_path,
        name,
        description,
        command,
        score: 0,
        selected: false,
        indices: Arc::new(vec![]),
    })
}

/// Given a binary file path, try to build a SearchResult
fn searchresult_from_bin(command_path: &Path) -> Option<SearchResult> {
    let name = match command_path.file_stem() {
        Some(os_str) => {
            if let Some(str_ref) = os_str.to_str() {
                str_ref.to_string()
            } else {
                return None;
            }
        }
        None => return None,
    };

    let description = match command_path.as_os_str().to_str() {
        Some(desc) => desc.to_string(),
        None => return None,
    };
    let command = description.clone();

    Some(SearchResult {
        icon_path: search_icon("terminal"),
        desktop_entry_path: None,
        name,
        description,
        command,
        score: 0,
        selected: false,
        indices: Arc::new(vec![]),
    })
}

/// Search all applications and collect them in a Vec of SearchResult
/// This should be the only public api in this module.
pub fn build_cache() -> Vec<SearchResult> {
    let mut results: Vec<SearchResult> = Vec::new();
    // Build SearchResults for all desktop files we can find
    for mut data_dir in search_dirs() {
        data_dir.push("applications");
        results.append(
            &mut fs::read_dir(data_dir)
                .expect("Can't find data dir")
                .filter_map(|path| searchresult_from_desktopentry(&path.unwrap().path()))
                .collect(),
        );
    }
    // Now build SearchResults for all binaries we can find
    for dir in &["/usr/bin", "/usr/local/bin", "/home/docler/.local/bin/"] {
        results.append(
            &mut fs::read_dir(dir)
                .expect("Can't find /usr/bin")
                .filter_map(|path| searchresult_from_bin(&path.unwrap().path()))
                .collect(),
        );
    }
    // That's it, return
    results
}
