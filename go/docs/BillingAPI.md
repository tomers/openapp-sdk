# \BillingAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GetOrgBillingPlan**](BillingAPI.md#GetOrgBillingPlan) | **Get** /orgs/{id}/billing/plan |
[**GetOrgBillingUsage**](BillingAPI.md#GetOrgBillingUsage) | **Get** /orgs/{id}/billing/usage |
[**GetPlans**](BillingAPI.md#GetPlans) | **Get** /plans |



## GetOrgBillingPlan

> BillingPlanResponse GetOrgBillingPlan(ctx, id).Execute()



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
	id := "id_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.BillingAPI.GetOrgBillingPlan(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `BillingAPI.GetOrgBillingPlan``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOrgBillingPlan`: BillingPlanResponse
	fmt.Fprintf(os.Stdout, "Response from `BillingAPI.GetOrgBillingPlan`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetOrgBillingPlanRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**BillingPlanResponse**](BillingPlanResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetOrgBillingUsage

> QuotaReport GetOrgBillingUsage(ctx, id).Execute()



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
	id := "id_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.BillingAPI.GetOrgBillingUsage(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `BillingAPI.GetOrgBillingUsage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOrgBillingUsage`: QuotaReport
	fmt.Fprintf(os.Stdout, "Response from `BillingAPI.GetOrgBillingUsage`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetOrgBillingUsageRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**QuotaReport**](QuotaReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetPlans

> PlansResponse GetPlans(ctx).Execute()



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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.BillingAPI.GetPlans(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `BillingAPI.GetPlans``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPlans`: PlansResponse
	fmt.Fprintf(os.Stdout, "Response from `BillingAPI.GetPlans`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetPlansRequest struct via the builder pattern


### Return type

[**PlansResponse**](PlansResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
