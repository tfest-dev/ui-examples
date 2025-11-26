use single_window_backend::{make_greeting, GreetingRequest, GreetingResponse};

#[tauri::command]
fn greet(name: String) -> GreetingResponse {
    let req = GreetingRequest { name };
    make_greeting(req)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
