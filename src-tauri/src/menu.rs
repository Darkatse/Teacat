use tauri::Emitter;
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{AppHandle, WebviewWindowBuilder};

/// 创建菜单
pub fn create_menu(app: &AppHandle) {
    let open = MenuItemBuilder::with_id("open", "Open").build(app).expect("open menu item");
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app).expect("quit menu item");

    let toggle_atom_labels = MenuItemBuilder::with_id("toggle_atom_labels", "Toggle Atom Labels")
        .build(app)
        .expect("toggle atom labels menu item");

    let set_repeats = MenuItemBuilder::with_id("set_repeats", "Set Repeats (x/y/z)")
        .build(app)
        .expect("set repeats menu item");

    let about = MenuItemBuilder::with_id("about", "About")
        .build(app)
        .expect("about menu item");

    let file_menu = SubmenuBuilder::new(app, "File")
        .items(&[&open, &quit])
        .build()
        .expect("file submenu");

    let view_menu = SubmenuBuilder::new(app, "View")
        .items(&[&toggle_atom_labels])
        .build()
        .expect("view submenu");

    let tools_menu = SubmenuBuilder::new(app, "Tools")
        .items(&[&set_repeats])
        .build()
        .expect("tools submenu");

    let help_menu = SubmenuBuilder::new(app, "Help")
        .items(&[&about])
        .build()
        .expect("help submenu");

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
pub async fn handle_menu_event(app_handle: AppHandle, menu_id: &str) {
    match menu_id {
        "open" => {
            println!("Open menu clicked");
            crate::file_dialog::open_file_dialog(app_handle).await;
        }
        "quit" => {
            println!("Quit menu clicked");
            std::process::exit(0);
        }
        "toggle_atom_labels" => {
            app_handle.emit("toggle-atom-labels", {}).unwrap();
        }
        "set_repeats" => {
            app_handle.emit("set-repeats", {}).unwrap();
        }
        "about" => {
            let _about_window = WebviewWindowBuilder::new(
                &app_handle,
                "about_window",
                tauri::WebviewUrl::App("src/about.html".into()),
            )
            .title("About")
            .inner_size(400.0, 300.0)
            .resizable(false)
            .build()
            .expect("Failed to build About window");
        }
        _ => {}
    }
}