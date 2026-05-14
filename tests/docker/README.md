<!-- markdownlint-disable MD013 MD060 -->
# SDK Docker toolchains

Run **Rust core**, **Python**, and **Go** SDK checks inside containers when you do not have every language installed on the host. CI on GitHub Actions uses host toolchains; Docker is for **local** parity with a minimal install (Docker + [just](https://github.com/casey/just)).

## Requirements

- Docker with Compose v2 (`docker compose`)
- Clone of this repository (tests are not shipped inside PyPI wheels)

## Layout

| Path | Role |
|------|------|
| [`compose.yaml`](compose.yaml) | Services `sdk-core`, `sdk-python`, `sdk-go` |
| [`rust/Dockerfile`](rust/Dockerfile) | Rust stable + rustfmt/clippy |
| [`python/Dockerfile`](python/Dockerfile) | Python 3.14 + uv + Rust (maturin) |
| [`go/Dockerfile`](go/Dockerfile) | Multi-stage: builds C bridge from `rust/`, copies `.so` into Go image |

Each service mounts the **repository root** at `/workspace`.

## Commands

From the **repository root**:

```bash
just docker core       # cargo fmt + clippy + test --workspace
just docker python     # uv sync + maturin develop + pytest unit
just docker go         # rebuild sdk-go image + vet + build + test
just test-docker       # core + python + go (CI-equivalent Docker slice)
```

Raw Compose:

```bash
SDK_DOCKER_UID="$(id -u)" SDK_DOCKER_GID="$(id -g)" \
  docker compose -f tests/docker/compose.yaml run --rm sdk-python \
  sh -euxc 'uv sync --all-extras && uv run maturin develop --features pyo3/extension-module && uv run pytest -q tests/unit'
```

See [`../TESTING.md`](../../TESTING.md) for the full command matrix.
