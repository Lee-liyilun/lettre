// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// 引入功能模块
mod db;
mod modules;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            // Mark 模块命令
            modules::mark::get_all_projects,
            modules::mark::add_project,
            modules::mark::update_project,
            modules::mark::delete_project,
            modules::mark::get_all_marks,
            modules::mark::add_mark,
            modules::mark::update_mark,
            modules::mark::delete_mark,
            modules::mark::get_weekly_overview
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {}! ", name)
}