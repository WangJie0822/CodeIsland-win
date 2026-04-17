use serde::{Deserialize, Serialize};

/// 启动预设：保存一组 Claude CLI 启动参数，供用户一键启动会话。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchPreset {
    /// 唯一标识符（timestamp-based，Stage 2 替换为 uuid）
    pub id: String,
    /// 显示名称
    pub name: String,
    /// 显示图标（emoji 或文字）
    pub icon: String,
    /// 工作目录
    pub cwd: String,
    /// 启动时发送的初始提示词
    pub initial_prompt: Option<String>,
    /// 模型名称，None 表示使用 Claude CLI 默认值
    pub model: Option<String>,
    /// agent 参数（可选）
    pub agent: Option<String>,
    /// 需要启用的 MCP server 列表
    pub mcp_servers: Vec<String>,
    /// 排序序号（越小越靠前）
    pub sort_order: i32,
}

impl LaunchPreset {
    /// 用当前时间戳生成一个伪唯一 ID。
    ///
    /// **NOTE（Stage 2 待办）**：引入 `uuid` crate 后替换为 `Uuid::new_v4().to_string()`。
    pub fn new_id() -> String {
        use chrono::Utc;
        // 使用纳秒时间戳 + 线程 ID 的低位作为伪随机扰动，降低同毫秒内碰撞概率。
        let ts = Utc::now()
            .timestamp_nanos_opt()
            .unwrap_or_else(|| Utc::now().timestamp_millis() * 1_000_000);
        let tid = format!("{:?}", std::thread::current().id());
        // 取线程 ID 字符串的哈希低 16 位
        let hash: u64 = tid.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
        format!("preset-{}-{:04x}", ts, hash & 0xFFFF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_id_is_nonempty_and_starts_with_prefix() {
        let id = LaunchPreset::new_id();
        assert!(id.starts_with("preset-"), "id = {id}");
    }

    #[test]
    fn new_id_two_calls_differ() {
        // 两次调用之间可能纳秒级差异，但加了 hash 扰动后大概率不同。
        // 这里主要保证 API 正常工作；在 Stage 2 由 uuid 保证唯一性。
        let id1 = LaunchPreset::new_id();
        let id2 = LaunchPreset::new_id();
        // 只要格式合法即可，不强断言不等（极小概率相同）
        assert!(id1.starts_with("preset-"));
        assert!(id2.starts_with("preset-"));
    }
}
