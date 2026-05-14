<!-- markdownlint-disable MD013 MD060 -->
# SDK verification

Cross-language tests for the OpenApp SDKs in this repository.

| Location | Contents |
|----------|----------|
| [`../python/tests/`](../python/tests/) | Python unit and contract tests (mocked; no API key required) |
| [`../go/test/`](../go/test/) | Go OpenAPI client tests |
| [`../go/gherkin_tier1_test.go`](../go/gherkin_tier1_test.go) | Go Cucumber runner over shared Gherkin |
| [`../rust/`](../rust/) | Rust core workspace (`cargo test --workspace`) |
| [`features/`](features/) | Shared `.feature` scenarios (Behave / godog) |
| [`contracts/`](contracts/) | Pact consumer fixtures |
| [`docker/`](docker/) | Optional Docker toolchains when host Rust/Go/Python are not installed |

## Quick start

From the repository root:

```bash
# Host toolchains (Rust, uv, Go 1.26+, maturin) — same slice as CI
just test

# Docker only (no host SDK toolchains beyond Docker + just)
just test-docker
```

See [`TESTING.md`](../TESTING.md) for prerequisites, tier-0 smoke, and report artifacts.
