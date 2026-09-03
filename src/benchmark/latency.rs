use std::time::Instant;

#[derive(Debug, Clone)]
pub struct LatencyBenchmarkReport {
    pub cold_start_ms: f64,
    pub avg_inference_us: f64,
    pub p95_inference_us: f64,
    pub p99_inference_us: f64,
    pub total_samples: usize,
}

pub struct LatencyBenchmark;

impl LatencyBenchmark {
    pub fn run_benchmark(iterations: usize) -> LatencyBenchmarkReport {
        let cold_start_begin = Instant::now();
        let engine = crate::grammar::GrammarEngine::new();
        let cold_start_ms = cold_start_begin.elapsed().as_secs_f64() * 1000.0;

        let test_corpus = [
            "He go to school.",
            "I am go office right now.",
            "I recieved teh document yesterday.",
            "She have a apple in her bag.",
            "We could of won if we tried harder.",
            "Their is something wrong with this computer.",
            "Dont make noise in the library .",
            "He dont know what to do next.",
            "Please listen music quietly.",
            "I am very interested on this project.",
        ];

        let mut latencies_us = Vec::with_capacity(iterations);

        for i in 0..iterations {
            let sentence = test_corpus[i % test_corpus.len()];
            let start = Instant::now();
            let _ = engine.correct_sentence(sentence);
            let elapsed_us = start.elapsed().as_micros() as f64;
            latencies_us.push(elapsed_us);
        }

        latencies_us.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let sum: f64 = latencies_us.iter().sum();
        let avg = sum / (latencies_us.len() as f64);
        let p95_idx = ((latencies_us.len() as f64) * 0.95) as usize;
        let p99_idx = ((latencies_us.len() as f64) * 0.99) as usize;

        LatencyBenchmarkReport {
            cold_start_ms,
            avg_inference_us: avg,
            p95_inference_us: latencies_us[p95_idx.min(latencies_us.len() - 1)],
            p99_inference_us: latencies_us[p99_idx.min(latencies_us.len() - 1)],
            total_samples: iterations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latency_performance() {
        let report = LatencyBenchmark::run_benchmark(500);
        println!(
            "LATENCY BENCHMARK: ColdStart={:.2}ms, Avg={:.2}us ({:.3}ms), P95={:.2}us, P99={:.2}us",
            report.cold_start_ms,
            report.avg_inference_us,
            report.avg_inference_us / 1000.0,
            report.p95_inference_us,
            report.p99_inference_us
        );
        // Average inference must be sub-5ms (5,000 us)
        assert!(report.avg_inference_us < 5000.0);
    }
}
