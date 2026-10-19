use crate::db::ClipboardDB;
#[cfg(all(desktop, not(target_os = "android")))]
use arboard::Clipboard;
use std::collections::HashMap;
use tauri::Emitter;
use tauri::Manager;

#[tauri::command]
pub fn get_history(
    app: tauri::AppHandle,
    search: Option<String>,
) -> Result<Vec<crate::db::ClipboardItem>, String> {
    let db = app.state::<ClipboardDB>();
    db.get_history(search).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_entry(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.delete_item(id).map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn toggle_permanent(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.toggle_permanent(id).map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn set_category(app: tauri::AppHandle, id: i64, category: String) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.set_category(id, category).map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn get_categories(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let db = app.state::<ClipboardDB>();
    db.get_categories().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_group(
    app: tauri::AppHandle,
    name: String,
    parent_id: Option<i64>,
    icon: Option<String>,
) -> Result<i64, String> {
    let db = app.state::<ClipboardDB>();
    let id = db.create_group(name, parent_id, icon).map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(id)
}

#[tauri::command]
pub fn get_groups_detailed(app: tauri::AppHandle) -> Result<Vec<crate::db::Group>, String> {
    let db = app.state::<ClipboardDB>();
    db.get_groups_detailed().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_items_batch(app: tauri::AppHandle, ids: Vec<i64>) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.delete_items_batch(ids).map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn add_items_to_group_batch(
    app: tauri::AppHandle,
    ids: Vec<i64>,
    group_name: String,
) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.add_items_to_group_batch(ids, group_name).map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn remove_items_from_group_batch(
    app: tauri::AppHandle,
    ids: Vec<i64>,
    group_name: String,
) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.remove_items_from_group_batch(ids, group_name).map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn minimize_popup_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(popup) = app.get_webview_window("popup") {
        let _ = popup.minimize();
    }
    Ok(())
}

#[tauri::command]
pub fn maximize_popup_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(popup) = app.get_webview_window("popup") {
        if popup.is_maximized().unwrap_or(false) {
            let _ = popup.unmaximize();
        } else {
            let _ = popup.maximize();
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_group(app: tauri::AppHandle, name: String) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.delete_group(name).map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn rename_group(
    app: tauri::AppHandle,
    old_name: String,
    new_name: String,
) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.rename_group(old_name, new_name)
        .map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn export_group(
    app: tauri::AppHandle,
    name: String,
    path: String,
) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.export_group(name, std::path::PathBuf::from(path))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_group(
    app: tauri::AppHandle,
    name: String,
    path: String,
) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.import_group(name, std::path::PathBuf::from(path))
        .map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn add_to_group(
    app: tauri::AppHandle,
    item_id: i64,
    group_name: String,
) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.add_to_group(item_id, group_name)
        .map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn remove_from_group(
    app: tauri::AppHandle,
    item_id: i64,
    group_name: String,
) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.remove_from_group(item_id, group_name)
        .map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn export_all_txt(app: tauri::AppHandle, path: String) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.export_all_txt(std::path::PathBuf::from(path))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_manual_item(
    app: tauri::AppHandle,
    content: String,
    description: Option<String>,
    group_name: Option<String>,
) -> Result<i64, String> {
    let db = app.state::<ClipboardDB>();
    let id = db
        .add_manual_item(content, description, group_name)
        .map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(id)
}

#[tauri::command]
pub fn update_item(
    app: tauri::AppHandle,
    id: i64,
    content: String,
    description: Option<String>,
) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.update_item(id, content, description)
        .map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn list_snippets(app: tauri::AppHandle) -> Result<Vec<crate::db::Snippet>, String> {
    let db = app.state::<ClipboardDB>();
    db.list_snippets().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_snippet(app: tauri::AppHandle, name: String, body: String) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.upsert_snippet(name, body).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_snippet(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.delete_snippet(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn render_snippet(template: String) -> Result<String, String> {
    let now = chrono::Local::now();
    let mut rendered = template;

    rendered = rendered.replace("{{date}}", &now.format("%Y-%m-%d").to_string());
    rendered = rendered.replace("{{time}}", &now.format("%H:%M:%S").to_string());
    rendered = rendered.replace("{{datetime}}", &now.format("%Y-%m-%d %H:%M:%S").to_string());
    rendered = rendered.replace("{{iso}}", &now.to_rfc3339());

    Ok(rendered)
}

#[tauri::command]
pub fn transform_content(content: String, mode: String) -> Result<String, String> {
    let mode = mode.trim().to_lowercase();
    match mode.as_str() {
        "uppercase" | "upper" => Ok(content.to_uppercase()),
        "lowercase" | "lower" => Ok(content.to_lowercase()),
        "trim" => Ok(content.trim().to_string()),
        "single_line" | "singleline" => {
            let collapsed = content
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            Ok(collapsed)
        }
        "json_compact" => {
            let value: serde_json::Value =
                serde_json::from_str(&content).map_err(|e| format!("Invalid JSON: {}", e))?;
            serde_json::to_string(&value).map_err(|e| e.to_string())
        }
        "json_pretty" => {
            let value: serde_json::Value =
                serde_json::from_str(&content).map_err(|e| format!("Invalid JSON: {}", e))?;
            serde_json::to_string_pretty(&value).map_err(|e| e.to_string())
        }
        "url_encode" => Ok(urlencoding::encode(&content).into_owned()),
        "url_decode" => urlencoding::decode(&content)
            .map(|cow| cow.into_owned())
            .map_err(|e| e.to_string()),
        "base64_encode" => {
            use base64::Engine;
            Ok(base64::engine::general_purpose::STANDARD.encode(content.as_bytes()))
        }
        "base64_decode" => {
            use base64::Engine;
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(content.trim())
                .map_err(|e| format!("Invalid Base64: {}", e))?;
            String::from_utf8(bytes).map_err(|e| format!("Invalid UTF-8 payload: {}", e))
        }
        _ => Err(format!("Unsupported transform mode: {}", mode)),
    }
}

#[cfg(all(desktop, not(target_os = "android")))]
fn release_held_modifiers(enigo: &mut enigo::Enigo) {
    use enigo::{Direction, Key, Keyboard};
    let _ = enigo.key(Key::Shift, Direction::Release);
    let _ = enigo.key(Key::Alt, Direction::Release);
    let _ = enigo.key(Key::Control, Direction::Release);
    let _ = enigo.key(Key::Meta, Direction::Release);
}

/// Simulates a system paste keystroke (Cmd+V on macOS, Ctrl+V on Windows/Linux).
#[cfg(all(desktop, not(target_os = "android")))]
fn simulate_paste_keystroke() -> Result<(), String> {
    use enigo::{Direction, Enigo, Key, Keyboard, Settings};

    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;

    release_held_modifiers(&mut enigo);

    #[cfg(target_os = "macos")]
    {
        let _ = enigo.key(Key::Meta, Direction::Press);
        // Enigo Linux VK offset maps CGKeyCode 9 ('v') to Other(9) on macOS.
        // Using Key::Unicode('v') or Key::Other(9) avoids keycode offset bugs.
        let _ = enigo.key(Key::Other(9), Direction::Click);
        let _ = enigo.key(Key::Meta, Direction::Release);
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = enigo.key(Key::Control, Direction::Press);
        let _ = enigo.key(Key::Unicode('v'), Direction::Click);
        let _ = enigo.key(Key::Control, Direction::Release);
    }
    Ok(())
}

/// Simulates a system copy keystroke (Cmd+C on macOS, Ctrl+C on Windows/Linux).
#[cfg(all(desktop, not(target_os = "android")))]
fn simulate_copy_keystroke() -> Result<(), String> {
    use enigo::{Direction, Enigo, Key, Keyboard, Settings};

    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;

    release_held_modifiers(&mut enigo);

    #[cfg(target_os = "macos")]
    {
        let _ = enigo.key(Key::Meta, Direction::Press);
        let _ = enigo.key(Key::Other(8), Direction::Click);
        let _ = enigo.key(Key::Meta, Direction::Release);
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = enigo.key(Key::Control, Direction::Press);
        let _ = enigo.key(Key::Unicode('c'), Direction::Click);
        let _ = enigo.key(Key::Control, Direction::Release);
    }
    Ok(())
}

#[cfg(any(mobile, target_os = "android"))]
fn simulate_paste_keystroke() -> Result<(), String> {
    Err("Keystroke simulation not supported on mobile".into())
}

#[cfg(any(mobile, target_os = "android"))]
fn simulate_copy_keystroke() -> Result<(), String> {
    Err("Keystroke simulation not supported on mobile".into())
}

#[tauri::command]
pub async fn paste_item(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    let (content_type, mut raw_content) = {
        let db = app.state::<ClipboardDB>();
        db.get_item_payload(id).map_err(|e| e.to_string())?
    };

    // Unmask sensitive item before writing to clipboard.
    if raw_content.is_empty() {
        if let Ok(key) = crate::crypto::get_or_create_key(&app) {
            let db = app.state::<ClipboardDB>();
            if let Ok(items) = db.get_history(None) {
                if let Some(_item) = items.iter().find(|i| i.id == id) {
                    // Fetch full row directly via db call
                    let conn_payload = db.get_item_payload(id);
                    if let Ok((_, stored)) = conn_payload {
                        if let Ok(plain) = crate::crypto::decrypt(&key, &stored) {
                            raw_content = plain;
                        }
                    }
                }
            }
        }
    }

    set_clipboard_payload(&app, &content_type, &raw_content)?;

    // Give OS time to process clipboard update before pasting
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    simulate_paste_keystroke()
}

#[tauri::command]
pub fn copy_item_to_clipboard(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    let (content_type, mut raw_content) = {
        let db = app.state::<ClipboardDB>();
        db.get_item_payload(id).map_err(|e| e.to_string())?
    };

    // Unmask sensitive item before writing to clipboard.
    if raw_content.is_empty() {
        if let Ok(key) = crate::crypto::get_or_create_key(&app) {
            let db = app.state::<ClipboardDB>();
            if let Ok((_, stored)) = db.get_item_payload(id) {
                if let Ok(plain) = crate::crypto::decrypt(&key, &stored) {
                    raw_content = plain;
                }
            }
        }
    }

    set_clipboard_payload(&app, &content_type, &raw_content)
}

#[tauri::command]
pub fn get_image_thumbnail(app: tauri::AppHandle, hash: String) -> Result<String, String> {
    let db = app.state::<ClipboardDB>();
    let bytes = db.get_blob_thumb(&hash).map_err(|e| e.to_string())?;
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:image/png;base64,{}", b64))
}

#[tauri::command]
pub fn get_file_thumbnail(path: String) -> Result<String, String> {
    // Generate an SVG icon data-URL representation based on file extension.
    let ext = std::path::Path::new(&path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let (label, color) = match ext.as_str() {
        "pdf" => ("PDF", "#EF4444"),
        "doc" | "docx" => ("DOC", "#3B82F6"),
        "xls" | "xlsx" | "csv" => ("XLS", "#10B981"),
        "ppt" | "pptx" => ("PPT", "#F59E0B"),
        "zip" | "tar" | "gz" | "7z" | "rar" => ("ZIP", "#8B5CF6"),
        "mp3" | "wav" | "flac" | "aac" | "ogg" => ("AUDIO", "#EC4899"),
        "mp4" | "mkv" | "mov" | "avi" | "webm" => ("VIDEO", "#6366F1"),
        "rs" | "js" | "ts" | "py" | "c" | "cpp" | "go" | "html" | "css" | "json" => ("CODE", "#06B6D4"),
        "txt" | "md" => ("TXT", "#6B7280"),
        _ => ("FILE", "#6B7280"),
    };

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="120" height="120" viewBox="0 0 120 120">
            <rect width="120" height="120" rx="16" fill="{color}" fill-opacity="0.15"/>
            <path d="M40 30h28l20 20v40a8 8 0 0 1-8 8H40a8 8 0 0 1-8-8V38a8 8 0 0 1 8-8z" fill="none" stroke="{color}" stroke-width="4"/>
            <path d="M68 30v20h20" fill="none" stroke="{color}" stroke-width="4"/>
            <text x="60" y="82" font-family="system-ui, sans-serif" font-weight="bold" font-size="16" fill="{color}" text-anchor="middle">{label}</text>
        </svg>"#
    );

    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(svg.as_bytes());
    Ok(format!("data:image/svg+xml;base64,{}", b64))
}

#[tauri::command]
pub fn set_item_sensitive(app: tauri::AppHandle, id: i64, sensitive: bool) -> Result<(), String> {
    let key = crate::crypto::get_or_create_key(&app)?;
    let db = app.state::<ClipboardDB>();
    let (_, raw) = db.get_item_payload(id).map_err(|e| e.to_string())?;

    if sensitive {
        // Encrypt plaintext -> ciphertext, set sensitive = 1
        let ciphertext = crate::crypto::encrypt(&key, &raw)?;
        db.set_raw_and_sensitive(id, &ciphertext, true).map_err(|e| e.to_string())?;
    } else {
        // Decrypt ciphertext -> plaintext, set sensitive = 0
        let plaintext = crate::crypto::decrypt(&key, &raw)?;
        db.set_raw_and_sensitive(id, &plaintext, false).map_err(|e| e.to_string())?;
    }
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn reveal_item(app: tauri::AppHandle, id: i64) -> Result<String, String> {
    let key = crate::crypto::get_or_create_key(&app)?;
    let db = app.state::<ClipboardDB>();
    let (_, stored) = db.get_item_payload(id).map_err(|e| e.to_string())?;
    crate::crypto::decrypt(&key, &stored)
}

#[tauri::command]
pub fn get_setting(app: tauri::AppHandle, key: String) -> Result<Option<String>, String> {
    let db = app.state::<ClipboardDB>();
    db.get_setting(&key).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_setting(app: tauri::AppHandle, key: String, value: String) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.set_setting(&key, &value).map_err(|e| e.to_string())?;

    // Shortcut settings update registered global hotkeys dynamically.
    if key.starts_with("shortcut_") {
        let _ = crate::register_global_shortcuts(&app);
        let _ = app.emit("shortcuts-updated", ());
    }
    Ok(())
}

#[tauri::command]
pub fn get_shortcuts(app: tauri::AppHandle) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for action in crate::SHORTCUT_ACTIONS {
        map.insert(action.to_string(), crate::accelerator_for(&app, action));
    }
    map
}

#[tauri::command]
pub fn get_default_shortcuts() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for action in crate::SHORTCUT_ACTIONS {
        map.insert(action.to_string(), crate::default_accelerator(action).to_string());
    }
    map
}

#[tauri::command]
pub fn set_shortcut(app: tauri::AppHandle, action: String, accelerator: String) -> Result<(), String> {
    if !crate::SHORTCUT_ACTIONS.contains(&action.as_str()) {
        return Err(format!("Unknown action: {action}"));
    }
    let key = format!("shortcut_{action}");
    let db = app.state::<ClipboardDB>();
    db.set_setting(&key, &accelerator).map_err(|e| e.to_string())?;
    let _ = crate::register_global_shortcuts(&app);
    let _ = app.emit("shortcuts-updated", ());
    Ok(())
}

#[tauri::command]
pub fn reset_shortcuts(app: tauri::AppHandle) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    for action in crate::SHORTCUT_ACTIONS {
        let key = format!("shortcut_{action}");
        let _ = db.set_setting(&key, "");
    }
    let _ = crate::register_global_shortcuts(&app);
    let _ = app.emit("shortcuts-updated", ());
    Ok(())
}

// --- Paste Stack ---

#[tauri::command]
pub fn stack_add(app: tauri::AppHandle, id: i64) -> Result<Vec<crate::db::ClipboardItem>, String> {
    if let Some(state) = app.try_state::<crate::PasteStack>() {
        if let Ok(mut stack) = state.0.lock() {
            stack.retain(|&x| x != id);
            stack.push(id);
        }
    }
    stack_list(app)
}

#[tauri::command]
pub fn stack_remove(app: tauri::AppHandle, id: i64) -> Result<Vec<crate::db::ClipboardItem>, String> {
    if let Some(state) = app.try_state::<crate::PasteStack>() {
        if let Ok(mut stack) = state.0.lock() {
            stack.retain(|&x| x != id);
        }
    }
    stack_list(app)
}

#[tauri::command]
pub fn stack_clear(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(state) = app.try_state::<crate::PasteStack>() {
        if let Ok(mut stack) = state.0.lock() {
            stack.clear();
        }
    }
    let _ = app.emit("stack-updated", ());
    Ok(())
}

#[tauri::command]
pub fn stack_list(app: tauri::AppHandle) -> Result<Vec<crate::db::ClipboardItem>, String> {
    let ids = match app.try_state::<crate::PasteStack>() {
        Some(state) => state.0.lock().map(|s| s.clone()).unwrap_or_default(),
        None => Vec::new(),
    };
    let db = app.state::<ClipboardDB>();
    db.get_items_by_ids(&ids).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn paste_next_from_stack(app: tauri::AppHandle) -> Result<Option<i64>, String> {
    let next_id = match app.try_state::<crate::PasteStack>() {
        Some(state) => state.0.lock().ok().and_then(|mut s| s.pop()),
        None => None,
    };
    let Some(id) = next_id else {
        return Ok(None);
    };

    let _ = app.emit("stack-updated", ());
    paste_item(app, id).await?;
    Ok(Some(id))
}

#[tauri::command]
pub async fn copy_selection_to_stack(app: tauri::AppHandle) -> Result<Option<i64>, String> {
    // 1. Simulate copy to place selection on clipboard
    simulate_copy_keystroke()?;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // 2. Read new top item from DB (clipboard listener saved it)
    let db = app.state::<ClipboardDB>();
    let items = db.get_history(None).map_err(|e| e.to_string())?;
    let top_id = items.first().map(|i| i.id);

    // 3. Add to stack
    if let Some(id) = top_id {
        let _ = stack_add(app, id);
    }
    Ok(top_id)
}

#[tauri::command]
pub fn copy_as(app: tauri::AppHandle, id: i64, format: String) -> Result<(), String> {
    let (content_type, mut text) = {
        let db = app.state::<ClipboardDB>();
        db.get_item_payload(id).map_err(|e| e.to_string())?
    };

    if text.is_empty() {
        if let Ok(key) = crate::crypto::get_or_create_key(&app) {
            let db = app.state::<ClipboardDB>();
            if let Ok((_, stored)) = db.get_item_payload(id) {
                if let Ok(plain) = crate::crypto::decrypt(&key, &stored) {
                    text = plain;
                }
            }
        }
    }

    if content_type != "text" {
        return Err("Copy-as is only supported for text items".into());
    }

    let transformed = transform_content(text, format)?;
    set_clipboard_text_desktop(&app, &transformed)
}

#[tauri::command]
pub fn set_clipboard_text(app: tauri::AppHandle, text: String) -> Result<(), String> {
    set_clipboard_text_desktop(&app, &text)
}

#[tauri::command]
pub async fn copy_item_and_paste(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    paste_item(app, id).await
}

#[tauri::command]
pub async fn copy_item_and_paste_from_popup(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    // Hide popup first so focus returns to previous application
    if let Some(popup) = app.get_webview_window("popup") {
        let _ = popup.hide();
    }

    #[cfg(target_os = "macos")]
    {
        let target = app
            .try_state::<crate::PopupPasteTarget>()
            .and_then(|t| t.0.lock().ok().and_then(|g| g.clone()));

        if let Some(bundle_id) = target {
            let _ = activate_app_macos(&bundle_id);
            tokio::time::sleep(std::time::Duration::from_millis(120)).await;
        } else {
            tokio::time::sleep(std::time::Duration::from_millis(80)).await;
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        tokio::time::sleep(std::time::Duration::from_millis(80)).await;
    }

    paste_item(app, id).await
}

#[tauri::command]
pub fn get_macos_accessibility_status() -> bool {
    #[cfg(target_os = "macos")]
    {
        is_process_trusted_macos()
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}

#[cfg(target_os = "macos")]
fn is_process_trusted_macos() -> bool {
    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn AXIsProcessTrusted() -> bool;
    }
    unsafe { AXIsProcessTrusted() }
}

#[tauri::command]
pub fn open_macos_accessibility_settings() {
    #[cfg(target_os = "macos")]
    {
        use cocoa::base::{id, nil};
        use cocoa::foundation::NSString;
        use objc::{class, msg_send, sel, sel_impl};
        unsafe {
            let url_str = NSString::alloc(nil).init_str("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility");
            let url: id = msg_send![class!(NSURL), URLWithString: url_str];
            let ws: id = msg_send![class!(NSWorkspace), sharedWorkspace];
            let _: bool = msg_send![ws, openURL: url];
        }
    }
}

#[tauri::command]
pub fn manual_cleanup(app: tauri::AppHandle) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.prune_expired().map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

#[tauri::command]
pub fn close_window(window: tauri::Window) {
    let _ = window.hide();
}

#[tauri::command]
pub fn backup_data(app: tauri::AppHandle, groups: Option<Vec<String>>) -> Result<String, String> {
    let db = app.state::<ClipboardDB>();
    db.get_all_data_json(groups).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn restore_data(
    app: tauri::AppHandle,
    json_content: String,
    mode: String,
) -> Result<(), String> {
    let db = app.state::<ClipboardDB>();
    db.restore_from_json(&json_content, &mode)
        .map_err(|e| e.to_string())?;
    let _ = app.emit("clipboard-updated", ());
    Ok(())
}

// ── Helpers ───────────────────────────────────────────────────────────────────

#[cfg(all(desktop, not(target_os = "android")))]
fn set_clipboard_text_desktop(_app: &tauri::AppHandle, text: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e: arboard::Error| e.to_string())?;
    clipboard.set_text(text).map_err(|e: arboard::Error| e.to_string())
}

#[cfg(any(mobile, target_os = "android"))]
fn set_clipboard_text_desktop(app: &tauri::AppHandle, text: &str) -> Result<(), String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    app.clipboard().write_text(text.to_string()).map_err(|e| e.to_string())
}

fn set_clipboard_payload(app: &tauri::AppHandle, content_type: &str, raw: &str) -> Result<(), String> {
    match content_type {
        "text" => set_clipboard_text_desktop(app, raw),
        "image" => {
            #[cfg(all(desktop, not(target_os = "android")))]
            {
                let db = app.state::<ClipboardDB>();
                let bytes = db.get_blob(raw).map_err(|e| e.to_string())?;
                let img = image::load_from_memory(&bytes).map_err(|e| e.to_string())?;
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();

                let mut clipboard = Clipboard::new().map_err(|e: arboard::Error| e.to_string())?;
                clipboard
                    .set_image(arboard::ImageData {
                        width: w as usize,
                        height: h as usize,
                        bytes: std::borrow::Cow::Owned(rgba.into_raw()),
                    })
                    .map_err(|e: arboard::Error| e.to_string())
            }
            #[cfg(any(mobile, target_os = "android"))]
            {
                Err("Image clipboard setting not supported on mobile".into())
            }
        }
        "files" => {
            let paths: Vec<String> = serde_json::from_str(raw).map_err(|e| e.to_string())?;
            if paths.is_empty() {
                return Ok(());
            }

            #[cfg(target_os = "macos")]
            {
                if crate::write_clipboard_file_paths(&paths) {
                    return Ok(());
                }
            }

            set_clipboard_text_desktop(app, &paths.join("\n"))
        }
        _ => Err(format!("Unsupported content type: {content_type}")),
    }
}

#[cfg(target_os = "macos")]
fn activate_app_macos(bundle_id: &str) -> Result<(), String> {
    use cocoa::base::{id, nil};
    use cocoa::foundation::NSString;
    use objc::{class, msg_send, sel, sel_impl};

    unsafe {
        let c_str = std::ffi::CString::new(bundle_id).map_err(|e| e.to_string())?;
        let ns_bundle: id = msg_send![class!(NSString), stringWithUTF8String: c_str.as_ptr()];
        let apps: id = msg_send![
            class!(NSRunningApplication),
            runningApplicationsWithBundleIdentifier: ns_bundle
        ];
        if apps == nil {
            return Err("NSRunningApplication query failed".into());
        }
        let count: usize = msg_send![apps, count];
        if count == 0 {
            return Err(format!("App with bundle ID '{bundle_id}' not running"));
        }
        let app: id = msg_send![apps, objectAtIndex: 0];
        let ok: bool = msg_send![app, activateWithOptions: 1u64 << 1]; // NSApplicationActivateIgnoringOtherApps
        if ok {
            Ok(())
        } else {
            Err("activateWithOptions returned false".into())
        }
    }
}
