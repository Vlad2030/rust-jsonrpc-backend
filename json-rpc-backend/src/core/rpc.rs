use derive_more;
use http;
use ntex;
use serde;
use serde_json;

use crate::handlers::healthcheck;
use crate::handlers::phone_number;

#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Request {
    jsonrpc: String,
    id: serde_json::Value,
    method: String,
    pub params: Option<serde_json::Value>,
}

impl Request {
    pub fn version_is_valid(&self) -> bool {
        self.jsonrpc == "2.0"
    }

    pub fn id_is_valid(&self) -> bool {
        self.id.is_number() || self.id.is_string() || self.id.is_null()
    }
}

#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Response {
    jsonrpc: String,
    id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl Response {
    pub fn new(
        id: serde_json::Value,
        result: Option<serde_json::Value>,
        error: Option<Error>,
    ) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result,
            error,
        }
    }

    pub fn status_code(&self) -> http::StatusCode {
        if self.error.is_some() {
            match self.error.clone().unwrap().code {
                -32700 => return Errors::ParseError.status_code(),
                -32600 => return Errors::InvalidRequest.status_code(),
                -32601 => return Errors::MethodNotFound.status_code(),
                -32602 => return Errors::InvalidParams.status_code(),
                -32603 => return Errors::InternalError.status_code(),
                -32000 => return Errors::ServerError.status_code(),
                _ => return Errors::ServerError.status_code(),
            }
        }
        http::StatusCode::OK
    }

    pub fn to_http_response(&self) -> ntex::web::HttpResponse {
        ntex::web::HttpResponse::build(self.status_code())
            .set_header("content-type", "application/json")
            .json(&self)
    }
}

#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Error {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}

impl Error {
    pub fn new(
        error: Errors,
        data: Option<serde_json::Value>,
    ) -> Self {
        Self {
            code: error.code(),
            message: error.message(),
            data,
        }
    }
}

#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(derive_more::Display, derive_more::Error)]
pub enum Errors {
    #[display("parse error")]
    ParseError,
    #[display("invalid request")]
    InvalidRequest,
    #[display("method not found")]
    MethodNotFound,
    #[display("invalid params")]
    InvalidParams,
    #[display("internal error")]
    InternalError,
    #[display("server error")]
    ServerError,
}

impl Errors {
    pub fn code(&self) -> i32 {
        match self {
            Self::ParseError => -32700,
            Self::InvalidRequest => -32600,
            Self::MethodNotFound => -32601,
            Self::InvalidParams => -32602,
            Self::InternalError => -32603,
            Self::ServerError => -32000,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::ParseError => "Parse error".into(),
            Self::InvalidRequest => "Invalid Request".into(),
            Self::MethodNotFound => "Method not found".into(),
            Self::InvalidParams => "Invalid params".into(),
            Self::InternalError => "Internal error".into(),
            Self::ServerError => "Server error".into(),
        }
    }

    pub fn to_error(
        self,
        data: Option<serde_json::Value>,
    ) -> Error {
        Error::new(self, data)
    }

    pub fn status_code(&self) -> http::StatusCode {
        match *self {
            Self::ParseError => http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidRequest => http::StatusCode::BAD_REQUEST,
            Self::MethodNotFound => http::StatusCode::NOT_FOUND,
            Self::InvalidParams => http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalError => http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::ServerError => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn handler(requests: ntex::web::types::Json<Vec<Request>>) -> ntex::web::HttpResponse {
    let mut responses: Vec<Response> = Vec::new();

    // TODO: add tokio::spawn tasks for much perfomance
    for request in requests.iter() {
        let mut response: Response = Response::new(request.id.clone(), None, None);

        if !request.version_is_valid() || !request.id_is_valid() {
            response.error = Some(Errors::InvalidRequest.to_error(None));

            if !request.id_is_valid() {
                response.id = serde_json::Value::Null;
            }

            responses.push(response);
            continue;
        }

        match request.method.as_str() {
            "healthcheck" => {
                response = healthcheck::healthcheck_handler(request, response).await
            },
            "validate_phone_number" => {
                response = phone_number::validate_handler(request, response).await
            },
            _ => response.error = Some(Errors::MethodNotFound.to_error(None))
        }
        // for (method, handler) in HANDLERS {
        //     // ...
        // }

        if response.result.is_none() && response.result.is_none() {
            response.error = Some(Errors::MethodNotFound.to_error(None));
        }

        responses.push(response);
    }

    if responses.len() == 1 {
        return responses[0].to_http_response()
    }

    ntex::web::HttpResponse::Ok().json(&responses)
}
