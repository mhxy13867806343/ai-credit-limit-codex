use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub struct ModelBucket {
    #[serde(rename = "limitId")]
    pub limit_id: Option<String>,
    #[serde(rename = "limitName")]
    pub limit_name: Option<String>,
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
    #[serde(rename = "grantedAt")]
    pub granted_at: Option<i64>,
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
    pub rate_limits: Option<ModelBucket>,
    #[serde(rename = "rateLimitsByLimitId")]
    pub rate_limits_by_limit_id: Option<HashMap<String, ModelBucket>>,
    #[serde(rename = "rateLimitResetCredits")]
    pub rate_limit_reset_credits: Option<RateLimitResetCreditsSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRecord {
    pub email: Option<String>,
    #[serde(rename = "planType")]
    pub plan_type: Option<String>,
    #[serde(rename = "type")]
    pub account_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountResponse {
    pub account: Option<AccountRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTokenUsageDailyBucket {
    #[serde(rename = "startDate")]
    pub start_date: String,
    pub tokens: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsageSummary {
    #[serde(rename = "lifetimeTokens")]
    pub lifetime_tokens: Option<i64>,
    #[serde(rename = "peakDailyTokens")]
    pub peak_daily_tokens: Option<i64>,
    #[serde(rename = "longestStreakDays")]
    pub longest_streak_days: Option<i64>,
    #[serde(rename = "currentStreakDays")]
    pub current_streak_days: Option<i64>,
    #[serde(rename = "longestRunningTurnSec")]
    pub longest_running_turn_sec: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenUsageResponse {
    #[serde(rename = "dailyUsageBuckets", alias = "dailyBuckets")]
    pub daily_buckets: Option<Vec<AccountTokenUsageDailyBucket>>,
    pub summary: Option<TokenUsageSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUsageItem {
    pub model_name: String,
    pub thread_count: i64,
    pub tokens_used: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyUsageItem {
    pub date: String,
    pub tokens: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalResetInfo {
    pub latest_text: Option<String>,
    pub author: Option<String>,
    pub announced_at: Option<String>,
    pub days_since_last: Option<f64>,
    pub avg_interval_days: Option<f64>,
    pub total_resets: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastCoupon {
    pub id: String,
    pub title: String,
    pub desc: String,
    pub status: String,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CodexUsage {
    pub account_email: Option<String>,
    pub user_display_name: Option<String>,
    pub user_handle: Option<String>,
    pub plan_type: Option<String>,

    pub primary_percent: Option<i32>,
    pub primary_window_mins: Option<i64>,
    pub primary_resets_at: Option<i64>,

    pub secondary_percent: Option<i32>,
    pub secondary_window_mins: Option<i64>,
    pub secondary_resets_at: Option<i64>,

    pub spark_model_name: Option<String>,
    pub spark_percent: Option<i32>,
    pub spark_window_mins: Option<i64>,

    pub credits_balance: Option<String>,
    pub reset_credits_count: Option<i64>,
    pub reset_credit_title: Option<String>,
    pub reset_credit_expires_at: Option<i64>,

    pub lifetime_tokens: Option<i64>,
    pub peak_daily_tokens: Option<i64>,
    pub longest_running_turn_sec: Option<i64>,
    pub longest_streak_days: Option<i64>,
    pub current_streak_days: Option<i64>,

    pub today_tokens: Option<i64>,
    pub week_tokens: Option<i64>,
    pub hourly_tokens: Option<i64>,
    pub hourly_quota_rate: Option<String>,

    pub daily_buckets: Vec<DailyUsageItem>,
    pub top_models: Vec<ModelUsageItem>,
    pub most_used_reasoning: Option<String>,
    pub total_threads: Option<i64>,
    pub global_reset: Option<GlobalResetInfo>,
    pub reset_credits_list: Vec<RateLimitResetCredit>,

    pub updated_at: String,
    pub is_rate_limited: bool,
    pub raw_error: Option<String>,
}


