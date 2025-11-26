use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreetingRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreetingResponse {
    pub message: String,
}

pub fn make_greeting(req: GreetingRequest) -> GreetingResponse {
    let name = if req.name.trim().is_empty() {
        "World".to_string()
    } else {
        req.name.trim().to_string()
    };

    GreetingResponse {
        message: format!(
            "Hello, {name}! This message is generated in the Rust backend and sent to the UI via a typed API.",
        ),
    }
}
