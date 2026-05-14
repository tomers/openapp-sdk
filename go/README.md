<!-- markdownlint-disable MD013 MD060 -->
# Go SDK (`github.com/tomers/openapp-sdk/go`)

Official Go SDK for **OpenApp — Physical Security as a Service (PSaaS)**: API-first
access control for doors, gates, virtual intercom, guest invitations, policies, and
audit across heterogeneous hardware and automation. The public API is generated from
OpenAPI; the default client transport uses `openapp-sdk-core-c-bridge` so auth,
retries, error mapping, and wire dispatch stay aligned with the other SDKs.

- Docs: [openapp.house/docs/sdk/go/](https://openapp.house/docs/sdk/go/)
- OpenAPI: [openapp-openapi.json](https://openapp.house/docs/api-spec/openapp-openapi.json)

## Installation

```bash
go get github.com/tomers/openapp-sdk/go
```

## Requirements

- Go 1.22+
- OpenApp API key

## Quick start

```go
package main

import (
  "context"
  "fmt"
  "log"
  "os"

  openapp "github.com/tomers/openapp-sdk/go"
)

func main() {
  client, err := openapp.NewAPIClient(os.Getenv("OPENAPP_API_KEY"))
  if err != nil {
    log.Fatalf("NewAPIClient: %v", err)
  }
  defer client.Close()

  status, _, err := client.StatusAPI.GetBackendStatus(context.Background()).Execute()
  if err != nil {
    log.Fatalf("GetBackendStatus: %v", err)
  }
  fmt.Printf("status: %#v\n", status)
}
```

## Calling authenticated APIs

```go
resp, httpResp, err := client.OrgsAPI.ListOrgs(context.Background()).Execute()
if err != nil {
  _ = httpResp
  // Generated client errors include status/body details.
  log.Fatal(err)
}
fmt.Printf("orgs: %#v\n", resp)
```

## Maintainer docs

Shared Gherkin story packs run through `go test` via `gherkin_tier1_test.go`.
For the PR-fast slice, use `just sdk go gherkin-tier0`.

Internal development, codegen, CI, and release notes are documented in
[`../docs/GO_MAINTAINERS.md`](../docs/GO_MAINTAINERS.md) and
[`../docs/SDK_DEVELOPMENT_POLICY.md`](../docs/SDK_DEVELOPMENT_POLICY.md).

## Documentation

- [Agents & automation](https://openapp.house/docs/guides/agents/overview/)
- [API reference](https://openapp.house/docs/api-reference/)
- [Time-bound guest invitation](https://openapp.house/docs/guides/agents/time-bound-guest-invitation/)
