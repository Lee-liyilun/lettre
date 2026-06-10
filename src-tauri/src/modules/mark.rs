use crate::db;
use chrono::{Datelike, Duration, NaiveDate};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// 首先导入 log crate 的宏
use tauri_plugin_log::log::{info, warn, error, debug, trace};

// 项目结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub project_id: Option<i64>,
    pub name: String,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

// 印记结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Mark {
    pub mark_id: Option<i64>,
    pub project_id: i64,
    pub category_id: i32,
    pub content: String,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

// 最底层的分类对象
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryGroup {
    pub category_id: i32,
    pub contents: Vec<String>, // 存放该分类下的所有印记内容
}

// 中间层的项目对象
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectGroup {
    pub project_id: i64,
    pub project_name: String,
    pub categories: Vec<CategoryGroup>,
}

// 最外层的周对象
#[derive(Debug, Serialize, Deserialize)]
pub struct WeeklyGroup {
    pub week_start: String, // 例如 "2026-05-31"
    pub week_end: String,   // 例如 "2026-06-06"
    pub projects: Vec<ProjectGroup>,
}

// ========== 项目相关命令 ==========

// 获取所有项目
#[tauri::command]
pub fn get_all_projects() -> Vec<Project> {
    info!("调用-get_all_projects");
    let conn = db::init_db();
    let mut stmt = conn.prepare("SELECT project_id, name, create_time, update_time FROM project ORDER BY create_time DESC").unwrap();

    let projects = stmt
        .query_map([], |row| {
            Ok(Project {
                project_id: row.get(0)?,
                name: row.get(1)?,
                create_time: row.get(2)?,
                update_time: row.get(3)?,
            })
        })
        .unwrap();

    projects.map(|p| p.unwrap()).collect()
}

