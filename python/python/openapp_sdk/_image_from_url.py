"""HTTP(S) download helpers for ``upload_image_from_url`` on resource clients."""

from __future__ import annotations

import mimetypes
from urllib.parse import unquote, urlparse

import httpx

from .errors import TransportError

ALLOWED_IMAGE_TYPES = frozenset({"image/jpeg", "image/png", "image/webp"})

_EXT_DEFAULT: dict[str, str] = {
    "image/jpeg": ".jpg",
    "image/png": ".png",
    "image/webp": ".webp",
}


def _sniff_image_content_type(data: bytes) -> str | None:
    if len(data) >= 3 and data[0:3] == b"\xff\xd8\xff":
        return "image/jpeg"
    if len(data) >= 8 and data[0:8] == b"\x89PNG\r\n\x1a\n":
        return "image/png"
    if len(data) >= 12 and data[0:4] == b"RIFF" and data[8:12] == b"WEBP":
        return "image/webp"
    return None


def _normalize_filename(name: str, content_type: str) -> str:
    name = unquote(name).strip()
    if not name or name.endswith("/"):
        return f"upload{_EXT_DEFAULT.get(content_type, '')}"
    return name


def _resolve_content_type(header_ct: str, url: str, data: bytes) -> str:
    header_ct = header_ct.split(";")[0].strip().lower()
    if header_ct in ALLOWED_IMAGE_TYPES:
        return header_ct

    parsed = urlparse(url)
    guessed, _ = mimetypes.guess_type(parsed.path)
    if guessed in ALLOWED_IMAGE_TYPES:
        return guessed

    sniffed = _sniff_image_content_type(data)
    if sniffed is not None:
        return sniffed

    raise ValueError(
        "could not determine image type (expected Content-Type image/jpeg, "
        "image/png, or image/webp, a recognizable URL extension, or JPEG/PNG/WebP bytes)"
    )


async def fetch_image_for_upload(url: str) -> tuple[bytes, str, str]:
    """Download ``url`` (HTTP or HTTPS only) and return ``(data, content_type, filename)``.

    Raises :class:`~openapp_sdk.errors.TransportError` if the request fails.
    Raises ``ValueError`` if the response is not a supported image type.
    """

    parsed = urlparse(url)
    if parsed.scheme not in ("http", "https"):
        raise ValueError("upload_image_from_url only supports http and https URLs")

    try:
        async with httpx.AsyncClient() as http:
            resp = await http.get(
                url,
                follow_redirects=True,
                timeout=30.0,
                headers={"User-Agent": "OpenApp-Python-SDK/1.0 (image fetch)"},
            )
            resp.raise_for_status()
            data = resp.content
            header_ct = resp.headers.get("content-type", "")
    except httpx.HTTPError as exc:
        raise TransportError(f"failed to fetch image URL: {exc}") from exc

    content_type = _resolve_content_type(header_ct, url, data)
    path_last = urlparse(url).path.rstrip("/").rsplit("/", 1)[-1]
    filename = _normalize_filename(path_last, content_type)
    return data, content_type, filename
