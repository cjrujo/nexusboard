use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

struct WorkspaceState {
    dir: Arc<Mutex<PathBuf>>,
}

#[derive(Serialize)]
struct AppInfo {
    mode: String,
    version: String,
    workspace_dir: String,
    platform: String,
}

#[derive(Serialize)]
struct TabYamlFile {
    slug: String,
    path: String,
    yaml_content: String,
}

fn sanitize_slug(slug: &str) -> Result<String, String> {
    let clean: String = slug
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    if clean.is_empty() {
        return Err("Invalid project tab slug".to_string());
    }
    Ok(clean)
}

fn ensure_workspace_dirs(base: &Path) -> Result<(), String> {
    fs::create_dir_all(base.join("projects"))
        .map_err(|e| format!("Failed to create projects dir: {}", e))?;
    fs::create_dir_all(base.join("summaries"))
        .map_err(|e| format!("Failed to create summaries dir: {}", e))?;
    Ok(())
}

fn default_workspace_dir() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        let local_nxb = cwd.join(".nexusboard");
        if local_nxb.exists() {
            return local_nxb;
        }
        if let Some(parent) = cwd.parent() {
            let parent_nxb = parent.join(".nexusboard");
            if parent_nxb.exists() {
                return parent_nxb;
            }
        }
    }
    if let Some(home) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
        return PathBuf::from(home).join(".nexusboard");
    }
    PathBuf::from(".nexusboard")
}

fn collect_yaml_files(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_yaml_files(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
                out.push(path);
            }
        }
    }
}

#[tauri::command]
fn get_app_info(state: State<'_, WorkspaceState>) -> AppInfo {
    let dir = state.dir.lock().unwrap().clone();
    AppInfo {
        mode: "tauri-v2".to_string(),
        version: "1.0.0".to_string(),
        workspace_dir: dir.to_string_lossy().to_string(),
        platform: std::env::consts::OS.to_string(),
    }
}

#[tauri::command]
fn set_workspace_dir(path: String, state: State<'_, WorkspaceState>) -> Result<String, String> {
    let raw = PathBuf::from(path.trim());
    let target = if raw.file_name().and_then(|n| n.to_str()) == Some(".nexusboard") {
        raw
    } else {
        raw.join(".nexusboard")
    };
    ensure_workspace_dirs(&target)?;
    let mut guard = state.dir.lock().unwrap();
    *guard = target.clone();
    Ok(target.to_string_lossy().to_string())
}

#[tauri::command]
fn list_project_tabs(state: State<'_, WorkspaceState>) -> Result<Vec<TabYamlFile>, String> {
    let base = state.dir.lock().unwrap().clone();
    ensure_workspace_dirs(&base)?;
    let projects_dir = base.join("projects");
    let mut files = Vec::new();
    collect_yaml_files(&projects_dir, &mut files);
    files.sort();

    let mut result = Vec::new();
    for file_path in files {
        let slug = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("tab")
            .to_string();
        let yaml_content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read {}: {}", file_path.display(), e))?;
        result.push(TabYamlFile {
            slug,
            path: file_path.to_string_lossy().to_string(),
            yaml_content,
        });
    }
    Ok(result)
}

#[tauri::command]
fn read_tab_yaml(slug: String, state: State<'_, WorkspaceState>) -> Result<String, String> {
    let clean = sanitize_slug(&slug)?;
    let base = state.dir.lock().unwrap().clone();
    let target = base.join("projects").join(format!("{}.yaml", clean));
    fs::read_to_string(&target).map_err(|e| format!("Failed to read {}: {}", target.display(), e))
}

#[tauri::command]
fn write_tab_yaml(
    slug: String,
    yaml_content: String,
    state: State<'_, WorkspaceState>,
) -> Result<String, String> {
    let clean = sanitize_slug(&slug)?;
    let base = state.dir.lock().unwrap().clone();
    ensure_workspace_dirs(&base)?;
    let projects_dir = base.join("projects");
    let target = projects_dir.join(format!("{}.yaml", clean));
    let tmp = projects_dir.join(format!(".{}.yaml.tmp", clean));

    {
        let mut f =
            fs::File::create(&tmp).map_err(|e| format!("Failed to create temp file: {}", e))?;
        f.write_all(yaml_content.as_bytes())
            .map_err(|e| format!("Failed to write temp file: {}", e))?;
        f.sync_all()
            .map_err(|e| format!("Failed to fsync temp file: {}", e))?;
    }

    fs::rename(&tmp, &target).map_err(|e| format!("Atomic rename failed: {}", e))?;
    Ok(target.to_string_lossy().to_string())
}

