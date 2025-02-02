use ntex;
use serde_json;

use crate::core::rpc;

pub async fn invalid_request() -> Result<ntex::web::HttpResponse, ntex::web::Error> {
    let response: rpc::Response = rpc::Response::new(
        serde_json::Value::Null,
        None,
        Some(rpc::Errors::InvalidRequest.to_error(None)),
    );
    Ok(response.to_http_response())
}

pub async fn method_not_allowed() -> Result<ntex::web::HttpResponse, ntex::web::Error> {
    Ok(ntex::web::HttpResponse::MethodNotAllowed().finish())
}

pub async fn error_handler(
    request: &rpc::Request,
    mut response: rpc::Response,
    error: rpc::Errors,
    data: Option<serde_json::Value>,
) -> rpc::Response {
    response.error = Some(error.to_error(data));

    response
}
