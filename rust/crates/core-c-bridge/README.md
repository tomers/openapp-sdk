# openapp-sdk-core-c-bridge

C ABI bridge over `openapp-sdk-core` for non-Rust consumers.

- Exposes stable C-callable functions for runtime/client lifecycle
- Allows other language SDKs to reuse the Rust core behavior
- Keeps transport/auth/retry/error semantics aligned across bindings

This crate is intended for FFI consumers and companion language adapters.
