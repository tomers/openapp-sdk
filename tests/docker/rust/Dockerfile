# syntax=docker/dockerfile:1
# Rust + fmt/clippy for openapp-sdk (rust/ at repo root).
FROM rust:slim-trixie

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        git pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && rustup component add rustfmt clippy \
    && groupadd --gid 1000 sdk \
    && useradd --uid 1000 --gid 1000 --create-home --shell /bin/bash sdk \
    && mkdir -p /usr/local/cargo/registry /usr/local/cargo/git \
    && chown -R sdk:sdk /usr/local/cargo \
    && chmod -R a+rwX /usr/local/cargo

ENV CARGO_HOME=/usr/local/cargo

WORKDIR /workspace
USER sdk
