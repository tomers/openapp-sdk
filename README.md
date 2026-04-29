# OpenApp SDK (open source)

Public home for OpenApp **client SDKs** and the shared **Rust core**. The
product application remains proprietary; this repository contains only
integration code you can audit, fork, and build against.

Layout:

| Directory   | Contents |
|-------------|----------|
| `rust/`   | Rust workspace (`openapp-sdk-common`, `openapp-sdk-core`, C bridge) |
| `python/` | PyPI package [`openapp-sdk`](https://pypi.org/project/openapp-sdk/) (PyO3 + Python) |
| `api-spec/` | OpenAPI source used for codegen |

The canonical development tree for maintainers lives in the private
[`tomers/openapp`](https://github.com/tomers/openapp) monorepo; CI mirrors
this snapshot here on each Python SDK release so users can browse sources and
[file issues](https://github.com/tomers/openapp-sdk/issues) without access to
that repo.

**Released snapshot:** `sdk-python-v0.1.19` (Python package version **0.1.19**)

- PyPI (Python): https://pypi.org/project/openapp-sdk/
- Docs: https://docs.openapp.house/sdk/python
