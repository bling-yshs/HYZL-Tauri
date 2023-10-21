use crate::entity::hyzl_path;
use crate::entity::return_data::ReturnData;
use crate::utils::cmd_command::{run_command, WindowType};

#[tauri::command]
pub async fn reinstall_dependence() -> String {
    //检查YUNZAI_DIR文件夹下是否存在node_modules文件夹，如有则删除他
    if hyzl_path::YUNZAI_DIR.join("node_modules").exists() {
        match std::fs::remove_dir_all(hyzl_path::YUNZAI_DIR.join("node_modules")) {
            Ok(_) => {}
            Err(_) => return ReturnData::fast_failure("删除 node_modules 文件夹失败"),
        };
    }
    //在YUNZAI_DIR文件夹下执行pnpm install
    let output = run_command(&hyzl_path::YUNZAI_DIR, "pnpm install", WindowType::Default);
    if output.status.success() {
        return ReturnData::fast_success("重装依赖成功");
    } else {
        return ReturnData::fast_failure("重装依赖失败");
    }
}