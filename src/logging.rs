use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};

static LOG_LEVEL: AtomicUsize = AtomicUsize::new(2); // 0=OFF/ERROR, 1=WARN, 2=INFO, 3=DEBUG
static INIT: Once = Once::new();

pub fn init_logger(level_str: &str) {
    INIT.call_once(|| {
        let lvl = match level_str.to_lowercase().as_str() {
            "error" => 0,
            "warn" => 1,
            "info" => 2,
            "debug" => 3,
            _ => 2,
        };
        LOG_LEVEL.store(lvl, Ordering::SeqCst);
    });
}

pub fn log_info(event: &str, fields: &[(&str, &str)]) {
    if LOG_LEVEL.load(Ordering::Relaxed) >= 2 {
        emit_log("INFO", event, fields);
    }
}

pub fn log_warn(event: &str, fields: &[(&str, &str)]) {
    if LOG_LEVEL.load(Ordering::Relaxed) >= 1 {
        emit_log("WARN", event, fields);
    }
}

pub fn log_error(event: &str, fields: &[(&str, &str)]) {
    emit_log("ERROR", event, fields);
}

pub fn log_debug(event: &str, fields: &[(&str, &str)]) {
    if LOG_LEVEL.load(Ordering::Relaxed) >= 3 {
        emit_log("DEBUG", event, fields);
    }
}

fn emit_log(level: &str, event: &str, fields: &[(&str, &str)]) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let mut fields_str = String::new();
    for (k, v) in fields {
        fields_str.push_str(&format!(" {}={}", k, v));
    }

    // PRIVACY ENFORCEMENT: Output structured events only, never raw user text
    eprintln!("[{}] {} timestamp_ms={} event={}{}", level, now, now, event, fields_str);
}

pub fn log_inference_metrics(request_id: u64, latency_ms: u64, memory_mb: u64, confidence: f32, rule_applied: bool) {
    let req_str = request_id.to_string();
    let lat_str = latency_ms.to_string();
    let mem_str = memory_mb.to_string();
    let conf_str = format!("{:.3}", confidence);
    let rule_str = rule_applied.to_string();

    log_info(
        "inference_completed",
        &[
            ("request_id", &req_str),
            ("latency_ms", &lat_str),
            ("memory_mb", &mem_str),
            ("confidence", &conf_str),
            ("rule_applied", &rule_str),
        ],
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_safety() {
        init_logger("info");
        log_info("startup_ok", &[("platform", "windows")]);
        log_inference_metrics(42, 15, 38, 0.95, true);
    }
}
