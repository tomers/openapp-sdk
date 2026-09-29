# \HouseholdInvitationsAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**Cancel**](HouseholdInvitationsAPI.md#Cancel) | **Delete** /groups/{group_id}/household-invitations/{id} |
[**Create**](HouseholdInvitationsAPI.md#Create) | **Post** /groups/{group_id}/household-invitations |
[**ListManager**](HouseholdInvitationsAPI.md#ListManager) | **Get** /groups/{group_id}/household-invitations |
[**Preview**](HouseholdInvitationsAPI.md#Preview) | **Post** /household-invitations/preview |
[**Redeem**](HouseholdInvitationsAPI.md#Redeem) | **Post** /household-invitations/redeem |
[**Resend**](HouseholdInvitationsAPI.md#Resend) | **Post** /groups/{group_id}/household-invitations/{id}/resend |



## Cancel

> Cancel(ctx, groupId, id).Execute()



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
	groupId := "groupId_example" // string |
	id := "id_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.HouseholdInvitationsAPI.Cancel(context.Background(), groupId, id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `HouseholdInvitationsAPI.Cancel``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**groupId** | **string** |  |
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCancelRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



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


## Create

> HouseholdInvitationResponse Create(ctx, groupId).CreateHouseholdInvitation(createHouseholdInvitation).Execute()



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
	groupId := "groupId_example" // string |
	createHouseholdInvitation := *openapiclient.NewCreateHouseholdInvitation() // CreateHouseholdInvitation |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.HouseholdInvitationsAPI.Create(context.Background(), groupId).CreateHouseholdInvitation(createHouseholdInvitation).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `HouseholdInvitationsAPI.Create``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `Create`: HouseholdInvitationResponse
	fmt.Fprintf(os.Stdout, "Response from `HouseholdInvitationsAPI.Create`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**groupId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCreateRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **createHouseholdInvitation** | [**CreateHouseholdInvitation**](CreateHouseholdInvitation.md) |  |

### Return type

[**HouseholdInvitationResponse**](HouseholdInvitationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListManager

> []HouseholdInvitationResponse ListManager(ctx, groupId).Execute()



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
	groupId := "groupId_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.HouseholdInvitationsAPI.ListManager(context.Background(), groupId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `HouseholdInvitationsAPI.ListManager``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListManager`: []HouseholdInvitationResponse
	fmt.Fprintf(os.Stdout, "Response from `HouseholdInvitationsAPI.ListManager`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**groupId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListManagerRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**[]HouseholdInvitationResponse**](HouseholdInvitationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## Preview

> HouseholdInvitationPreview Preview(ctx).InvitationTokenRequest(invitationTokenRequest).Execute()



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
	invitationTokenRequest := *openapiclient.NewInvitationTokenRequest("Token_example") // InvitationTokenRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.HouseholdInvitationsAPI.Preview(context.Background()).InvitationTokenRequest(invitationTokenRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `HouseholdInvitationsAPI.Preview``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `Preview`: HouseholdInvitationPreview
	fmt.Fprintf(os.Stdout, "Response from `HouseholdInvitationsAPI.Preview`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPreviewRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **invitationTokenRequest** | [**InvitationTokenRequest**](InvitationTokenRequest.md) |  |

### Return type

[**HouseholdInvitationPreview**](HouseholdInvitationPreview.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## Redeem

> RedeemHouseholdInvitationResponse Redeem(ctx).RedeemHouseholdInvitationRequest(redeemHouseholdInvitationRequest).Execute()



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
	redeemHouseholdInvitationRequest := *openapiclient.NewRedeemHouseholdInvitationRequest("Pin_example", "Token_example") // RedeemHouseholdInvitationRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.HouseholdInvitationsAPI.Redeem(context.Background()).RedeemHouseholdInvitationRequest(redeemHouseholdInvitationRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `HouseholdInvitationsAPI.Redeem``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `Redeem`: RedeemHouseholdInvitationResponse
	fmt.Fprintf(os.Stdout, "Response from `HouseholdInvitationsAPI.Redeem`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiRedeemRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **redeemHouseholdInvitationRequest** | [**RedeemHouseholdInvitationRequest**](RedeemHouseholdInvitationRequest.md) |  |

### Return type

[**RedeemHouseholdInvitationResponse**](RedeemHouseholdInvitationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## Resend

> HouseholdInvitationResponse Resend(ctx, groupId, id).Execute()



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
	groupId := "groupId_example" // string |
	id := "id_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.HouseholdInvitationsAPI.Resend(context.Background(), groupId, id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `HouseholdInvitationsAPI.Resend``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `Resend`: HouseholdInvitationResponse
	fmt.Fprintf(os.Stdout, "Response from `HouseholdInvitationsAPI.Resend`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**groupId** | **string** |  |
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiResendRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



### Return type

[**HouseholdInvitationResponse**](HouseholdInvitationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
