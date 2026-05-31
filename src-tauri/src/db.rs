use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

// 获取数据库路径
pub fn get_db_path() -> PathBuf {
    let app_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let db_dir = app_dir.join("data").join("db");
    
    // 如果目录不存在，创建目录
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir).unwrap();
    }
    
    db_dir.join("lettre.db")
}

// 定义所有需要初始化的表名
const REQUIRED_TABLES: &[&str] = &["project", "mark"];

// 检查数据库是否需要初始化（所有必需的表是否都存在）
fn needs_initialization(conn: &Connection) -> bool {
    for table_name in REQUIRED_TABLES {
        let result: Result<i64, _> = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name=?1",
            [table_name],
            |row| row.get(0),
        );
        
        match result {
            Ok(count) => {
                if count == 0 {
                    // 只要有一个表不存在，就需要初始化
                    return true;
                }
            },
            Err(_) => return true,  // 查询失败也进行初始化
        }
    }
    
    false  // 所有表都存在，不需要初始化
}

// 初始化数据库并返回连接
pub fn init_db() -> Connection {
    let db_path = get_db_path();
    let db_exists = db_path.exists();
    
    let conn = Connection::open(&db_path).unwrap();
    
    // 只有当数据库文件不存在或表不存在时才初始化
    if !db_exists || needs_initialization(&conn) {
        initialize_tables(&conn);
    }
    
    conn
}

// 执行建表操作
fn initialize_tables(conn: &Connection) {
    // 创建 project 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS project (
            project_id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            create_time TEXT DEFAULT (datetime('now', 'localtime')),
            update_time TEXT DEFAULT (datetime('now', 'localtime'))
        )",
        [],
    ).unwrap();
    
    // 创建 mark 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mark (
            mark_id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            category_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            create_time TEXT DEFAULT (datetime('now', 'localtime')),
            update_time TEXT DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (project_id) REFERENCES project(project_id)
        )",
        [],
    ).unwrap();
}
