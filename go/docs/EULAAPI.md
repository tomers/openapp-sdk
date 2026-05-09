# \EULAAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**AcceptEula**](EULAAPI.md#AcceptEula) | **Post** /eula/accept | Accept EULA and provision user. Requires X-User-Email (set by Oathkeeper cookie_session).
[**GetEula**](EULAAPI.md#GetEula) | **Get** /eula | Get EULA text. Anonymous GET is allowed at the edge so reading the agreement is never blocked by auth.



## AcceptEula

> AcceptEulaResponse AcceptEula(ctx).AcceptEulaRequest(acceptEulaRequest).Execute()

Accept EULA and provision user. Requires X-User-Email (set by Oathkeeper cookie_session).

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
	acceptEulaRequest := *openapiclient.NewAcceptEulaRequest(false) // AcceptEulaRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EULAAPI.AcceptEula(context.Background()).AcceptEulaRequest(acceptEulaRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EULAAPI.AcceptEula``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `AcceptEula`: AcceptEulaResponse
	fmt.Fprintf(os.Stdout, "Response from `EULAAPI.AcceptEula`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiAcceptEulaRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **acceptEulaRequest** | [**AcceptEulaRequest**](AcceptEulaRequest.md) |  |

### Return type

[**AcceptEulaResponse**](AcceptEulaResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetEula

> EulaResponse GetEula(ctx).Execute()

Get EULA text. Anonymous GET is allowed at the edge so reading the agreement is never blocked by auth.

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
	resp, r, err := apiClient.EULAAPI.GetEula(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EULAAPI.GetEula``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetEula`: EulaResponse
	fmt.Fprintf(os.Stdout, "Response from `EULAAPI.GetEula`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetEulaRequest struct via the builder pattern


### Return type

[**EulaResponse**](EulaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
