"""Steps for smoke / parity scenarios (Python runner)."""

from __future__ import annotations

from behave import given, then, when


@given("the SDK is installed")
def step_the_sdk_is_installed(context: object) -> None:
    del context


@when("the client library is imported")
def step_the_client_library_is_imported(context: object) -> None:
    import openapp_sdk

    context.openapp_sdk = openapp_sdk


@then("a non-empty version string is available")
def step_a_nonempty_version_string_is_available(context: object) -> None:
    pkg = getattr(context, "openapp_sdk", None)
    assert pkg is not None
    ver = getattr(pkg, "__version__", "") or ""
    assert ver.strip(), "openapp_sdk.__version__ must be non-empty"
