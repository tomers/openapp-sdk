<!-- markdownlint-disable MD013 -->
# API contracts (consumer-driven)

Pact consumer artifacts and broker-oriented workflows for the OpenApp HTTP API.

## Today

- **OpenAPI** remains the canonical schema (`api-spec/openapi.json`), enforced by `just sdk openapi-check`.
- **Python consumer pact** — `tests/contract/test_status_pact.py` writes `pacts/openapp-python-sdk-openapp-http-api.json` under `pacts/` (gitignored `*.json`; directory kept via `.gitkeep`).
- **Broker publish (optional)** — after contract tests, CI can run `python/scripts/publish_pacts_to_broker.sh` when `PACT_BROKER_BASE_URL` is set (see `https://openapp.house/docs/sdk/`).
- **Provider verify from broker (optional)** — `tests/contract/test_broker_provider_optional.py` runs when `PACT_BROKER_VERIFY=1` and broker auth + `PACT_PROVIDER_BASE_URL` are set (staging-style pipelines).

## Layout

- `pacts/` — generated JSON from consumer tests; do not commit `*.json`.
- `README.md` (this file) — pointers to broker setup and CI knobs.

## Alternatives

OpenAPI-first tools (Prism, Dredd, etc.) can complement Pact; document choices in `packages/sdk/docs/`.
