# Releasing `openapp-sdk`

Bootstrap steps for PyPI + mirror credentials and policy live here; **semver and
tag semantics** are authoritative in **[`../docs/VERSIONING.md`](../docs/VERSIONING.md)** (Rule A — unified release).

Public SDK sources are mirrored to [`tomers/openapp-sdk`](https://github.com/tomers/openapp-sdk).

## One-time bootstrap

Follow **sections 1–3** of the prior detailed checklist (GitHub App for push +
mirror, PyPI Trusted Publishing, docs subdomain). Update PyPI **Trusted Publishing**
to register workflow filename **`sdk-unified-release.yml`** (not `openapp-sdk-release.yml`).

Repository / owner values remain the private monorepo where releases run.

## Day-to-day release (manual)

Automated bump-on-`main` workflows were removed. Ship everything from:

```bash
just sdk release-check   # optional local gates
just sdk release         # patch bump, or: just sdk release minor | 0.2.0
```

That triggers **`.github/workflows/sdk-unified-release.yml`**, which bumps the unified
semver (see [`packages/sdk/SDK_VERSION`](../SDK_VERSION)), tags `sdk-python-v*`,
`sdk-rust-v*`, `sdk-go-v*`, `sdk-node-v*` at one commit, runs CI gates, publishes
PyPI → mirror → crates.io → npm.

Requires [`gh`](https://cli.github.com/) authenticated for the repo.

## Legacy references

Older docs mentioned `sdk-python auto release` and `openapp-sdk-release.yml`; those
paths are **retired** in favor of **`sdk-unified release`**.
