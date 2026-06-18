use tauri_plugin_log::log::{info, warn, error, debug, trace};
use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
struct StockConfig {
    stock_code: &'static str,
    link: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
struct DanjuanResponse {
    data: DanjuanData,
    result_code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DanjuanData {
    index_code: String,
    name: String,
    pe: f64,
    pe_percentile: f64,
    eva_type: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockResult {
    pub stock_code: String,
    pub link: String,
    pub name: String,
    pub eva_type: String,
    pub pe: f64,
    pub pe_percentile: f64,
}

const STOCK_LIST: [StockConfig; 5] = [
    StockConfig {
        stock_code: "SH000016",
        link: "https://danjuanfunds.com/dj-valuation-table-detail/SH000016",
    },
    StockConfig {
        stock_code: "SH000300",
        link: "https://danjuanfunds.com/dj-valuation-table-detail/SH000300",
    },
    StockConfig {
        stock_code: "SH000905",
        link: "https://danjuanfunds.com/dj-valuation-table-detail/SH000905",
    },
    StockConfig {
        stock_code: "SZ399989",
        link: "https://danjuanfunds.com/dj-valuation-table-detail/SZ399989",
    },
    StockConfig {
        stock_code: "HKHSTECH",
        link: "https://danjuanfunds.com/dj-valuation-table-detail/HKHSTECH",
    },
];

async fn fetch_stock_info(stock_code: &str) -> Result<DanjuanData, String> {
    let url = format!("https://danjuanfunds.com/djapi/index_eva/detail/{}", stock_code);
    
    debug!("请求URL: {}", url);
    
    let client = reqwest::Client::new();
    
    let response = client.get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("请求失败，状态码: {}", response.status()));
    }
    
    let json: DanjuanResponse = response.json()
        .await
        .map_err(|e| format!("解析JSON失败: {}", e))?;
    
    if json.result_code != 0 {
        return Err(format!("接口返回错误: {}", json.result_code));
    }
    
    Ok(json.data)
}

#[tauri::command]
pub async fn get_stock_list() -> Result<Vec<StockResult>, String> {
    info!("调用-get_stock_list");
    
    let mut results: Vec<StockResult> = Vec::new();
    
    for stock in STOCK_LIST.iter() {
        info!("处理股票: {}", stock.stock_code);
        match fetch_stock_info(&stock.stock_code).await {
            Ok(data) => {
                results.push(StockResult {
                    stock_code: stock.stock_code.to_string(),
                    link: stock.link.to_string(),
                    name: data.name,
                    eva_type: data.eva_type,
                    pe: data.pe,
                    pe_percentile: data.pe_percentile * 100.0,
                });
            },
            Err(e) => {
                warn!("获取股票信息失败: {} - {}", stock.stock_code, e);
            }
        }
    }
    
    Ok(results)
}