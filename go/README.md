<!-- markdownlint-disable MD013 MD060 -->
# Go SDK (`github.com/tomers/openapp-sdk/go`)

Official Go SDK for OpenApp. The public API is generated from OpenAPI, while the
default client transport goes through `openapp-sdk-core-c-bridge` so auth,
retries, error mapping, and wire dispatch stay aligned with the other SDKs.

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

Internal development, codegen, CI, and release notes are documented in
[`../docs/GO_MAINTAINERS.md`](../docs/GO_MAINTAINERS.md) and
[`../docs/SDK_DEVELOPMENT_POLICY.md`](../docs/SDK_DEVELOPMENT_POLICY.md).
