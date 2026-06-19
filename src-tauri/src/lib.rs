mod agents;

use dotenv;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn format_markdown(text: String) -> String {
    agents::md_with_groq(text).await

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, format_markdown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



