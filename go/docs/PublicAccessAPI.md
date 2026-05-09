# \PublicAccessAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GetPublicInvite**](PublicAccessAPI.md#GetPublicInvite) | **Get** /public/access/invites/{inviteToken} |
[**GetPublicPortal**](PublicAccessAPI.md#GetPublicPortal) | **Get** /public/access/portals/{publicPortalId} |
[**GetPublicPortalReachable**](PublicAccessAPI.md#GetPublicPortalReachable) | **Get** /public/access/portals/{publicPortalId}/reachable |
[**GetPublicPortalTargets**](PublicAccessAPI.md#GetPublicPortalTargets) | **Get** /public/access/portals/{publicPortalId}/targets |
[**GetPublicSession**](PublicAccessAPI.md#GetPublicSession) | **Get** /public/access/sessions/{sessionId} |
[**GetPublicSessionStreams**](PublicAccessAPI.md#GetPublicSessionStreams) | **Get** /public/access/sessions/{sessionId}/streams |
[**PostPublicInviteClaim**](PublicAccessAPI.md#PostPublicInviteClaim) | **Post** /public/access/invites/{inviteToken}/claim |
[**PostPublicInviteExecute**](PublicAccessAPI.md#PostPublicInviteExecute) | **Post** /public/access/invites/{inviteToken}/execute |
[**PostPublicInviteSession**](PublicAccessAPI.md#PostPublicInviteSession) | **Post** /public/access/invites/{inviteToken}/session |
[**PostPublicPortalLights**](PublicAccessAPI.md#PostPublicPortalLights) | **Post** /public/access/portals/{publicPortalId}/lights |
[**PostPublicPortalOpen**](PublicAccessAPI.md#PostPublicPortalOpen) | **Post** /public/access/portals/{publicPortalId}/open |
[**PostPublicPortalSessions**](PublicAccessAPI.md#PostPublicPortalSessions) | **Post** /public/access/portals/{publicPortalId}/sessions |
[**PostPublicSessionCancel**](PublicAccessAPI.md#PostPublicSessionCancel) | **Post** /public/access/sessions/{sessionId}/cancel | Caller hangs up while ringing so callees polling GET session can dismiss incoming UI.
[**PostPublicSessionDecline**](PublicAccessAPI.md#PostPublicSessionDecline) | **Post** /public/access/sessions/{sessionId}/decline | Callee rejects the ring before answering; caller can observe &#x60;state&#x60; via GET session.
[**PostPublicSessionLights**](PublicAccessAPI.md#PostPublicSessionLights) | **Post** /public/access/sessions/{sessionId}/lights |
[**PostPublicSessionNotifyMessage**](PublicAccessAPI.md#PostPublicSessionNotifyMessage) | **Post** /public/access/sessions/{sessionId}/notify-message | Caller notifies apartment residents with the chat message text (Web Push).
[**PostPublicSessionOpen**](PublicAccessAPI.md#PostPublicSessionOpen) | **Post** /public/access/sessions/{sessionId}/open |



## GetPublicInvite

> PublicInviteResponse GetPublicInvite(ctx, inviteToken).Execute()



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
	inviteToken := "inviteToken_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.PublicAccessAPI.GetPublicInvite(context.Background(), inviteToken).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.GetPublicInvite``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPublicInvite`: PublicInviteResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAccessAPI.GetPublicInvite`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**inviteToken** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetPublicInviteRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**PublicInviteResponse**](PublicInviteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetPublicPortal

> PublicPortalResponse GetPublicPortal(ctx, publicPortalId).Execute()



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
	publicPortalId := "publicPortalId_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.PublicAccessAPI.GetPublicPortal(context.Background(), publicPortalId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.GetPublicPortal``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPublicPortal`: PublicPortalResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAccessAPI.GetPublicPortal`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**publicPortalId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetPublicPortalRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**PublicPortalResponse**](PublicPortalResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetPublicPortalReachable

> PublicPortalReachableResponse GetPublicPortalReachable(ctx, publicPortalId).Execute()



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
	publicPortalId := "publicPortalId_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.PublicAccessAPI.GetPublicPortalReachable(context.Background(), publicPortalId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.GetPublicPortalReachable``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPublicPortalReachable`: PublicPortalReachableResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAccessAPI.GetPublicPortalReachable`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**publicPortalId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetPublicPortalReachableRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**PublicPortalReachableResponse**](PublicPortalReachableResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetPublicPortalTargets

> PublicPortalTargetsResponse GetPublicPortalTargets(ctx, publicPortalId).Execute()



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
	publicPortalId := "publicPortalId_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.PublicAccessAPI.GetPublicPortalTargets(context.Background(), publicPortalId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.GetPublicPortalTargets``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPublicPortalTargets`: PublicPortalTargetsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAccessAPI.GetPublicPortalTargets`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**publicPortalId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetPublicPortalTargetsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**PublicPortalTargetsResponse**](PublicPortalTargetsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetPublicSession

> PublicSessionResponse GetPublicSession(ctx, sessionId).Token(token).Execute()



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
	sessionId := "sessionId_example" // string |
	token := "token_example" // string | Session token (alternative to Bearer when headers stripped) (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.PublicAccessAPI.GetPublicSession(context.Background(), sessionId).Token(token).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.GetPublicSession``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPublicSession`: PublicSessionResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAccessAPI.GetPublicSession`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**sessionId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetPublicSessionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **token** | **string** | Session token (alternative to Bearer when headers stripped) |

### Return type

[**PublicSessionResponse**](PublicSessionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetPublicSessionStreams

> PublicSessionStreamsResponse GetPublicSessionStreams(ctx, sessionId).Token(token).Execute()



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
	sessionId := "sessionId_example" // string |
	token := "token_example" // string |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.PublicAccessAPI.GetPublicSessionStreams(context.Background(), sessionId).Token(token).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.GetPublicSessionStreams``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPublicSessionStreams`: PublicSessionStreamsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAccessAPI.GetPublicSessionStreams`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**sessionId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetPublicSessionStreamsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **token** | **string** |  |

### Return type

[**PublicSessionStreamsResponse**](PublicSessionStreamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostPublicInviteClaim

> PostPublicInviteClaim(ctx, inviteToken).Execute()



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
	inviteToken := "inviteToken_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.PublicAccessAPI.PostPublicInviteClaim(context.Background(), inviteToken).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.PostPublicInviteClaim``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**inviteToken** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostPublicInviteClaimRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostPublicInviteExecute

> PublicInviteExecuteResponse PostPublicInviteExecute(ctx, inviteToken).PublicInviteExecuteRequest(publicInviteExecuteRequest).Execute()



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
	inviteToken := "inviteToken_example" // string |
	publicInviteExecuteRequest := *openapiclient.NewPublicInviteExecuteRequest("GrantId_example") // PublicInviteExecuteRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.PublicAccessAPI.PostPublicInviteExecute(context.Background(), inviteToken).PublicInviteExecuteRequest(publicInviteExecuteRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.PostPublicInviteExecute``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostPublicInviteExecute`: PublicInviteExecuteResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAccessAPI.PostPublicInviteExecute`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**inviteToken** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostPublicInviteExecuteRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **publicInviteExecuteRequest** | [**PublicInviteExecuteRequest**](PublicInviteExecuteRequest.md) |  |

### Return type

[**PublicInviteExecuteResponse**](PublicInviteExecuteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostPublicInviteSession

> PostPublicInviteSession(ctx, inviteToken).Execute()



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
	inviteToken := "inviteToken_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.PublicAccessAPI.PostPublicInviteSession(context.Background(), inviteToken).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.PostPublicInviteSession``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**inviteToken** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostPublicInviteSessionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostPublicPortalLights

> PostPublicPortalLights(ctx, publicPortalId).Execute()



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
	publicPortalId := "publicPortalId_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.PublicAccessAPI.PostPublicPortalLights(context.Background(), publicPortalId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.PostPublicPortalLights``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**publicPortalId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostPublicPortalLightsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostPublicPortalOpen

> PostPublicPortalOpen(ctx, publicPortalId).Execute()



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
	publicPortalId := "publicPortalId_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.PublicAccessAPI.PostPublicPortalOpen(context.Background(), publicPortalId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.PostPublicPortalOpen``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**publicPortalId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostPublicPortalOpenRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostPublicPortalSessions

> PublicPortalCreateSessionResponse PostPublicPortalSessions(ctx, publicPortalId).PublicPortalCreateSessionRequest(publicPortalCreateSessionRequest).Execute()



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
	publicPortalId := "publicPortalId_example" // string |
	publicPortalCreateSessionRequest := *openapiclient.NewPublicPortalCreateSessionRequest("Mode_example", "TargetEntityId_example") // PublicPortalCreateSessionRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.PublicAccessAPI.PostPublicPortalSessions(context.Background(), publicPortalId).PublicPortalCreateSessionRequest(publicPortalCreateSessionRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.PostPublicPortalSessions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostPublicPortalSessions`: PublicPortalCreateSessionResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAccessAPI.PostPublicPortalSessions`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**publicPortalId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostPublicPortalSessionsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **publicPortalCreateSessionRequest** | [**PublicPortalCreateSessionRequest**](PublicPortalCreateSessionRequest.md) |  |

### Return type

[**PublicPortalCreateSessionResponse**](PublicPortalCreateSessionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostPublicSessionCancel

> PostPublicSessionCancel(ctx, sessionId).Token(token).Execute()

Caller hangs up while ringing so callees polling GET session can dismiss incoming UI.

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
	sessionId := "sessionId_example" // string |
	token := "token_example" // string |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.PublicAccessAPI.PostPublicSessionCancel(context.Background(), sessionId).Token(token).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.PostPublicSessionCancel``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**sessionId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostPublicSessionCancelRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **token** | **string** |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostPublicSessionDecline

> PostPublicSessionDecline(ctx, sessionId).Token(token).Execute()

Callee rejects the ring before answering; caller can observe `state` via GET session.

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
	sessionId := "sessionId_example" // string |
	token := "token_example" // string |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.PublicAccessAPI.PostPublicSessionDecline(context.Background(), sessionId).Token(token).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.PostPublicSessionDecline``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**sessionId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostPublicSessionDeclineRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **token** | **string** |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostPublicSessionLights

> PostPublicSessionLights(ctx, sessionId).Token(token).Execute()



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
	sessionId := "sessionId_example" // string |
	token := "token_example" // string |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.PublicAccessAPI.PostPublicSessionLights(context.Background(), sessionId).Token(token).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.PostPublicSessionLights``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**sessionId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostPublicSessionLightsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **token** | **string** |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostPublicSessionNotifyMessage

> PostPublicSessionNotifyMessage(ctx, sessionId).NotifyPortalMessageBody(notifyPortalMessageBody).Token(token).Execute()

Caller notifies apartment residents with the chat message text (Web Push).

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
	sessionId := "sessionId_example" // string |
	notifyPortalMessageBody := *openapiclient.NewNotifyPortalMessageBody("Text_example") // NotifyPortalMessageBody |
	token := "token_example" // string |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.PublicAccessAPI.PostPublicSessionNotifyMessage(context.Background(), sessionId).NotifyPortalMessageBody(notifyPortalMessageBody).Token(token).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.PostPublicSessionNotifyMessage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**sessionId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostPublicSessionNotifyMessageRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **notifyPortalMessageBody** | [**NotifyPortalMessageBody**](NotifyPortalMessageBody.md) |  |
 **token** | **string** |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostPublicSessionOpen

> PostPublicSessionOpen(ctx, sessionId).Token(token).Execute()



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
	sessionId := "sessionId_example" // string |
	token := "token_example" // string |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.PublicAccessAPI.PostPublicSessionOpen(context.Background(), sessionId).Token(token).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAccessAPI.PostPublicSessionOpen``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**sessionId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostPublicSessionOpenRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **token** | **string** |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
