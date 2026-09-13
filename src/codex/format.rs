use std::time::{SystemTime, UNIX_EPOCH};
use super::models::CodexUsage;

pub fn format_tokens(tokens: i64, include_en_suffix: bool) -> String {
    if tokens >= 100_000_000 {
        let yi = tokens as f64 / 100_000_000.0;
        if include_en_suffix {
            format!("{:.2} 亿 ({:.2}B)", yi, tokens as f64 / 1_000_000_000.0)
        } else {
            format!("{:.2} 亿", yi)
        }
    } else if tokens >= 10_000 {
        let wan = tokens as f64 / 10_000.0;
        if include_en_suffix {
            format!("{:.1} 万 ({:.2}M)", wan, tokens as f64 / 1_000_000.0)
        } else {
            format!("{:.1} 万", wan)
        }
    } else {
        tokens.to_string()
    }
}

#[allow(dead_code)]
pub fn format_tokens_human(tokens: i64) -> String {
    format_tokens(tokens, true)
}

pub fn format_tokens_compact(tokens: i64) -> String {
    format_tokens(tokens, false)
}

pub fn format_window_name(mins: i64) -> String {
    match mins {
        60 => "1小时窗口".to_string(),
        1440 => "24小时窗口".to_string(),
        10080 => "7天周窗口".to_string(),
        m => format!("{}分钟窗口", m),
    }
}

pub fn format_duration_hms(total_sec: i64) -> String {
    let hours = total_sec / 3600;
    let mins = (total_sec % 3600) / 60;
    let secs = total_sec % 60;
    if hours > 0 {
        format!("{}小时{}分", hours, mins)
    } else if mins > 0 {
        format!("{}分{}秒", mins, secs)
    } else {
        format!("{}秒", secs)
    }
}

pub fn format_relative_time(epoch_sec: i64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let diff = epoch_sec - now;
    if diff <= 0 {
        let past = -diff;
        if past < 60 { "刚刚".to_string() }
        else if past < 3600 { format!("{:.1}分钟前", past as f64 / 60.0) }
        else if past < 86400 { format!("{:.1}小时前", past as f64 / 3600.0) }
        else { format!("{:.1}天前", past as f64 / 86400.0) }
    } else {
        if diff < 60 { format!("{}秒后", diff) }
        else if diff < 3600 { format!("{:.1}分钟后", diff as f64 / 60.0) }
        else if diff < 86400 { format!("{:.1}小时后", diff as f64 / 3600.0) }
        else { format!("{:.1}天后", diff as f64 / 86400.0) }
    }
}

pub fn current_local_time_str() -> String {
    let mut now: libc::time_t = 0;
    unsafe { libc::time(&mut now) };
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    unsafe { libc::localtime_r(&now, &mut tm) };
    format!("{:02}:{:02}:{:02}", tm.tm_hour, tm.tm_min, tm.tm_sec)
}

impl Default for CodexUsage {
    fn default() -> Self {
        Self {
            account_email: None,
            user_display_name: None,
            user_handle: None,
            plan_type: None,
            primary_percent: None,
            primary_window_mins: None,
            primary_resets_at: None,
            secondary_percent: None,
            secondary_window_mins: None,
            secondary_resets_at: None,
            spark_model_name: None,
            spark_percent: None,
            spark_window_mins: None,
            credits_balance: None,
            reset_credits_count: None,
            reset_credit_title: None,
            reset_credit_expires_at: None,
            lifetime_tokens: None,
            peak_daily_tokens: None,
            longest_running_turn_sec: None,
            longest_streak_days: None,
            current_streak_days: None,
            today_tokens: None,
            week_tokens: None,
            hourly_tokens: None,
            hourly_quota_rate: None,
            daily_buckets: Vec::new(),
            top_models: Vec::new(),
            most_used_reasoning: None,
            total_threads: None,
            global_reset: None,
            reset_credits_list: Vec::new(),
            updated_at: current_local_time_str(),
            is_rate_limited: false,
            raw_error: None,
        }
    }
}

impl CodexUsage {
    pub fn display_plan_name(&self) -> String {
        match self.plan_type.as_deref() {
            Some("pro") | Some("plus") | Some("prolite") => "Pro (5x)".to_string(),
            Some("team") => "Team (10x)".to_string(),
            Some("enterprise") => "Enterprise (Unlimited)".to_string(),
            Some(other) => other.to_string(),
            None => "Pro (5x)".to_string(),
        }
    }

    pub fn menu_bar_title(&self) -> String {
        if let Some(err) = &self.raw_error {
            return format!("☁ Codex [{}]", err);
        }
        if let Some(used) = self.primary_percent {
            let rem = (100 - used).max(0);
            return format!("☁ Codex: {}% ({})", rem, self.display_plan_name());
        }
        "☁ Codex".to_string()
    }

    pub fn remaining_percent(&self) -> Option<i32> {
        self.primary_percent.map(|u| (100 - u).max(0))
    }

    pub fn format_lifetime_tokens_compact(&self) -> String {
        self.lifetime_tokens.map(format_tokens_compact).unwrap_or_else(|| "--".to_string())
    }

    pub fn format_peak_daily_tokens_compact(&self) -> String {
        self.peak_daily_tokens.map(format_tokens_compact).unwrap_or_else(|| "--".to_string())
    }

    pub fn format_turn_duration(&self) -> String {
        self.longest_running_turn_sec.map(format_duration_hms).unwrap_or_else(|| "--".to_string())
    }

    pub fn format_hourly_tokens(&self) -> String {
        self.hourly_tokens.map(format_tokens_compact).unwrap_or_else(|| "--".to_string())
    }

    pub fn format_week_tokens(&self) -> String {
        self.week_tokens.map(format_tokens_compact).unwrap_or_else(|| "--".to_string())
    }

    pub fn format_reset_credits(&self) -> Vec<String> {
        if self.reset_credits_list.is_empty() {
            vec!["暂无可用限额重置券".to_string()]
        } else {
            self.reset_credits_list
                .iter()
                .map(|c| {
                    let title = c.title.as_deref().unwrap_or("限额重置特权");
                    let status = c.status.as_deref().unwrap_or("可用");
                    format!("🎟️ {} · 状态: {}", title, status)
                })
                .collect()
        }
    }
}
