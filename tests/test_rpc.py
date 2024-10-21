import os
import typing
import dotenv
import requests

dotenv.load_dotenv()

rpc_ip = os.getenv("RPC_SERVICE_IP")
rpc_port = os.getenv("RPC_SERVICE_PORT")
rpc_url = f"http://{rpc_ip}:{rpc_port}"


def json_rpc_request(
    http_method: str = "POST",
    http_endpoint: str = "/",
    json: list[dict] | None = None,
) -> requests.Response:
    response = requests.request(
        http_method,
        (rpc_url + http_endpoint),
        json=json,
    )
    return response


def test_rpc_http_methods() -> None:
    req = json_rpc_request("GET")
    assert req.status_code == 405

    req = json_rpc_request("POST")
    assert req.status_code == 400

    req = json_rpc_request("DELETE")
    assert req.status_code == 405

    req = json_rpc_request("PUT")
    assert req.status_code == 405

    req = json_rpc_request("PATCH")
    assert req.status_code == 405

    req = json_rpc_request("HEAD")
    assert req.status_code == 405

    req = json_rpc_request("OPTIONS")
    assert req.status_code == 405

    req = json_rpc_request("TRACE")
    assert req.status_code == 405

    req = json_rpc_request("CONNECT")
    assert req.status_code == 405


def test_rpc_jsonrpc() -> None:
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": None,
                "id": 1,
                "method": "healthcheck",
            }
        ],
    )
    assert req.status_code == 400
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": "",
                "id": 1,
                "method": "healthcheck",
            }
        ],
    )
    assert req.status_code == 400
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": "2.1",
                "id": 1,
                "method": "healthcheck",
            }
        ],
    )
    assert req.status_code == 400
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": 2.0,
                "id": 1,
                "method": "healthcheck",
            }
        ],
    )
    assert req.status_code == 400
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "healthcheck",
            }
        ],
    )
    assert req.status_code == 200


def test_rpc_id() -> None:
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": "2.0",
                "id": [1],
                "method": "healthcheck",
            }
        ],
    )
    assert req.status_code == 400
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": "2.0",
                "id": {},
                "method": "healthcheck",
            }
        ],
    )
    assert req.status_code == 400
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": "2.0",
                "id": 1.123123123,
                "method": "healthcheck",
            }
        ],
    )
    assert req.status_code == 200  # TODO FIX


def test_rpc_method() -> None:
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": None,
            }
        ],
    )
    assert req.status_code == 400
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "",
            }
        ],
    )
    assert req.status_code == 404
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "_",
            }
        ],
    )
    assert req.status_code == 404
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "healthcheck",
            }
        ],
    )
    assert req.status_code == 200


def test_rpc_json_dict() -> None:
    req = json_rpc_request(
        json={
            "jsonrpc": "2.0",
            "id": 1,
            "method": "healthcheck",
        }
    )
    assert req.status_code == 400


def test_rpc_json_list() -> None:
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": "2.0",
                "id": id,
                "method": "validate_phone_number",
            }
            for id in range(256)
        ]
    )
    assert req.status_code == 200


def test_rpc_number_validator() -> None:
    req = json_rpc_request(
        json=[
            {
                "jsonrpc": "2.0",
                "id": id,
                "method": "validate_phone_number",
                "params": {
                    "number": "+79121234567",
                },
            }
            for id in range(256)
        ]
    )
    assert req.status_code == 200
