# \LANAgentAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GetIntegrationLanAgentTasks**](LANAgentAPI.md#GetIntegrationLanAgentTasks) | **Get** /integrations/{integration_id}/lan-agent/tasks | GET /integrations/{id}/lan-agent/tasks
[**PostIntegrationLanAgentTaskSpec**](LANAgentAPI.md#PostIntegrationLanAgentTaskSpec) | **Post** /integrations/{integration_id}/lan-agent/task-spec | POST /integrations/{id}/lan-agent/task-spec



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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
