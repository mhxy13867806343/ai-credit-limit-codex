use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitWindow {
    #[serde(rename = "usedPercent")]
    pub used_percent: i32,
    #[serde(rename = "windowDurationMins")]
    pub window_duration_mins: Option<i64>,
    #[serde(rename = "resetsAt")]
    pub resets_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditsSnapshot {
    pub balance: Option<String>,
    #[serde(rename = "hasCredits")]
    pub has_credits: Option<bool>,
    pub unlimited: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitSnapshot {
    pub primary: Option<RateLimitWindow>,
    pub secondary: Option<RateLimitWindow>,
    pub credits: Option<CreditsSnapshot>,
    #[serde(rename = "planType")]
    pub plan_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountRateLimitsResponse {
    #[serde(rename = "rateLimits")]
    pub rate_limits: Option<RateLimitSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RpcEnvelope {
    pub id: Option<i64>,
    pub result: Option<GetAccountRateLimitsResponse>,
    pub error: Option<RpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct CodexUsage {
    pub primary_percent: Option<i32>,
    pub primary_window_mins: Option<i64>,
    pub primary_resets_at: Option<i64>,
    pub secondary_percent: Option<i32>,
    pub secondary_window_mins: Option<i64>,
    pub secondary_resets_at: Option<i64>,
    pub plan_type: Option<String>,
    pub credits_balance: Option<String>,
    pub updated_at: String,
    pub is_rate_limited: bool,
    pub raw_error: Option<String>,
}

impl Default for CodexUsage {
    fn default() -> Self {
        Self {
            primary_percent: None,
            primary_window_mins: None,
            primary_resets_at: None,
            secondary_percent: None,
            secondary_window_mins: None,
            secondary_resets_at: None,
            plan_type: None,
            credits_balance: None,
            updated_at: current_time_str(),
            is_rate_limited: false,
            raw_error: None,
        }
    }
}

impl CodexUsage {
    /// Format string for display in macOS menu bar title (e.g. "☁ Codex 72%")
    pub fn menu_bar_title(&self) -> String {
        if let Some(err) = &self.raw_error {
            if self.primary_percent.is_none() {
                if err.contains("Not logged in") || err.contains("未登录") {
                    return "☁ Codex (未登录)".to_string();
                }
                return "☁ Codex (!)".to_string();
            }
        }

        match self.primary_percent {
            Some(pct) => {
                if pct >= 100 {
                    format!("⚠️ Codex {}%", pct)
                } else {
                    format!("☁ Codex {}%", pct)
                }
            }
            None => "☁ Codex ...".to_string(),
        }
    }

    /// Primary window name (e.g. "5h Window")
    pub fn primary_window_label(&self) -> String {
        match self.primary_window_mins {
            Some(mins) => format_window_name(mins),
            None => "5h Window".to_string(),
        }
    }

    /// Secondary window name (e.g. "Weekly Limit")
    pub fn secondary_window_label(&self) -> String {
        match self.secondary_window_mins {
            Some(mins) => format_window_name(mins),
            None => "Weekly Limit".to_string(),
        }
    }

    /// Remaining time until reset
    pub fn reset_countdown_label(&self) -> String {
        match self.primary_resets_at {
            Some(ts) => format_time_left(ts),
            None => "--".to_string(),
        }
    }
}

/// Helper: fetch usage through Codex CLI stdio JSON-RPC protocol
pub fn fetch_via_app_server(codex_bin: &std::path::Path) -> Result<CodexUsage, String> {
    let mut child = Command::new(codex_bin)
        .arg("app-server")
        .arg("--stdio")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn codex app-server: {}", e))?;

    let mut stdin = child.stdin.take().ok_or("Failed to open stdin")?;
    let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
    let mut reader = BufReader::new(stdout);

    // Step 1: Send `initialize` request
    let init_req = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"clientInfo":{"name":"codex-monitor","version":"0.1.0"}}}"#;
    writeln!(stdin, "{}", init_req).map_err(|e| format!("Failed to write initialize: {}", e))?;
    stdin.flush().map_err(|e| format!("Flush error: {}", e))?;

    let mut init_resp = String::new();
    // Read lines until we find response with id: 1
    let mut got_init = false;
    for _ in 0..10 {
        init_resp.clear();
        if reader.read_line(&mut init_resp).map_err(|e| e.to_string())? == 0 {
            break;
        }
        if init_resp.contains("\"id\":1") || init_resp.contains("\"id\": 1") {
            got_init = true;
            break;
        }
    }

    if !got_init {
        let _ = child.kill();
        return Err("No initialize response from codex app-server".to_string());
    }

    // Step 2: Send `account/rateLimits/read` request
    let rate_req = r#"{"jsonrpc":"2.0","id":2,"method":"account/rateLimits/read","params":null}"#;
    writeln!(stdin, "{}", rate_req).map_err(|e| format!("Failed to write rateLimits: {}", e))?;
    stdin.flush().map_err(|e| format!("Flush error: {}", e))?;

    let mut rate_resp = String::new();
    let mut response_line = None;

    for _ in 0..15 {
        rate_resp.clear();
        if reader.read_line(&mut rate_resp).map_err(|e| e.to_string())? == 0 {
            break;
        }
        if rate_resp.contains("\"id\":2") || rate_resp.contains("\"id\": 2") {
            response_line = Some(rate_resp.clone());
            break;
        }
    }

    // Terminate child gracefully
    let _ = child.kill();

    let line = response_line.ok_or("Timeout or no response for account/rateLimits/read")?;
    parse_rate_limits_json(&line)
}

/// Fallback: Read ~/.codex/auth.json and request direct API
pub fn fetch_via_auth_file() -> Result<CodexUsage, String> {
    let home = std::env::var("HOME").map_err(|_| "No HOME environment variable")?;
    let auth_path = std::path::PathBuf::from(home).join(".codex").join("auth.json");

    if !auth_path.exists() {
        return Err("~/.codex/auth.json not found".to_string());
    }

    let content = std::fs::read_to_string(&auth_path)
        .map_err(|e| format!("Failed to read auth.json: {}", e))?;

    let v: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Invalid JSON in auth.json: {}", e))?;

    let token = v
        .get("tokens")
        .and_then(|t| t.get("access_token"))
        .and_then(|a| a.as_str())
        .or_else(|| v.get("access_token").and_then(|a| a.as_str()))
        .or_else(|| v.get("api_key").and_then(|a| a.as_str()))
        .ok_or("No access token found in auth.json")?;

    // Try OpenAI Codex / Wham usage endpoint
    let url = "https://chatgpt.com/backend-api/wham/usage";
    let resp = ureq::get(url)
        .header("Authorization", &format!("Bearer {}", token))
        .header("User-Agent", "codex-monitor/0.1.0")
        .call()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let body = resp
        .into_body()
        .read_to_string()
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    parse_rate_limits_json(&body)
}

/// Main fetch function: orchestrates CLI query and fallback strategies
pub fn fetch_codex_usage() -> CodexUsage {
    // 1. Try Codex CLI app-server
    if let Some(bin) = crate::config::find_codex_bin() {
        match fetch_via_app_server(&bin) {
            Ok(usage) => return usage,
            Err(e) => {
                eprintln!("[codex-monitor] app-server query failed: {}", e);
            }
        }
    }

    // 2. Try auth file fallback
    match fetch_via_auth_file() {
        Ok(usage) => return usage,
        Err(e) => {
            eprintln!("[codex-monitor] auth file fallback failed: {}", e);
        }
    }

    // 3. Return informative error / offline status
    let mut usage = CodexUsage::default();
    usage.raw_error = Some("未检测到登录态或 Codex CLI".to_string());
    usage
}

/// Parse rate limits JSON from response
pub fn parse_rate_limits_json(raw_json: &str) -> Result<CodexUsage, String> {
    // Try parsing as RPC envelope first
    if let Ok(env) = serde_json::from_str::<RpcEnvelope>(raw_json) {
        if let Some(err) = env.error {
            return Err(format!("RPC error {}: {}", err.code, err.message));
        }
        if let Some(res) = env.result {
            if let Some(rl) = res.rate_limits {
                return Ok(build_usage_from_snapshot(&rl));
            }
        }
    }

    // Try parsing directly as GetAccountRateLimitsResponse
    if let Ok(res) = serde_json::from_str::<GetAccountRateLimitsResponse>(raw_json) {
        if let Some(rl) = res.rate_limits {
            return Ok(build_usage_from_snapshot(&rl));
        }
    }

    // Try parsing directly as RateLimitSnapshot
    if let Ok(snapshot) = serde_json::from_str::<RateLimitSnapshot>(raw_json) {
        return Ok(build_usage_from_snapshot(&snapshot));
    }

    // Try finding rateLimits object in loose JSON
    let val: serde_json::Value =
        serde_json::from_str(raw_json).map_err(|e| format!("JSON parse error: {}", e))?;

    if let Some(rl_val) = val.get("rateLimits").or_else(|| {
        val.get("result")
            .and_then(|r| r.get("rateLimits"))
    }) {
        if let Ok(snapshot) = serde_json::from_value::<RateLimitSnapshot>(rl_val.clone()) {
            return Ok(build_usage_from_snapshot(&snapshot));
        }
    }

    Err("Could not extract rate limits from response".to_string())
}

fn build_usage_from_snapshot(snapshot: &RateLimitSnapshot) -> CodexUsage {
    let mut usage = CodexUsage::default();
    usage.plan_type = snapshot.plan_type.clone();

    if let Some(primary) = &snapshot.primary {
        usage.primary_percent = Some(primary.used_percent);
        usage.primary_window_mins = primary.window_duration_mins;
        usage.primary_resets_at = primary.resets_at;
        if primary.used_percent >= 100 {
            usage.is_rate_limited = true;
        }
    }

    if let Some(sec) = &snapshot.secondary {
        usage.secondary_percent = Some(sec.used_percent);
        usage.secondary_window_mins = sec.window_duration_mins;
        usage.secondary_resets_at = sec.resets_at;
    }

    if let Some(cr) = &snapshot.credits {
        usage.credits_balance = cr.balance.clone();
    }

    usage.updated_at = current_time_str();
    usage
}

/// Format duration in minutes into a friendly window label
pub fn format_window_name(mins: i64) -> String {
    if mins <= 0 {
        return "Window".to_string();
    }
    if mins == 300 {
        return "5h Window".to_string();
    }
    if mins == 10080 {
        return "Weekly Limit".to_string();
    }
    if mins == 1440 {
        return "Daily Limit".to_string();
    }
    if mins % 1440 == 0 {
        return format!("{}d Window", mins / 1440);
    }
    if mins % 60 == 0 {
        return format!("{}h Window", mins / 60);
    }
    format!("{}m Window", mins)
}

/// Format unix timestamp into remaining time string like "2h 13m"
pub fn format_time_left(resets_at: i64) -> String {
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(_) => return "--".to_string(),
    };

    if resets_at <= now {
        return "Ready (0m)".to_string();
    }

    let diff = resets_at - now;
    let days = diff / 86400;
    let hours = (diff % 86400) / 3600;
    let mins = (diff % 3600) / 60;

    if days > 0 {
        format!("{}d {}h", days, hours)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins.max(1))
    }
}

/// Get current time formatted as HH:MM:SS
fn current_time_str() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs();
    // Simple UTC or local time approximation (macOS time)
    let sec_in_day = now % 86400;
    // Note: timezone offset approximation or simple format
    let h = (sec_in_day / 3600) % 24;
    let m = (sec_in_day % 3600) / 60;
    let s = sec_in_day % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_window_name() {
        assert_eq!(format_window_name(300), "5h Window");
        assert_eq!(format_window_name(10080), "Weekly Limit");
        assert_eq!(format_window_name(1440), "Daily Limit");
        assert_eq!(format_window_name(60), "1h Window");
        assert_eq!(format_window_name(45), "45m Window");
    }

    #[test]
    fn test_format_time_left() {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        let future_2h13m = now + 2 * 3600 + 13 * 60;
        assert_eq!(format_time_left(future_2h13m), "2h 13m");

        let future_45m = now + 45 * 60;
        assert_eq!(format_time_left(future_45m), "45m");

        let past = now - 10;
        assert_eq!(format_time_left(past), "Ready (0m)");
    }

    #[test]
    fn test_parse_rate_limits_json() {
        let sample = r#"{
            "jsonrpc": "2.0",
            "id": 2,
            "result": {
                "rateLimits": {
                    "primary": {
                        "usedPercent": 72,
                        "windowDurationMins": 300,
                        "resetsAt": 1757732000
                    },
                    "secondary": {
                        "usedPercent": 48,
                        "windowDurationMins": 10080,
                        "resetsAt": 1758200000
                    },
                    "planType": "pro",
                    "credits": {
                        "balance": "$15.00",
                        "hasCredits": true,
                        "unlimited": false
                    }
                }
            }
        }"#;

        let usage = parse_rate_limits_json(sample).expect("Parse sample json");
        assert_eq!(usage.primary_percent, Some(72));
        assert_eq!(usage.secondary_percent, Some(48));
        assert_eq!(usage.plan_type.as_deref(), Some("pro"));
        assert_eq!(usage.primary_window_label(), "5h Window");
        assert_eq!(usage.secondary_window_label(), "Weekly Limit");
        assert_eq!(usage.menu_bar_title(), "☁ Codex 72%");
    }
}
