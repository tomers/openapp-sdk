# Task: Full test coverage for `sdk-python`

Deferred-work plan. Pick this up later; do not implement inline with unrelated
changes. Update the "Decisions" section (never delete) if scope shifts during
implementation so the final PR matches what was agreed.

## Goal

Raise `packages/sdk/python` from its current smoke-level coverage to:

1. **100% method coverage at the unit layer** — every public resource method
   has a test that asserts the exact HTTP request it constructs (method, path,
   query, headers, body).
2. **Real end-to-end scenario coverage at the integration layer** — live-backend
   tests that walk the actual resource lifecycles (CRUD + cross-resource
   composition) the way a user would drive them, not one test per method.

## Decisions (locked in alignment Q&A on 2026-04-24)

<!-- markdownlint-disable MD013 MD060 -->

| Question                    | Answer                                                                                 |
| --------------------------- | -------------------------------------------------------------------------------------- |
| Coverage shape              | **Both**: respx-mocked unit tests for every method + live end-to-end scenario suite    |
| Headless-undriveable endpoints | **Mock only** (no live coverage attempt for push, EULA-on-used-account, lan_agent CLI token handshake, multi-party public-session flows) |
| Where live tests run        | **Local only** — developer provisions a token via `just tests sdk-python provision-token`; CI stays at unit tests (current state) |

<!-- markdownlint-enable MD013 MD060 -->

## Current state

- Unit suite (`tests/unit/`): 4 files covering bridge selection, client
  construction, error mapping, token parsing. Zero resource coverage.
- Integration suite (`tests/integration/test_live_api.py`): 3 smoke tests —
  `status.get`, `auth.whoami`, `orgs.list`. Skipped unless
  `OPENAPP_SDK_TEST_API_KEY` is set.
- CI runs unit only; live suite runs on developer machines against the dev env.

## Inventory: everything that must be covered

### Resource methods (113 total across 15 clients)

Count is exhaustive — every `async def` on a public `*Client`. Names are
`<client>.<method>`.

<!-- markdownlint-disable MD013 MD060 -->

| Client                 | Count | Methods                                                                                                                                                                                                                                                |
| ---------------------- | ----: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `apartment_residents`  |     3 | `list`, `add`, `remove`                                                                                                                                                                                                                                |
| `api_keys`             |     6 | `list`, `create`, `update`, `revoke`, `restore`, `purge`                                                                                                                                                                                               |
| `auth`                 |     6 | `whoami`, `session`, `kratos_identity`, `provisioned`, `logout`, `sign_out`                                                                                                                                                                            |
| `devices`              |    10 | `list`, `create`, `get`, `update`, `delete`, `purge`, `restore`, `door_restrictions`, `set_door_restrictions`, `metadata_definition`                                                                                                                   |
| `dsl`                  |     1 | `execute`                                                                                                                                                                                                                                              |
| `entities`             |    13 | `list`, `create`, `get`, `update`, `patch`, `delete`, `purge`, `restore`, `actions`, `metadata_definition`, `by_device`, `device_entities_metadata_definition`, `apartment_floors`                                                                     |
| `eula`                 |     2 | `get`, `accept`                                                                                                                                                                                                                                        |
| `integrations`         |    21 | `list`, `create`, `provider_types`, `provider_definition`, `get`, `update`, `purge`, `restore`, `device_metadata_schema`, `discovered_devices`, `entities`, `ops`, `run_op`, `list_access_portals`, `create_access_portal`, `get_access_portal`, `update_access_portal`, `delete_access_portal`, `update_access_invite`, `delete_access_invite`, `restore_access_invite` |
| `lan_agent`            |     6 | `meta`, `bootstrap_script`, `bootstrap_token`, `token`, `submit_task_spec`, `list_tasks`                                                                                                                                                               |
| `me`                   |     5 | `apartments`, `invitations`, `push_subscription_status`, `push_vapid_public_key`, `subscribe_push`                                                                                                                                                     |
| `orgs`                 |     8 | `list`, `create`, `get`, `update`, `delete`, `purge`, `permissions`, `users`                                                                                                                                                                           |
| `public_access`        |    17 | `get_invite`, `claim_invite`, `execute_invite`, `start_invite_session`, `get_portal`, `portal_lights`, `portal_open`, `portal_reachable`, `portal_start_session`, `portal_targets`, `get_session`, `cancel_session`, `decline_session`, `session_lights`, `session_notify_message`, `session_open`, `session_streams` |
| `status`               |     1 | `get`                                                                                                                                                                                                                                                  |
| `users`                |     8 | `create`, `search`, `get`, `update`, `delete`, `purge`, `add_roles`, `remove_roles`                                                                                                                                                                    |
| `zones`                |     6 | `create`, `get`, `update`, `delete`, `purge`, `by_integration`                                                                                                                                                                                         |

