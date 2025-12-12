// src-tauri/src/main.rs (修正版)

#[tauri::command]
fn greet(name: &str) -> String {
    // 您可以在這裡添加或保留 println! 用於後端日誌
    println!("DEBUG: 'greet' function was called with name: {}", name); 
    format!("Hello, {}! 您已從 Rust 收到問候！", name)
}

fn main() {
    tauri::Builder::default()
        // 【關鍵】確保這一行在 .run() 之前！
        .invoke_handler(tauri::generate_handler![greet]) 
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
