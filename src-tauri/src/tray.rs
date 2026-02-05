use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, ActivationPolicy,
};

const MENU_COPY_STATS: &str = "copy_stats";
const MENU_QUIT: &str = "quit";

pub fn setup_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        app.set_activation_policy(ActivationPolicy::Accessory);
    }
    
    let copy_i = MenuItem::with_id(app, MENU_COPY_STATS, "Copy Stats to Clipboard", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, MENU_QUIT, "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&copy_i, &quit_i])?;
    
    TrayIconBuilder::with_id("main-tray")
        .tooltip("Clipper")
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(true)
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            MENU_COPY_STATS => copy_current_stats(app),
            MENU_QUIT => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                rect,
                ..
            } = event
            {
                toggle_window_at_tray(tray.app_handle(), &rect);
            }
        })
        .build(app)?;
    
    Ok(())
}

fn toggle_window_at_tray(app: &AppHandle, rect: &tauri::Rect) {
    if let Some(window) = app.get_webview_window("main") {
        // Check if window is visible
        let is_visible = window.is_visible().unwrap_or(false);
        
        if is_visible {
            // Hide the window if it's currently visible
            let _ = window.hide();
        } else {
            // Show and position the window if it's hidden
            
            // Get scale factor for the window (needed for proper coordinate conversion)
            let scale_factor = window.scale_factor().unwrap_or(1.0);
            
            // Get window size in logical pixels
            let win_size = window.outer_size().unwrap_or(tauri::PhysicalSize { width: 340, height: 320 });
            let win_w = (win_size.width as f64 / scale_factor) as i32;
            
            // Tray rect is in physical pixels - convert to logical for positioning
            let (tray_x, tray_y, tray_w, tray_h) = match (&rect.position, &rect.size) {
                (tauri::Position::Physical(pos), tauri::Size::Physical(size)) => {
                    (
                        (pos.x as f64 / scale_factor) as i32,
                        (pos.y as f64 / scale_factor) as i32,
                        (size.width as f64 / scale_factor) as i32,
                        (size.height as f64 / scale_factor) as i32,
                    )
                }
                (tauri::Position::Logical(pos), tauri::Size::Logical(size)) => {
                    (pos.x as i32, pos.y as i32, size.width as i32, size.height as i32)
                }
                (tauri::Position::Physical(pos), tauri::Size::Logical(size)) => {
                    (
                        (pos.x as f64 / scale_factor) as i32,
                        (pos.y as f64 / scale_factor) as i32,
                        size.width as i32,
                        size.height as i32,
                    )
                }
                (tauri::Position::Logical(pos), tauri::Size::Physical(size)) => {
                    (
                        pos.x as i32,
                        pos.y as i32,
                        (size.width as f64 / scale_factor) as i32,
                        (size.height as f64 / scale_factor) as i32,
                    )
                }
            };
            
            // Center window horizontally on the tray icon
            let tray_center_x = tray_x + (tray_w / 2);
            let window_x = tray_center_x - (win_w / 2);
            
            // Position window right below the tray icon (tray_y is already the top of the tray)
            let window_y = tray_y + tray_h;
            
            // Use logical position for setting - Tauri handles the conversion
            let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition {
                x: window_x as f64,
                y: window_y as f64,
            }));
            
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

fn copy_current_stats(app: &AppHandle) {
    use crate::AppState;
    
    if let Some(state) = app.try_state::<AppState>() {
        let monitor = state.clipboard_monitor.lock().unwrap();
        let stats = monitor.get_stats();
        drop(monitor);
        
        let summary = format!(
            "Characters: {}\nWords: {}\nLines: {}\nBytes: {}\nNon-whitespace: {}",
            stats.char_count,
            stats.word_count,
            stats.line_count,
            stats.byte_count,
            stats.non_whitespace_chars
        );
        
        crate::clipboard::set_text(&summary);
    }
}
