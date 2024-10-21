use serde_json;

use crate::core::rpc;
use crate::utils::phone_numbers;

pub async fn validate_handler(
    request: &rpc::Request,
    mut response: rpc::Response,
) -> rpc::Response {
    let params: Option<serde_json::Value> = request.clone().params;

    if params.is_none() {
        response.error = Some(rpc::Errors::InvalidParams.to_error(None));
        return response
    }

    let params_data: serde_json::Value = params.clone().unwrap();
    let number: Option<&serde_json::Value> = params_data.get("number");

    if number.is_none() {
        let data: serde_json::Value = serde_json::json!({
            "detail": format!("number is {:#?}", number)
        });
        response.error = Some(rpc::Errors::InvalidParams.to_error(Some(data)));
        return response
    }

    let formated: Option<String> = phone_numbers::PhoneParser::new(
        number
            .unwrap()
            .as_str()
            .unwrap()
            .into()
    ).parse();

    let result: serde_json::Value = serde_json::json!({
        "number": formated,
    });

    response.result = Some(result);

    response
}
