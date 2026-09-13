use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use super::analytics::fetch_sqlite_analytics;
use super::models::{
    AccountTokenUsageDailyBucket, CodexUsage, DailyUsageItem, GetAccountRateLimitsResponse,
    GetAccountResponse, TokenUsageSummary,
};

#[derive(serde::Deserialize)]
struct GetAccountUsageResponse {
    #[serde(rename = "dailyUsageBuckets", alias = "dailyBuckets")]
    daily_buckets: Option<Vec<AccountTokenUsageDailyBucket>>,
    summary: Option<TokenUsageSummary>,
}

fn rpc_call(
    stdin: &mut std::process::ChildStdin,
    reader: &mut BufReader<std::process::ChildStdout>,
    id: i64,
    method: &str,
    params: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let req = serde_json::json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params });
    let msg = serde_json::to_string(&req).map_err(|e| e.to_string())?;
    writeln!(stdin, "{}", msg).map_err(|e| e.to_string())?;
    stdin.flush().map_err(|e| e.to_string())?;

    let mut line = String::new();
    for _ in 0..20 {
        line.clear();
        if reader.read_line(&mut line).map_err(|e| e.to_string())? == 0 { break; }
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&line) {
            if val.get("id").and_then(|v| v.as_i64()) == Some(id) {
                return Ok(val);
            }
        }
    }
    Err(format!("Timeout waiting for RPC response for {}", method))
}

pub fn populate_from_rate_limits(usage: &mut CodexUsage, res: &GetAccountRateLimitsResponse) {
    if let Some(snapshot) = &res.rate_limits {
        usage.plan_type = snapshot.plan_type.clone();
        if let Some(primary) = &snapshot.primary {
            usage.primary_percent = Some(primary.used_percent);
            usage.primary_window_mins = primary.window_duration_mins;
            usage.primary_resets_at = primary.resets_at;
            if primary.used_percent >= 100 { usage.is_rate_limited = true; }
        }
        if let Some(sec) = &snapshot.secondary {
            if sec.window_duration_mins != usage.primary_window_mins {
                usage.secondary_percent = Some(sec.used_percent);
                usage.secondary_window_mins = sec.window_duration_mins;
                usage.secondary_resets_at = sec.resets_at;
            }
        }
        if let Some(credits) = &snapshot.credits {
            usage.credits_balance = credits.balance.clone();
        }
    }
    if let Some(by_id) = &res.rate_limits_by_limit_id {
        for (limit_id, bucket) in by_id {
            if limit_id.contains("spark") || limit_id.contains("bengalfox") {
                usage.spark_model_name = bucket.limit_name.clone();
                if let Some(p) = &bucket.primary {
                    usage.spark_percent = Some(p.used_percent);
                    usage.spark_window_mins = p.window_duration_mins;
                }
            }
        }
    }
    if let Some(rc) = &res.rate_limit_reset_credits {
        usage.reset_credits_count = Some(rc.available_count);
        if let Some(credits) = &rc.credits {
            usage.reset_credits_list = credits.clone();
            if let Some(first) = credits.iter().find(|c| c.status.as_deref() == Some("available")) {
                usage.reset_credit_title = first.title.clone();
                usage.reset_credit_expires_at = first.expires_at;
            }
        }
    }
}

