<!-- markdownlint-disable MD013 MD060 -->
# Running SDK tests

Tests live in each language package plus shared assets under [`tests/`](tests/). Everything in this repository runs against **mocked** HTTP fixtures — you do not need an OpenApp deployment or API key for `just test` or CI.

## Prerequisites

### `just test` (host toolchains)

- **Rust** stable (`rustfmt`, `clippy`)
- **uv** + **Python 3.12+**
- **Go 1.26+** with **CGO** enabled
- Build the C bridge once before Go tests: `cargo build -p openapp-sdk-core-c-bridge --release` in `rust/`

### `just test-docker` (recommended without local SDK stacks)

- Docker with Compose v2 (`docker compose`)
- **just** ([install](https://github.com/casey/just))

## Commands

| Command | What it runs |
|---------|----------------|
| `just test` | Rust core fmt/clippy/test, OpenAPI drift check, Python unit+contract+Behave, Go vet/build/test |
| `just test-tier0` | Fast `@tier0` Gherkin + Python tier-0 unit/contract slice |
| `just test-docker` | Same CI slice inside `tests/docker/` containers |
| `just docker core` | Rust workspace only (container) |
| `just docker python` | Python unit tests (container) |
| `just docker go` | Go OpenAPI drift + vet/build/test (container; builds C bridge image) |
| `just behave` | Full shared Gherkin via Behave (`tests/features/`) |
| `just report-allure` | Merge pytest/Behave Allure inputs and open HTML (needs `just test` first) |

## Layout

```text
openapp-sdk/
  rust/              # Rust core workspace (includes unit tests)
  python/tests/      # pytest + contract tests
  go/test/           # generated client tests
  tests/
    features/        # shared Gherkin (.feature)
    contracts/       # Pact fixtures
    docker/          # Compose services sdk-core, sdk-python, sdk-go
  scripts/           # Allure merge + CLI bootstrap (used by CI)
  justfile           # entrypoint for the commands above
```

Published wheels on PyPI (`pip install openapp-sdk`) do **not** include this tree; clone this repo or check out a release tag to run the verification suite.

**Credentials:** Fixture tokens such as `TEST_SECRET` and `testsecret` are intentional test doubles. Do not commit real API keys. If you experiment against your own deployment, pass a key via environment variables locally and keep it out of git.

## CI and reports

GitHub Actions workflow [`.github/workflows/ci.yml`](.github/workflows/ci.yml) runs the same host-toolchain slice as `just test` on every push to `main`. Merged Allure HTML is published to [tomers.github.io/openapp-sdk](https://tomers.github.io/openapp-sdk/).
