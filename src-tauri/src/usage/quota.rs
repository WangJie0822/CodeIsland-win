/// 5 小时滑动窗口 token 配额（示例默认值，可通过应用设置调整）
pub const MAX_TOKENS_PER_5H: u64 = 200_000;

/// 7 天滑动窗口 token 配额（示例默认值，可通过应用设置调整）
///
/// 注意：此值为示例配额，不代表 Anthropic 官方限制。
/// Stage 2 主会话可通过设置界面让用户自定义。
pub const MAX_TOKENS_PER_7D: u64 = 5_000_000;

/// 根据已用 tokens 和配额计算百分比（超出时 > 1.0）
pub fn calc_percent(tokens: u64, quota: u64) -> f32 {
    if quota == 0 {
        return 0.0;
    }
    tokens as f32 / quota as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_usage() {
        assert_eq!(calc_percent(0, MAX_TOKENS_PER_5H), 0.0);
    }

    #[test]
    fn test_50_percent() {
        let pct = calc_percent(MAX_TOKENS_PER_5H / 2, MAX_TOKENS_PER_5H);
        assert!((pct - 0.5).abs() < 1e-6, "期望 0.5，实际 {pct}");
    }

    #[test]
    fn test_80_percent() {
        let tokens = (MAX_TOKENS_PER_5H as f64 * 0.8) as u64;
        let pct = calc_percent(tokens, MAX_TOKENS_PER_5H);
        assert!((pct - 0.8).abs() < 1e-4, "期望 0.8，实际 {pct}");
    }

    #[test]
    fn test_100_percent() {
        let pct = calc_percent(MAX_TOKENS_PER_5H, MAX_TOKENS_PER_5H);
        assert!((pct - 1.0).abs() < 1e-6, "期望 1.0，实际 {pct}");
    }

    #[test]
    fn test_over_100_percent() {
        let pct = calc_percent(MAX_TOKENS_PER_5H + 1, MAX_TOKENS_PER_5H);
        assert!(pct > 1.0, "超出配额时应 > 1.0，实际 {pct}");
    }

    #[test]
    fn test_zero_quota_returns_zero() {
        assert_eq!(calc_percent(1000, 0), 0.0);
    }
}
