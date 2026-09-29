# openapp-sdk-core

Rust workspace for the shared OpenApp SDK core: OpenAPI-generated models, the
high-level HTTP client, and the C bridge used by the Python and Go packages in
this repository.

| Crate | Purpose |
| --- | --- |
| `openapp-sdk-common` | Models and low-level async HTTP client |
| `openapp-sdk-core` | `ClientBuilder`, auth, retries, pagination |
| `openapp-sdk-core-c-bridge` | C ABI for language bindings |

See [`ARCHITECTURE.md`](./ARCHITECTURE.md) for design notes.

## Building and testing

From the repository root:

```sh
just test       # full suite — see TESTING.md
just test-tier0 # fast smoke
```

With Rust installed locally:

```sh
cd rust
cargo build --workspace
cargo test --workspace
```

## Documentation

- [SDK overview](https://openapp.house/docs/sdk/)
- [API reference](https://openapp.house/docs/api-reference/)
- [OpenAPI JSON](https://openapp.house/docs/api-spec/openapp-openapi.json)
