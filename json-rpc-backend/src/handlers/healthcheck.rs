use serde_json;

use crate::core::rpc;

pub async fn healthcheck_handler(
    request: &rpc::Request,
    mut response: rpc::Response,
) -> rpc::Response {
    let timestamp: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let result: serde_json::Value = serde_json::json!({
        "status": true,
        "timestamp": timestamp,
    });

    response.result = Some(result);

    response
}
