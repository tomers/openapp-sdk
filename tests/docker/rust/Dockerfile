# syntax=docker/dockerfile:1
# Rust + fmt/clippy for openapp-sdk (rust/ at repo root).
ARG RUST_VERSION=invalid-version-must-be-provided
ARG RUST_DEBIAN=invalid-debian-must-be-provided
FROM rust:${RUST_VERSION}-slim-${RUST_DEBIAN}

ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV RUSTUP_AUTO_INSTALL=0

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        git pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && rustup component add rustfmt clippy \
    && groupadd --gid 1000 sdk \
    && useradd --uid 1000 --gid 1000 --create-home --shell /bin/bash sdk \
    && mkdir -p /usr/local/cargo/registry /usr/local/cargo/git /usr/local/rustup \
    && chown -R sdk:sdk /usr/local/cargo /usr/local/rustup \
    && chmod -R a+rwX /usr/local/cargo /usr/local/rustup

WORKDIR /workspace
USER sdk
