#![allow(unused)]

// 引入功能模块
mod db;
mod modules;

use tauri_plugin_updater;

// 导出给前端使用
pub use modules::cipher::*;
pub use modules::mark::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            // Mark 模块
            modules::mark::get_all_projects,
            modules::mark::add_project,
            modules::mark::update_project,
            modules::mark::delete_project,
            modules::mark::get_all_marks,
            modules::mark::add_mark,
            modules::mark::update_mark,
            modules::mark::delete_mark,
            modules::mark::get_weekly_overview,

            // Cipher 模块（你要的密语）
            modules::cipher::get_all_ciphers,
            modules::cipher::add_cipher,
            modules::cipher::update_cipher,
            modules::cipher::delete_cipher,
            modules::cipher::search_cipher,
            modules::cipher::import_key,
            modules::cipher::export_key,
            modules::cipher::generate_first_key,
            modules::cipher::check_key_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
