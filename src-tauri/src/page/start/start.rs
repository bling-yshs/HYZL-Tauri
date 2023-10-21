use std::fs;
use std::str::from_utf8;

use serde_json::json;
use log::{info, warn};

use return_data::ReturnData;

use crate::entity::hyzl_path;
use crate::entity::return_data;
use crate::utils::cmd_command::{run_command, WindowType};

#[tauri::command]
pub async fn start_yunzai() -> String {
    json!(ReturnData::fast_success("启动成功")).to_string()
}

#[tauri::command]
pub async fn start_yunzai_and_api() -> String {
    if !hyzl_path::SIGN_API_DIR.exists() {
        return ReturnData::fast_failure("签名API文件夹不存在");
    }
    tokio::spawn(async {
        run_command(&hyzl_path::SIGN_API_DIR, "start.bat", WindowType::WithWindowKeepOpen);
    });

    tokio::spawn(async {
        run_command(&hyzl_path::YUNZAI_DIR, "node app", WindowType::WithWindowKeepOpen);
    });

    return ReturnData::fast_success("启动成功");
}

#[tauri::command]
pub async fn download_sign_api() -> String {
    info!("调用了 download_sign_api fn");
    let output = run_command(&hyzl_path::PROGRAM_DIR, "git clone --depth 1 https://github.com/bling-yshs/unidbg-fetch-qsign", WindowType::Default);
    if output.status.success() {
        info!("API下载成功");
        match fs::rename("unidbg-fetch-qsign", "API") {
            Ok(_) => { return json!(ReturnData::fast_success("下载成功!")).to_string(); }
            Err(err) => {
                warn!("文件夹重命名失败：{}", err);
                return ReturnData::fast_failure(&err.to_string()[..]);
            }
        }
    } else {
        warn!("download_sign_api fn 错误: stderr: {}",from_utf8(&output.stderr).unwrap().to_string());
        return ReturnData::fast_failure(from_utf8(&output.stderr).unwrap());
    }
}

#[test]
fn t() {
    let output = run_command(&hyzl_path::PROGRAM_DIR, "chdidd", WindowType::Default);
    if output.stderr.is_empty() { println!("空的！") } else { println!("错误") }
}
