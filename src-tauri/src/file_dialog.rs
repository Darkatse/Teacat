use tauri::Emitter;
use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, FilePath};

/// 打开文件对话框并解析文件的异步函数
#[tauri::command]
pub async fn open_file_dialog(app_handle: AppHandle) {
    let file_path = app_handle.dialog().file().blocking_pick_file();

    if let Some(file_path) = file_path {
        parse_and_send_data(app_handle, file_path).await;
    }
}

/// 解析文件并发送数据的异步函数
#[tauri::command]
pub async fn parse_and_send_data(app_handle: AppHandle, file_path: FilePath) {
    let data = crate::parsers::cif_parser::parse_cif(&app_handle, &file_path);
    app_handle.emit("cif-data", data).unwrap();
}