#[tauri::command]
fn delete_tab_yaml(slug: String, state: State<'_, WorkspaceState>) -> Result<bool, String> {
    let clean = sanitize_slug(&slug)?;
    let base = state.dir.lock().unwrap().clone();
    let target = base.join("projects").join(format!("{}.yaml", clean));
    if target.exists() {
        fs::remove_file(&target).map_err(|e| format!("Failed to delete tab: {}", e))?;
        return Ok(true);
    }
    Ok(false)
}

fn spawn_yaml_watcher(app: AppHandle, workspace_dir: Arc<Mutex<PathBuf>>) {
    std::thread::spawn(move || {
        let mut mtimes: HashMap<PathBuf, SystemTime> = HashMap::new();
        loop {
            std::thread::sleep(Duration::from_millis(1200));
            let base = workspace_dir.lock().unwrap().clone();
            let projects_dir = base.join("projects");
            if !projects_dir.exists() {
                continue;
            }
            let mut files = Vec::new();
            collect_yaml_files(&projects_dir, &mut files);

            let mut changed = false;
            let mut next_mtimes = HashMap::new();
            for p in files {
                if let Ok(meta) = fs::metadata(&p) {
                    if let Ok(mtime) = meta.modified() {
                        if mtimes.get(&p) != Some(&mtime) {
                            changed = true;
                        }
                        next_mtimes.insert(p, mtime);
                    }
                }
            }
            if next_mtimes.len() != mtimes.len() {
                changed = true;
            }
            if changed && !mtimes.is_empty() {
                let _ = app.emit("nxb://yaml-changed", base.to_string_lossy().to_string());
            }
            mtimes = next_mtimes;
        }
    });
}

fn focus_main_window(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.unminimize();
        let _ = win.show();
        let _ = win.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let initial_dir = default_workspace_dir();
    let _ = ensure_workspace_dirs(&initial_dir);
    let shared_dir = Arc::new(Mutex::new(initial_dir));

    let watcher_dir = Arc::clone(&shared_dir);
    let quick_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Space);
    let quick_shortcut_alt = Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::Space);

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if (shortcut == &quick_shortcut || shortcut == &quick_shortcut_alt) && event.state() == ShortcutState::Pressed {
                        focus_main_window(app);
                        let _ = app.emit("nxb://quick-capture", true);
                    }
                })
                .build(),
        )
        .manage(WorkspaceState { dir: shared_dir })
        .setup(move |app| {
            let handle = app.handle().clone();
            spawn_yaml_watcher(handle.clone(), watcher_dir);

            // Register Ctrl+Shift+Space global Quick-Capture shortcut (non-fatal if already bound)
            let _ = app.global_shortcut().register(quick_shortcut);
            let _ = app.global_shortcut().register(quick_shortcut_alt);

            // System Tray Icon & Menu
            let show_i = MenuItem::with_id(app, "show", "Open NexusBoard", true, None::<&str>)?;
            let quick_i = MenuItem::with_id(
                app,
                "quick_capture",
                "⚡ Quick-Capture Task (Ctrl+Shift+Space)",
                true,
                None::<&str>,
            )?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit NexusBoard", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quick_i, &quit_i])?;

            let mut tray_builder = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("NexusBoard v1.0 — Human × AI YAML Orchestration")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => focus_main_window(app),
                    "quick_capture" => {
                        focus_main_window(app);
                        let _ = app.emit("nxb://quick-capture", true);
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        focus_main_window(tray.app_handle());
                    }
                });

            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }
            let _ = tray_builder.build(app);

            if let Some(win) = app.get_webview_window("main") {
                let _ = win.set_title("NexusBoard v1.0 — Human × AI YAML Orchestration");
                let _ = win.set_focus();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            set_workspace_dir,
            list_project_tabs,
            read_tab_yaml,
            write_tab_yaml,
            delete_tab_yaml
        ])
        .run(tauri::generate_context!())
        .expect("error while running NexusBoard GUI");
}
