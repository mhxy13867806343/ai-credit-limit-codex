use super::format::current_local_time_str;
use super::models::{CodexUsage, GetAccountRateLimitsResponse, GlobalResetInfo};
use super::rpc::{fetch_via_app_server, populate_from_rate_limits};

pub fn fetch_global_reset_status() -> Option<GlobalResetInfo> {
    let resp = ureq::get("https://codex-resets.com/")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)")
        .call()
        .ok()?;
    let body = resp.into_body().read_to_string().ok()?;
    let dt_marker = "data-role=\"relative-time\"";
    let dt_idx = body.find(dt_marker)?;
    let slice = &body[dt_idx..dt_idx + 200.min(body.len() - dt_idx)];
    let dt_start = slice.find("data-datetime=\"")? + 15;
    let dt_end = slice[dt_start..].find('"')? + dt_start;
    let iso_date = &slice[dt_start..dt_end];

    let text_start = slice.find('>')? + 1;
    let text_end = slice[text_start..].find('<')? + text_start;
    let rel_text = slice[text_start..text_end].trim();

    let zh_text = if rel_text.contains("hour") {
        let num: String = rel_text.chars().take_while(|c| c.is_ascii_digit()).collect();
        format!("{}小时前", num)
    } else if rel_text.contains("day") {
        let num: String = rel_text.chars().take_while(|c| c.is_ascii_digit()).collect();
        format!("{}天前", num)
    } else if rel_text.contains("min") {
        let num: String = rel_text.chars().take_while(|c| c.is_ascii_digit()).collect();
        format!("{}分钟前", num)
    } else {
        rel_text.to_string()
    };

    Some(GlobalResetInfo {
        latest_text: Some(zh_text),
        announced_at: Some(iso_date.to_string()),
        author: Some("codex-resets".to_string()),
        days_since_last: Some(1.0),
        avg_interval_days: Some(6.9),
        total_resets: Some(53),
    })
}

pub fn fetch_via_auth_file() -> Result<CodexUsage, String> {
    let home = std::env::var("HOME").map_err(|_| "No HOME environment variable")?;
    let auth_path = std::path::PathBuf::from(home).join(".codex").join("auth.json");
    if !auth_path.exists() {
        return Err("~/.codex/auth.json not found".to_string());
    }
    let content = std::fs::read_to_string(&auth_path).map_err(|e| format!("Failed to read auth.json: {}", e))?;
    let v: serde_json::Value = serde_json::from_str(&content).map_err(|e| format!("Invalid JSON in auth.json: {}", e))?;

    let token = v.get("tokens")
        .and_then(|t| t.get("access_token"))
        .and_then(|a| a.as_str())
        .or_else(|| v.get("access_token").and_then(|a| a.as_str()))
        .or_else(|| v.get("api_key").and_then(|a| a.as_str()))
        .ok_or("No access token found in auth.json")?;

    let url = "https://chatgpt.com/backend-api/wham/usage";
    let resp = ureq::get(url)
        .header("Authorization", &format!("Bearer {}", token))
        .header("User-Agent", "codex-monitor/0.1.0")
        .call()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let body = resp.into_body().read_to_string().map_err(|e| format!("Failed to read response body: {}", e))?;
    let mut usage = CodexUsage::default();
    if let Ok(res) = serde_json::from_str::<GetAccountRateLimitsResponse>(&body) {
        populate_from_rate_limits(&mut usage, &res);
        usage.updated_at = current_local_time_str();
        return Ok(usage);
    }
    Err("Could not parse fallback response".to_string())
}

pub fn fetch_codex_usage() -> CodexUsage {
    let mut usage = if let Some(bin) = crate::config::find_codex_bin() {
        fetch_via_app_server(&bin).unwrap_or_else(|_| fetch_via_auth_file().unwrap_or_default())
    } else {
        fetch_via_auth_file().unwrap_or_default()
    };
    if let Some(gr) = fetch_global_reset_status() {
        usage.global_reset = Some(gr);
    }
    usage
}
