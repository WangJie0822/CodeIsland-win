//! 使用 tauri-specta 导出前端类型的测试入口。
//! 将 TypeScript 类型写入 ../src/types/generated.ts。

#[cfg(test)]
mod tests {
    use crate::commands;
    use specta_typescript::Typescript;
    use std::path::PathBuf;
    use tauri_specta::{collect_commands, Builder};

    #[test]
    fn generate_typescript_bindings() {
        let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
            commands::session::get_sessions,
            commands::session::get_session_count,
            commands::session::send_to_terminal,
            commands::approval::approve_permission,
            commands::approval::deny_permission,
            commands::settings::get_sound_enabled,
            commands::settings::set_sound_enabled,
            commands::window_control::set_ignore_cursor_events,
            commands::window_control::set_window_size,
            commands::window_control::set_window_position,
        ]);

        let out = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("src")
            .join("types")
            .join("generated.ts");

        std::fs::create_dir_all(out.parent().unwrap()).unwrap();

        builder
            .export(Typescript::default(), &out)
            .expect("导出 TypeScript 类型失败");

        let content = std::fs::read_to_string(&out).unwrap();
        assert!(
            content.contains("export type SessionSummary"),
            "generated.ts 未包含 SessionSummary: {}",
            content
        );
        assert!(
            content.contains("approve_permission"),
            "generated.ts 未包含命令: {}",
            content
        );
        assert!(
            content.contains("set_ignore_cursor_events"),
            "generated.ts 未包含 set_ignore_cursor_events: {}",
            content
        );
    }
}
