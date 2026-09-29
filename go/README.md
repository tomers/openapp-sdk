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

## Authentication

### api_key

`NewAPIClient` takes an OpenApp API key of the form `{origin}_openapp_{secret}`,
where `{origin}` is the bare deployment origin (scheme, host, optional port; no path),
for example `https://openapp.house_openapp_SECRET`. The client derives the API root
`{origin}/api/v1` from the key and sends the full key as the `X-API-Key` header on
every request. Never put an API key in `Authorization`: the gateway reserves that
header for JWTs and rejects an API key there with 401.

To send requests somewhere other than the key's origin (staging, a proxy, or a dev-env
key that embeds an in-container host), use `NewAPIClientWithBaseURL(apiKey, baseURL)`.
`baseURL` is the full API root, including `/api/v1`, for example
`http://localhost:4455/api/v1`.

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

## Documentation

- [SDK overview](https://openapp.house/docs/sdk/)
- [API reference](https://openapp.house/docs/api-reference/)
- [AI index (llms.txt)](https://openapp.house/llms.txt)
