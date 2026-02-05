// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clipboard;
mod stats;
mod tray;

use clipboard::ClipboardMonitor;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, State};

pub struct AppState {
    clipboard_monitor: Arc<Mutex<ClipboardMonitor>>,
}

#[tauri::command]
fn get_clipboard_stats(state: State<AppState>) -> stats::ClipboardStats {
    let monitor = state.clipboard_monitor.lock().unwrap();
    monitor.get_stats()
}

#[tauri::command]
fn copy_stats_to_clipboard(stats: stats::ClipboardStats) {
    let summary = format!(
        "Characters: {}\nWords: {}\nLines: {}\nBytes: {}",
        stats.char_count, stats.word_count, stats.line_count, stats.byte_count
    );
    clipboard::set_text(&summary);
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Apply macOS vibrancy effect to the main window
            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};
                
                if let Some(window) = app.get_webview_window("main") {
                    // Apply vibrancy with active state
                    let _ = apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, Some(NSVisualEffectState::Active), Some(16.0));
                }
            }
            
            // Apply blur effect on Windows
            #[cfg(target_os = "windows")]
            {
                use window_vibrancy::apply_blur;
                if let Some(window) = app.get_webview_window("main") {
                    let _ = apply_blur(&window, Some((18, 18, 18, 125)));
                }
            }
            let clipboard_monitor = Arc::new(Mutex::new(ClipboardMonitor::new()));
            
            // Set up system tray
            tray::setup_tray(app)?;
            
            // Start clipboard monitoring
            let app_handle = app.handle().clone();
            let monitor = clipboard_monitor.clone();
            std::thread::spawn(move || {
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    
                    let mut m = monitor.lock().unwrap();
                    if m.check_update() {
                        let stats = m.get_stats();
                        drop(m);
                        
                        // Update tray tooltip with basic stats
                        if let Some(tray) = app_handle.tray_by_id("main-tray") {
                            let _ = tray.set_tooltip(Some(&format!(
                                "Clipper: {} chars, {} words",
                                stats.char_count, stats.word_count
                            )));
                        }
                        
                        // Emit event to UI if window is open
                        app_handle.emit("clipboard-updated", stats).ok();
                    }
                }
            });
            
            app.manage(AppState { clipboard_monitor });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_clipboard_stats,
            copy_stats_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
