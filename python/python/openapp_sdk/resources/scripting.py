"""OpenApp Scripting sub-client.

Scripting is an asynchronous job API: :meth:`ScriptingClient.create_execution`
submits a program and returns a job, which :meth:`ScriptingClient.execute` polls
until ``status`` is terminal.
"""

from __future__ import annotations

import asyncio
import time
from typing import Any

from ..errors import SdkError, TransportError
from ._base import _BaseResource

#: Job statuses after which no further polling can change the outcome.
TERMINAL_STATUSES = frozenset({"succeeded", "failed", "canceled"})

#: First wait before re-reading a job; doubles on every poll up to
#: ``_POLL_MAX_DELAY``.
_POLL_INITIAL_DELAY = 1.0
_POLL_MAX_DELAY = 5.0

#: Backend runtime cap: an execution that runs longer is failed server-side, so
#: a client that has waited this long will never observe a result.
_EXECUTION_TIMEOUT = 900.0


class ScriptingClient(_BaseResource):
    """Submit, poll, and cancel OpenApp Scripting executions."""

    async def create_execution(
        self,
        *,
        script: Any,
        **extra: Any,
    ) -> dict[str, Any]:
        """`POST /scripting/executions` — submit a program and return the job.

        ``script`` is the source string. Extra keyword arguments are merged
        into the JSON body (``script`` is the only documented field).
        """

        return await self._client._request(
            "POST",
            "/scripting/executions",
            body={"script": script, **extra},
        )

    async def get_execution(self, id: str) -> dict[str, Any]:
        """`GET /scripting/executions/{id}` — the only route carrying ``result``."""

        return await self._client._request("GET", f"/scripting/executions/{id}")

    async def list_executions(self, *, limit: int | None = None) -> list[dict[str, Any]]:
        """`GET /scripting/executions` — summaries (no ``result``), newest first.

        ``limit`` defaults to 20 on the server and is capped at 100.
        """

        return await self._client._request(
            "GET", "/scripting/executions", query=self._query(limit=limit)
        )

    async def cancel_execution(self, id: str) -> None:
        """`DELETE /scripting/executions/{id}` — request cooperative cancellation.

        Cancellation is not a rollback: effects the program already applied stay
        applied, and the worker only notices within a few seconds.
        """

        await self._client._request("DELETE", f"/scripting/executions/{id}")

    async def execute(
        self,
        *,
        script: Any,
        **extra: Any,
    ) -> Any:
        """Submit a program and wait for it to finish, returning its ``result``.

        Polls :meth:`get_execution` every second, backing off to five seconds,
        until the job is terminal or fifteen minutes elapse. A script fault is
        **not** an HTTP error — the submission still succeeds and the failure
        lands in the job — so ``failed`` / ``canceled`` raise :class:`SdkError`
        carrying the job's ``error`` text.
        """

        job = await self.create_execution(script=script, **extra)
        job = await self._wait_for_execution(job["id"])
        status = job.get("status")
        if status == "succeeded":
            return job.get("result")
        error = job.get("error") or "no error reported"
        raise SdkError(f"script execution {status}: {error}")

    async def _wait_for_execution(self, execution_id: str) -> dict[str, Any]:
        deadline = time.monotonic() + _EXECUTION_TIMEOUT
        delay = _POLL_INITIAL_DELAY
        while True:
            await asyncio.sleep(delay)
            job = await self.get_execution(execution_id)
            if job.get("status") in TERMINAL_STATUSES:
                return job
            if time.monotonic() >= deadline:
                raise TransportError(
                    f"execution {execution_id} did not finish within {_EXECUTION_TIMEOUT}s"
                )
            delay = min(delay * 2, _POLL_MAX_DELAY)
