use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

#[derive(Debug, Clone, Copy)]
pub struct ProcessMemorySnapshot {
    pub rss_bytes: u64,
    pub rss_mb: f64,
}

pub struct MemoryTracker {
    system: System,
    pid: Pid,
}

impl MemoryTracker {
    pub fn new() -> Self {
        let pid = Pid::from_u32(std::process::id());
        let mut system = System::new_with_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::new().with_memory()),
        );
        system.refresh_processes_specifics(ProcessRefreshKind::new().with_memory());

        Self { system, pid }
    }

    /// Captures current process Resident Set Size (RSS) memory
    pub fn capture_rss(&mut self) -> ProcessMemorySnapshot {
        self.system.refresh_processes_specifics(ProcessRefreshKind::new().with_memory());
        let rss_bytes = if let Some(proc) = self.system.process(self.pid) {
            proc.memory()
        } else {
            0
        };

        ProcessMemorySnapshot {
            rss_bytes,
            rss_mb: (rss_bytes as f64) / (1024.0 * 1024.0),
        }
    }
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct MemoryBenchmarkReport {
    pub idle_rss_mb: f64,
    pub model_loaded_rss_mb: f64,
    pub peak_inference_rss_mb: f64,
    pub passes_100mb_threshold: bool,
}

impl MemoryBenchmarkReport {
    pub fn run_benchmark() -> Self {
        let mut tracker = MemoryTracker::new();
        let idle = tracker.capture_rss();

        // Load full engine
        let engine = crate::grammar::GrammarEngine::new();
        let loaded = tracker.capture_rss();

        let mut peak_rss = loaded.rss_mb;

        // Run 1,000 sequential test inferences
        let test_sentences = [
            "He go to school every day.",
            "I recieved teh package on monday.",
            "I am go office right now.",
            "She have a apple in her bag.",
            "We could of done this yesterday.",
            "Their is a big problem with the system.",
            "The dog bark very loud.",
            "They was going to the park.",
            "Its a very nice morning today.",
            "Dont forget to check the car.",
        ];

        for i in 0..1000 {
            let sentence = test_sentences[i % test_sentences.len()];
            let (_corrected, _candidates) = engine.correct_sentence(sentence);
            if i % 100 == 0 {
                let cur = tracker.capture_rss();
                if cur.rss_mb > peak_rss {
                    peak_rss = cur.rss_mb;
                }
            }
        }

        let final_snap = tracker.capture_rss();
        if final_snap.rss_mb > peak_rss {
            peak_rss = final_snap.rss_mb;
        }

        Self {
            idle_rss_mb: idle.rss_mb,
            model_loaded_rss_mb: loaded.rss_mb,
            peak_inference_rss_mb: peak_rss,
            passes_100mb_threshold: peak_rss < 100.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_under_100mb() {
        let report = MemoryBenchmarkReport::run_benchmark();
        println!(
            "MEMORY BENCHMARK RESULT: Idle={:.2}MB, Loaded={:.2}MB, Peak={:.2}MB, Passes={}",
            report.idle_rss_mb,
            report.model_loaded_rss_mb,
            report.peak_inference_rss_mb,
            report.passes_100mb_threshold
        );
        assert!(
            report.passes_100mb_threshold,
            "Memory exceeded 100 MB budget! Peak was: {:.2} MB",
            report.peak_inference_rss_mb
        );
    }
}
