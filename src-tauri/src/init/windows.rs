use winver::WindowsVersion;
use crate::entity::return_data::ReturnData;

#[tauri::command]
pub fn is_win11() -> String {
    let version = WindowsVersion::detect().unwrap();
    return if version >= WindowsVersion::new(10, 0, 22000) {
        ReturnData::fast_success("true")
    } else {
        ReturnData::fast_failure("false")
    }
}