pub fn fetch_via_app_server(codex_bin: &std::path::Path) -> Result<CodexUsage, String> {
    let cur_path = std::env::var("PATH").unwrap_or_default();
    let cur_home = std::env::var("HOME").unwrap_or_default();
    let mut child = Command::new(codex_bin)
        .arg("app-server")
        .arg("--stdio")
        .env("PATH", cur_path)
        .env("HOME", cur_home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn codex app-server: {}", e))?;

    let mut stdin = child.stdin.take().ok_or("Failed to open stdin")?;
    let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
    let mut reader = BufReader::new(stdout);

    let _ = rpc_call(&mut stdin, &mut reader, 1, "initialize", serde_json::json!({"clientInfo":{"name":"codex-monitor","version":"0.1.0"}}))?;
    let notif = serde_json::json!({"jsonrpc":"2.0","method":"initialized","params":{}});
    let _ = writeln!(stdin, "{}", serde_json::to_string(&notif).unwrap_or_default());
    let _ = stdin.flush();

    let rate_resp = rpc_call(&mut stdin, &mut reader, 2, "account/rateLimits/read", serde_json::Value::Null).ok();
    let account_resp = rpc_call(&mut stdin, &mut reader, 3, "account/read", serde_json::json!({})).ok();
    let usage_resp = rpc_call(&mut stdin, &mut reader, 4, "account/usage/read", serde_json::json!({})).ok();
    let _ = child.kill();

    let mut usage = CodexUsage::default();
    if let Some(res_val) = rate_resp.as_ref().and_then(|r| r.get("result")) {
        if let Ok(rate_obj) = serde_json::from_value::<GetAccountRateLimitsResponse>(res_val.clone()) {
            populate_from_rate_limits(&mut usage, &rate_obj);
        }
    }
    if let Some(acc_val) = account_resp.and_then(|r| r.get("result").cloned()) {
        if let Ok(acc_obj) = serde_json::from_value::<GetAccountResponse>(acc_val) {
            if let Some(acc) = acc_obj.account {
                if let Some(ref email) = acc.email {
                    let display = email.split('@').next().unwrap_or(email).to_string();
                    usage.user_display_name = Some(display);
                }
                usage.account_email = acc.email;
                if usage.plan_type.is_none() { usage.plan_type = acc.plan_type; }
            }
        }
    }
    if let Some(usg_val) = usage_resp.and_then(|r| r.get("result").cloned()) {
        if let Ok(usg_obj) = serde_json::from_value::<GetAccountUsageResponse>(usg_val) {
            if let Some(summary) = usg_obj.summary {
                usage.lifetime_tokens = summary.lifetime_tokens;
                usage.peak_daily_tokens = summary.peak_daily_tokens;
                usage.longest_streak_days = summary.longest_streak_days;
                usage.current_streak_days = summary.current_streak_days;
                usage.longest_running_turn_sec = summary.longest_running_turn_sec;
            }
            if let Some(buckets) = usg_obj.daily_buckets {
                let mut mapped: Vec<DailyUsageItem> = buckets.into_iter().map(|b| DailyUsageItem { date: b.start_date, tokens: b.tokens }).collect();
                mapped.sort_by(|a, b| a.date.cmp(&b.date));
                if let Some(last) = mapped.last() {
                    usage.today_tokens = Some(last.tokens);
                    usage.hourly_tokens = Some(last.tokens / 24);
                }
                let week_sum: i64 = mapped.iter().rev().take(7).map(|b| b.tokens).sum();
                if week_sum > 0 { usage.week_tokens = Some(week_sum); }
                usage.daily_buckets = mapped;
            }
        }
    }

    let (top_models, sqlite_daily, reasoning, total_threads) = fetch_sqlite_analytics();
    if !top_models.is_empty() { usage.top_models = top_models; }
    if let Some(r) = reasoning { usage.most_used_reasoning = Some(r); }
    if total_threads > 0 { usage.total_threads = Some(total_threads); }
    if usage.daily_buckets.is_empty() && !sqlite_daily.is_empty() {
        usage.daily_buckets = sqlite_daily.clone();
        let week_sum: i64 = sqlite_daily.iter().rev().take(7).map(|b| b.tokens).sum();
        if usage.week_tokens.is_none() && week_sum > 0 { usage.week_tokens = Some(week_sum); }
    }
    if let Some(last_day) = usage.daily_buckets.last().or(sqlite_daily.last()) {
        if usage.today_tokens.is_none() {
            usage.today_tokens = Some(last_day.tokens);
            usage.hourly_tokens = Some(last_day.tokens / 24);
        }
    }
    usage.updated_at = super::format::current_local_time_str();
    Ok(usage)
}
