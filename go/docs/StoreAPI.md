# \StoreAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**AddBillingAccountMember**](StoreAPI.md#AddBillingAccountMember) | **Post** /orgs/{org_id}/billing-accounts/{account_id}/members |
[**CreateBillingAccount**](StoreAPI.md#CreateBillingAccount) | **Post** /orgs/{org_id}/billing-accounts |
[**ListBillingAccounts**](StoreAPI.md#ListBillingAccounts) | **Get** /orgs/{org_id}/billing-accounts |



## AddBillingAccountMember

> AddBillingAccountMember(ctx, orgId, accountId).AddBillingMemberRequest(addBillingMemberRequest).Execute()



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
	orgId := "orgId_example" // string |
	accountId := "accountId_example" // string |
	addBillingMemberRequest := *openapiclient.NewAddBillingMemberRequest("UserId_example") // AddBillingMemberRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.StoreAPI.AddBillingAccountMember(context.Background(), orgId, accountId).AddBillingMemberRequest(addBillingMemberRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `StoreAPI.AddBillingAccountMember``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |
**accountId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiAddBillingAccountMemberRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **addBillingMemberRequest** | [**AddBillingMemberRequest**](AddBillingMemberRequest.md) |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CreateBillingAccount

> BillingAccount CreateBillingAccount(ctx, orgId).CreateBillingAccountRequest(createBillingAccountRequest).Execute()



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
	orgId := "orgId_example" // string |
	createBillingAccountRequest := *openapiclient.NewCreateBillingAccountRequest(*openapiclient.NewLocalizedString(map[string]string{"key": "Inner_example"})) // CreateBillingAccountRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.StoreAPI.CreateBillingAccount(context.Background(), orgId).CreateBillingAccountRequest(createBillingAccountRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `StoreAPI.CreateBillingAccount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateBillingAccount`: BillingAccount
	fmt.Fprintf(os.Stdout, "Response from `StoreAPI.CreateBillingAccount`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCreateBillingAccountRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **createBillingAccountRequest** | [**CreateBillingAccountRequest**](CreateBillingAccountRequest.md) |  |

### Return type

[**BillingAccount**](BillingAccount.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListBillingAccounts

> []BillingAccount ListBillingAccounts(ctx, orgId).Execute()



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
	orgId := "orgId_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.StoreAPI.ListBillingAccounts(context.Background(), orgId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `StoreAPI.ListBillingAccounts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListBillingAccounts`: []BillingAccount
	fmt.Fprintf(os.Stdout, "Response from `StoreAPI.ListBillingAccounts`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListBillingAccountsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**[]BillingAccount**](BillingAccount.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
