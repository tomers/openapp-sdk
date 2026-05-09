# Releasing `openapp-sdk`

This document covers the one-time bootstrap for publishing the Python SDK and
the day-to-day release process.

Public SDK sources are mirrored to [`tomers/openapp-sdk`](https://github.com/tomers/openapp-sdk)
(**Option B**: one open-source monorepo for Rust core, Python package, Go module,
and API spec — `rust/`, `python/`, `go/`, `api-spec/`).

## One-time bootstrap

Do these steps exactly once, before the first release. They require admin
access to both GitHub and PyPI.

### 1. Create the public SDK monorepo

We mirror OSS-facing subtrees so users can browse code, subscribe to releases,
and file issues without access to the private product monorepo.

1. Sign in to GitHub as `tomers` (or the owner account).
2. Create a new **public** repo named `openapp-sdk`:
   * Description: *Open-source OpenApp client SDKs (Rust core, Python, Go, API spec).*
   * Homepage: `https://pypi.org/project/openapp-sdk/` (Python); future SDKs can
     extend this.
   * No `.gitignore` or README — the mirror workflow seeds `README.md`.
   * License: `MIT` (matches `packages/sdk/python/pyproject.toml` and Rust crates).
3. Under **Settings → General → Features**, enable *Issues* and disable
   *Wikis* and *Projects* (optional, avoids confusion).
4. Create a GitHub App for SDK release automation:
   * **Settings → Developer settings → GitHub Apps → New GitHub App**
   * Name suggestion: `openapp-sdk-release-bot`
   * Homepage URL: `https://github.com/tomers/openapp`
   * Repository permissions:
     * `Contents`: **Read and write**
     * `Metadata`: **Read**
   * Webhooks: disabled (not needed for release mirroring)
5. Install the app on both repos used by release automation:
   * App page → **Install App**
   * Account: `tomers`
   * Repository access: **Only select repositories**
   * Select:
     * `openapp` (auto-release workflow pushes version-bump commit + tag)
     * `openapp-sdk` (mirror workflow pushes mirrored source/tag)
6. Generate app credentials and save them in the **private** `tomers/openapp`
   repo under `Settings → Secrets and variables → Actions`:
   * **Release flow** (tag/version pushes on `openapp`):
     **`SDK_PYTHON_RELEASE_APP_CLIENT_ID`** (OAuth **Client ID** from the app’s
     *About* page) and **`SDK_PYTHON_RELEASE_APP_PRIVATE_KEY`** (PEM).
   * **Mirror flow** (push to `openapp-sdk`):
     **`OPENAPP_SDK_MIRROR_APP_CLIENT_ID`** and
     **`OPENAPP_SDK_MIRROR_APP_PRIVATE_KEY`**.
   * A single app can serve both flows; if so, use the same app’s Client ID and
     private key value for both secret pairs.
7. The auto-release workflow mints a short-lived installation token via
   `actions/create-github-app-token` and uses it to push the version-bump
   commit + `sdk-python-v*` tag from `openapp`.
8. The auto-release workflow then dispatches `openapp-sdk-release.yml`
   (`workflow_dispatch`) with the created tag as input. This avoids relying on
   implicit tag-trigger fanout from bot-generated pushes.
9. The release workflow mints a short-lived installation token via
   `actions/create-github-app-token` and passes it to
   `scripts/publish-openapp-sdk-mirror.sh` as `MIRROR_TOKEN`.

**Migrating from `openapp-sdk-python`:** If you previously created
[`tomers/openapp-sdk-python`](https://github.com/tomers/openapp-sdk-python),
archive it and add a short README pointing to `tomers/openapp-sdk`. Install the
mirror GitHub App on **`openapp-sdk`** (same app can serve multiple repos).
No installation-ID secret is required in Actions: `create-github-app-token`
mints a token from the app Client ID + private key; ensure the app has
**Contents: Write** on `openapp-sdk`.

### 2. Reserve the PyPI name and configure trusted publishing

We use [PyPI Trusted Publishing (OIDC)](https://docs.pypi.org/trusted-publishers/)
so no long-lived API token is stored in GitHub.

1. Sign in to [PyPI](https://pypi.org/) and — if you do not already own it —
   [request the `openapp-sdk` project name](https://pypi.org/help/#project-name-claim).
2. Go to **Your projects → openapp-sdk → Publishing** (or *Your account →
   Publishing → Add a new pending publisher* if you have not uploaded a first
   version yet).
3. Add a **pending publisher** with:
   * PyPI project name: `openapp-sdk`
   * Owner: `tomers`
   * Repository name: `openapp` *(the **private** monorepo, because that is
     where `openapp-sdk-release.yml` lives and where the tag is pushed)*
   * Workflow filename: `openapp-sdk-release.yml`
   * Environment name: `pypi`
4. In the private repo, create the matching environment:
   * `Settings → Environments → New environment → pypi`
   * No required reviewers (OIDC trust is sufficient), but you may add
     manual approval as defense in depth.
5. (Recommended) Repeat on [Test PyPI](https://test.pypi.org/) with
   environment name `testpypi` if you want to do dry runs — add a corresponding
   job in the workflow that only runs on pre-release tags.

If Trusted Publishing was already configured with the old filename
(`sdk-python-release.yml`), edit the publisher on PyPI and set **Workflow
filename** to **`openapp-sdk-release.yml`** so OIDC keeps matching.

### 3. Reserve the docs subdomain

The Python SDK docs live under `https://docs.openapp.house/sdk/python`. The
docs content is generated from `apps/docs` (Starlight); no DNS changes needed.

## Releasing a new version (push-based, no manual tagging)

`openapp-sdk` releases are automated from pushes to `main`.

### How automation works

1. `sdk-python auto release` (`.github/workflows/sdk-python-auto-release.yml`)
   triggers on pushes to `main` that touch:
   * `packages/sdk/python/**`
   * `packages/sdk/core/**`
   * `packages/api-spec/openapi.json`
2. The workflow determines semantic bump level from commit messages since the
   previous `sdk-python-v*` tag:
   * **major**: commit subject/body contains `BREAKING`, `BREAKING:`, or
     `BREAKING CHANGE:`
   * **minor**: commit subject starts with `feat(...)` / `feat:`
   * **patch**: commit subject starts with `fix(...)` / `fix:`
   * **default**: patch when no marker is found
3. The workflow (not a human) bumps version and commits it on `main` in:
   * `packages/sdk/python/pyproject.toml`
   * `packages/sdk/python/python/openapp_sdk/_version.py`
4. The workflow creates and pushes tag `sdk-python-vX.Y.Z`.
5. The auto-generated release commit (`chore(sdk-python): release vX.Y.Z`) is
   intentionally ignored by `sdk-python auto release` to avoid release loops.
6. The workflow dispatches **`openapp-sdk release`**
   (`.github/workflows/openapp-sdk-release.yml`) with `release_tag=sdk-python-vX.Y.Z`.
7. **`openapp-sdk release`** checks out that exact tag and:
   1. Builds wheels for Linux, macOS (x86_64 + arm64), and Windows.
   2. Builds the sdist.
   3. Publishes everything to PyPI via OIDC.
   4. Mirrors `packages/sdk/core` → `rust/`, `packages/sdk/python` → `python/`,
      `packages/sdk/go` → `go/`, `packages/api-spec` → `api-spec/` on
      `tomers/openapp-sdk` and pushes the tag on the mirror.

### Authoring commit messages for release level

Use Conventional Commits and include a breaking marker for major releases:

* Patch examples:
  * `fix(sdk-python): normalize retry backoff`
  * `chore(sdk-python): refresh generated models` (falls back to patch)
* Minor example:
  * `feat(sdk-python): add webhook signature helper`
* Major examples:
  * `feat(sdk-python)!: rename auth client interface`
  * Commit body includes `BREAKING CHANGE: ...`
  * Commit body includes `BREAKING: ...`

### Post-release verification

After a push that touches SDK paths, verify:

1. `sdk-python auto release` succeeds (version bump commit + tag push).
2. **`openapp-sdk release`** succeeds.
3. `pip install --upgrade openapp-sdk` works in a clean venv.
4. `https://github.com/tomers/openapp-sdk/tree/sdk-python-v0.1.0`
   (replace with actual tag) shows `rust/`, `python/`, `go/`, and `api-spec/`.
5. The `Releases` page on `openapp-sdk` has the new tag.

### Notes

* Human operators should not manually edit SDK version numbers during normal
  releases; the automation is the source of truth for release bumps.
* `packages/sdk/python/CHANGELOG.md` can still be maintained manually if desired,
  but it is not required for publishing.

## Manual release fallback (exception only)

If GitHub Actions is unavailable, you can still cut a release manually:

1. Update SDK versions in the two files listed above.
2. Commit and push to `main`.
3. Create and push `sdk-python-vX.Y.Z`.
4. Verify the **`openapp-sdk release`** workflow run from that tag.

## Verify release outputs

For any release path, verify:

1. `pip install --upgrade openapp-sdk` works in a clean venv.
2. `https://github.com/tomers/openapp-sdk/tree/sdk-python-v0.1.0`
   shows `rust/`, `python/`, `go/`, and `api-spec/`.
3. The `Releases` page on `openapp-sdk` has the new tag.

## Rollback

If a release is broken:

1. **PyPI**: you cannot delete a release, but you can
   [yank it](https://pypi.org/help/#yanked). Yanked releases stay installable
   by exact version but are hidden from new installs.
2. **Mirror**: push a revert commit; do not delete the tag.
3. Cut a new patch version (`sdk-python-v0.1.1`) with the fix.
