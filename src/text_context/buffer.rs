use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct TextContextBuffer {
    buffer: String,
    cursor_pos: usize,
    max_capacity: usize,
    last_updated: Instant,
    ttl: Duration,
    app_identifier: String,
}

impl TextContextBuffer {
    pub fn new(max_capacity: usize, ttl: Duration) -> Self {
        Self {
            buffer: String::with_capacity(max_capacity.min(1024)),
            cursor_pos: 0,
            max_capacity,
            last_updated: Instant::now(),
            ttl,
            app_identifier: String::new(),
        }
    }

    pub fn set_app(&mut self, app: &str) {
        if self.app_identifier != app {
            self.clear();
            self.app_identifier = app.to_string();
        }
    }

    pub fn app(&self) -> &str {
        &self.app_identifier
    }

    pub fn insert_char(&mut self, ch: char) {
        self.check_ttl();
        if self.cursor_pos >= self.buffer.len() {
            self.buffer.push(ch);
            self.cursor_pos = self.buffer.len();
        } else {
            let mut byte_idx = 0;
            for (idx, _) in self.buffer.char_indices() {
                if idx >= self.cursor_pos {
                    byte_idx = idx;
                    break;
                }
            }
            self.buffer.insert(byte_idx, ch);
            self.cursor_pos += ch.len_utf8();
        }
        self.enforce_capacity();
        self.last_updated = Instant::now();
    }

    pub fn insert_str(&mut self, text: &str) {
        self.check_ttl();
        if self.cursor_pos >= self.buffer.len() {
            self.buffer.push_str(text);
            self.cursor_pos = self.buffer.len();
        } else {
            self.buffer.insert_str(self.cursor_pos, text);
            self.cursor_pos += text.len();
        }
        self.enforce_capacity();
        self.last_updated = Instant::now();
    }

    pub fn backspace(&mut self) {
        self.check_ttl();
        if self.cursor_pos > 0 && !self.buffer.is_empty() {
            let mut target_idx = 0;
            let mut prev_idx = 0;
            for (idx, ch) in self.buffer.char_indices() {
                if idx + ch.len_utf8() >= self.cursor_pos {
                    target_idx = idx;
                    break;
                }
                prev_idx = idx + ch.len_utf8();
            }
            if target_idx < self.buffer.len() {
                self.buffer.remove(target_idx);
                self.cursor_pos = prev_idx;
            }
        }
        self.last_updated = Instant::now();
    }

    pub fn delete_forward(&mut self) {
        self.check_ttl();
        if self.cursor_pos < self.buffer.len() {
            self.buffer.remove(self.cursor_pos);
        }
        self.last_updated = Instant::now();
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            let mut prev = 0;
            for (idx, ch) in self.buffer.char_indices() {
                if idx + ch.len_utf8() >= self.cursor_pos {
                    self.cursor_pos = prev;
                    return;
                }
                prev = idx + ch.len_utf8();
            }
            self.cursor_pos = prev;
        }
    }

    pub fn move_cursor_right(&mut self) {
        for (idx, ch) in self.buffer.char_indices() {
            if idx >= self.cursor_pos {
                self.cursor_pos = idx + ch.len_utf8();
                return;
            }
        }
        self.cursor_pos = self.buffer.len();
    }

    pub fn set_text(&mut self, text: &str, cursor_pos: usize) {
        self.buffer.clear();
        self.buffer.push_str(text);
        self.cursor_pos = cursor_pos.min(self.buffer.len());
        self.enforce_capacity();
        self.last_updated = Instant::now();
    }

    pub fn get_text(&self) -> &str {
        if self.is_expired() {
            ""
        } else {
            &self.buffer
        }
    }

    pub fn cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.cursor_pos = 0;
        self.last_updated = Instant::now();
    }

    fn check_ttl(&mut self) {
        if self.is_expired() {
            self.clear();
        }
    }

    pub fn is_expired(&self) -> bool {
        self.last_updated.elapsed() > self.ttl
    }

    fn enforce_capacity(&mut self) {
        if self.buffer.len() > self.max_capacity {
            let excess = self.buffer.len() - self.max_capacity;
            let mut cut = excess;
            while cut < self.buffer.len() && !self.buffer.is_char_boundary(cut) {
                cut += 1;
            }
            self.buffer.drain(..cut);
            self.cursor_pos = self.cursor_pos.saturating_sub(cut);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_typing_and_backspace() {
        let mut buf = TextContextBuffer::new(500, Duration::from_secs(60));
        for c in "Hello worrld".chars() {
            buf.insert_char(c);
        }
        assert_eq!(buf.get_text(), "Hello worrld");
        // Backspace 4 times: 'd', 'l', 'r', 'r'
        buf.backspace();
        buf.backspace();
        buf.backspace();
        buf.backspace();
        assert_eq!(buf.get_text(), "Hello wo");
        buf.insert_str("rld");
        assert_eq!(buf.get_text(), "Hello world");
    }

    #[test]
    fn test_buffer_capacity_limit() {
        let mut buf = TextContextBuffer::new(20, Duration::from_secs(60));
        buf.insert_str("This is a very long sentence exceeding limit.");
        assert!(buf.get_text().len() <= 20);
    }

    #[test]
    fn test_buffer_ttl_expiration() {
        let mut buf = TextContextBuffer::new(500, Duration::from_millis(50));
        buf.insert_str("Transient text");
        assert_eq!(buf.get_text(), "Transient text");
        std::thread::sleep(Duration::from_millis(60));
        assert_eq!(buf.get_text(), "");
    }
}
