// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::menu::{Menu, MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::Emitter;
use tauri::{AppHandle, WebviewWindowBuilder};
use tauri_plugin_dialog::{DialogExt, FilePath};

mod cif_parser;

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
            create_menu(app.handle());

            // 注册菜单事件监听器
            app.on_menu_event(move |app_handle, event| {
                let app_handle_clone = app_handle.clone();
                // 使用 tokio::spawn 或 tauri::async_runtime::spawn 来处理异步任务
                tauri::async_runtime::spawn(async move {
                    handle_menu_event(app_handle_clone, event.id().as_ref()).await;
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

/// 创建菜单
fn create_menu(app: &AppHandle) {
    // 创建菜单项
    let open = MenuItemBuilder::with_id("open", "Open")
        .build(app)
        .expect("open menu item");
    let quit = MenuItemBuilder::with_id("quit", "Quit")
        .build(app)
        .expect("quit menu item");

    // 创建 View 菜单项
    let toggle_atom_labels = MenuItemBuilder::with_id("toggle_atom_labels", "Toggle Atom Labels")
        .build(app)
        .expect("toggle atom labels menu item");

    // 创建 Tools 菜单项
    let set_repeats = MenuItemBuilder::with_id("set_repeats", "Set Repeats (x/y/z)")
        .build(app)
        .expect("set repeats menu item");

    // 创建 Help 菜单项
    let about = MenuItemBuilder::with_id("about", "About")
        .build(app)
        .expect("about menu item");

    // 创建子菜单 - File
    let file_menu = SubmenuBuilder::new(app, "File")
        .items(&[&open, &quit])
        .build()
        .expect("file submenu");

    // 创建子菜单 - View
    let view_menu = SubmenuBuilder::new(app, "View")
        .items(&[&toggle_atom_labels])
        .build()
        .expect("view submenu");

    // 创建子菜单 - Tools
    let tools_menu = SubmenuBuilder::new(app, "Tools")
        .items(&[&set_repeats])
        .build()
        .expect("tools submenu");

    // 创建子菜单 - Help
    let help_menu = SubmenuBuilder::new(app, "Help")
        .items(&[&about])
        .build()
        .expect("help submenu");

    // 创建菜单并设置到应用程序中
    let menu = MenuBuilder::new(app)
        .item(&file_menu)
        .item(&view_menu)
        .item(&tools_menu)
        .item(&help_menu)
        .build()
        .expect("menu");

    app.set_menu(menu).expect("failed to set menu");
}

/// 处理菜单事件
async fn handle_menu_event(app_handle: AppHandle, menu_id: &str) {
    match menu_id {
        "open" => {
            println!("Open menu clicked");
            open_file_dialog(app_handle).await;
        }
        "quit" => {
            println!("Quit menu clicked");
            std::process::exit(0);
        }
        "toggle_atom_labels" => {
            println!("Toggling atom labels");
            // 你可以在这里发送事件到前端，切换显示/隐藏原子名称的状态
            app_handle.emit("toggle-atom-labels", {}).unwrap();
        }
        "set_repeats" => {
            println!("Set Repeats clicked");
            // 弹出一个对话框或者滑块以设置 x/y/z 方向的重复晶格数量
            app_handle.emit("set-repeats", {}).unwrap();
        }
        "about" => {
            // 打开一个新窗口，显示 About 页面
            println!("About clicked");

            let _balnk_menu = Menu::new(&app_handle);

            // 创建一个新窗口
            let _about_window = WebviewWindowBuilder::new(
                &app_handle,                                     // AppHandle
                "about_window",                                  // 窗口标识符
                tauri::WebviewUrl::App("src/about.html".into()), // 加载 about.html 页面
            )
            .title("About") // 设置窗口标题
            .inner_size(400.0, 300.0) // 设置窗口大小
            .resizable(false) // 禁止调整窗口大小
            .menu(_balnk_menu.unwrap())
            .build()
            .expect("Failed to build About window");
        }
        _ => {}
    }
}

/// 打开文件对话框的异步函数
#[tauri::command]
async fn open_file_dialog(app_handle: AppHandle) {
    let file_path = app_handle.dialog().file().blocking_pick_file();

    if let Some(file_path) = file_path {
        parse_and_send_data(app_handle, file_path).await;
    }
}

/// 解析文件并发送数据的异步函数
#[tauri::command]
async fn parse_and_send_data(app_handle: AppHandle, file_path: FilePath) {
    let data = cif_parser::parse_cif(&app_handle, &file_path);
    app_handle.emit("cif-data", data).unwrap();
}
