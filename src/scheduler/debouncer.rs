use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct InferenceRequest {
    pub request_id: u64,
    pub text: String,
    pub cursor_pos: usize,
    pub app_name: String,
    pub timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct InferenceResponse {
    pub request_id: u64,
    pub original_text: String,
    pub corrected_text: String,
    pub app_name: String,
    pub is_stale: bool,
    pub duration_ms: u64,
}

pub struct DebounceScheduler {
    debounce_duration: Duration,
    current_request_id: Arc<AtomicU64>,
    last_keystroke_time: Mutex<Instant>,
    pending_request: Mutex<Option<InferenceRequest>>,
}

impl DebounceScheduler {
    pub fn new(debounce_ms: u64) -> Self {
        Self {
            debounce_duration: Duration::from_millis(debounce_ms),
            current_request_id: Arc::new(AtomicU64::new(1)),
            last_keystroke_time: Mutex::new(Instant::now()),
            pending_request: Mutex::new(None),
        }
    }

    /// Submits a new text event from user input. Generates a fresh monotonic request ID.
    pub fn submit_input(&self, text: &str, cursor_pos: usize, app_name: &str) -> u64 {
        let req_id = self.current_request_id.fetch_add(1, Ordering::SeqCst) + 1;
        let mut last_time = self.last_keystroke_time.lock().unwrap();
        *last_time = Instant::now();

        let req = InferenceRequest {
            request_id: req_id,
            text: text.to_string(),
            cursor_pos,
            app_name: app_name.to_string(),
            timestamp: *last_time,
        };

        let mut pending = self.pending_request.lock().unwrap();
        *pending = Some(req);

        req_id
    }

    /// Checks if debounce period has elapsed and returns the ready request if valid
    pub fn poll_ready_request(&self) -> Option<InferenceRequest> {
        let last_time = *self.last_keystroke_time.lock().unwrap();
        if last_time.elapsed() >= self.debounce_duration {
            let mut pending = self.pending_request.lock().unwrap();
            pending.take()
        } else {
            None
        }
    }

    /// Checks if a request ID is still current or has been invalidated by newer input
    pub fn is_request_current(&self, request_id: u64) -> bool {
        self.current_request_id.load(Ordering::SeqCst) == request_id
    }

    pub fn latest_request_id(&self) -> u64 {
        self.current_request_id.load(Ordering::SeqCst)
    }

    pub fn cancel_all(&self) {
        self.current_request_id.fetch_add(1, Ordering::SeqCst);
        let mut pending = self.pending_request.lock().unwrap();
        *pending = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debouncing_and_versioning() {
        let scheduler = DebounceScheduler::new(50);
        let id1 = scheduler.submit_input("He go", 5, "notepad.exe");
        assert_eq!(scheduler.poll_ready_request().map(|r| r.request_id), None);

        // Immediate subsequent typing updates request ID
        let id2 = scheduler.submit_input("He go to school", 15, "notepad.exe");
        assert!(id2 > id1);
        assert!(!scheduler.is_request_current(id1));
        assert!(scheduler.is_request_current(id2));

        // Wait for debounce timeout
        std::thread::sleep(Duration::from_millis(60));
        let ready = scheduler.poll_ready_request();
        assert!(ready.is_some());
        let r = ready.unwrap();
        assert_eq!(r.request_id, id2);
        assert_eq!(r.text, "He go to school");
    }

    #[test]
    fn test_stale_request_detection() {
        let scheduler = DebounceScheduler::new(20);
        let id1 = scheduler.submit_input("Sentence 1", 10, "app.exe");
        std::thread::sleep(Duration::from_millis(30));
        let _ = scheduler.poll_ready_request();

        // User types something new while inference is in flight
        let id2 = scheduler.submit_input("Sentence 1 continuing", 21, "app.exe");
        assert!(!scheduler.is_request_current(id1));
        assert!(scheduler.is_request_current(id2));
    }
}
