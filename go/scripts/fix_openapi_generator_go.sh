#!/usr/bin/env bash
# Post-process the generated Go client; the python script documents the patches it applies.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
python3 "${root}/scripts/fix_openapi_generator_go_client_postprocess.py"
