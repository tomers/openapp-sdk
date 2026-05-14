<!-- markdownlint-disable MD013 -->
# Gherkin features (golden scenarios)

This directory holds **shared** `.feature` files that describe behavior every language SDK must satisfy.
Language runners attach step definitions in their own packages (for example Behave for Python, Cucumber for Node,
`cucumber-rs` for Rust).

## Conventions

- Prefer **stable scenario titles** so CI dashboards stay comparable across languages.
- **`@tier0`** — PR-critical smoke; `scripts/check_gherkin_tier0.py` enforces a minimum scenario count across `@tier0` features.
- **`@tier1`** — broader regression stories (auth errors, invitation packs, CRUD reads); `scripts/check_gherkin_story_packs.py` enforces a minimum count across `*_story_pack.feature` files.
- **Stable title guardrail** — `scripts/check_gherkin_stable_titles.py` enforces globally unique `Scenario:` titles across shared feature files.
- **`@smoke`** — subset used for fast cross-SDK selection in runners that support tag filters.
- **`@mock_api`** — Python fake bridge (`environment.py`); Node loopback HTTP server (`gherkin/tier1_steps.mjs`, so requests still pass through the Rust core transport); Rust **wiremock** (`tests/gherkin_tier1.rs`); Go `httptest` through the core-backed `RoundTripper` (`gherkin_tier1_test.go`). Same `.feature` files; transport differs per language.
- Keep steps **API- or user-outcome-shaped**, not binding implementation details.

## Status

### Tier 0

- [`smoke_example.feature`](smoke_example.feature) — import / version sanity.
- [`http_api_smoke.feature`](http_api_smoke.feature) — mocked status + org create.
- [`crud_smoke.feature`](crud_smoke.feature) — mocked entity action (CRUD-shaped).
- [`invitations_smoke.feature`](invitations_smoke.feature) — mocked `GET /me/invitations`.

### Story packs (`@tier1`, `*_story_pack.feature`)

- [`auth_errors_story_pack.feature`](auth_errors_story_pack.feature) — `AuthError` / `ApiError` mapping.
- [`invitations_story_pack.feature`](invitations_story_pack.feature) — empty and mixed invitation lists, public invite get + claim.
- [`crud_story_pack.feature`](crud_story_pack.feature) — org get, device list envelope, org list envelope.

Planned extensions and non-automated gaps: [`../docs/GOLDEN_TESTING_BACKLOG.md`](../docs/GOLDEN_TESTING_BACKLOG.md).

### Run shared features locally

- **Python** (`packages/sdk/python`): `uv run behave ../features --tags=-wip` (or `just sdk behave-python`).
- **Node** (`packages/sdk/node`): `npm run test` (build + Node smoke + **cucumber-js** on story packs + `smoke_example.feature`), or `npm run gherkin` alone; Docker: `just sdk docker node` then `npm ci && npm test`.
- **Rust** (`packages/sdk/rust`): `cargo test --test gherkin_tier1` (or `cargo test`; includes smoke + tier-1 cucumber harness).
