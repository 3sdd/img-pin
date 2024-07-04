use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

pub fn create_menu() -> Menu {
    let open=CustomMenuItem::new("open","Open");
    let open_recent=CustomMenuItem::new("open_recent","Open Recent");
    let save=CustomMenuItem::new("save", "Save");
    let save_as=CustomMenuItem::new("save_as", "Save As");
    // サブメニューを作成
    let submenu_file = Submenu::new("File", Menu::new()
        .add_item(open)
        .add_item(open_recent)  
        .add_native_item(MenuItem::Separator)
        .add_item(save)
        .add_item(save_as)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit)
    );

    let submenu_edit=Submenu::new("Edit",Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
    );

    // メニューを作成
    Menu::new().add_submenu(submenu_file)
        .add_submenu(submenu_edit)
}

pub fn handle_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "open" => {
            println!("[menu] open clicked");
        },
        "open_recent"=>{
            println!("[menu] open_recent clicked");
        }
        "save"=>{
            println!("[menu] save clicked");
        },
        "save_as"=>{
            println!("[menu] save_as recent clicked");
        },
        _ => {},
    }
}