use crate::db;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use rand;

// ==============================
// Cipher 结构体
// ==============================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cipher {
    pub id: Option<i64>,
    pub name: String,
    pub company: Option<String>,
    pub content: String,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

// ==============================
// 密钥路径
// ==============================
fn get_secret_key_path() -> PathBuf {
    let home = dirs::data_dir().unwrap();
    let app_data = home.join("lettre");
    let cipher_dir = app_data.join("cipher");

    if !cipher_dir.exists() {
        fs::create_dir_all(&cipher_dir).expect("创建 cipher 目录失败");
    }

    cipher_dir.join("secret.key")
}

// ==============================
// 安全获取密钥（不会自动生成）
// ==============================
fn get_user_secret_key() -> Vec<u8> {
    let key_path = get_secret_key_path();

    if !key_path.exists() {
        panic!("未找到密钥文件，请先备份或导入密钥！");
    }

    fs::read(&key_path).expect("读取密钥失败")
}

// ==============================
// 首次生成密钥
// ==============================
#[tauri::command]
pub fn generate_first_key() -> Result<String, String> {
    let key_path = get_secret_key_path();

    if key_path.exists() {
        return Err("密钥已存在，不可重复生成".into());
    }

    let key: Vec<u8> = rand::random::<[u8; 32]>().to_vec();
    fs::write(&key_path, &key).map_err(|e| e.to_string())?;

    Ok("密钥生成成功，请立即备份！".into())
}

// ==============================
// 导入密钥（用于换电脑恢复）
// ==============================
#[tauri::command]
pub fn import_key(key_data: Vec<u8>) -> Result<String, String> {
    let key_path = get_secret_key_path();
    fs::write(&key_path, &key_data).map_err(|e| e.to_string())?;
    Ok("密钥导入成功".into())
}

// ==============================
// 导出密钥（备份用）
// ==============================
#[tauri::command]
pub fn export_key() -> Result<Vec<u8>, String> {
    let key_path = get_secret_key_path();
    if !key_path.exists() {
        return Err("密钥不存在".into());
    }
    let data = fs::read(key_path).map_err(|e| e.to_string())?;
    Ok(data)
}

// ==============================
// 检查密钥是否存在
// ==============================
#[tauri::command]
pub fn check_key_exists() -> bool {
    get_secret_key_path().exists()
}

// ==============================
// XOR 加密
// ==============================
fn encrypt(plain: &str) -> Vec<u8> {
    let key = get_user_secret_key();
    plain
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect()
}

// ==============================
// XOR 解密
// ==============================
fn decrypt(cipher: &[u8]) -> String {
    let key = get_user_secret_key();
    let plain: Vec<u8> = cipher
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect();
    String::from_utf8_lossy(&plain).to_string()
}

// ==============================
// 业务逻辑
// ==============================
#[tauri::command]
pub fn get_all_ciphers(limit: i64) -> Vec<Cipher> {
    let conn = db::init_db();
    
    // 使用 LIMIT 限制返回条数
    let mut stmt = conn
        .prepare("SELECT id, name, company, content, create_time, update_time FROM cipher ORDER BY update_time DESC LIMIT ?")
        .unwrap();

    let ciphers = stmt.query_map([limit], |row| {
        let content_blob: Vec<u8> = row.get(3)?;
        Ok(Cipher {
            id: row.get(0)?,
            name: row.get(1)?,
            company: row.get(2)?,
            content: decrypt(&content_blob),
            create_time: row.get(4)?,
            update_time: row.get(5)?,
        })
    }).unwrap();

    ciphers.map(|c| c.unwrap()).collect()
}

#[tauri::command]
pub fn add_cipher(cipher: Cipher) -> Result<String, String> {
    let conn = db::init_db();
    let encrypted = encrypt(&cipher.content);

    match conn.execute(
        "INSERT INTO cipher (name, company, content) VALUES (?1, ?2, ?3)",
        params![cipher.name, cipher.company, encrypted],
    ) {
        Ok(_) => Ok("添加成功".to_string()),
        Err(e) => Err(format!("添加失败: {}", e)),
    }
}

#[tauri::command]
pub fn update_cipher(cipher: Cipher) -> Result<String, String> {
    let conn = db::init_db();
    let encrypted = encrypt(&cipher.content);

    match conn.execute(
        "UPDATE cipher SET name=?1, company=?2, content=?3, update_time=datetime('now','localtime') WHERE id=?4",
        params![
            cipher.name,
            cipher.company,
            encrypted,
            cipher.id.unwrap_or(0)
        ],
    ) {
        Ok(_) => Ok("更新成功".to_string()),
        Err(e) => Err(format!("更新失败: {}", e)),
    }
}

#[tauri::command]
pub fn delete_cipher(id: i64) -> Result<String, String> {
    let conn = db::init_db();
    match conn.execute("DELETE FROM cipher WHERE id=?1", [id]) {
        Ok(_) => Ok("删除成功".to_string()),
        Err(e) => Err(format!("删除失败: {}", e)),
    }
}

#[tauri::command]
pub fn search_cipher(keyword: String) -> Vec<Cipher> {
    let all = get_all_ciphers(99999999);
    let kw = keyword.to_lowercase();

    all.into_iter()
        .filter(|c| {
            c.name.to_lowercase().contains(&kw)
                || c.company.as_ref().map(|s| s.to_lowercase()).unwrap_or_default().contains(&kw)
                || c.content.to_lowercase().contains(&kw)
        })
        .collect()
}