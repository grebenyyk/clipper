use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardStats {
    pub char_count: usize,
    pub word_count: usize,
    pub line_count: usize,
    pub byte_count: usize,
    pub non_whitespace_chars: usize,
}

impl ClipboardStats {
    pub fn from_text(text: &str) -> Self {
        let char_count = text.chars().count();
        let byte_count = text.len();
        let non_whitespace_chars = text.chars().filter(|c| !c.is_whitespace()).count();
        
        // Word count - split on whitespace
        let word_count = text
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .count();
        
        // Line count
        let line_count = if text.is_empty() {
            0
        } else {
            text.lines().count()
        };
        
        Self {
            char_count,
            word_count,
            line_count,
            byte_count,
            non_whitespace_chars,
        }
    }
    
    /// Format stats as a human-readable string
    pub fn format_summary(&self) -> String {
        format!(
            "{} chars | {} words | {} lines",
            self.char_count, self.word_count, self.line_count
        )
    }
    
    /// Format stats with full details
    pub fn format_detailed(&self) -> String {
        format!(
            "Characters: {}\nWords: {}\nLines: {}\nBytes: {}\nNon-whitespace chars: {}",
            self.char_count,
            self.word_count,
            self.line_count,
            self.byte_count,
            self.non_whitespace_chars
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_text() {
        let stats = ClipboardStats::from_text("");
        assert_eq!(stats.char_count, 0);
        assert_eq!(stats.word_count, 0);
        assert_eq!(stats.line_count, 0);
        assert_eq!(stats.byte_count, 0);
    }
    
    #[test]
    fn test_simple_text() {
        let stats = ClipboardStats::from_text("Hello world");
        assert_eq!(stats.char_count, 11);
        assert_eq!(stats.word_count, 2);
        assert_eq!(stats.line_count, 1);
        assert_eq!(stats.byte_count, 11);
    }
    
    #[test]
    fn test_multiline_text() {
        let stats = ClipboardStats::from_text("Line 1\nLine 2\nLine 3");
        assert_eq!(stats.char_count, 17);
        assert_eq!(stats.word_count, 6);
        assert_eq!(stats.line_count, 3);
    }
    
    #[test]
    fn test_unicode_text() {
        let stats = ClipboardStats::from_text("Hello 世界 🌍");
        assert_eq!(stats.char_count, 10); // Including emoji and CJK chars
        assert_eq!(stats.word_count, 3);
    }
}
