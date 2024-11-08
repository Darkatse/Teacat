// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use file_dialog::{open_file_dialog,parse_and_send_data};

mod menu;
mod file_dialog;
mod parsers;
mod utils;

fn main() {
    // n卡会白屏，WebKit新版的渲染器与nvidia驱动暂时还不兼容导致
    // 使用env WEBKIT_DISABLE_DMABUF_RENDERER=1可以退回旧版渲染器解决问题
    if std::env::consts::OS == "linux" {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    // tauri_teacat_lib::run()
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // 创建菜单
            menu::create_menu(app.handle());

            // 注册菜单事件监听器
            app.on_menu_event(move |app_handle, event| {
                let app_handle_clone = app_handle.clone();
                // 使用 tokio::spawn 或 tauri::async_runtime::spawn 来处理异步任务
                tauri::async_runtime::spawn(async move {
                    menu::handle_menu_event(app_handle_clone, event.id().as_ref()).await;
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_file_dialog,
            parse_and_send_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
