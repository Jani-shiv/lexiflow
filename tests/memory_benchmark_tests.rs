use lexiflow::benchmark::MemoryBenchmarkReport;

#[test]
fn test_runtime_memory_strictly_under_100mb() {
    let result = MemoryBenchmarkReport::run_benchmark();
    assert!(result.passes_100mb_threshold, "Memory exceeded 100MB requirement! Peak: {:.2} MB", result.peak_inference_rss_mb);
    assert!(result.peak_inference_rss_mb < 100.0, "Expected peak memory under 100MB, but was {:.2} MB", result.peak_inference_rss_mb);
}
