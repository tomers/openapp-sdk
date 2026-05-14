# syntax=docker/dockerfile:1
# uv + Python + Rust (maturin/PyO3) for python/ without a host toolchain.
FROM python:3.14.4-slim-trixie

ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV UV_INSTALL_DIR=/usr/local/bin
ENV PATH="/usr/local/cargo/bin:/usr/local/bin:${PATH}"

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        curl git pkg-config libssl-dev build-essential ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable \
    && curl -LsSf https://astral.sh/uv/install.sh | sh \
    && mkdir -p /workspace/python/.venv /usr/local/cargo/registry /usr/local/cargo/git \
    && rm -rf /usr/local/rustup/toolchains/*/share/doc /usr/local/rustup/tmp/* \
    && groupadd --gid 1000 sdk \
    && useradd --uid 1000 --gid 1000 --create-home --shell /bin/bash sdk \
    && chown -R sdk:sdk /usr/local/cargo /usr/local/rustup \
    && chmod -R a+rwX /usr/local/cargo /workspace/python/.venv

WORKDIR /workspace
USER sdk
