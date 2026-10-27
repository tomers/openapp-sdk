# \InvitationsAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**AcceptMyPendingInvitation**](InvitationsAPI.md#AcceptMyPendingInvitation) | **Post** /me/org-invitations/{id}/accept | Accept an invitation addressed to the caller&#39;s verified phone or email, without the PIN.
[**ListMyPendingInvitations**](InvitationsAPI.md#ListMyPendingInvitations) | **Get** /me/org-invitations | List pending invitations addressed to the signed-in account&#39;s verified phone or email.
[**PreviewInvitation**](InvitationsAPI.md#PreviewInvitation) | **Post** /invitations/preview | Preview an invitation by its token.
[**RedeemInvitation**](InvitationsAPI.md#RedeemInvitation) | **Post** /invitations/redeem | Redeem an invitation: exchange &#x60;token&#x60; + &#x60;pin&#x60; for org membership.



## AcceptMyPendingInvitation

> RedeemInvitationResponse AcceptMyPendingInvitation(ctx, id).Execute()

Accept an invitation addressed to the caller's verified phone or email, without the PIN.



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
	id := "id_example" // string | Invitation ID from `GET /me/org-invitations`

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.InvitationsAPI.AcceptMyPendingInvitation(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `InvitationsAPI.AcceptMyPendingInvitation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `AcceptMyPendingInvitation`: RedeemInvitationResponse
	fmt.Fprintf(os.Stdout, "Response from `InvitationsAPI.AcceptMyPendingInvitation`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Invitation ID from &#x60;GET /me/org-invitations&#x60; |

### Other Parameters

Other parameters are passed through a pointer to a apiAcceptMyPendingInvitationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**RedeemInvitationResponse**](RedeemInvitationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListMyPendingInvitations

> PendingInvitationListResponse ListMyPendingInvitations(ctx).Execute()

List pending invitations addressed to the signed-in account's verified phone or email.



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
	resp, r, err := apiClient.InvitationsAPI.ListMyPendingInvitations(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `InvitationsAPI.ListMyPendingInvitations``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListMyPendingInvitations`: PendingInvitationListResponse
	fmt.Fprintf(os.Stdout, "Response from `InvitationsAPI.ListMyPendingInvitations`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiListMyPendingInvitationsRequest struct via the builder pattern


### Return type

[**PendingInvitationListResponse**](PendingInvitationListResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PreviewInvitation

> InvitationPreviewResponse PreviewInvitation(ctx).InvitationPreviewRequest(invitationPreviewRequest).Execute()

Preview an invitation by its token.



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
	invitationPreviewRequest := *openapiclient.NewInvitationPreviewRequest("Token_example") // InvitationPreviewRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.InvitationsAPI.PreviewInvitation(context.Background()).InvitationPreviewRequest(invitationPreviewRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `InvitationsAPI.PreviewInvitation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PreviewInvitation`: InvitationPreviewResponse
	fmt.Fprintf(os.Stdout, "Response from `InvitationsAPI.PreviewInvitation`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPreviewInvitationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **invitationPreviewRequest** | [**InvitationPreviewRequest**](InvitationPreviewRequest.md) |  |

### Return type

[**InvitationPreviewResponse**](InvitationPreviewResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RedeemInvitation

> RedeemInvitationResponse RedeemInvitation(ctx).RedeemInvitationRequest(redeemInvitationRequest).Execute()

Redeem an invitation: exchange `token` + `pin` for org membership.



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
	redeemInvitationRequest := *openapiclient.NewRedeemInvitationRequest("Pin_example", "Token_example") // RedeemInvitationRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.InvitationsAPI.RedeemInvitation(context.Background()).RedeemInvitationRequest(redeemInvitationRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `InvitationsAPI.RedeemInvitation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `RedeemInvitation`: RedeemInvitationResponse
	fmt.Fprintf(os.Stdout, "Response from `InvitationsAPI.RedeemInvitation`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiRedeemInvitationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **redeemInvitationRequest** | [**RedeemInvitationRequest**](RedeemInvitationRequest.md) |  |

### Return type

[**RedeemInvitationResponse**](RedeemInvitationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