<!-- markdownlint-enable MD013 MD060 -->

### Non-resource code paths

Already partially covered by existing unit tests — gaps to close:

- `client.AsyncClient.__init__` / `connect` / `close` / `__aenter__` /
  `__aexit__` / `_request` — happy path is covered, but retry/interceptor
  composition and `close()` idempotency are not.
- `client.Client` (sync wrapper) — `_wrap` / `_run` / context-manager entry-exit
  cycle not exercised.
- `client._serialize_body` — dataclass branch, pydantic branch (via
  `contrib.pydantic.dump`), `None` branch, raw-bytes passthrough.
- `client._json_default` — dataclass, enum, datetime, UUID, decimal.
- `interceptor._compose_request` + `Interceptor` protocol conformance.
- `bridge.python_fallback._decode` / `_extract_message` / `_exp_backoff` /
  `_sleep_for_retry` — retry-after header honoured, non-JSON error path.
- `bridge.rust.RustBridge` — only reachable when the native extension is built;
  already has one selection test. Add a request round-trip test that runs only
  when `OPENAPP_SDK_BRIDGE=rust` is available.
- `contrib.pydantic.parse_as` / `dump` — happy path + validation-error mapping
  to `SerializationError`.
- `errors.from_bridge_payload` — every discriminator branch (api, http, auth,
  transport, validation, config, serialization).
- `token.ApiKey.parse` — happy path is covered; add malformed-token cases
  (missing scheme, missing secret, embedded whitespace).

## Architecture

Two tiers, strictly separated. No cross-contamination of mocks and live calls.

### Tier 1 — `tests/unit/resources/` (new directory)

- One `test_<client>.py` per resource file, mirroring the source layout 1:1.
- Each file uses `respx` to intercept the bridge's `httpx.AsyncClient` and
  asserts the exact request shape. Responses are minimal fixtures (just
  enough to let the SDK deserialize).
- Every method gets at least: `happy_path`, `passes_query_params` (if
  applicable), `passes_body` (if applicable), and an `api_error_maps` test
  (server returns `4xx` JSON error → SDK raises `ApiError` with matching fields).
- Shared fixtures go in `tests/unit/resources/conftest.py`:
  - `mocked_client` — an `AsyncClient` wired to the python bridge with `respx`
    already active and a baked-in fake API token.
  - `respx_mock` scoped per-test (function scope) for isolation.
- Runs in CI on every PR. Required to stay green.

### Tier 2 — `tests/integration/scenarios/` (new directory)

- Scenario-oriented, not method-oriented. Each file walks a real lifecycle end
  to end, calling the SDK exactly as a user would:
  - `test_org_lifecycle.py` — create org → list → get → update → users → delete → purge.
  - `test_integration_lifecycle.py` — create integration → provider_types →
    provider_definition → device_metadata_schema → list → get → update →
    discovered_devices → entities → ops → run_op → restore → purge.
  - `test_device_entity_lifecycle.py` — provision integration → create device →
    list → get → update → door_restrictions → set_door_restrictions →
    metadata_definition → create entity → patch → actions → by_device →
    delete entity → delete device → purge both.
  - `test_zone_lifecycle.py` — create zone → get → update → by_integration →
    delete → purge.
  - `test_api_key_lifecycle.py` — create → list → update → revoke → restore →
    purge (isolated; does not share state with other scenarios).
  - `test_auth_readonly.py` — whoami, session, kratos_identity, provisioned,
    and logout-in-teardown (logout invalidates the fixture token, so must run
    last of any suite that shares it).
  - `test_public_access_portal.py` — the parts that are drivable without a peer:
    provision integration+portal → `get_portal`, `portal_targets`,
    `portal_reachable`. The multi-party session operations are explicitly
    out of this scenario; they live in unit-only coverage (see below).
  - `test_apartment_residents.py` — integration requires apartment-capable
    provider; creates apartment-enabled integration → list → add → remove.
  - `test_me_and_eula.py` — read-only `me.apartments` / `me.invitations` +
    `eula.get`. `eula.accept` runs only when a throwaway user fixture is
    available (see Scaffolding below); otherwise skipped with a clear reason.
  - `test_status_smoke.py` — retains the existing `status.get` smoke.