// 添加项目
#[tauri::command]
pub fn add_project(name: String) -> Result<String, String> {
    info!(r#"调用-add_project-[name]: {}"#, name);
    let conn = db::init_db();
    match conn.execute("INSERT INTO project (name) VALUES (?1)", [&name]) {
        Ok(_) => Ok("添加成功".to_string()),
        Err(e) => Err(format!("添加失败: {}", e)),
    }
}

// 更新项目
#[tauri::command]
pub fn update_project(project_id: i64, name: String) -> Result<String, String> {
    info!("调用-update_project");
    debug!(r#"调用-update_project-[SQL-UPDATE]: UPDATE project SET name = '{}', update_time = datetime('now', 'localtime') WHERE project_id = '{}'"#, name, project_id);
    let conn = db::init_db();
    match conn.execute(
        "UPDATE project SET name = ?1, update_time = datetime('now', 'localtime') WHERE project_id = ?2",
        [&name, &project_id.to_string()],
    ) {
        Ok(_) => Ok("更新成功".to_string()),
        Err(e) => Err(format!("更新失败: {}", e)),
    }
}

// 删除项目
#[tauri::command]
pub fn delete_project(project_id: i64) -> Result<String, String> {
    info!(r#"调用-delete_project-[project_id]: {}"#, project_id);
    let conn = db::init_db();
    match conn.execute(
        "DELETE FROM project WHERE project_id = ?1",
        [&project_id.to_string()],
    ) {
        Ok(_) => Ok("删除成功".to_string()),
        Err(e) => Err(format!("删除失败: {}", e)),
    }
}

// ========== 印记相关命令 ==========

// 获取所有印记（按创建时间降序）
#[tauri::command]
pub fn get_all_marks() -> Vec<Mark> {
    info!("调用-get_all_marks");
    let conn = db::init_db();
    let mut stmt = conn.prepare("SELECT mark_id, project_id, category_id, content, create_time, update_time FROM mark WHERE create_time >= date('now', '-14 days') ORDER BY create_time DESC").unwrap();

    let marks = stmt
        .query_map([], |row| {
            Ok(Mark {
                mark_id: row.get(0)?,
                project_id: row.get(1)?,
                category_id: row.get(2)?,
                content: row.get(3)?,
                create_time: row.get(4)?,
                update_time: row.get(5)?,
            })
        })
        .unwrap();

    marks.map(|m| m.unwrap()).collect()
}

// 添加印记
#[tauri::command]
pub fn add_mark(mark: Mark) -> Result<String, String> {
    info!("调用-add_mark");
    debug!(r#"调用-add_mark-[SQL-INSERT]: INSERT INTO mark (project_id, category_id, content) VALUES ('{}', '{}', '... ...')"#, mark.project_id, mark.category_id);
    let conn = db::init_db();
    match conn.execute(
        "INSERT INTO mark (project_id, category_id, content) VALUES (?1, ?2, ?3)",
        params![mark.project_id, mark.category_id, mark.content],
    ) {
        Ok(_) => Ok("添加成功".to_string()),
        Err(e) => Err(format!("添加失败: {}", e)),
    }
}

// 更新印记
#[tauri::command]
pub fn update_mark(mark: Mark) -> Result<String, String> {
    info!("调用-update_mark");
    debug!(r#"调用-update_mark-[SQL-UPDATE]: UPDATE mark SET project_id = '{}', category_id = '{}', content = '... ...', update_time = datetime('now', 'localtime') WHERE mark_id = {:?}"#, mark.project_id, mark.category_id, mark.mark_id);
    let conn = db::init_db();
    match conn.execute(
        "UPDATE mark SET project_id = ?1, category_id = ?2, content = ?3, update_time = datetime('now', 'localtime') WHERE mark_id = ?4",
        params![mark.project_id, mark.category_id, mark.content, mark.mark_id],
    ) {
        Ok(_) => Ok("更新成功".to_string()),
        Err(e) => Err(format!("更新失败: {}", e)),
    }
}

// 删除印记
#[tauri::command]
pub fn delete_mark(mark_id: i64) -> Result<String, String> {
    info!(r#"调用-delete_mark-[mark_id]: {}"#, mark_id);
    let conn = db::init_db();
    match conn.execute(
        "DELETE FROM mark WHERE mark_id = ?1",
        [&mark_id.to_string()],
    ) {
        Ok(_) => Ok("删除成功".to_string()),
        Err(e) => Err(format!("删除失败: {}", e)),
    }
}

// 获取每周总览数据
#[tauri::command]
pub fn get_weekly_overview() -> Result<Vec<WeeklyGroup>, String> {
    info!("调用-get_weekly_overview");
    let conn = db::init_db();

    // 1. 查询最近3个月的所有印记及关联信息
    let mut stmt = conn
        .prepare(
            "SELECT 
            m.create_time,
            p.project_id,
            p.name,
            m.category_id,
            m.content
         FROM mark m
         JOIN project p ON m.project_id = p.project_id
         WHERE m.create_time >= date('now', '-3 months')
         ORDER BY m.create_time DESC",
        )
        .map_err(|e| e.to_string())?;

    let marks = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?, // create_time
                row.get::<_, i64>(1)?,    // project_id
                row.get::<_, String>(2)?, // project_name
                row.get::<_, i32>(3)?,    // category_id
                row.get::<_, String>(4)?, // content
            ))
        })
        .map_err(|e| e.to_string())?;

    // 2. 使用 HashMap 进行三级分组：周 -> 项目 -> 分类
    // 外层 Key 是周起始日期字符串
    let mut weekly_map: HashMap<String, HashMap<i64, HashMap<i32, (String, Vec<String>)>>> =
        HashMap::new();

    for mark in marks {
        let (create_time, project_id, project_name, category_id, content) =
            mark.map_err(|e| e.to_string())?;

        // 计算当前日期属于哪一周（以周日为起点）
        // 假设 create_time 格式为 "YYYY-MM-DD HH:MM:SS"
        let date_part = create_time.split(' ').next().unwrap_or(&create_time);
        let week_start = get_week_start(date_part);

        // 逐级获取或插入 HashMap
        let project_map = weekly_map
            .entry(week_start.clone())
            .or_insert_with(HashMap::new);
        let category_map = project_map.entry(project_id).or_insert_with(HashMap::new);
        let (_, contents) = category_map
            .entry(category_id)
            .or_insert_with(|| (project_name, Vec::new()));

        contents.push(content);
    }

    // 3. 将 HashMap 转换为有序的 Vec，并满足你的排序要求
    let mut result: Vec<WeeklyGroup> = weekly_map
        .iter()
        .map(|(week_start, project_map)| {
            let week_end = get_week_end(week_start);

            let projects: Vec<ProjectGroup> = project_map
                .iter()
                .map(|(project_id, category_map)| {
                    // 拿到该项目下任意一个分类里的 project_name（因为同一个项目名是一样的）
                    let first_category = category_map.values().next().unwrap();
                    let project_name = first_category.0.clone();

                    let mut categories: Vec<CategoryGroup> = category_map
                        .iter()
                        .map(|(category_id, (_, contents))| CategoryGroup {
                            category_id: *category_id,
                            contents: contents.clone(),
                        })
                        .collect();

                    // 分类按照 category_id 升序排序
                    categories.sort_by_key(|c| c.category_id);

                    ProjectGroup {
                        project_id: *project_id,
                        project_name,
                        categories,
                    }
                })
                .collect();

            WeeklyGroup {
                week_start: week_start.clone(),
                week_end,
                projects,
            }
        })
        .collect();

    // 周对象按照降序排序
    result.sort_by(|a, b| b.week_start.cmp(&a.week_start));

    Ok(result)
}
// 辅助函数：获取周起始日期
fn get_week_start(date_str: &str) -> String {
    let dt = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    let days_to_subtract = dt.weekday().num_days_from_sunday() as i64;
    let start = dt - Duration::days(days_to_subtract);
    start.format("%Y-%m-%d").to_string()
}
// 辅助函数：获取周结束日期
fn get_week_end(start_str: &str) -> String {
    let start = NaiveDate::parse_from_str(start_str, "%Y-%m-%d").unwrap();
    let end = start + Duration::days(6);
    end.format("%Y-%m-%d").to_string()
}
