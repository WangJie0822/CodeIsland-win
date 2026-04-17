use chrono::{DateTime, Duration, Utc};
use log::{debug, warn};
use std::path::Path;
use crate::usage::quota::{calc_percent, MAX_TOKENS_PER_5H, MAX_TOKENS_PER_7D};
use crate::usage::types::{UsageBucket, UsageReport};

/// 从单行 JSONL 提取 token 用量和时间戳
struct EntryTokens {
    tokens: u64,
    ts: DateTime<Utc>,
}

/// 解析一行 JSONL，提取 message.usage 字段中的 token 合计
fn parse_line(line: &str) -> Option<EntryTokens> {
    let json: serde_json::Value = serde_json::from_str(line.trim()).ok()?;

    // 只统计包含 message.usage 的行（assistant 消息）
    let usage = json.get("message")?.get("usage")?;

    let input = usage
        .get("input_tokens")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let output = usage
        .get("output_tokens")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let cache_creation = usage
        .get("cache_creation_input_tokens")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let cache_read = usage
        .get("cache_read_input_tokens")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let tokens = input + output + cache_creation + cache_read;

    let ts_str = json.get("timestamp")?.as_str()?;
    let ts: DateTime<Utc> = ts_str.parse().ok()?;

    Some(EntryTokens { tokens, ts })
}

/// 扫描单个 JSONL 文件，返回所有 (tokens, timestamp) 条目
fn scan_file(path: &Path) -> Vec<EntryTokens> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            debug!("[usage scanner] 读取文件失败 {:?}: {}", path, e);
            return vec![];
        }
    };

    content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(parse_line)
        .collect()
}

/// 扫描 `%USERPROFILE%/.claude/projects/**/*.jsonl`，
/// 按 5h/7d 滑动窗口累加 token 用量，生成 `UsageReport`。
///
/// `now` 参数允许测试注入特定时间点（生产时传 `Utc::now()`）。
pub fn scan_usage(now: DateTime<Utc>) -> UsageReport {
    let claude_dir = match dirs::home_dir() {
        Some(h) => h.join(".claude"),
        None => {
            warn!("[usage scanner] 无法获取 home 目录");
            return empty_report(now);
        }
    };

    let projects_dir = claude_dir.join("projects");
    if !projects_dir.exists() {
        debug!("[usage scanner] projects 目录不存在: {:?}", projects_dir);
        return empty_report(now);
    }

    let cutoff_5h = now - Duration::hours(5);
    let cutoff_7d = now - Duration::days(7);

    let mut tokens_5h: u64 = 0;
    let mut tokens_7d: u64 = 0;

    // 遍历 projects/*/  下的所有 *.jsonl 文件
    let project_dirs = match std::fs::read_dir(&projects_dir) {
        Ok(d) => d,
        Err(e) => {
            warn!("[usage scanner] 读取 projects 目录失败: {}", e);
            return empty_report(now);
        }
    };

    for proj_entry in project_dirs.flatten() {
        let proj_path = proj_entry.path();
        if !proj_path.is_dir() {
            continue;
        }

        let jsonl_files = match std::fs::read_dir(&proj_path) {
            Ok(d) => d,
            Err(_) => continue,
        };

        for file_entry in jsonl_files.flatten() {
            let file_path = file_entry.path();
            if file_path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
                continue;
            }

            for entry in scan_file(&file_path) {
                if entry.ts >= cutoff_7d {
                    tokens_7d += entry.tokens;
                }
                if entry.ts >= cutoff_5h {
                    tokens_5h += entry.tokens;
                }
            }
        }
    }

    // 5h 窗口重置时间：最早落在窗口内的那条记录加上 5h
    // 此处简化为当前时间加 5h（即下个完整窗口起点）
    let resets_5h = Some((now + Duration::hours(5)).timestamp());
    let resets_7d = Some((now + Duration::days(7)).timestamp());

    UsageReport {
        window_5h: UsageBucket {
            tokens: tokens_5h,
            quota: MAX_TOKENS_PER_5H,
            percent: calc_percent(tokens_5h, MAX_TOKENS_PER_5H),
            resets_at: resets_5h,
        },
        window_7d: UsageBucket {
            tokens: tokens_7d,
            quota: MAX_TOKENS_PER_7D,
            percent: calc_percent(tokens_7d, MAX_TOKENS_PER_7D),
            resets_at: resets_7d,
        },
        last_refresh_ts: now.timestamp(),
    }
}

