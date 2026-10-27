# \OrgInvitationsAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CancelInvitation**](OrgInvitationsAPI.md#CancelInvitation) | **Delete** /orgs/{id}/invitations/{invite_id} | Cancel an invitation.
[**CreateInvitation**](OrgInvitationsAPI.md#CreateInvitation) | **Post** /orgs/{id}/invitations | Create an invitation and send its PIN to the invitee. Create an org invitation.
[**HardDeleteInvitation**](OrgInvitationsAPI.md#HardDeleteInvitation) | **Delete** /orgs/{id}/invitations/{invite_id}/purge | Permanently delete an invitation, whatever its status.
[**ListInvitations**](OrgInvitationsAPI.md#ListInvitations) | **Get** /orgs/{id}/invitations | List invitations of the org.
[**ResendInvitation**](OrgInvitationsAPI.md#ResendInvitation) | **Post** /orgs/{id}/invitations/{invite_id}/resend | Resend an invitation with a freshly issued token and PIN.
[**UpdateInvitation**](OrgInvitationsAPI.md#UpdateInvitation) | **Patch** /orgs/{id}/invitations/{invite_id} | Edit a pending invitation.



## CancelInvitation

> CancelInvitation(ctx, id, inviteId).Execute()

Cancel an invitation.

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
	inviteId := "inviteId_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.OrgInvitationsAPI.CancelInvitation(context.Background(), id, inviteId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgInvitationsAPI.CancelInvitation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCancelInvitationRequest struct via the builder pattern


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


## CreateInvitation

> CreateInvitationResponse CreateInvitation(ctx, id).CreateInvitationRequest(createInvitationRequest).Execute()

Create an invitation and send its PIN to the invitee. Create an org invitation.



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
	createInvitationRequest := *openapiclient.NewCreateInvitationRequest() // CreateInvitationRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgInvitationsAPI.CreateInvitation(context.Background(), id).CreateInvitationRequest(createInvitationRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgInvitationsAPI.CreateInvitation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateInvitation`: CreateInvitationResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgInvitationsAPI.CreateInvitation`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCreateInvitationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **createInvitationRequest** | [**CreateInvitationRequest**](CreateInvitationRequest.md) |  |

### Return type

[**CreateInvitationResponse**](CreateInvitationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## HardDeleteInvitation

> HardDeleteInvitation(ctx, id, inviteId).Execute()

Permanently delete an invitation, whatever its status.

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
	inviteId := "inviteId_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.OrgInvitationsAPI.HardDeleteInvitation(context.Background(), id, inviteId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgInvitationsAPI.HardDeleteInvitation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiHardDeleteInvitationRequest struct via the builder pattern


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


## ListInvitations

> ListInvitationsResponse ListInvitations(ctx, id).ListingEntityId(listingEntityId).Execute()

List invitations of the org.



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
	listingEntityId := "listingEntityId_example" // string | When set, only invitations that grant this directory listing are returned. Callers with `users:list` may filter any listing in the org. Listing admins without `users:list` may only request a listing they administer (or omit this to receive every invitation for listings they administer). (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgInvitationsAPI.ListInvitations(context.Background(), id).ListingEntityId(listingEntityId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgInvitationsAPI.ListInvitations``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListInvitations`: ListInvitationsResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgInvitationsAPI.ListInvitations`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListInvitationsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **listingEntityId** | **string** | When set, only invitations that grant this directory listing are returned. Callers with &#x60;users:list&#x60; may filter any listing in the org. Listing admins without &#x60;users:list&#x60; may only request a listing they administer (or omit this to receive every invitation for listings they administer). |

### Return type

[**ListInvitationsResponse**](ListInvitationsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ResendInvitation

> UserInvitationDto ResendInvitation(ctx, id, inviteId).ResendInvitationRequest(resendInvitationRequest).Execute()

Resend an invitation with a freshly issued token and PIN.

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
	inviteId := "inviteId_example" // string |
	resendInvitationRequest := *openapiclient.NewResendInvitationRequest() // ResendInvitationRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgInvitationsAPI.ResendInvitation(context.Background(), id, inviteId).ResendInvitationRequest(resendInvitationRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgInvitationsAPI.ResendInvitation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ResendInvitation`: UserInvitationDto
	fmt.Fprintf(os.Stdout, "Response from `OrgInvitationsAPI.ResendInvitation`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiResendInvitationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **resendInvitationRequest** | [**ResendInvitationRequest**](ResendInvitationRequest.md) |  |

### Return type

[**UserInvitationDto**](UserInvitationDto.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateInvitation

> UserInvitationDto UpdateInvitation(ctx, id, inviteId).UpdateInvitationRequest(updateInvitationRequest).Execute()

Edit a pending invitation.



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
	inviteId := "inviteId_example" // string |
	updateInvitationRequest := *openapiclient.NewUpdateInvitationRequest() // UpdateInvitationRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgInvitationsAPI.UpdateInvitation(context.Background(), id, inviteId).UpdateInvitationRequest(updateInvitationRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgInvitationsAPI.UpdateInvitation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateInvitation`: UserInvitationDto
	fmt.Fprintf(os.Stdout, "Response from `OrgInvitationsAPI.UpdateInvitation`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateInvitationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **updateInvitationRequest** | [**UpdateInvitationRequest**](UpdateInvitationRequest.md) |  |

### Return type

[**UserInvitationDto**](UserInvitationDto.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
