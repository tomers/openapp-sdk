# \MeAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GetMeApartments**](MeAPI.md#GetMeApartments) | **Get** /me/apartments | GET /me/apartments — returns apartments the current user is a resident of.
[**GetMeInvitations**](MeAPI.md#GetMeInvitations) | **Get** /me/invitations | GET /me/invitations — returns invitations the current user has claimed.
[**GetMePushSubscriptionStatus**](MeAPI.md#GetMePushSubscriptionStatus) | **Get** /me/push-subscription-status | GET /me/push-subscription-status — returns whether the current user has any push subscription.
[**GetMePushVapidPublicKey**](MeAPI.md#GetMePushVapidPublicKey) | **Get** /me/push-vapid-public-key | GET /me/push-vapid-public-key — returns VAPID public key for client subscription. 404 when not configured.
[**PostMePushSubscription**](MeAPI.md#PostMePushSubscription) | **Post** /me/push-subscriptions | POST /me/push-subscriptions — store Web Push subscription for call notifications.



## GetMeApartments

> MeApartmentsResponse GetMeApartments(ctx).Execute()

GET /me/apartments — returns apartments the current user is a resident of.

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
	resp, r, err := apiClient.MeAPI.GetMeApartments(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MeAPI.GetMeApartments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetMeApartments`: MeApartmentsResponse
	fmt.Fprintf(os.Stdout, "Response from `MeAPI.GetMeApartments`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetMeApartmentsRequest struct via the builder pattern


### Return type

[**MeApartmentsResponse**](MeApartmentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetMeInvitations

> MeInvitationsResponse GetMeInvitations(ctx).Execute()

GET /me/invitations — returns invitations the current user has claimed.

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
	resp, r, err := apiClient.MeAPI.GetMeInvitations(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MeAPI.GetMeInvitations``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetMeInvitations`: MeInvitationsResponse
	fmt.Fprintf(os.Stdout, "Response from `MeAPI.GetMeInvitations`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetMeInvitationsRequest struct via the builder pattern


### Return type

[**MeInvitationsResponse**](MeInvitationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetMePushSubscriptionStatus

> MePushSubscriptionStatusResponse GetMePushSubscriptionStatus(ctx).Execute()

GET /me/push-subscription-status — returns whether the current user has any push subscription.

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
	resp, r, err := apiClient.MeAPI.GetMePushSubscriptionStatus(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MeAPI.GetMePushSubscriptionStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetMePushSubscriptionStatus`: MePushSubscriptionStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `MeAPI.GetMePushSubscriptionStatus`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetMePushSubscriptionStatusRequest struct via the builder pattern


### Return type

[**MePushSubscriptionStatusResponse**](MePushSubscriptionStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetMePushVapidPublicKey

> MePushVapidPublicKeyResponse GetMePushVapidPublicKey(ctx).Execute()

GET /me/push-vapid-public-key — returns VAPID public key for client subscription. 404 when not configured.

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
	resp, r, err := apiClient.MeAPI.GetMePushVapidPublicKey(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MeAPI.GetMePushVapidPublicKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetMePushVapidPublicKey`: MePushVapidPublicKeyResponse
	fmt.Fprintf(os.Stdout, "Response from `MeAPI.GetMePushVapidPublicKey`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetMePushVapidPublicKeyRequest struct via the builder pattern


### Return type

[**MePushVapidPublicKeyResponse**](MePushVapidPublicKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostMePushSubscription

> PostMePushSubscription(ctx).PostMePushSubscriptionPayload(postMePushSubscriptionPayload).Execute()

POST /me/push-subscriptions — store Web Push subscription for call notifications.

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
	postMePushSubscriptionPayload := *openapiclient.NewPostMePushSubscriptionPayload("Auth_example", "Endpoint_example", "P256dh_example") // PostMePushSubscriptionPayload |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.MeAPI.PostMePushSubscription(context.Background()).PostMePushSubscriptionPayload(postMePushSubscriptionPayload).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `MeAPI.PostMePushSubscription``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostMePushSubscriptionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **postMePushSubscriptionPayload** | [**PostMePushSubscriptionPayload**](PostMePushSubscriptionPayload.md) |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
