# OpenApp SDK

[![CI](https://github.com/tomers/openapp-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/tomers/openapp-sdk/actions/workflows/ci.yml)
[![Test report](https://img.shields.io/badge/test%20report-Allure-blue)](https://tomers.github.io/openapp-sdk/)
[![PyPI version](https://img.shields.io/pypi/v/openapp-sdk)](https://pypi.org/project/openapp-sdk/)
[![Go Reference](https://pkg.go.dev/badge/github.com/tomers/openapp-sdk/go.svg)](https://pkg.go.dev/github.com/tomers/openapp-sdk/go)
[![crates.io](https://img.shields.io/crates/v/openapp-sdk)](https://crates.io/crates/openapp-sdk)
[![npm version](https://img.shields.io/npm/v/@tomers/openapp-sdk)](https://www.npmjs.com/package/@tomers/openapp-sdk)

**Physical Security as a Service (PSaaS)** — official **Python**, **Go**, and **Rust** sources plus the **OpenAPI** specification for OpenApp: API-first access control for doors, gates, virtual intercom, guest invitations, policies, and audit across heterogeneous hardware (Home Assistant, PalGate, MQTT, KNX, and more).

## Use cases

- Apartment and office buildings — directory, intercom, delegated resident management
- Hotels and short-term rentals — time-bound guest invitations, PMS integration
- Campus and multi-site — zones, scripting provisioning, locker/parcel matrices
- Home and parking — gates and barriers via PalGate Cloud and relays
- Automation — agents, webhooks, and MCP against the same HTTP API

## Running tests

Clone this repository (tests are not included in PyPI wheels). From the repo root:

```bash
just test          # host toolchains — same slice as CI
just test-docker   # Docker only (see tests/docker/)
```

Details: [`TESTING.md`](TESTING.md) · shared scenarios under [`tests/`](tests/).

Tests in this repository use mocked fixtures only. Never commit real API keys.

## Documentation

- Product: https://openapp.house/
- SDK docs: https://openapp.house/docs/sdk/
- Agents & automation: https://openapp.house/docs/guides/agents/overview/
- Access control by sector: https://openapp.house/docs/guides/access-control-architecture/access-control-model-by-sector/
- API reference: https://openapp.house/docs/api-reference/
- OpenAPI JSON: https://openapp.house/docs/api-spec/openapp-openapi.json
- AI index (llms.txt): https://openapp.house/llms.txt

| Directory | Contents |
|-----------|----------|
| `rust/` | Rust workspace (core, common, C bridge) |
| `python/` | Python package (`openapp-sdk` on PyPI) |
| `go/` | Go module (`go get github.com/tomers/openapp-sdk/go`) |
| `api-spec/` | OpenAPI definitions |
| `tests/features/` | Shared Gherkin scenarios (Behave / godog) |
| `tests/contracts/` | Pact consumer fixtures |
| `tests/docker/` | Optional Docker toolchains for local test runs |
| `justfile` | Run `just test` or `just test-docker` — see [`TESTING.md`](TESTING.md) |

**Synced:** `sdk-python-v0.1.61` · [PyPI](https://pypi.org/project/openapp-sdk/) · [Go module](https://pkg.go.dev/github.com/tomers/openapp-sdk/go) · [crates.io](https://crates.io/crates/openapp-sdk) · [npm](https://www.npmjs.com/package/@tomers/openapp-sdk) · [Test report](https://tomers.github.io/openapp-sdk/) · [Issues](https://github.com/tomers/openapp-sdk/issues)
