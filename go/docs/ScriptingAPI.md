# \ScriptingAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ExecuteScripting**](ScriptingAPI.md#ExecuteScripting) | **Post** /scripting/execute | Execute an OpenApp Scripting program. Requires execute permission in at least one org.



## ExecuteScripting

> ExecuteScriptingResponse ExecuteScripting(ctx).ExecuteScriptingRequest(executeScriptingRequest).Execute()

Execute an OpenApp Scripting program. Requires execute permission in at least one org.

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
	executeScriptingRequest := *openapiclient.NewExecuteScriptingRequest("Script_example") // ExecuteScriptingRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.ScriptingAPI.ExecuteScripting(context.Background()).ExecuteScriptingRequest(executeScriptingRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ScriptingAPI.ExecuteScripting``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ExecuteScripting`: ExecuteScriptingResponse
	fmt.Fprintf(os.Stdout, "Response from `ScriptingAPI.ExecuteScripting`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiExecuteScriptingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **executeScriptingRequest** | [**ExecuteScriptingRequest**](ExecuteScriptingRequest.md) |  |

### Return type

[**ExecuteScriptingResponse**](ExecuteScriptingResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
