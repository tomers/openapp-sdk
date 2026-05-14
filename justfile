# openapp-sdk — test entrypoints for this repository.

_compose := source_directory() / "tests/docker/compose.yaml"
_root := source_directory()

_default:
    @just --list

# CI-equivalent suite on the host (Rust, uv/Python, Go+cgo). See TESTING.md.
test:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{ _root }}"
    cd "${root}/rust"
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace
    cd "${root}/rust/crates/common"
    cargo run --features openapi-gen --bin openapp-sdk-openapi-gen -- --check
    cd "${root}/python"
    rm -rf .venv
    uv sync --all-extras
    uv run maturin develop --features pyo3/extension-module
    mkdir -p .tmp/sdk-reports/behave-junit
    uv run pytest -v tests/unit tests/contract \
      --junitxml=.tmp/sdk-reports/junit-python-ci.xml \
      --alluredir=.tmp/sdk-reports/allure-results
    uv run behave ../tests/features --tags=-wip --no-color \
      -f allure_behave.formatter:AllureFormatter -o .tmp/sdk-reports/allure-results \
      -f plain --junit --junit-directory=.tmp/sdk-reports/behave-junit
    cd "${root}/rust"
    cargo build -p openapp-sdk-core-c-bridge --release
    cd "${root}/go"
    export CGO_ENABLED=1
    export LD_LIBRARY_PATH="${root}/rust/target/release/deps${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}"
    mkdir -p .tmp/sdk-reports
    go vet ./...
    go build ./...
    go test ./... -json > .tmp/sdk-reports/go-test.jsonl

# Fast @tier0 slice (Python unit/contract + shared Gherkin).
test-tier0:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{ _root }}"
    cd "${root}/python"
    uv sync --all-extras
    uv run maturin develop --features pyo3/extension-module
    uv run pytest -q -m tier_0 tests/unit tests/contract
    uv run behave ../tests/features --tags="@tier0 and not @wip"

# Docker-backed CI slice — no host Rust/Go/Python beyond Docker + just.
test-docker: docker-core docker-python docker-go

behave:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ _root }}/python"
    uv sync --all-extras
    uv run maturin develop --features pyo3/extension-module
    uv run behave ../tests/features --tags=-wip

docker-core:
    SDK_DOCKER_UID="${SDK_DOCKER_UID:-$(id -u)}" SDK_DOCKER_GID="${SDK_DOCKER_GID:-$(id -g)}" \
      docker compose -f {{ _compose }} run --rm sdk-core \
      sh -euxc 'cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo test --workspace'

docker-python:
    SDK_DOCKER_UID="${SDK_DOCKER_UID:-$(id -u)}" SDK_DOCKER_GID="${SDK_DOCKER_GID:-$(id -g)}" \
      docker compose -f {{ _compose }} run --rm sdk-python \
      sh -euxc 'uv sync --all-extras && uv run maturin develop --features pyo3/extension-module && uv run pytest -v tests/unit tests/contract'

docker-go:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ _root }}"
    SDK_DOCKER_UID="${SDK_DOCKER_UID:-$(id -u)}" SDK_DOCKER_GID="${SDK_DOCKER_GID:-$(id -g)}" \
      docker compose -f tests/docker/compose.yaml build sdk-go
    SDK_DOCKER_UID="${SDK_DOCKER_UID:-$(id -u)}" SDK_DOCKER_GID="${SDK_DOCKER_GID:-$(id -g)}" \
      docker compose -f tests/docker/compose.yaml run --rm sdk-go \
      sh -euxc 'go vet ./... && go build ./... && go test ./...'

# Merge Allure inputs and generate HTML (after `just test`).
report-allure:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{ _root }}"
    python3 "${root}/scripts/sdk_merge_allure_results.py" \
      --layout mirror \
      --repo "${root}" \
      --out "${root}/.tmp/sdk-reports/allure-results-merged"
    # shellcheck source=/dev/null
    source "${root}/scripts/sdk-prepare-allure-cli.sh"
    allure generate "${root}/.tmp/sdk-reports/allure-results-merged" \
      -o "${root}/.tmp/sdk-reports/allure-report" --clean
    echo "Allure HTML: ${root}/.tmp/sdk-reports/allure-report/index.html"
