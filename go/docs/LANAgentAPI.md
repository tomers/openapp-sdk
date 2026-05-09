# \LANAgentAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GetIntegrationLanAgentTasks**](LANAgentAPI.md#GetIntegrationLanAgentTasks) | **Get** /integrations/{integration_id}/lan-agent/tasks | GET /integrations/{id}/lan-agent/tasks
[**GetLanAgentCliBootstrapSh**](LANAgentAPI.md#GetLanAgentCliBootstrapSh) | **Get** /lan-agent/cli/bootstrap.sh | GET /lan-agent/cli/bootstrap.sh?key&#x3D;… — bash that downloads a versioned binary and runs the task.
[**GetLanAgentMeta**](LANAgentAPI.md#GetLanAgentMeta) | **Get** /lan-agent/meta | GET /lan-agent/meta
[**PostIntegrationLanAgentTaskSpec**](LANAgentAPI.md#PostIntegrationLanAgentTaskSpec) | **Post** /integrations/{integration_id}/lan-agent/task-spec | POST /integrations/{id}/lan-agent/task-spec
[**PostLanAgentCliBootstrapToken**](LANAgentAPI.md#PostLanAgentCliBootstrapToken) | **Post** /lan-agent/cli/bootstrap-token | POST /lan-agent/cli/bootstrap-token — ULID for &#x60;bootstrap.sh?key&#x3D;…&#x60; (JWT in Redis).
[**PostLanAgentCliToken**](LANAgentAPI.md#PostLanAgentCliToken) | **Post** /lan-agent/cli/token | POST /lan-agent/cli/token — session cookie or Bearer; returns JWT for LAN agent (iss&#x3D;oathkeeper).



## GetIntegrationLanAgentTasks

> GetIntegrationLanAgentTasks(ctx, integrationId).XOrg(xOrg).Execute()

GET /integrations/{id}/lan-agent/tasks

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/tomers/openapp-sdk/go"
)

func main() {
	integrationId := "integrationId_example" // string | Integration ULID
	xOrg := "xOrg_example" // string | Organization ULID

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.LANAgentAPI.GetIntegrationLanAgentTasks(context.Background(), integrationId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LANAgentAPI.GetIntegrationLanAgentTasks``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**integrationId** | **string** | Integration ULID |

### Other Parameters

Other parameters are passed through a pointer to a apiGetIntegrationLanAgentTasksRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** | Organization ULID |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetLanAgentCliBootstrapSh

> GetLanAgentCliBootstrapSh(ctx).Key(key).Execute()

GET /lan-agent/cli/bootstrap.sh?key=… — bash that downloads a versioned binary and runs the task.

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/tomers/openapp-sdk/go"
)

func main() {
	key := "key_example" // string | ULID from `POST /lan-agent/cli/bootstrap-token`.

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.LANAgentAPI.GetLanAgentCliBootstrapSh(context.Background()).Key(key).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LANAgentAPI.GetLanAgentCliBootstrapSh``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiGetLanAgentCliBootstrapShRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **key** | **string** | ULID from &#x60;POST /lan-agent/cli/bootstrap-token&#x60;. |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/x-shellscript

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetLanAgentMeta

> LanAgentMetaResponse GetLanAgentMeta(ctx).Execute()

GET /lan-agent/meta

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/tomers/openapp-sdk/go"
)

func main() {

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LANAgentAPI.GetLanAgentMeta(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LANAgentAPI.GetLanAgentMeta``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetLanAgentMeta`: LanAgentMetaResponse
	fmt.Fprintf(os.Stdout, "Response from `LANAgentAPI.GetLanAgentMeta`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetLanAgentMetaRequest struct via the builder pattern


### Return type

[**LanAgentMetaResponse**](LanAgentMetaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostIntegrationLanAgentTaskSpec

> LanAgentTaskSpecResponse PostIntegrationLanAgentTaskSpec(ctx, integrationId).XOrg(xOrg).LanAgentTaskSpecRequest(lanAgentTaskSpecRequest).Execute()

POST /integrations/{id}/lan-agent/task-spec

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/tomers/openapp-sdk/go"
)

func main() {
	integrationId := "integrationId_example" // string | Integration ULID
	xOrg := "xOrg_example" // string | Organization ULID
	lanAgentTaskSpecRequest := *openapiclient.NewLanAgentTaskSpecRequest(int32(123), interface{}(123), "TaskId_example") // LanAgentTaskSpecRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LANAgentAPI.PostIntegrationLanAgentTaskSpec(context.Background(), integrationId).XOrg(xOrg).LanAgentTaskSpecRequest(lanAgentTaskSpecRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LANAgentAPI.PostIntegrationLanAgentTaskSpec``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostIntegrationLanAgentTaskSpec`: LanAgentTaskSpecResponse
	fmt.Fprintf(os.Stdout, "Response from `LANAgentAPI.PostIntegrationLanAgentTaskSpec`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**integrationId** | **string** | Integration ULID |

### Other Parameters

Other parameters are passed through a pointer to a apiPostIntegrationLanAgentTaskSpecRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** | Organization ULID |
 **lanAgentTaskSpecRequest** | [**LanAgentTaskSpecRequest**](LanAgentTaskSpecRequest.md) |  |

### Return type

[**LanAgentTaskSpecResponse**](LanAgentTaskSpecResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostLanAgentCliBootstrapToken

> LanAgentBootstrapTokenResponse PostLanAgentCliBootstrapToken(ctx).LanAgentBootstrapTokenRequest(lanAgentBootstrapTokenRequest).Execute()

POST /lan-agent/cli/bootstrap-token — ULID for `bootstrap.sh?key=…` (JWT in Redis).

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/tomers/openapp-sdk/go"
)

func main() {
	lanAgentBootstrapTokenRequest := *openapiclient.NewLanAgentBootstrapTokenRequest("ApiBaseUrl_example", "TaskId_example") // LanAgentBootstrapTokenRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LANAgentAPI.PostLanAgentCliBootstrapToken(context.Background()).LanAgentBootstrapTokenRequest(lanAgentBootstrapTokenRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LANAgentAPI.PostLanAgentCliBootstrapToken``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostLanAgentCliBootstrapToken`: LanAgentBootstrapTokenResponse
	fmt.Fprintf(os.Stdout, "Response from `LANAgentAPI.PostLanAgentCliBootstrapToken`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostLanAgentCliBootstrapTokenRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lanAgentBootstrapTokenRequest** | [**LanAgentBootstrapTokenRequest**](LanAgentBootstrapTokenRequest.md) |  |

### Return type

[**LanAgentBootstrapTokenResponse**](LanAgentBootstrapTokenResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostLanAgentCliToken

> LanAgentCliTokenResponse PostLanAgentCliToken(ctx).Execute()

POST /lan-agent/cli/token — session cookie or Bearer; returns JWT for LAN agent (iss=oathkeeper).

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/tomers/openapp-sdk/go"
)

func main() {

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LANAgentAPI.PostLanAgentCliToken(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LANAgentAPI.PostLanAgentCliToken``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostLanAgentCliToken`: LanAgentCliTokenResponse
	fmt.Fprintf(os.Stdout, "Response from `LANAgentAPI.PostLanAgentCliToken`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiPostLanAgentCliTokenRequest struct via the builder pattern


### Return type

[**LanAgentCliTokenResponse**](LanAgentCliTokenResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
