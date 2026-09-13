use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

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
pub struct RateLimitResetCredit {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "expiresAt")]
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitResetCreditsSummary {
    #[serde(rename = "availableCount")]
    pub available_count: i64,
    pub credits: Option<Vec<RateLimitResetCredit>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountRateLimitsResponse {
    #[serde(rename = "rateLimits")]
    pub rate_limits: Option<RateLimitSnapshot>,
    #[serde(rename = "rateLimitResetCredits")]
    pub rate_limit_reset_credits: Option<RateLimitResetCreditsSummary>,
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
    pub reset_credits_count: Option<i64>,
    pub reset_credit_title: Option<String>,
    pub reset_credit_expires_at: Option<i64>,
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
            reset_credits_count: None,
            reset_credit_title: None,
            reset_credit_expires_at: None,
            updated_at: current_local_time_str(),
            is_rate_limited: false,
            raw_error: None,
        }
    }
}

impl CodexUsage {
    /// Calculate remaining percentage (e.g. used 59% -> remaining 41%)
    pub fn remaining_percent(&self) -> Option<i32> {
        self.primary_percent.map(|used| (100 - used).clamp(0, 100))
    }

    /// Friendly formatted Plan Name: prolite -> Pro (5x)
    pub fn display_plan_name(&self) -> String {
        format_plan_name(self.plan_type.as_deref())
    }

    /// Menu bar title (e.g. "☁ 剩余 41%")
    pub fn menu_bar_title(&self) -> String {
        if let Some(err) = &self.raw_error {
            if self.primary_percent.is_none() {
                if err.contains("Not logged in") || err.contains("未登录") {
                    return "☁ Codex (未登录)".to_string();
                }
                return "☁ Codex (!)".to_string();
            }
        }

        match self.remaining_percent() {
            Some(rem) => {
                if rem <= 5 {
                    format!("⚠️ 剩余 {}%", rem)
                } else {
                    format!("☁ 剩余 {}%", rem)
                }
            }
            None => match self.primary_percent {
                Some(pct) => format!("☁ Codex {}%", pct),
                None => "☁ Codex ...".to_string(),
            },
        }
    }

    /// Primary window label
    pub fn primary_window_label(&self) -> String {
        match self.primary_window_mins {
            Some(mins) => format_window_name(mins),
            None => "周使用限额".to_string(),
        }
    }

    /// Remaining time countdown
    pub fn reset_countdown_label(&self) -> String {
        match self.primary_resets_at {
            Some(ts) => format_time_left(ts),
            None => "--".to_string(),
        }
    }

    /// Format reset credit information (e.g. "可用 1 次 (完全重置 • 10/5 到期)")
    pub fn reset_credit_label(&self) -> Option<String> {
        let count = self.reset_credits_count?;
        if count <= 0 {
            return None;
        }

        let title = self.reset_credit_title.as_deref().unwrap_or("完全重置");
        if let Some(exp) = self.reset_credit_expires_at {
            let exp_str = format_date_short(exp);
            Some(format!("可用 {} 次 ({} • {} 到期)", count, title, exp_str))
        } else {
            Some(format!("可用 {} 次 ({})", count, title))
        }
    }
}

