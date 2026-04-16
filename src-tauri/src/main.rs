#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hook;
mod session;
mod terminal;
mod sound;

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("启动 Code Island 失败");
}
