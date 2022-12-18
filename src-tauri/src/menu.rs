use tauri::{WindowMenuEvent, Wry};
use tauri::{Menu, MenuItem, Submenu, CustomMenuItem};
#[cfg(target_os = "macos")]
use tauri::AboutMetadata;

pub fn create_menu(#[allow(unused)] app_name: &str) -> Menu {
    let mut menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            app_name,
            Menu::new()
                .add_native_item(MenuItem::About(
                    app_name.to_string(),
                    AboutMetadata::default(),
                ))
                .add_native_item(MenuItem::Separator)
                .add_item(CustomMenuItem::new("preference", "Preference..."))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Services)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ));
    }

    let mut file_menu = Menu::new();
    file_menu = file_menu.add_item(
        CustomMenuItem::new("init", "Initialize Repository...")
    );
    file_menu = file_menu.add_native_item(MenuItem::Separator);
    file_menu = file_menu.add_item(
        CustomMenuItem::new("add_local", "Add Local Repository...")
    );
    file_menu = file_menu.add_item(
        CustomMenuItem::new("clone", "Close Repository...")
    );
    #[cfg(not(target_os = "macos"))]
    {
        file_menu = file_menu.add_native_item(MenuItem::Separator);
        file_menu = file_menu.add_item(
            CustomMenuItem::new("preference", "Preference...")
        );
    }
    menu = menu.add_submenu(Submenu::new("File", file_menu));

    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            "View",
            Menu::new().add_native_item(MenuItem::EnterFullScreen),
        ));
    }

    let mut window_menu = Menu::new();
    window_menu = window_menu.add_native_item(MenuItem::Minimize);
    #[cfg(target_os = "macos")]
    {
        window_menu = window_menu.add_native_item(MenuItem::Zoom);
        window_menu = window_menu.add_native_item(MenuItem::Separator);
    }
    window_menu = window_menu.add_native_item(MenuItem::CloseWindow);
    menu = menu.add_submenu(Submenu::new("Window", window_menu));

    menu
}

pub fn event_handler(event: WindowMenuEvent<Wry>) {
    let event_name = event.menu_item_id();
    event.window().emit("menu-event", event_name).unwrap();
    log::trace!("{}", event_name);
}
