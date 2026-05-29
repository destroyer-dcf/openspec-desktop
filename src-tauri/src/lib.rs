mod commands;
mod openspec;

use commands::project::{
    archive_proposals, bootstrap_state, check_openspec_cli, delete_proposals, get_active_index, get_projects,
    get_versions,
    create_spec_document, get_proposal, get_state, init_project, list_proposals, list_spec_documents, open_project, pick_project_folder,
    read_file, save_proposal, set_active_project, unlink_project, write_file, AppState,
    copy_to_clipboard,
};
use tauri::Manager;

const APP_ICON: tauri::image::Image<'_> = tauri::include_image!("./icons/128x128.png");

#[cfg(desktop)]
fn build_app_menu<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<tauri::menu::Menu<R>> {
    use tauri::menu::{AboutMetadata, Menu, PredefinedMenuItem, Submenu};

    let about = AboutMetadata {
        name: Some("OpenSpec Desktop".to_string()),
        version: Some(env!("CARGO_PKG_VERSION").to_string()),
        short_version: Some(env!("CARGO_PKG_VERSION").to_string()),
        icon: Some(APP_ICON.clone()),
        ..Default::default()
    };

    let window_menu = Submenu::with_items(
        app,
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(app, None)?,
            &PredefinedMenuItem::maximize(app, None)?,
            #[cfg(target_os = "macos")]
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::close_window(app, None)?,
        ],
    )?;

    let help_menu = Submenu::with_items(app, "Help", true, &[])?;

    Menu::with_items(
        app,
        &[
            #[cfg(target_os = "macos")]
            &Submenu::with_items(
                app,
                "OpenSpec Desktop",
                true,
                &[
                    &PredefinedMenuItem::about(app, None, Some(about))?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::services(app, None)?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::hide(app, None)?,
                    &PredefinedMenuItem::hide_others(app, None)?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::quit(app, None)?,
                ],
            )?,
            &Submenu::with_items(
                app,
                "File",
                true,
                &[
                    &PredefinedMenuItem::close_window(app, None)?,
                    #[cfg(not(target_os = "macos"))]
                    &PredefinedMenuItem::quit(app, None)?,
                ],
            )?,
            &Submenu::with_items(
                app,
                "Edit",
                true,
                &[
                    &PredefinedMenuItem::undo(app, None)?,
                    &PredefinedMenuItem::redo(app, None)?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::cut(app, None)?,
                    &PredefinedMenuItem::copy(app, None)?,
                    &PredefinedMenuItem::paste(app, None)?,
                    &PredefinedMenuItem::select_all(app, None)?,
                ],
            )?,
            #[cfg(target_os = "macos")]
            &Submenu::with_items(app, "View", true, &[&PredefinedMenuItem::fullscreen(app, None)?])?,
            &window_menu,
            &help_menu,
        ],
    )
}

#[cfg(target_os = "macos")]
fn set_macos_application_icon() {
    use objc2::{AllocAnyThread, MainThreadMarker};
    use objc2_app_kit::{NSApplication, NSImage};
    use objc2_foundation::NSData;

    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };

    let app = NSApplication::sharedApplication(mtm);
    let data = NSData::with_bytes(include_bytes!("../icons/128x128.png"));
    if let Some(image) = NSImage::initWithData(NSImage::alloc(), &data) {
        unsafe { app.setApplicationIconImage(Some(&image)) };
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .menu(build_app_menu)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::default())
        .setup(|app| {
            let handle = app.handle();
            let state = handle.state::<AppState>();
            if let Some(window) = app.get_webview_window("main") {
                window.set_icon(APP_ICON.clone())?;
            }
            #[cfg(target_os = "macos")]
            set_macos_application_icon();
            bootstrap_state(&handle, &state).map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_project,
            get_state,
            get_projects,
            get_active_index,
            set_active_project,
            unlink_project,
            pick_project_folder,
            read_file,
            write_file,
            list_spec_documents,
            create_spec_document,
            list_proposals,
            get_proposal,
            save_proposal,
            archive_proposals,
            delete_proposals,
            copy_to_clipboard,
            check_openspec_cli,
            get_versions,
            init_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