/// 扫描指定目录（测试/自定义用）
pub fn scan_usage_from_dir(projects_dir: &Path, now: DateTime<Utc>) -> UsageReport {
    let cutoff_5h = now - Duration::hours(5);
    let cutoff_7d = now - Duration::days(7);

    let mut tokens_5h: u64 = 0;
    let mut tokens_7d: u64 = 0;

    if !projects_dir.exists() {
        return empty_report(now);
    }

    let project_dirs = match std::fs::read_dir(projects_dir) {
        Ok(d) => d,
        Err(_) => return empty_report(now),
    };

    for proj_entry in project_dirs.flatten() {
        let proj_path = proj_entry.path();
        if !proj_path.is_dir() {
            continue;
        }

        let jsonl_files = match std::fs::read_dir(&proj_path) {
            Ok(d) => d,
            Err(_) => continue,
        };

        for file_entry in jsonl_files.flatten() {
            let file_path = file_entry.path();
            if file_path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
                continue;
            }

            for entry in scan_file(&file_path) {
                if entry.ts >= cutoff_7d {
                    tokens_7d += entry.tokens;
                }
                if entry.ts >= cutoff_5h {
                    tokens_5h += entry.tokens;
                }
            }
        }
    }

    let resets_5h = Some((now + Duration::hours(5)).timestamp());
    let resets_7d = Some((now + Duration::days(7)).timestamp());

    UsageReport {
        window_5h: UsageBucket {
            tokens: tokens_5h,
            quota: MAX_TOKENS_PER_5H,
            percent: calc_percent(tokens_5h, MAX_TOKENS_PER_5H),
            resets_at: resets_5h,
        },
        window_7d: UsageBucket {
            tokens: tokens_7d,
            quota: MAX_TOKENS_PER_7D,
            percent: calc_percent(tokens_7d, MAX_TOKENS_PER_7D),
            resets_at: resets_7d,
        },
        last_refresh_ts: now.timestamp(),
    }
}

fn empty_report(now: DateTime<Utc>) -> UsageReport {
    UsageReport {
        window_5h: UsageBucket {
            tokens: 0,
            quota: MAX_TOKENS_PER_5H,
            percent: 0.0,
            resets_at: None,
        },
        window_7d: UsageBucket {
            tokens: 0,
            quota: MAX_TOKENS_PER_7D,
            percent: 0.0,
            resets_at: None,
        },
        last_refresh_ts: now.timestamp(),
    }
}

#[cfg(test)]
mod scanner_test {
    use super::*;

    fn fixture_dir() -> std::path::PathBuf {
        // tests/fixtures/projects 相对于 CARGO_MANIFEST_DIR
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("projects")
    }

    /// fixture 中的条目时间戳均为 2025-01-15T10:xx:xxZ，
    /// 用一个晚于所有条目且在 7d 窗口内的 now 触发全量统计
    fn fake_now() -> DateTime<Utc> {
        // 2025-01-15T12:00:00Z — 条目在 5h 窗口内（10:00~10:04 均在 07:00+ 范围）
        "2025-01-15T12:00:00Z".parse().unwrap()
    }

    #[test]
    fn test_fixture_total_tokens_5h() {
        let now = fake_now();
        let report = scan_usage_from_dir(&fixture_dir(), now);

        // fixture 三条 assistant 消息：
        // 1. input=100, output=50, cache_creation=0, cache_read=0   → 150
        // 2. input=200, output=150, cache_creation=30, cache_read=20 → 400
        // 3. input=50,  output=25,  cache_creation=0, cache_read=10  → 85
        // 合计 635，全部在 2h 内，远在 5h 窗口内
        assert_eq!(
            report.window_5h.tokens,
            635,
            "5h 窗口 token 统计错误，期望 635，实际 {}",
            report.window_5h.tokens
        );
    }

    #[test]
    fn test_fixture_total_tokens_7d() {
        let now = fake_now();
        let report = scan_usage_from_dir(&fixture_dir(), now);

        // 与 5h 相同，所有条目都在 7d 内
        assert_eq!(
            report.window_7d.tokens,
            635,
            "7d 窗口 token 统计错误，期望 635，实际 {}",
            report.window_7d.tokens
        );
    }

    #[test]
    fn test_5h_window_excludes_old_entries() {
        // now = 2025-01-15T16:00:00Z，cutoff_5h = 11:00:00Z
        // fixture 条目全部在 10:00~10:04，均在窗口外
        let now: DateTime<Utc> = "2025-01-15T16:00:00Z".parse().unwrap();
        let report = scan_usage_from_dir(&fixture_dir(), now);

        assert_eq!(
            report.window_5h.tokens, 0,
            "5h 窗口应为 0（条目已过期），实际 {}",
            report.window_5h.tokens
        );
    }

    #[test]
    fn test_7d_still_includes_entries_after_5h_cutoff() {
        // now = 2025-01-15T16:00:00Z
        // 5h 窗口不含，7d 窗口仍含（条目距 now < 7d）
        let now: DateTime<Utc> = "2025-01-15T16:00:00Z".parse().unwrap();
        let report = scan_usage_from_dir(&fixture_dir(), now);

        assert_eq!(
            report.window_7d.tokens,
            635,
            "7d 窗口应仍含全部条目，实际 {}",
            report.window_7d.tokens
        );
    }

    #[test]
    fn test_empty_projects_dir_returns_zero() {
        let tmp = std::env::temp_dir().join("codeisland_test_empty_projects");
        std::fs::create_dir_all(&tmp).unwrap();
        let report = scan_usage_from_dir(&tmp, fake_now());
        assert_eq!(report.window_5h.tokens, 0);
        assert_eq!(report.window_7d.tokens, 0);
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn test_percent_within_range() {
        let now = fake_now();
        let report = scan_usage_from_dir(&fixture_dir(), now);
        // 635 远小于配额，percent 应在 (0, 1) 范围内
        assert!(report.window_5h.percent > 0.0);
        assert!(report.window_5h.percent < 1.0);
    }
}
