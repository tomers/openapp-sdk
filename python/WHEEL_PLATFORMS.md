# PyPI wheel platforms

Keep this in sync with `.github/workflows/openapp-sdk-release.yml` (`build-wheels`
job). CI compares these bullets to the workflow matrix via
`scripts/check_wheel_readme_drift.py`.

Prebuilt wheels are published for:

* Linux: `manylinux2014` x86_64, `manylinux_2_28` aarch64
* macOS: 11+ x86_64 and arm64 (wheels built on macOS 14; compatibility via `MACOSX_DEPLOYMENT_TARGET=11.0`)
* Windows: x86_64
