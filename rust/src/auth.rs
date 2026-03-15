use serde_json::{json, Value};

pub fn api_key_auth(key_id: &str, secret_hint: &str, request_id: u64) -> Value {
    json!({
        "id": request_id,
        "auth": {
            "key_id": key_id,
            "secret_hint": secret_hint
        }
    })
}
