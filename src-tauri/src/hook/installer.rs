use std::path::PathBuf;
use std::fs;
use log::{info, warn};

pub fn claude_dir() -> PathBuf {
    dirs::home_dir()
        .expect("无法获取 home 目录")
        .join(".claude")
}

pub fn install_if_needed(script_content: &[u8]) {
    let claude = claude_dir();
    let hooks_dir = claude.join("hooks");
    let script_path = hooks_dir.join("codeisland-state.py");
    let settings_path = claude.join("settings.json");

    if let Err(e) = fs::create_dir_all(&hooks_dir) {
        warn!("[Installer] 创建 hooks 目录失败: {}", e);
        return;
    }

    if let Err(e) = fs::write(&script_path, script_content) {
        warn!("[Installer] 写入 hook 脚本失败: {}", e);
        return;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755));
    }

    info!("[Installer] Hook 脚本已安装: {:?}", script_path);
    update_settings(&settings_path);
}

fn update_settings(settings_path: &PathBuf) {
    let mut json: serde_json::Map<String, serde_json::Value> = if settings_path.exists() {
        let content = fs::read_to_string(settings_path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        serde_json::Map::new()
    };

    let python = detect_python();

    #[cfg(target_os = "windows")]
    let command = format!("{} %USERPROFILE%\\.claude\\hooks\\codeisland-state.py", python);
    #[cfg(not(target_os = "windows"))]
    let command = format!("{} ~/.claude/hooks/codeisland-state.py", python);

    let hook_entry = serde_json::json!([{"type": "command", "command": command}]);
    let hook_entry_timeout = serde_json::json!([{"type": "command", "command": command, "timeout": 86400}]);

    let with_matcher = |hooks: &serde_json::Value| serde_json::json!([{"matcher": "*", "hooks": hooks}]);
    let without_matcher = |hooks: &serde_json::Value| serde_json::json!([{"hooks": hooks}]);
    let precompact_config = serde_json::json!([
        {"matcher": "auto", "hooks": hook_entry},
        {"matcher": "manual", "hooks": hook_entry}
    ]);

    let hooks = json.entry("hooks")
        .or_insert_with(|| serde_json::json!({}))
        .as_object_mut()
        .expect("hooks 应是 object");

    let events: Vec<(&str, serde_json::Value)> = vec![
        ("UserPromptSubmit", without_matcher(&hook_entry)),
        ("PreToolUse", with_matcher(&hook_entry)),
        ("PostToolUse", with_matcher(&hook_entry)),
        ("PermissionRequest", with_matcher(&hook_entry_timeout)),
        ("Notification", with_matcher(&hook_entry)),
        ("Stop", without_matcher(&hook_entry)),
        ("SubagentStop", without_matcher(&hook_entry)),
        ("SessionStart", without_matcher(&hook_entry)),
        ("SessionEnd", without_matcher(&hook_entry)),
        ("PreCompact", precompact_config),
    ];

    for (event_name, config) in events {
        let has_our_hook = hooks.get(event_name)
            .and_then(|v| v.as_array())
            .map(|entries| {
                entries.iter().any(|entry| {
                    entry.get("hooks")
                        .and_then(|h| h.as_array())
                        .map(|hooks| hooks.iter().any(|h| {
                            h.get("command")
                                .and_then(|c| c.as_str())
                                .map(|c| c.contains("codeisland-state.py"))
                                .unwrap_or(false)
                        }))
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false);

        if !has_our_hook {
            if let Some(existing) = hooks.get_mut(event_name).and_then(|v| v.as_array_mut()) {
                if let Some(new_entries) = config.as_array() {
                    existing.extend(new_entries.clone());
                }
            } else {
                hooks.insert(event_name.to_string(), config);
            }
        }
    }

    let json_str = serde_json::to_string_pretty(&json).unwrap_or_default();
    if let Err(e) = fs::write(settings_path, json_str) {
        warn!("[Installer] 写入 settings.json 失败: {}", e);
    } else {
        info!("[Installer] settings.json 已更新");
    }
}

fn detect_python() -> String {
    #[cfg(target_os = "windows")]
    let candidates = ["python3", "python", "py"];
    #[cfg(not(target_os = "windows"))]
    let candidates = ["python3", "python"];

    for candidate in candidates {
        if std::process::Command::new(candidate)
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            return candidate.to_string();
        }
    }
    "python3".to_string()
}

pub fn uninstall() {
    let claude = claude_dir();
    let script = claude.join("hooks").join("codeisland-state.py");
    let settings_path = claude.join("settings.json");

    let _ = fs::remove_file(&script);

    if let Ok(content) = fs::read_to_string(&settings_path) {
        if let Ok(mut json) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&content) {
            if let Some(hooks) = json.get_mut("hooks").and_then(|v| v.as_object_mut()) {
                for (_, value) in hooks.iter_mut() {
                    if let Some(entries) = value.as_array_mut() {
                        entries.retain(|entry| {
                            !entry.get("hooks")
                                .and_then(|h| h.as_array())
                                .map(|hooks| hooks.iter().any(|h| {
                                    h.get("command")
                                        .and_then(|c| c.as_str())
                                        .map(|c| c.contains("codeisland-state.py"))
                                        .unwrap_or(false)
                                }))
                                .unwrap_or(false)
                        });
                    }
                }
                hooks.retain(|_, v| v.as_array().map(|a| !a.is_empty()).unwrap_or(true));
            }
            if let Ok(json_str) = serde_json::to_string_pretty(&json) {
                let _ = fs::write(&settings_path, json_str);
            }
        }
    }
    info!("[Installer] Hook 已卸载");
}