/// Map OpenAI internal plan enums to official marketing names
pub fn format_plan_name(raw_plan: Option<&str>) -> String {
    match raw_plan.map(|s| s.to_lowercase()).as_deref() {
        Some("prolite") | Some("self_serve_business_prolite") => "Pro (5x)".to_string(),
        Some("pro") => "Pro".to_string(),
        Some("plus") => "Plus".to_string(),
        Some("team") => "Team".to_string(),
        Some("enterprise") | Some("ent26") => "Enterprise".to_string(),
        Some("free") => "Free".to_string(),
        Some(other) => {
            let mut c = other.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        }
        None => "Pro (5x)".to_string(),
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

    let mut line_buf = String::new();
    let mut got_init = false;
    for _ in 0..10 {
        line_buf.clear();
        if reader.read_line(&mut line_buf).map_err(|e| e.to_string())? == 0 {
            break;
        }
        if line_buf.contains("\"id\":1") || line_buf.contains("\"id\": 1") {
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

    let mut rate_line = None;
    for _ in 0..15 {
        line_buf.clear();
        if reader.read_line(&mut line_buf).map_err(|e| e.to_string())? == 0 {
            break;
        }
        if line_buf.contains("\"id\":2") || line_buf.contains("\"id\": 2") {
            rate_line = Some(line_buf.clone());
            break;
        }
    }

    // Terminate child gracefully
    let _ = child.kill();

    let line = rate_line.ok_or("Timeout or no response for account/rateLimits/read")?;
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
    if let Ok(env) = serde_json::from_str::<RpcEnvelope>(raw_json) {
        if let Some(err) = env.error {
            return Err(format!("RPC error {}: {}", err.code, err.message));
        }
        if let Some(res) = env.result {
            return Ok(build_usage_from_response(&res));
        }
    }

    if let Ok(res) = serde_json::from_str::<GetAccountRateLimitsResponse>(raw_json) {
        return Ok(build_usage_from_response(&res));
    }

    if let Ok(snapshot) = serde_json::from_str::<RateLimitSnapshot>(raw_json) {
        let res = GetAccountRateLimitsResponse {
            rate_limits: Some(snapshot),
            rate_limit_reset_credits: None,
        };
        return Ok(build_usage_from_response(&res));
    }

    let val: serde_json::Value =
        serde_json::from_str(raw_json).map_err(|e| format!("JSON parse error: {}", e))?;

    let mut res = GetAccountRateLimitsResponse {
        rate_limits: None,
        rate_limit_reset_credits: None,
    };

    if let Some(rl_val) = val.get("rateLimits").or_else(|| {
        val.get("result").and_then(|r| r.get("rateLimits"))
    }) {
        res.rate_limits = serde_json::from_value::<RateLimitSnapshot>(rl_val.clone()).ok();
    }

    if let Some(rc_val) = val.get("rateLimitResetCredits").or_else(|| {
        val.get("result").and_then(|r| r.get("rateLimitResetCredits"))
    }) {
        res.rate_limit_reset_credits =
            serde_json::from_value::<RateLimitResetCreditsSummary>(rc_val.clone()).ok();
    }

    if res.rate_limits.is_some() || res.rate_limit_reset_credits.is_some() {
        return Ok(build_usage_from_response(&res));
    }

    Err("Could not extract rate limits from response".to_string())
}

fn build_usage_from_response(res: &GetAccountRateLimitsResponse) -> CodexUsage {
    let mut usage = CodexUsage::default();

    if let Some(snapshot) = &res.rate_limits {
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
            // Only populate secondary if it has a distinct window from primary
            if sec.window_duration_mins != usage.primary_window_mins {
                usage.secondary_percent = Some(sec.used_percent);
                usage.secondary_window_mins = sec.window_duration_mins;
                usage.secondary_resets_at = sec.resets_at;
            }
        }

        if let Some(cr) = &snapshot.credits {
            usage.credits_balance = cr.balance.clone();
        }
    }

    // Extract reset credits
    if let Some(rc) = &res.rate_limit_reset_credits {
        usage.reset_credits_count = Some(rc.available_count);
        if let Some(credits) = &rc.credits {
            if let Some(first) = credits.iter().find(|c| c.status.as_deref() == Some("available")) {
                usage.reset_credit_title = first.title.clone();
                usage.reset_credit_expires_at = first.expires_at;
            }
        }
    }

    usage.updated_at = current_local_time_str();
    usage
}

/// Format duration in minutes into a friendly window label
pub fn format_window_name(mins: i64) -> String {
    if mins <= 0 {
        return "使用窗口".to_string();
    }
    if mins == 300 {
        return "5h 滑动窗口".to_string();
    }
    if mins == 10080 {
        return "周使用限额".to_string();
    }
    if mins == 1440 {
        return "日使用限额".to_string();
    }
    if mins % 10080 == 0 {
        return format!("{}周限额", mins / 10080);
    }
    if mins % 1440 == 0 {
        return format!("{}天窗口", mins / 1440);
    }
    if mins % 60 == 0 {
        return format!("{}h 窗口", mins / 60);
    }
    format!("{}m 窗口", mins)
}

/// Format unix timestamp into remaining time string like "6d 6h" or "2h 13m"
pub fn format_time_left(resets_at: i64) -> String {
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(_) => return "--".to_string(),
    };

    if resets_at <= now {
        return "已重置".to_string();
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

/// Format date into short string like "10/5 06:16"
pub fn format_date_short(ts: i64) -> String {
    let t: libc::time_t = ts as libc::time_t;
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    unsafe { libc::localtime_r(&t, &mut tm) };
    format!("{}/{} {:02}:{:02}", tm.tm_mon + 1, tm.tm_mday, tm.tm_hour, tm.tm_min)
}

/// Get macOS local time formatted as HH:MM:SS using libc::localtime_r
pub fn current_local_time_str() -> String {
    let mut now: libc::time_t = 0;
    unsafe { libc::time(&mut now) };
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    unsafe { libc::localtime_r(&now, &mut tm) };
    format!("{:02}:{:02}:{:02}", tm.tm_hour, tm.tm_min, tm.tm_sec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_plan_name() {
        assert_eq!(format_plan_name(Some("prolite")), "Pro (5x)");
        assert_eq!(format_plan_name(Some("pro")), "Pro");
        assert_eq!(format_plan_name(Some("plus")), "Plus");
    }

    #[test]
    fn test_remaining_percent() {
        let mut usage = CodexUsage::default();
        usage.primary_percent = Some(59);
        assert_eq!(usage.remaining_percent(), Some(41));
        assert_eq!(usage.menu_bar_title(), "☁ 剩余 41%");
    }

    #[test]
    fn test_parse_reset_credits() {
        let sample = r#"{
            "jsonrpc": "2.0",
            "id": 2,
            "result": {
                "rateLimits": {
                    "primary": {
                        "usedPercent": 59,
                        "windowDurationMins": 10080,
                        "resetsAt": 1759644960
                    },
                    "planType": "prolite"
                },
                "rateLimitResetCredits": {
                    "availableCount": 1,
                    "credits": [
                        {
                            "id": "credit-1",
                            "title": "完全重置",
                            "status": "available",
                            "expiresAt": 1759644960
                        }
                    ]
                }
            }
        }"#;

        let usage = parse_rate_limits_json(sample).expect("Parse json with reset credits");
        assert_eq!(usage.display_plan_name(), "Pro (5x)");
        assert_eq!(usage.remaining_percent(), Some(41));
        assert_eq!(usage.reset_credits_count, Some(1));
        assert_eq!(usage.reset_credit_title.as_deref(), Some("完全重置"));
        assert!(usage.reset_credit_label().unwrap().contains("可用 1 次"));
    }
}
