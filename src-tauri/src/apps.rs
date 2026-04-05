use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};

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

static CACHE: OnceLock<Mutex<Option<Vec<DesktopEntry>>>> = OnceLock::new();

fn cache_mutex() -> &'static Mutex<Option<Vec<DesktopEntry>>> {
    CACHE.get_or_init(|| Mutex::new(None))
}

fn app_search_roots() -> Vec<PathBuf> {
    let mut dirs = vec![
        PathBuf::from("/usr/share/applications"),
        PathBuf::from("/usr/local/share/applications"),
        PathBuf::from("/var/lib/snapd/desktop/applications"),
        PathBuf::from("/var/lib/flatpak/exports/share/applications"),
    ];
    if let Ok(home) = std::env::var("HOME") {
        let h = PathBuf::from(home);
        dirs.push(h.join(".local/share/applications"));
        dirs.push(
            h.join(".local/share/flatpak/exports/share/applications"),
        );
    }
    dirs
}

fn load_index() -> Result<Vec<DesktopEntry>, String> {
    let mut out: Vec<DesktopEntry> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();

    for base in app_search_roots() {
        if base.is_dir() {
            collect_desktop_files(&base, &mut out, &mut seen);
        }
    }

    out.sort_by(|a, b| a.name_lower.cmp(&b.name_lower));
    Ok(out)
}

fn collect_desktop_files(dir: &Path, out: &mut Vec<DesktopEntry>, seen: &mut HashSet<PathBuf>) {
    let Ok(read_dir) = fs::read_dir(dir) else {
        return;
    };
    for ent in read_dir.flatten() {
        let path = ent.path();
        let Ok(ft) = ent.file_type() else {
            continue;
        };
        if ft.is_dir() {
            collect_desktop_files(&path, out, seen);
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) != Some("desktop") {
            continue;
        }
        let canon = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
        if !seen.insert(canon.clone()) {
            continue;
        }
        if let Some(entry) = parse_desktop_file(&canon) {
            out.push(entry);
        }
    }
}

fn parse_desktop_file(path: &Path) -> Option<DesktopEntry> {
    let mut content = String::new();
    fs::File::open(path).ok()?.read_to_string(&mut content).ok()?;

    let mut section = String::new();
    let mut type_line: Option<String> = None;
    let mut name_default: Option<String> = None;
    let mut name_locale: Option<String> = None;
    let mut no_display = false;
    let mut hidden = false;
    let mut has_exec = false;

    for raw_line in content.lines() {
        let line = raw_line.trim_end_matches('\r');
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            section = trimmed.to_string();
            continue;
        }
        if section != "[Desktop Entry]" {
            continue;
        }
        let Some((key, val)) = trimmed.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let val = val.trim();
        match key {
            "Type" => type_line = Some(val.to_string()),
            "Name" => {
                if name_default.is_none() {
                    name_default = Some(val.to_string());
                }
            }
            k if k.starts_with("Name[") => {
                if name_locale.is_none() {
                    name_locale = Some(val.to_string());
                }
            }
            "NoDisplay" => no_display = val == "true" || val == "1",
            "Hidden" => hidden = val == "true" || val == "1",
            "Exec" => has_exec = true,
            _ => {}
        }
    }

    if hidden || no_display {
        return None;
    }
    if let Some(t) = &type_line {
        if t != "Application" {
            return None;
        }
    }
    if !has_exec {
        return None;
    }

    let name = name_default.or(name_locale)?;
    let name_lower = name.to_lowercase();

    Some(DesktopEntry {
        name,
        name_lower,
        path: path.to_path_buf(),
    })
}

fn ensure_cache() -> Result<(), String> {
    let mut guard = cache_mutex()
        .lock()
        .map_err(|_| "index lock poisoned".to_string())?;
    if guard.is_none() {
        *guard = Some(load_index()?);
    }
    Ok(())
}

pub fn search_apps(query: String) -> Result<Vec<AppSuggestion>, String> {
    if !cfg!(target_os = "linux") {
        return Ok(vec![]);
    }
    ensure_cache()?;
    let guard = cache_mutex()
        .lock()
        .map_err(|_| "index lock poisoned".to_string())?;
    let index = guard.as_ref().ok_or_else(|| "index missing".to_string())?;

    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Ok(vec![]);
    }

    const LIMIT: usize = 16;
    let mut scored: Vec<(&DesktopEntry, u8)> = index
        .iter()
        .filter_map(|e| {
            if e.name_lower.starts_with(&q) {
                Some((e, 0))
            } else if e.name_lower.contains(&q) {
                Some((e, 1))
            } else {
                None
            }
        })
        .collect();

    scored.sort_by(|a, b| match a.1.cmp(&b.1) {
        std::cmp::Ordering::Equal => a.0.name_lower.cmp(&b.0.name_lower),
        o => o,
    });

    Ok(scored
        .into_iter()
        .take(LIMIT)
        .map(|(e, _)| AppSuggestion {
            name: e.name.clone(),
            desktop_path: e.path.to_string_lossy().into_owned(),
        })
        .collect())
}

pub fn launch_desktop_file(desktop_file_path: String) -> Result<(), String> {
    if !cfg!(target_os = "linux") {
        return Err("unsupported platform".into());
    }
    let path = PathBuf::from(&desktop_file_path);
    if !path.is_file() {
        return Err("desktop file not found".into());
    }

    if let Ok(mut child) = Command::new("gio")
        .args(["launch", desktop_file_path.as_str()])
        .spawn()
    {
        if let Ok(status) = child.wait() {
            if status.success() {
                return Ok(());
            }
        }
    }

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "invalid desktop file name".to_string())?;

    let mut child = Command::new("gtk-launch")
        .arg(stem)
        .spawn()
        .map_err(|e| format!("launch failed: {e}"))?;
    let status = child.wait().map_err(|e| format!("wait: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "gtk-launch exited with {}",
            status.code().unwrap_or(-1)
        ))
    }
}
