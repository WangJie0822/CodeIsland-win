use std::path::PathBuf;
use crate::presets::types::LaunchPreset;

/// 返回 presets.json 的存储路径：`%APPDATA%\codeisland\presets.json`。
pub fn presets_file_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("codeisland").join("presets.json"))
}

/// 从磁盘读取 preset 列表；文件不存在时返回空列表。
pub fn load_presets() -> Result<Vec<LaunchPreset>, String> {
    let path = presets_file_path()
        .ok_or_else(|| "[preset_io] 无法确定 APPDATA 目录".to_string())?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("[preset_io] 读取 presets.json 失败: {e}"))?;

    serde_json::from_str::<Vec<LaunchPreset>>(&content)
        .map_err(|e| format!("[preset_io] 解析 presets.json 失败: {e}"))
}

/// 将 preset 列表写入磁盘，路径不存在时自动创建父目录。
pub fn save_presets(presets: &[LaunchPreset]) -> Result<(), String> {
    let path = presets_file_path()
        .ok_or_else(|| "[preset_io] 无法确定 APPDATA 目录".to_string())?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("[preset_io] 创建目录失败: {e}"))?;
    }

    let content = serde_json::to_string_pretty(presets)
        .map_err(|e| format!("[preset_io] 序列化 presets 失败: {e}"))?;

    std::fs::write(&path, content)
        .map_err(|e| format!("[preset_io] 写入 presets.json 失败: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn make_test_preset(id: &str) -> LaunchPreset {
        LaunchPreset {
            id: id.to_string(),
            name: format!("Preset {id}"),
            icon: "🚀".to_string(),
            cwd: "C:/projects/test".to_string(),
            initial_prompt: Some("hello".to_string()),
            model: Some("claude-sonnet-4-5".to_string()),
            agent: None,
            mcp_servers: vec!["server-a".to_string()],
            sort_order: 0,
        }
    }

    #[test]
    fn roundtrip_json_serialize_deserialize() {
        // 直接测试 serde 序列化/反序列化，不依赖文件系统路径
        let presets = vec![make_test_preset("p1"), make_test_preset("p2")];

        let json = serde_json::to_string_pretty(&presets).unwrap();
        let loaded: Vec<LaunchPreset> = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id, "p1");
        assert_eq!(loaded[1].id, "p2");
        assert_eq!(loaded[0].name, "Preset p1");
        assert_eq!(loaded[0].mcp_servers, vec!["server-a"]);
        assert_eq!(loaded[0].initial_prompt, Some("hello".to_string()));
    }

    #[test]
    fn roundtrip_file_write_read() {
        // 使用系统临时目录做文件级 roundtrip，不依赖 dirs::config_dir()
        let tmp_dir = env::temp_dir();
        let path = tmp_dir.join(format!("codeisland_test_presets_{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos()
        ));

        let presets = vec![make_test_preset("p1"), make_test_preset("p2")];
        let json = serde_json::to_string_pretty(&presets).unwrap();
        std::fs::write(&path, &json).unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        let loaded: Vec<LaunchPreset> = serde_json::from_str(&content).unwrap();

        // 清理临时文件
        let _ = std::fs::remove_file(&path);

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id, "p1");
    }

    #[test]
    fn empty_list_roundtrip() {
        let presets: Vec<LaunchPreset> = vec![];
        let json = serde_json::to_string_pretty(&presets).unwrap();
        let loaded: Vec<LaunchPreset> = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.len(), 0);
    }

    #[test]
    fn preset_with_all_optional_none() {
        let preset = LaunchPreset {
            id: "x".to_string(),
            name: "X".to_string(),
            icon: "".to_string(),
            cwd: "C:/".to_string(),
            initial_prompt: None,
            model: None,
            agent: None,
            mcp_servers: vec![],
            sort_order: 99,
        };
        let json = serde_json::to_string(&preset).unwrap();
        let back: LaunchPreset = serde_json::from_str(&json).unwrap();
        assert_eq!(back.initial_prompt, None);
        assert_eq!(back.model, None);
        assert_eq!(back.mcp_servers.len(), 0);
        assert_eq!(back.sort_order, 99);
    }
}
