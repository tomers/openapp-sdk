"""OpenApp Scripting sub-client."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ._base import _BaseResource


class ScriptingClient(_BaseResource):
    """Execute OpenApp Scripting programs."""

    async def execute(
        self,
        *,
        script: Any | None = None,
        program: Any | None = None,
        **extra: Any,
    ) -> dict[str, Any]:
        """`POST /scripting/execute` — run an OpenApp Scripting program.

        ``script`` is the source string. ``program`` is accepted as a
        backwards-compatible alias for older SDK docs.
        """

        if script is None:
            if program is None:
                raise TypeError("execute() missing required keyword argument: 'script'")
            script = program

        return await self._client._request(
            "POST", "/scripting/execute", body={"script": script, **extra}
        )

    async def execute_file(
        self,
        path: str | Path,
        *,
        encoding: str = "utf-8",
        **extra: Any,
    ) -> dict[str, Any]:
        """Read a `.openapp` program from disk and execute it."""

        script = Path(path).read_text(encoding=encoding)
        return await self.execute(script=script, **extra)
