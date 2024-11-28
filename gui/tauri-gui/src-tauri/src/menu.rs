use tauri::{App, Manager, Wry};
use tauri::menu::{MenuBuilder, MenuItem, MenuItemBuilder, SubmenuBuilder};
use std::error::Error;
use crate::WINDOW;

pub fn set_menu_handlers(new_run: MenuItem<Wry>, about: MenuItem<Wry>) {
    WINDOW
        .get()
        .unwrap()
        .app_handle()
        .on_menu_event(move |_app, event| {
            if event.id() == new_run.id() {
                WINDOW
                    .get()
                    .unwrap()
                    .eval("window.location.href='/#/'")
                    .expect("Failed to open start page");
            } else if event.id() == about.id() {
                WINDOW
                    .get()
                    .unwrap()
                    .eval("window.location.href='/#/about'")
                    .expect("Failed to open about page");
            }
        });
}

pub fn build_menu(app: &mut App) -> Result<(MenuItem<Wry>, MenuItem<Wry>), Box<dyn Error>> {
    let new_run = MenuItemBuilder::new("New Run").build(app)?;
    let about = MenuItemBuilder::new("About").build(app)?;

    let file_submenu = SubmenuBuilder::new(app, "File").item(&new_run).build()?;
    let help_submenu = SubmenuBuilder::new(app, "Help").item(&about).build()?;
    let menu = MenuBuilder::new(app)
        .item(&file_submenu)
        .separator()
        .item(&help_submenu)
        .build()?;
    app.set_menu(menu).expect("Failed to set menu");
    Ok((new_run, about))
}