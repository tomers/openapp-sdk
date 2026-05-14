# syntax=docker/dockerfile:1
# sdk-go: Go toolchain + openapp-sdk-core-c-bridge for cgo tests in go/.

FROM rust:1.85-bookworm AS bridge-build
WORKDIR /build
COPY rust /build/core
WORKDIR /build/core
RUN cargo build -p openapp-sdk-core-c-bridge --release

FROM golang:1.26.2-trixie

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        git \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 1000 sdk \
    && useradd --uid 1000 --gid 1000 --create-home --shell /bin/bash sdk

COPY --from=bridge-build /build/core/target/release/deps/libopenapp_sdk_core_c_bridge.so /usr/local/lib/
RUN ldconfig

ENV CGO_ENABLED=1
ENV LD_LIBRARY_PATH=/usr/local/lib

WORKDIR /workspace
USER sdk
