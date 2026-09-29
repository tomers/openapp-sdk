# \DirectoryAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ApplyDirectoryListings**](DirectoryAPI.md#ApplyDirectoryListings) | **Post** /devices/{device_id}/directory-listings/apply | Apply a generation. Idempotent: re-running converges rather than duplicating.
[**GetDirectoryConfig**](DirectoryAPI.md#GetDirectoryConfig) | **Get** /devices/{device_id}/directory | Read the directory configuration.
[**PlanDirectoryListings**](DirectoryAPI.md#PlanDirectoryListings) | **Post** /devices/{device_id}/directory-listings/plan | Preview a generation without writing anything.
[**UpdateDirectoryConfig**](DirectoryAPI.md#UpdateDirectoryConfig) | **Put** /devices/{device_id}/directory | Update the directory configuration. This is the disable switch for the intercom.



## ApplyDirectoryListings

> GeneratePlanResponse ApplyDirectoryListings(ctx, deviceId).XOrg(xOrg).GenerateListingsPayload(generateListingsPayload).Execute()

Apply a generation. Idempotent: re-running converges rather than duplicating.

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
	deviceId := "deviceId_example" // string |
	xOrg := "xOrg_example" // string |
	generateListingsPayload := *openapiclient.NewGenerateListingsPayload([]string{"SourceGroupIds_example"}) // GenerateListingsPayload |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DirectoryAPI.ApplyDirectoryListings(context.Background(), deviceId).XOrg(xOrg).GenerateListingsPayload(generateListingsPayload).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryAPI.ApplyDirectoryListings``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ApplyDirectoryListings`: GeneratePlanResponse
	fmt.Fprintf(os.Stdout, "Response from `DirectoryAPI.ApplyDirectoryListings`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**deviceId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiApplyDirectoryListingsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **generateListingsPayload** | [**GenerateListingsPayload**](GenerateListingsPayload.md) |  |

### Return type

[**GeneratePlanResponse**](GeneratePlanResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetDirectoryConfig

> DirectoryConfigResponse GetDirectoryConfig(ctx, deviceId).XOrg(xOrg).Execute()

Read the directory configuration.

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
	deviceId := "deviceId_example" // string |
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DirectoryAPI.GetDirectoryConfig(context.Background(), deviceId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryAPI.GetDirectoryConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetDirectoryConfig`: DirectoryConfigResponse
	fmt.Fprintf(os.Stdout, "Response from `DirectoryAPI.GetDirectoryConfig`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**deviceId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetDirectoryConfigRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**DirectoryConfigResponse**](DirectoryConfigResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PlanDirectoryListings

> GeneratePlanResponse PlanDirectoryListings(ctx, deviceId).XOrg(xOrg).GenerateListingsPayload(generateListingsPayload).Execute()

Preview a generation without writing anything.

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
	deviceId := "deviceId_example" // string |
	xOrg := "xOrg_example" // string |
	generateListingsPayload := *openapiclient.NewGenerateListingsPayload([]string{"SourceGroupIds_example"}) // GenerateListingsPayload |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DirectoryAPI.PlanDirectoryListings(context.Background(), deviceId).XOrg(xOrg).GenerateListingsPayload(generateListingsPayload).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryAPI.PlanDirectoryListings``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PlanDirectoryListings`: GeneratePlanResponse
	fmt.Fprintf(os.Stdout, "Response from `DirectoryAPI.PlanDirectoryListings`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**deviceId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPlanDirectoryListingsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **generateListingsPayload** | [**GenerateListingsPayload**](GenerateListingsPayload.md) |  |

### Return type

[**GeneratePlanResponse**](GeneratePlanResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateDirectoryConfig

> DirectoryConfigResponse UpdateDirectoryConfig(ctx, deviceId).XOrg(xOrg).UpdateDirectoryConfigPayload(updateDirectoryConfigPayload).Execute()

Update the directory configuration. This is the disable switch for the intercom.

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
	deviceId := "deviceId_example" // string |
	xOrg := "xOrg_example" // string |
	updateDirectoryConfigPayload := *openapiclient.NewUpdateDirectoryConfigPayload() // UpdateDirectoryConfigPayload |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DirectoryAPI.UpdateDirectoryConfig(context.Background(), deviceId).XOrg(xOrg).UpdateDirectoryConfigPayload(updateDirectoryConfigPayload).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryAPI.UpdateDirectoryConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateDirectoryConfig`: DirectoryConfigResponse
	fmt.Fprintf(os.Stdout, "Response from `DirectoryAPI.UpdateDirectoryConfig`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**deviceId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateDirectoryConfigRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **updateDirectoryConfigPayload** | [**UpdateDirectoryConfigPayload**](UpdateDirectoryConfigPayload.md) |  |

### Return type

[**DirectoryConfigResponse**](DirectoryConfigResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
