# \ScriptingAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CancelScriptingExecution**](ScriptingAPI.md#CancelScriptingExecution) | **Delete** /scripting/executions/{id} | Cancel a pending or running execution.
[**CreateScriptingExecution**](ScriptingAPI.md#CreateScriptingExecution) | **Post** /scripting/executions | Submit an OpenApp Scripting program for asynchronous execution.
[**GetScriptingExecution**](ScriptingAPI.md#GetScriptingExecution) | **Get** /scripting/executions/{id} | Read one of the caller&#39;s own scripting executions.
[**ListScriptingExecutions**](ScriptingAPI.md#ListScriptingExecutions) | **Get** /scripting/executions | List the caller&#39;s own scripting executions, newest first.



## CancelScriptingExecution

> CancelScriptingExecution(ctx, id).Execute()

Cancel a pending or running execution.



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
	id := "id_example" // string | Execution id

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.ScriptingAPI.CancelScriptingExecution(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ScriptingAPI.CancelScriptingExecution``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Execution id |

### Other Parameters

Other parameters are passed through a pointer to a apiCancelScriptingExecutionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CreateScriptingExecution

> ScriptingExecution CreateScriptingExecution(ctx).CreateScriptingExecutionRequest(createScriptingExecutionRequest).Execute()

Submit an OpenApp Scripting program for asynchronous execution.



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
	createScriptingExecutionRequest := *openapiclient.NewCreateScriptingExecutionRequest("Script_example") // CreateScriptingExecutionRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.ScriptingAPI.CreateScriptingExecution(context.Background()).CreateScriptingExecutionRequest(createScriptingExecutionRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ScriptingAPI.CreateScriptingExecution``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateScriptingExecution`: ScriptingExecution
	fmt.Fprintf(os.Stdout, "Response from `ScriptingAPI.CreateScriptingExecution`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCreateScriptingExecutionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createScriptingExecutionRequest** | [**CreateScriptingExecutionRequest**](CreateScriptingExecutionRequest.md) |  |

### Return type

[**ScriptingExecution**](ScriptingExecution.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetScriptingExecution

> ScriptingExecution GetScriptingExecution(ctx, id).Execute()

Read one of the caller's own scripting executions.

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
	id := "id_example" // string | Execution id

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.ScriptingAPI.GetScriptingExecution(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ScriptingAPI.GetScriptingExecution``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetScriptingExecution`: ScriptingExecution
	fmt.Fprintf(os.Stdout, "Response from `ScriptingAPI.GetScriptingExecution`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Execution id |

### Other Parameters

Other parameters are passed through a pointer to a apiGetScriptingExecutionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ScriptingExecution**](ScriptingExecution.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListScriptingExecutions

> []ScriptingExecutionSummary ListScriptingExecutions(ctx).Limit(limit).Execute()

List the caller's own scripting executions, newest first.

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
	limit := int64(789) // int64 | Maximum number of executions to return (1-100, default 20). (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.ScriptingAPI.ListScriptingExecutions(context.Background()).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ScriptingAPI.ListScriptingExecutions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListScriptingExecutions`: []ScriptingExecutionSummary
	fmt.Fprintf(os.Stdout, "Response from `ScriptingAPI.ListScriptingExecutions`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListScriptingExecutionsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int64** | Maximum number of executions to return (1-100, default 20). |

### Return type

[**[]ScriptingExecutionSummary**](ScriptingExecutionSummary.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
