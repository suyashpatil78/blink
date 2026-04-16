use freedesktop_entry_parser::parse_entry;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSuggestion {
    pub name: String,
    pub desktop_path: String,
}

struct DesktopEntry {
    name: String,
    name_lower: String,
    path: PathBuf,
}

static APP_INDEX: Lazy<Vec<DesktopEntry>> = Lazy::new(build_index);

fn app_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![
        "/usr/share/applications",
        "/usr/local/share/applications",
        "/var/lib/snapd/desktop/applications",
        "/var/lib/flatpak/exports/share/applications",
    ]
    .into_iter()
    .map(PathBuf::from)
    .collect::<Vec<_>>();

    if let Ok(home) = std::env::var("HOME") {
        dirs.push(format!("{home}/.local/share/applications").into());
    }

    dirs
}

fn build_index() -> Vec<DesktopEntry> {
    app_dirs()
        .into_iter()
        .flat_map(|dir| WalkDir::new(dir).into_iter())
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|e| e == "desktop")
                .unwrap_or(false)
        })
        .filter_map(|entry| {
            let path = entry.path();

            let parsed = parse_entry(path).ok()?;

            let section = parsed.section("Desktop Entry")?;

            let hidden = section.attr("Hidden").first().is_some_and(|v| v == "true");
            let nodisplay = section.attr("NoDisplay").first().is_some_and(|v| v == "true");
            let is_app = section.attr("Type").first().is_some_and(|v| v == "Application");

            if hidden || nodisplay || !is_app {
                return None;
            }

            let name = section.attr("Name").first()?.clone();

            Some(DesktopEntry {
                name_lower: name.to_lowercase(),
                name,
                path: path.to_path_buf(),
            })
        })
        .collect()
}

pub fn search_apps(query: String) -> Result<Vec<AppSuggestion>, String> {
    let q = query.trim().to_lowercase();

    if q.is_empty() {
        return Ok(vec![]);
    }

    const LIMIT: usize = 16;

    let mut results: Vec<_> = APP_INDEX
        .iter()
        .filter_map(|app| {
            if app.name_lower.starts_with(&q) {
                Some((app, 0))
            } else if app.name_lower.contains(&q) {
                Some((app, 1))
            } else {
                None
            }
        })
        .collect();

    results.sort_by(|a, b| {
        a.1.cmp(&b.1)
            .then(a.0.name_lower.cmp(&b.0.name_lower))
    });

    Ok(results
        .into_iter()
        .take(LIMIT)
        .map(|(app, _)| AppSuggestion {
            name: app.name.clone(),
            desktop_path: app.path.to_string_lossy().into(),
        })
        .collect())
}

pub fn launch_desktop_file(path: String) -> Result<(), String> {
    Command::new("gio")
        .args(["launch", &path])
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}