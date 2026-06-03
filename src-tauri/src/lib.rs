#![allow(unused)]

// 引入模块
mod db;
mod modules;

// 日志 + Tauri 必备
use tauri::Manager;
use chrono::Local; // 本地日期
use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy, RotationStrategy};
use tauri_plugin_log::log::{LevelFilter};

// 导出给前端
pub use modules::cipher::*;
pub use modules::mark::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // ======================================
    // 你的应用目录（和数据库完全一致）
    // ======================================
    let home = dirs::data_dir().unwrap();
    let app_data = home.join("lettre");
    let log_dir = app_data.join("logs");
    std::fs::create_dir_all(&log_dir).unwrap();

    // 当日日期：2026-06-03
    let date_str = Local::now().format("%Y-%m-%d").to_string();
    // 完整文件名前缀=lettre_2026-06-03，插件自动补 .log
    let log_name = format!("lettre_{}", date_str);
 
    tauri::Builder::default()
        // 日志插件（前后端统一日志）
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(Target::new(TargetKind::Stdout)) // 控制台输出
                // ✅ Tauri2合法唯一写法：Folder指定目录+当日日期文件名
                .target(Target::new(TargetKind::Folder{
                    path: log_dir,
                    file_name: Some(log_name),
                }))
                .level(LevelFilter::Info)
                // 单文件超10MB自动同日期轮转：lettre_2026-06-03.1.log
                .max_file_size(10*1024*1024)
                .rotation_strategy(RotationStrategy::KeepAll)
                .timezone_strategy(TimezoneStrategy::UseLocal)
                .build(),
        )
        // 插件
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        // 命令注册
        .invoke_handler(tauri::generate_handler![
            // Mark
            modules::mark::get_all_projects,
            modules::mark::add_project,
            modules::mark::update_project,
            modules::mark::delete_project,
            modules::mark::get_all_marks,
            modules::mark::add_mark,
            modules::mark::update_mark,
            modules::mark::delete_mark,
            modules::mark::get_weekly_overview,

            // Cipher
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

        // 运行
        .run(tauri::generate_context!())
        .expect("应用启动失败");
}