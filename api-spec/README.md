# API Spec (OpenAPI)

- **Source of truth**: `apps/backend` (Rust, `utoipa`)
- **Contract artifact**: `packages/api-spec/openapi.json` (generated, committed)

## Update workflow

From repo root:

```bash
cd apps/backend
just openapi export
```

To verify there is no drift (used by backend pre-commit):

```bash
cd apps/backend
just openapi check
```
