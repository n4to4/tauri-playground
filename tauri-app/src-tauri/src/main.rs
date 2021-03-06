#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![hello_tauri])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn hello_tauri() -> String {
    "Hello Tauri 1.0 ＼(^o^)／".into()
}
