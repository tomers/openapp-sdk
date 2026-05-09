#!/usr/bin/env bash
# OpenAPI Generator go client emits ListDevicesQuery.HasExternalId() for ExternalId
# presence, which collides with the nullable query field HasExternalId (same OpenAPI
# names). Remove the redundant helper; callers use ExternalId.{IsSet,Get} directly.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
python3 "${root}/scripts/fix_openapi_generator_go.py" "${root}/model_list_devices_query.go"
python3 "${root}/scripts/fix_openapi_generator_go_client_postprocess.py"
