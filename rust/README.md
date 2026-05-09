# openapp-sdk-core

Rust workspace that hosts the language-agnostic core of the OpenApp SDK.

Published sources for each release also appear under
[`tomers/openapp-sdk`](https://github.com/tomers/openapp-sdk) in `rust/` (see
`packages/sdk/python/RELEASING.md`).

Every OpenApp SDK (Python today; .NET / Ruby / Go / TypeScript in the future) sits on
top of the crates in this workspace:

<!-- markdownlint-disable MD013 MD060 -->

| Crate                             | Purpose                                                                                 |
|-----------------------------------|-----------------------------------------------------------------------------------------|
| `openapp-sdk-common`              | Strongly-typed models + low-level async HTTP client generated from `packages/api-spec/openapi.json` via [`progenitor`](https://github.com/oxidecomputer/progenitor). |
| `openapp-sdk-core`                | High-level client: `ClientBuilder`, per-tag sub-clients, `TokenProvider`, retries, pagination, telemetry. |
| `openapp-sdk-core-c-bridge`       | Minimal C ABI wrapping `openapp-sdk-core` so languages without a native Rust binding can talk to it. |

<!-- markdownlint-enable MD013 MD060 -->

See [`ARCHITECTURE.md`](./ARCHITECTURE.md) for the design rationale and the invariants
we enforce (single wire contract, thin language SDKs, pluggable auth).

## Building and testing

Default **`just`** recipes (`lint`, `build`, `test`, `openapi-gen`, `pre-commit`, …)
run inside **Docker** (`sdk-core`) so a host Rust toolchain is optional — same as
`just sdk docker core` from the repo root. For local iteration with **`cargo`**
installed, use **`build-host`**, **`test-host`**, **`lint-host`**, and friends (see
`just --justfile packages/sdk/core/justfile --list`).

```sh
cd packages/sdk/core
just lint
just build
just test
```

```sh
# Host Rust (optional)
cd packages/sdk/core
cargo build --workspace
cargo test --workspace
```

## Regenerating the OpenAPI-derived client

The generated client lives in `crates/common/src/generated.rs` and is produced from
`packages/api-spec/openapi.json` at build time. To refresh the committed copy:

```sh
just sdk::core::openapi-gen
```

Drift is enforced in pre-commit via `just sdk::core::openapi-check`.