- Fixtures chain so downstream scenarios inherit upstream state via a
  `session`-scoped provisioning fixture. Each scenario's top-level fixture
  cleans up every resource it created (LIFO) in a `finally`/`try`/`async with`
  context. A `@pytest.mark.destructive` marker tags scenarios that mutate
  shared state; these opt in explicitly via `pytest -m destructive`.
- Runs locally via `just tests sdk-python test`; never in CI (per decision).

### Mock-only endpoints (no live coverage — explicitly deferred)

These need unit coverage but are out of the live suite:

- `me.subscribe_push`, `me.push_subscription_status`, `me.push_vapid_public_key`
  — require a browser push subscription.
- `eula.accept` against the canonical API-key user — would permanently mutate
  that account's EULA state. Unit-test only unless a throwaway-user fixture
  lands.
- `lan_agent.bootstrap_script`, `bootstrap_token`, `token`, `submit_task_spec`,
  `list_tasks` — require a running LAN agent. Unit-test only.
- `public_access.session_*`, `portal_open`, `portal_lights`,
  `portal_start_session`, `claim_invite`, `execute_invite`,
  `start_invite_session` — multi-party flows. Unit-test only.

Each entry above gets a `# pragma: unit-only` comment in the integration file
pointing back here so the exclusion is discoverable at read time.

## Scaffolding to build first

Build these before writing any tests. They're the expensive part; getting
them right up front makes every subsequent test cheap.

### Unit tier

1. `tests/unit/resources/conftest.py` with `mocked_client` fixture and a
   `json_response` helper that stamps `Content-Type: application/json` and
   handles the backend's error envelope shape.
2. A `parametrize`-driven test generator helper for the "assert method-path-body
   shape" tests, so each resource file stays declarative:

   ```python
   _CASES = [
       Case("list",   "GET",    "/orgs",              query={"limit": "10"}),
       Case("create", "POST",   "/orgs",              body={"name": "foo"}),
       Case("get",    "GET",    "/orgs/{id}"),
       Case("update", "PATCH",  "/orgs/{id}",         body={"name": "bar"}),
       Case("delete", "DELETE", "/orgs/{id}"),
       Case("purge",  "DELETE", "/orgs/{id}?purge=1"),
   ]
   ```

   Avoids ~5× boilerplate across 113 tests.

### Integration tier

1. `tests/integration/scenarios/conftest.py` with:
   - `session_client` — module-scoped `AsyncClient` backed by the cached
     `OPENAPP_SDK_TEST_API_KEY`.
   - `created_org` — session-scoped fixture that provisions an org, yields its
     id, teardown purges it.
   - `created_integration(provider_type)` — parametrizable factory for
     integrations with per-provider cleanup.
   - `created_device(integration_id, ...)`, `created_entity(...)`, etc.
   - `cleanup_stack` — an `contextlib.AsyncExitStack` exposed to each test so
     ad-hoc resources register themselves for LIFO teardown without plumbing
     `finally` blocks everywhere.
2. A `just tests sdk-python test-scenario <name>` recipe for running a single
   scenario in isolation during development (the existing `test` recipe stays
   the "run everything" entry point).

### Coverage reporting

- Add `coverage` + `pytest-cov` to `[project.optional-dependencies].dev`.
- New `just sdk-python coverage` recipe that runs `pytest --cov=openapp_sdk
  --cov-report=term-missing --cov-fail-under=95 tests/unit`. 95% floor, not
  100%, to leave room for the multi-party / push branches that are legitimately
  not unit-testable.
- `tests/integration/` is **excluded** from the coverage floor — live coverage
  varies by what's provisionable in the dev env that day.

