use serde::{Deserialize, Serialize};

/// 角色稀有度
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    /// 解锁所需累计完成会话数
    pub fn unlock_threshold(&self) -> u32 {
        match self {
            Rarity::Common => 1,
            Rarity::Rare => 10,
            Rarity::Epic => 50,
            Rarity::Legendary => 100,
        }
    }
}

/// Buddy 属性（0–100）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuddyStats {
    pub debug: u8,
    pub patience: u8,
    pub chaos: u8,
    pub wisdom: u8,
    pub sneak: u8,
}

/// 单个 Buddy 角色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buddy {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub rarity: Rarity,
    /// ASCII 帧数组（多帧时动画切换）
    pub ascii_art: Vec<String>,
    pub description: String,
    pub stats: BuddyStats,
    /// 解锁时间戳（Unix 秒），None 表示尚未解锁
    pub unlocked_at: Option<i64>,
}

/// 全局 Buddy 状态（传给前端）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuddyState {
    /// 当前选中角色 ID
    pub current: String,
    /// 所有角色（含未解锁），前端按 unlocked_at 区分
    pub roster: Vec<Buddy>,
    /// 已完成会话累计数
    pub completed_sessions: u32,
}

/// 持久化文件格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuddyPersist {
    pub current: String,
    /// 已解锁角色 ID 列表
    pub unlocked: Vec<String>,
    /// 各角色属性随机种子（保持属性稳定）
    pub stats_seeds: std::collections::HashMap<String, u64>,
    pub completed_sessions: u32,
}

impl Default for BuddyPersist {
    fn default() -> Self {
        Self {
            current: "kris".to_string(),
            unlocked: vec!["kris".to_string()],
            stats_seeds: std::collections::HashMap::new(),
            completed_sessions: 0,
        }
    }
}
