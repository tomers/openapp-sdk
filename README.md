<!-- markdownlint-disable MD013 MD060 MD034 -->
# OpenApp SDK

[![CI](https://github.com/tomers/openapp-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/tomers/openapp-sdk/actions/workflows/ci.yml)
[![Test report](https://img.shields.io/badge/test%20report-Allure-blue)](https://tomers.github.io/openapp-sdk/)
[![PyPI version](https://img.shields.io/pypi/v/openapp-sdk)](https://pypi.org/project/openapp-sdk/)
[![Go Reference](https://pkg.go.dev/badge/github.com/tomers/openapp-sdk/go.svg)](https://pkg.go.dev/github.com/tomers/openapp-sdk/go)
[![crates.io](https://img.shields.io/crates/v/openapp-sdk)](https://crates.io/crates/openapp-sdk)
[![npm version](https://img.shields.io/npm/v/@tomers/openapp-sdk)](https://www.npmjs.com/package/@tomers/openapp-sdk)

**Physical Security as a Service (PSaaS)** — official **Python**, **Go**, and **Rust** sources plus the **OpenAPI** specification for OpenApp: API-first access control for doors, gates, virtual intercom, guest invitations, policies, and audit across heterogeneous hardware (PalGate, Shelly, Waveshare, Home Assistant, MQTT, KNX, and more).

## Use cases

- Apartment and office buildings — directory, intercom, delegated resident management
- Hotels and short-term rentals — time-bound guest invitations, PMS integration
- Campus and multi-site — zones, scripting provisioning, locker/parcel matrices
- Home and parking — gates and barriers via PalGate Cloud and relays
- Automation — agents, webhooks, and MCP against the same HTTP API

## Running tests

`pip install openapp-sdk` and `go get` do **not** ship the test suite. Clone this repository (or check out a release tag) to run it locally.

All tests use **mocked HTTP fixtures**. You do not need an OpenApp deployment or API key.

### What `just test` covers

`just test` is the full verification pipeline — the same checks that run in [CI](https://github.com/tomers/openapp-sdk/actions/workflows/ci.yml):

| Part of the repo | What runs |
|------------------|-----------|
| **`rust/`** | Shared client core: formatting, static analysis, unit tests, and a check that generated bindings match **`api-spec/`** |
| **`python/`** | Unit tests, OpenAPI contract tests, and shared [Gherkin](tests/features/) scenarios (acceptance-style API behavior) |
| **`go/`** | Static analysis, build, and package tests for the Go client |

Shared scenarios live under [`tests/features/`](tests/features/). They describe API behavior once; Python and Go test runners exercise them as part of the suite.

This tree includes **Python**, **Go**, and the **Rust core** sources. The [**TypeScript**](https://www.npmjs.com/package/@tomers/openapp-sdk) package and the [**Rust crate on crates.io**](https://crates.io/crates/openapp-sdk) are installed from their registries for day-to-day use; their sources are not part of `just test` here.

### Commands

Install [`just`](https://github.com/casey/just), then from the repository root:

```bash
just test          # full suite (Rust, Python, and Go on your machine)
just test-docker   # same kinds of checks via Docker (see tests/docker/)
just test-tier0    # quick smoke (@tier0 scenarios + fast Python tests)
```

Toolchain versions, Docker setup, and HTML test reports: [`TESTING.md`](TESTING.md). Latest published results: [Allure test report](https://tomers.github.io/openapp-sdk/).

Never commit real API keys. Fixture values in the test tree are intentional fakes.

## Documentation

- Product: https://openapp.house/
- SDK docs: https://openapp.house/docs/sdk/
- Agents & automation: https://openapp.house/docs/guides/agents/overview/
- MCP server: https://openapp.house/docs/guides/agents/mcp/
- Voice control: https://openapp.house/docs/guides/voice-control/
- Access control by sector: https://openapp.house/docs/guides/access-control-architecture/access-control-model-by-sector/
- API reference: https://openapp.house/docs/api-reference/
- OpenAPI JSON: https://openapp.house/docs/api-spec/openapp-openapi.json
- AI index (llms.txt): https://openapp.house/llms.txt

| Directory | Contents |
|-----------|----------|
| `rust/` | Shared Rust core and API models |
| `python/` | Python package (`openapp-sdk` on PyPI) |
| `go/` | Go module (`go get github.com/tomers/openapp-sdk/go`) |
| `api-spec/` | OpenAPI definitions |
| `tests/features/` | Shared Gherkin scenarios |
| `tests/contracts/` | Contract-test fixtures |
| `tests/docker/` | Docker-based test runs (`just test-docker`) |
| `justfile` | Test entrypoints — see [`TESTING.md`](TESTING.md) |

**Synced:** `sdk-python-v1.0.1` · [PyPI](https://pypi.org/project/openapp-sdk/) · [Go module](https://pkg.go.dev/github.com/tomers/openapp-sdk/go) · [crates.io](https://crates.io/crates/openapp-sdk) · [npm](https://www.npmjs.com/package/@tomers/openapp-sdk) · [Test report](https://tomers.github.io/openapp-sdk/) · [Issues](https://github.com/tomers/openapp-sdk/issues)
