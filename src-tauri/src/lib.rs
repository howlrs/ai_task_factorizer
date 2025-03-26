mod api;
use serde_json::{json, Value};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(resource: &str) -> Result<Value, Value> {
    match api::gemini::request::for_disassemble(resource).await {
        Ok(v) => Ok(v),
        Err(e) => {
            eprintln!("error: {:?}", e);
            Err(json!({"error": e}))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::from_filename(".env.local").ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
