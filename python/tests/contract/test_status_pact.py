"""Pact consumer + provider self-check for ``GET /api/v1/status`` (BackendStatus).

Writes the pact under ``packages/sdk/contracts/pacts/`` (gitignored JSON) and
replays it against a minimal local HTTP stub so CI does not need a broker.
"""

from __future__ import annotations

import threading
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path

import httpx
import pytest
from pact import Pact, Verifier, match

PACTS_DIR = Path(__file__).resolve().parents[3] / "contracts" / "pacts"


@pytest.mark.contract
@pytest.mark.tier_0
@pytest.mark.filterwarnings("ignore::ResourceWarning")
def test_get_status_consumer_and_provider_roundtrip() -> None:
    PACTS_DIR.mkdir(parents=True, exist_ok=True)

    pact = Pact("openapp-python-sdk", "openapp-http-api")
    (
        pact.upon_receiving("GET backend status")
        .given("backend is running")
        .with_request("GET", "/api/v1/status")
        .will_respond_with(200)
        .with_header("Content-Type", "application/json")
        .with_body(
            {
                "environment": match.str("development"),
                "version": match.str("0.0.1"),
            },
        )
    )

    with pact.serve() as srv:
        base = str(srv.url).rstrip("/")
        response = httpx.get(f"{base}/api/v1/status", timeout=10.0)
        assert response.status_code == 200
        body = response.json()
        assert "environment" in body and "version" in body

    pact.write_file(PACTS_DIR, overwrite=True)
    pact_file = PACTS_DIR / "openapp-python-sdk-openapp-http-api.json"
    assert pact_file.is_file()

    class _Handler(BaseHTTPRequestHandler):
        def do_GET(self) -> None:
            if self.path != "/api/v1/status":
                self.send_error(404)
                return
            payload = b'{"environment":"development","version":"0.0.1"}'
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)

        def log_message(self, *_args: object) -> None:
            return

    httpd = HTTPServer(("localhost", 0), _Handler)
    port = httpd.server_address[1]
    threading.Thread(target=httpd.serve_forever, daemon=True).start()
    try:
        Verifier("openapp-http-api").add_transport(
            url=f"http://localhost:{port}",
        ).add_source(pact_file).verify()
    finally:
        httpd.shutdown()
        httpd.server_close()
