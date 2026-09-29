# \OrgInvitationsAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CancelInvitation**](OrgInvitationsAPI.md#CancelInvitation) | **Delete** /orgs/{id}/invitations/{invite_id} | DELETE &#x60;/orgs/{id}/invitations/{invite_id}&#x60; — mark invitation cancelled.
[**CreateInvitation**](OrgInvitationsAPI.md#CreateInvitation) | **Post** /orgs/{id}/invitations | POST &#x60;/orgs/{id}/invitations&#x60; — mint an invitation and dispatch its PIN.
[**HardDeleteInvitation**](OrgInvitationsAPI.md#HardDeleteInvitation) | **Delete** /orgs/{id}/invitations/{invite_id}/purge | DELETE &#x60;/orgs/{id}/invitations/{invite_id}/purge&#x60; — permanently remove any invitation status.
[**ListInvitations**](OrgInvitationsAPI.md#ListInvitations) | **Get** /orgs/{id}/invitations | GET &#x60;/orgs/{id}/invitations&#x60; — list all invitations attached to the org (any status).
[**ResendInvitation**](OrgInvitationsAPI.md#ResendInvitation) | **Post** /orgs/{id}/invitations/{invite_id}/resend | POST &#x60;/orgs/{id}/invitations/{invite_id}/resend&#x60; — re-issue fresh token+PIN.



## CancelInvitation

> CancelInvitation(ctx, id, inviteId).Execute()

DELETE `/orgs/{id}/invitations/{invite_id}` — mark invitation cancelled.

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

POST `/orgs/{id}/invitations` — mint an invitation and dispatch its PIN.

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

DELETE `/orgs/{id}/invitations/{invite_id}/purge` — permanently remove any invitation status.

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

> ListInvitationsResponse ListInvitations(ctx, id).Execute()

GET `/orgs/{id}/invitations` — list all invitations attached to the org (any status).

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
	resp, r, err := apiClient.OrgInvitationsAPI.ListInvitations(context.Background(), id).Execute()
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

> UserInvitationDto ResendInvitation(ctx, id, inviteId).Execute()

POST `/orgs/{id}/invitations/{invite_id}/resend` — re-issue fresh token+PIN.

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
	resp, r, err := apiClient.OrgInvitationsAPI.ResendInvitation(context.Background(), id, inviteId).Execute()
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



### Return type

[**UserInvitationDto**](UserInvitationDto.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
