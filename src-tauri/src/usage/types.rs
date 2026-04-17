use serde::{Deserialize, Serialize};
use specta::Type;

/// 5h 或 7d 滑动窗口的 token 用量桶
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct UsageBucket {
    /// 窗口内累计 token 数（input + output + cache_creation + cache_read）
    pub tokens: u64,
    /// 配额上限
    pub quota: u64,
    /// 使用百分比（0.0 ~ 1.0+，超出配额时可能 > 1.0）
    pub percent: f32,
    /// 窗口重置时间（Unix 时间戳，秒）；None 表示未知
    pub resets_at: Option<i64>,
}

/// Daily Report：5h + 7d 两个滑动窗口的汇总
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct UsageReport {
    /// 过去 5 小时滑动窗口
    pub window_5h: UsageBucket,
    /// 过去 7 天滑动窗口
    pub window_7d: UsageBucket,
    /// 上次刷新时间（Unix 时间戳，秒）
    pub last_refresh_ts: i64,
}