## Constraints & gotchas (discovered during scoping)

- `users.delete` followed by `users.purge` on the API-key's own user will
  invalidate the token mid-suite. The API-key-owning user must never be a
  target; scenarios create throwaway users for user-CRUD coverage.
- `auth.logout` / `auth.sign_out` invalidate the API key for the remainder of
  the session. They must be the final assertion in any scenario file that
  touches them, and a fresh session fixture must be used if later tests
  expect to authenticate.
- `integrations.create` requires a valid `provider_type`. The test suite must
  pick a provider that's enabled in the dev env (read from
  `integrations.provider_types` at fixture setup, not hard-coded).
- Many resources use `purge` for hard-delete and `delete` for soft-delete.
  Cleanup uses `purge` to keep the dev env clean across runs.
- The dev-env token issued by `provision-api-key` is org-scoped. Cross-org
  operations (listing all orgs, creating orgs) need a platform-role token;
  tests that need it get their own `platform_client` fixture that falls back to
  `pytest.skip` when the token doesn't have the role.

## Implementation plan (when picked up)

1. **Scaffolding PR**
   - `tests/unit/resources/conftest.py` with `mocked_client`, `Case`, and the
     parametrize generator.
   - `tests/integration/scenarios/conftest.py` with `session_client` and the
     core resource-factory fixtures.
   - `just sdk-python coverage` recipe + `pytest-cov` dev dep.
   - No actual test files yet. Land green, get review, merge.

2. **Unit coverage — resources, one PR per ~3 clients**
   - `api_keys`, `auth`, `status`, `dsl` (easiest; mostly read-only).
   - `orgs`, `users`, `me`, `eula`.
   - `devices`, `entities`, `zones`, `apartment_residents`.
   - `integrations`, `lan_agent`, `public_access`.
   - Each PR adds resource tests + hits 95% on the files it touches.

3. **Unit coverage — non-resource gaps**
   - `client._serialize_body` / `_json_default` / sync wrapper.
   - `interceptor._compose_request`.
   - `bridge.python_fallback._decode` / `_exp_backoff` / `_sleep_for_retry`.
   - `errors.from_bridge_payload` branches.
   - `contrib.pydantic` round-trips.
   - One PR.

4. **Integration scenario PR**
   - All scenario files in `tests/integration/scenarios/`.
   - `just tests sdk-python test-scenario` recipe.
   - CI untouched.

5. **Cleanup PR**
   - Remove the old `tests/integration/test_live_api.py` in favour of the
     scenarios (`test_status_smoke.py` replaces it).
   - Update `README.md` testing section.

## Out of scope

- Performance / load testing.
- Fuzzing.
- Type-stubs validation beyond what `mypy` already does.
- The `progenitor`-replacement spike for removing the 3.1 → 3.0 down-converter
  (tracked separately).
- Running live tests in CI — deliberately deferred per decision.
- A 3.2.0 spec bump (also deliberately deferred; see
  `packages/sdk/core/crates/common/src/bin/openapi_gen.rs` header for rationale).

## Acceptance criteria

- [ ] `just sdk-python coverage` reports ≥95% line coverage over
      `openapp_sdk/` from `tests/unit/` alone.
- [ ] Every one of the 113 resource methods has at least one unit test that
      asserts its HTTP request shape.
- [ ] `just tests sdk-python test` (with the cached token) runs the scenario
      suite to green against a running dev env and leaves it with no stray
      resources (verifiable via `just backend cli list-orgs` / equivalent).
- [ ] CI `lint + test` time does not regress by more than 30 s.
- [ ] All new files pass `ruff`, `ruff format`, and `mypy` in strict mode.

## Estimated effort

Rough order-of-magnitude only, not a commitment:

- Scaffolding PR: 2 – 3 h.
- Unit coverage PRs (4 batches): 2 – 3 h each = 8 – 12 h total.
- Integration scenarios PR: 4 – 6 h (most time sunk in cleanup-ordering bugs).
- Cleanup PR: 30 min.

Total: **15 – 22 focused hours** spread across 6 PRs. The unit tier alone
(≥95% coverage without live tests) is 10 – 15 h and can be shipped independently
if the live suite is deferred further.
