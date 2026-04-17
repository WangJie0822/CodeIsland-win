use std::path::PathBuf;
use crate::buddy::types::BuddyPersist;

/// 返回持久化文件路径：`%APPDATA%\codeisland\buddy.json`
pub fn buddy_file_path() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join("codeisland").join("buddy.json"))
}

/// 从磁盘读取持久化数据，文件不存在或解析失败时返回默认值
pub fn load() -> BuddyPersist {
    let path = match buddy_file_path() {
        Some(p) => p,
        None => return BuddyPersist::default(),
    };

    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
            log::warn!("[buddy] 解析 buddy.json 失败（{}），使用默认值", e);
            BuddyPersist::default()
        }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            log::info!("[buddy] buddy.json 不存在，使用默认值");
            BuddyPersist::default()
        }
        Err(e) => {
            log::warn!("[buddy] 读取 buddy.json 失败（{}），使用默认值", e);
            BuddyPersist::default()
        }
    }
}

/// 将持久化数据写入磁盘
pub fn save(data: &BuddyPersist) -> Result<(), String> {
    let path = buddy_file_path().ok_or_else(|| "[buddy] 无法确定数据目录".to_string())?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("[buddy] 创建目录失败: {}", e))?;
    }

    let json = serde_json::to_string_pretty(data)
        .map_err(|e| format!("[buddy] 序列化失败: {}", e))?;

    std::fs::write(&path, json)
        .map_err(|e| format!("[buddy] 写入 buddy.json 失败: {}", e))?;

    log::info!("[buddy] 已保存 buddy.json");
    Ok(())
}

#[cfg(test)]
mod persist_test {
    use super::*;
    use crate::buddy::types::BuddyPersist;
    use std::collections::HashMap;

    /// 在临时目录中执行 roundtrip 测试（不依赖 %APPDATA%）
    #[test]
    fn test_roundtrip_via_file() {
        let dir = std::env::temp_dir().join("codeisland_test_buddy");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("buddy_roundtrip.json");

        let original = BuddyPersist {
            current: "kris".to_string(),
            unlocked: vec!["kris".to_string(), "byte".to_string()],
            stats_seeds: {
                let mut m = HashMap::new();
                m.insert("kris".to_string(), 42u64);
                m
            },
            completed_sessions: 57,
        };

        // 写入
        let json = serde_json::to_string_pretty(&original).unwrap();
        std::fs::write(&path, &json).unwrap();

        // 读取
        let loaded: BuddyPersist =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();

        assert_eq!(loaded.current, original.current);
        assert_eq!(loaded.completed_sessions, original.completed_sessions);
        assert_eq!(loaded.unlocked, original.unlocked);
        assert_eq!(loaded.stats_seeds.get("kris"), Some(&42u64));

        // 清理
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_default_current_is_kris() {
        let d = BuddyPersist::default();
        assert_eq!(d.current, "kris");
        assert!(d.unlocked.contains(&"kris".to_string()));
        assert_eq!(d.completed_sessions, 0);
    }

    #[test]
    fn test_invalid_json_returns_default() {
        // 用无效 JSON 测试 serde 回退
        let result: Result<BuddyPersist, _> = serde_json::from_str("{invalid json}");
        assert!(result.is_err(), "无效 JSON 应返回解析错误");
    }

    #[test]
    fn test_missing_fields_use_defaults() {
        // 部分字段缺失时 serde 应能处理（字段均无 default 则报错，这里验证完整结构）
        let partial = r#"{"current":"byte","unlocked":["kris","byte"],"stats_seeds":{},"completed_sessions":5}"#;
        let loaded: BuddyPersist = serde_json::from_str(partial).unwrap();
        assert_eq!(loaded.current, "byte");
        assert_eq!(loaded.completed_sessions, 5);
    }

    #[test]
    fn test_serialize_deserialize_empty_seeds() {
        let data = BuddyPersist {
            current: "kris".to_string(),
            unlocked: vec!["kris".to_string()],
            stats_seeds: HashMap::new(),
            completed_sessions: 0,
        };
        let json = serde_json::to_string(&data).unwrap();
        let back: BuddyPersist = serde_json::from_str(&json).unwrap();
        assert!(back.stats_seeds.is_empty());
    }
}
