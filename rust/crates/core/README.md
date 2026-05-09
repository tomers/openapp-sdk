# openapp-sdk-core

Core runtime crate for OpenApp SDK behavior shared across language bindings.

- Client construction and request execution flow
- Authentication wiring and retry policy
- Resource-oriented API surface used by higher-level SDK packages

This crate centralizes behavior so language SDKs stay thin wrappers around one
implementation path.
