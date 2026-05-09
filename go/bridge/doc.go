// Package bridge provides Go bindings to [openapp-sdk-core] via the C ABI in
// openapp-sdk-core-c-bridge (Workstream E). With CGO enabled, this is the supported
// transport path: JSON ([Client.JSONRequest]), raw multipart / custom Content-Type
// ([Client.JSONRequestRaw]), asynchronous JSON ([Client.JSONRequestAsync]), and byte
// streams with optional HTTP status callback ([Client.StreamRequest],
// [Client.StreamRequestWithHTTPCallback]).
//
// The sdk-go Docker image installs libopenapp_sdk_core_c_bridge.so under /usr/local/lib.
// Run all Go commands via Docker (`just sdk go test`, `just sdk go pre-commit`, or
// `docker compose -f packages/sdk/docker/compose.yaml run --rm sdk-go …`) — do not rely
// on a host `go` binary for SDK work.
//
// [openapp-sdk-core]: https://github.com/tomers/openapp-sdk/tree/main/rust (public mirror;
// private monorepo path: packages/sdk/core).
package bridge
