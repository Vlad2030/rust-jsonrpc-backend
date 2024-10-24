import os
import typing
import dotenv
import requests
import multiprocessing

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
    json = [
        {
            "jsonrpc": None,
            "id": 1,
            "method": "healthcheck",
        }
    ]

    req = json_rpc_request(json=json)
    assert req.status_code == 400

    json[0]["jsonrpc"] = ""
    req = json_rpc_request(json=json)
    assert req.status_code == 400

    json[0]["jsonrpc"] = "2.1"
    req = json_rpc_request(json=json)
    assert req.status_code == 400

    json[0]["jsonrpc"] = 2.0
    req = json_rpc_request(json=json)
    assert req.status_code == 400

    json[0]["jsonrpc"] = "2.0"
    req = json_rpc_request(json=json)
    assert req.status_code == 200


def test_rpc_id() -> None:
    json = [
        {
            "jsonrpc": "2.0",
            "id": [1],
            "method": "healthcheck",
        }
    ]

    req = json_rpc_request(json=json)
    assert req.status_code == 400

    json[0]["id"] = {}
    req = json_rpc_request(json=json)
    assert req.status_code == 400

    json[0]["id"] = 1.123123123
    req = json_rpc_request(json=json)
    assert req.status_code == 200  # TODO FIX


def test_rpc_method() -> None:
    json = [
        {
            "jsonrpc": "2.0",
            "id": 1,
            "method": None,
        }
    ]

    req = json_rpc_request(json=json)
    assert req.status_code == 400

    json[0]["method"] = ""
    req = json_rpc_request(json=json)
    assert req.status_code == 404

    json[0]["method"] = "_"
    req = json_rpc_request(json=json)
    assert req.status_code == 404

    json[0]["method"] = "healthcheck"
    req = json_rpc_request(json=json)
    assert req.status_code == 200


def test_rpc_json_dict() -> None:
    json = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "healthcheck",
    }

    req = json_rpc_request(json=json)
    assert req.status_code == 400


def test_rpc_json_big_list() -> None:
    json = [
        {
            "jsonrpc": "2.0",
            "id": id,
            "method": "healthcheck",
        }
        for id in range(256)
    ]

    req = json_rpc_request(json=json)
    assert req.status_code == 200


def test_rpc_json_highload() -> None:
    json = [
        {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "healthcheck",
        }
    ]

    for _ in range(2048):
        req = json_rpc_request(json=json)
        assert req.status_code == 200


def test_rpc_json_big_list_highload() -> None:
    processes = []

    def worker() -> None:
        json = [
            {
                "jsonrpc": "2.0",
                "id": id,
                "method": "healthcheck",
            }
            for id in range(256)
        ]

        for _ in range(2048):
            req = json_rpc_request(json=json)
            assert req.status_code == 200

    for _ in range(4):
        process = multiprocessing.Process(target=worker)
        processes.append(process)
        process.start()

    for process in processes:
        process.join()
