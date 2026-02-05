use arboard::Clipboard;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::stats::ClipboardStats;

pub struct ClipboardMonitor {
    last_content: String,
    has_update: AtomicBool,
    current_stats: ClipboardStats,
}

impl ClipboardMonitor {
    pub fn new() -> Self {
        let last_content = get_text().unwrap_or_default();
        let current_stats = ClipboardStats::from_text(&last_content);
        
        Self {
            last_content,
            has_update: AtomicBool::new(false),
            current_stats,
        }
    }
    
    /// Check if clipboard content has changed
    pub fn check_update(&mut self) -> bool {
        if let Some(content) = get_text() {
            if content != self.last_content {
                self.last_content = content.clone();
                self.current_stats = ClipboardStats::from_text(&content);
                self.has_update.store(true, Ordering::SeqCst);
                return true;
            }
        }
        false
    }
    
    /// Get current clipboard stats
    pub fn get_stats(&self) -> ClipboardStats {
        self.current_stats.clone()
    }
    
    /// Get raw clipboard content
    pub fn get_content(&self) -> String {
        self.last_content.clone()
    }
    
    /// Clear update flag
    pub fn clear_update_flag(&self) {
        self.has_update.store(false, Ordering::SeqCst);
    }
}

/// Set text to clipboard
pub fn set_text(text: &str) {
    if let Ok(mut clipboard) = Clipboard::new() {
        let _ = clipboard.set_text(text.to_string());
    }
}

/// Get text from clipboard
pub fn get_text() -> Option<String> {
    Clipboard::new().ok()?.get_text().ok()
}
