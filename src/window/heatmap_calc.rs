use std::collections::HashMap;

use objc2::rc::Retained;
use objc2_app_kit::{NSColor, NSTextField};
use objc2_foundation::NSString;

use crate::codex::{self, DailyUsageItem};
use super::types::HeatmapMode;
use super::ui_helpers::cell_date_dynamic;

pub fn render_heatmap_grid(
    mode: HeatmapMode,
    buckets: &[DailyUsageItem],
    tiles: &[Retained<NSTextField>],
    leg_tiles: &[Retained<NSTextField>],
    hm_total_metric: &Retained<NSTextField>,
    cached_lifetime: i64,
) {
    if buckets.is_empty() {
        return;
    }

    let mut date_map: HashMap<String, i64> = HashMap::new();
    for b in buckets.iter() {
        date_map.insert(b.date.clone(), b.tokens);
    }

    let hm_cols = 24;
    let hm_rows = 7;
    let mut cell_tokens = Vec::with_capacity(hm_cols * hm_rows);
    let mut cell_dates = Vec::with_capacity(hm_cols * hm_rows);

    for col in 0..hm_cols {
        for row in 0..hm_rows {
            let (y, m, d) = cell_date_dynamic(col, row, hm_cols);
            let date_str = format!("{:04}-{:02}-{:02}", y, m, d);
            let tokens = date_map.get(&date_str).copied().unwrap_or(0);
            cell_dates.push(date_str);
            cell_tokens.push(tokens);
        }
    }

    let values: Vec<i64> = match mode {
        HeatmapMode::Daily => cell_tokens.clone(),
        HeatmapMode::Weekly => {
            let mut weekly_vals = Vec::with_capacity(cell_tokens.len());
            for col in 0..hm_cols {
                let start = col * 7;
                let sum: i64 = cell_tokens[start..start + 7].iter().sum();
                for _ in 0..7 {
                    weekly_vals.push(sum);
                }
            }
            weekly_vals
        }
        HeatmapMode::Cumulative => {
            let mut run_sum = 0i64;
            let mut cumul_vals = Vec::with_capacity(cell_tokens.len());
            for &t in &cell_tokens {
                run_sum += t;
                cumul_vals.push(run_sum);
            }
            cumul_vals
        }
        HeatmapMode::All => cell_tokens.clone(),
    };

    let non_zero: Vec<i64> = values.iter().copied().filter(|&v| v > 0).collect();
    let max_val = non_zero.iter().copied().max().unwrap_or(1);

    let colors: [Retained<NSColor>; 5] = match mode {
        HeatmapMode::Daily => [
            NSColor::colorWithSRGBRed_green_blue_alpha(0.14, 0.15, 0.18, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.10, 0.28, 0.55, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.13, 0.42, 0.85, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.16, 0.55, 1.0, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.45, 0.78, 1.0, 1.0),
        ],
        HeatmapMode::Weekly => [
            NSColor::colorWithSRGBRed_green_blue_alpha(0.14, 0.15, 0.18, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.45, 0.12, 0.15, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.68, 0.16, 0.20, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.88, 0.24, 0.28, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(1.00, 0.50, 0.50, 1.0),
        ],
        HeatmapMode::Cumulative => [
            NSColor::colorWithSRGBRed_green_blue_alpha(0.14, 0.15, 0.18, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.08, 0.32, 0.20, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.10, 0.50, 0.32, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.15, 0.72, 0.45, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.45, 0.95, 0.65, 1.0),
        ],
        HeatmapMode::All => [
            NSColor::colorWithSRGBRed_green_blue_alpha(0.14, 0.15, 0.18, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.30, 0.14, 0.52, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.48, 0.20, 0.78, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.65, 0.30, 0.95, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.85, 0.60, 1.00, 1.0),
        ],
    };

    for (idx, tile) in leg_tiles.iter().enumerate() {
        if idx < colors.len() {
            tile.setBackgroundColor(Some(&colors[idx]));
        }
    }

    let (_, start_m, _) = cell_date_dynamic(0, 0, hm_cols);
    let range_label = format!("{}月至今", start_m);

    for (i, tile) in tiles.iter().enumerate() {
        if i >= values.len() {
            break;
        }
        let val = values[i];
        let date_str = &cell_dates[i];
        let color_idx = if val == 0 {
            0
        } else {
            let ratio = val as f64 / max_val as f64;
            if ratio < 0.25 {
                1
            } else if ratio < 0.50 {
                2
            } else if ratio < 0.75 {
                3
            } else {
                4
            }
        };
        tile.setBackgroundColor(Some(&colors[color_idx]));

        let tip = match mode {
            HeatmapMode::Daily => {
                if val > 0 {
                    format!("{}: {} Tokens ({})", date_str, codex::format_tokens_compact(val), val)
                } else {
                    format!("{}: 0 Tokens (无活动)", date_str)
                }
            }
            HeatmapMode::Weekly => {
                let col = i / 7;
                format!("第 {} 周 ({} 所在周): 周总量 {} Tokens", col + 1, date_str, codex::format_tokens_compact(val))
            }
            HeatmapMode::Cumulative => {
                format!("截至 {}: {}累计用量 {} Tokens", date_str, range_label, codex::format_tokens_compact(val))
            }
            HeatmapMode::All => {
                format!("{}: 当日用量 {} · 累计进度 (历史总计: {})", date_str, codex::format_tokens_compact(cell_tokens[i]), codex::format_tokens_compact(cached_lifetime))
            }
        };
        tile.setToolTip(Some(&NSString::from_str(&tip)));
    }

    let total_apr_tokens: i64 = cell_tokens.iter().sum();
    let active_days = cell_tokens.iter().filter(|&&t| t > 0).count();
    let avg_tokens = if active_days > 0 { total_apr_tokens / active_days as i64 } else { 0 };

    let summary_line = match mode {
        HeatmapMode::Daily => format!(
            "📊 {}消耗总量: {} ({} Tokens) · 活跃天数: {}天 · 日均消耗: {}",
            range_label,
            codex::format_tokens_compact(total_apr_tokens),
            total_apr_tokens,
            active_days,
            codex::format_tokens_compact(avg_tokens)
        ),
        HeatmapMode::Weekly => format!(
            "📅 {} {} 周周配额分布 · 最高周用量: {} · 累计周数: {}周",
            range_label,
            hm_cols,
            codex::format_tokens_compact(max_val),
            hm_cols
        ),
        HeatmapMode::Cumulative => format!(
            "📈 {}累计总额: {} Tokens (历史总计: {} · {} Tokens)",
            range_label,
            codex::format_tokens_compact(total_apr_tokens),
            codex::format_tokens_compact(cached_lifetime),
            cached_lifetime
        ),
        HeatmapMode::All => format!(
            "🌌 {}全景总量: {} ({} Tokens) · 历史总用量: {} ({} Tokens)",
            range_label,
            codex::format_tokens_compact(total_apr_tokens),
            total_apr_tokens,
            codex::format_tokens_compact(cached_lifetime),
            cached_lifetime
        ),
    };
    hm_total_metric.setStringValue(&NSString::from_str(&summary_line));
}
