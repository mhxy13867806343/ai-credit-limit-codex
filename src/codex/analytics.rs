use std::path::PathBuf;
use std::process::Command;

use super::models::{DailyUsageItem, ModelUsageItem};

/// Parse local sqlite DB (state_5.sqlite) and models_cache.json dynamically
pub fn fetch_sqlite_analytics() -> (Vec<ModelUsageItem>, Vec<DailyUsageItem>, Option<String>, i64) {
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => return (Vec::new(), Vec::new(), None, 0),
    };
    let db_path = PathBuf::from(&home).join(".codex").join("state_5.sqlite");
    if !db_path.exists() {
        return (Vec::new(), Vec::new(), None, 0);
    }
    let db_str = db_path.to_string_lossy().to_string();

    // 1. Top models from SQLite threads table
    let mut top_models = Vec::new();
    let models_out = Command::new("/usr/bin/sqlite3")
        .arg(&db_str)
        .arg("SELECT model, count(*), sum(tokens_used) FROM threads WHERE model IS NOT NULL AND model != '' GROUP BY model ORDER BY sum(tokens_used) DESC;")
        .output();

    if let Ok(out) = models_out {
        if let Ok(text) = String::from_utf8(out.stdout) {
            let mut total_tokens: i64 = 0;
            let mut rows = Vec::new();
            for line in text.lines() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 3 {
                    let m = parts[0].to_string();
                    let cnt = parts[1].parse::<i64>().unwrap_or(0);
                    let tok = parts[2].parse::<i64>().unwrap_or(0);
                    total_tokens += tok;
                    rows.push((m, cnt, tok));
                }
            }
            for (m, cnt, tok) in rows {
                let pct = if total_tokens > 0 { (tok as f64 / total_tokens as f64) * 100.0 } else { 0.0 };
                top_models.push(ModelUsageItem {
                    model_name: m,
                    thread_count: cnt,
                    tokens_used: tok,
                    percentage: pct,
                });
            }
        }
    }

    // 2. Discover additional models dynamically from ~/.codex/models_cache.json (zero hardcoding)
    let cache_path = PathBuf::from(&home).join(".codex").join("models_cache.json");
    if cache_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&cache_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(models_arr) = json.get("models").and_then(|m| m.as_array()) {
                    for m_obj in models_arr {
                        if let Some(slug) = m_obj.get("slug").and_then(|s| s.as_str()) {
                            if !top_models.iter().any(|item| item.model_name == slug) {
                                top_models.push(ModelUsageItem {
                                    model_name: slug.to_string(),
                                    thread_count: 0,
                                    tokens_used: 0,
                                    percentage: 0.0,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    top_models.sort_by(|a, b| b.tokens_used.cmp(&a.tokens_used).then_with(|| b.thread_count.cmp(&a.thread_count)));

    // 3. Daily buckets from sqlite
    let mut daily_items = Vec::new();
    let daily_out = Command::new("/usr/bin/sqlite3")
        .arg(&db_str)
        .arg("SELECT strftime('%Y-%m-%d', updated_at, 'unixepoch', 'localtime') as day, sum(tokens_used) FROM threads WHERE updated_at IS NOT NULL GROUP BY day ORDER BY day ASC;")
        .output();
    if let Ok(out) = daily_out {
        if let Ok(text) = String::from_utf8(out.stdout) {
            for line in text.lines() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 2 {
                    let day = parts[0].to_string();
                    let tok = parts[1].parse::<i64>().unwrap_or(0);
                    daily_items.push(DailyUsageItem { date: day, tokens: tok });
                }
            }
        }
    }

    // 4. Total threads count
    let mut threads_cnt = 0;
    let t_out = Command::new("/usr/bin/sqlite3")
        .arg(&db_str)
        .arg("SELECT count(*) FROM threads;")
        .output();
    if let Ok(out) = t_out {
        if let Ok(text) = String::from_utf8(out.stdout) {
            threads_cnt = text.trim().parse::<i64>().unwrap_or(0);
        }
    }

    // 5. Reasoning effort with dynamic percentage calculation
    let mut reasoning = None;
    let r_out = Command::new("/usr/bin/sqlite3")
        .arg(&db_str)
        .arg("SELECT COALESCE(reasoning_effort, 'none'), count(*), sum(tokens_used) FROM threads GROUP BY reasoning_effort ORDER BY sum(tokens_used) DESC LIMIT 1;")
        .output();
    if let Ok(out) = r_out {
        if let Ok(text) = String::from_utf8(out.stdout) {
            if let Some(line) = text.lines().next() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 2 {
                    let raw_level = parts[0];
                    let r_count = parts[1].parse::<i64>().unwrap_or(0);
                    let label = match raw_level {
                        "high" | "xhigh" | "max" => "高",
                        "medium" => "中",
                        "low" => "低",
                        other => other,
                    };
                    let pct = if threads_cnt > 0 {
                        (r_count as f64 / threads_cnt as f64 * 100.0).round() as i64
                    } else {
                        0
                    };
                    reasoning = Some(format!("{} · {}%", label, pct));
                }
            }
        }
    }

    (top_models, daily_items, reasoning, threads_cnt)
}
