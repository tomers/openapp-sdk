# \InvitationsAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**PreviewInvitation**](InvitationsAPI.md#PreviewInvitation) | **Post** /invitations/preview | POST &#x60;/invitations/preview&#x60; — public-ish; returns enough metadata to render the redeem page without leaking the PIN.
[**RedeemInvitation**](InvitationsAPI.md#RedeemInvitation) | **Post** /invitations/redeem | POST &#x60;/invitations/redeem&#x60; — exchange &#x60;token&#x60; + &#x60;pin&#x60; for org membership.



## PreviewInvitation

> InvitationPreviewResponse PreviewInvitation(ctx).InvitationPreviewRequest(invitationPreviewRequest).Execute()

POST `/invitations/preview` — public-ish; returns enough metadata to render the redeem page without leaking the PIN.

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

POST `/invitations/redeem` — exchange `token` + `pin` for org membership.



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
