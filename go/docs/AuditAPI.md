# \AuditAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateAuditExport**](AuditAPI.md#CreateAuditExport) | **Post** /orgs/{org_id}/audit/exports |
[**CreateWebhook**](AuditAPI.md#CreateWebhook) | **Post** /orgs/{org_id}/webhooks |
[**DeleteWebhook**](AuditAPI.md#DeleteWebhook) | **Delete** /orgs/{org_id}/webhooks/{id} |
[**GetAuditExport**](AuditAPI.md#GetAuditExport) | **Get** /orgs/{org_id}/audit/exports/{job_id} |
[**ListAuditEvents**](AuditAPI.md#ListAuditEvents) | **Get** /orgs/{org_id}/audit/events |
[**ListAuditExports**](AuditAPI.md#ListAuditExports) | **Get** /orgs/{org_id}/audit/exports |
[**ListWebhooks**](AuditAPI.md#ListWebhooks) | **Get** /orgs/{org_id}/webhooks |
[**TestWebhook**](AuditAPI.md#TestWebhook) | **Post** /orgs/{org_id}/webhooks/{id}/test |
[**UpdateWebhook**](AuditAPI.md#UpdateWebhook) | **Patch** /orgs/{org_id}/webhooks/{id} |



## CreateAuditExport

> AuditExportJob CreateAuditExport(ctx, orgId).CreateAuditExportRequest(createAuditExportRequest).Execute()



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
	createAuditExportRequest := *openapiclient.NewCreateAuditExportRequest() // CreateAuditExportRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AuditAPI.CreateAuditExport(context.Background(), orgId).CreateAuditExportRequest(createAuditExportRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AuditAPI.CreateAuditExport``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateAuditExport`: AuditExportJob
	fmt.Fprintf(os.Stdout, "Response from `AuditAPI.CreateAuditExport`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCreateAuditExportRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **createAuditExportRequest** | [**CreateAuditExportRequest**](CreateAuditExportRequest.md) |  |

### Return type

[**AuditExportJob**](AuditExportJob.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CreateWebhook

> CreateWebhookResponse CreateWebhook(ctx, orgId).CreateWebhookRequest(createWebhookRequest).Execute()



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
	createWebhookRequest := *openapiclient.NewCreateWebhookRequest("Url_example") // CreateWebhookRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AuditAPI.CreateWebhook(context.Background(), orgId).CreateWebhookRequest(createWebhookRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AuditAPI.CreateWebhook``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateWebhook`: CreateWebhookResponse
	fmt.Fprintf(os.Stdout, "Response from `AuditAPI.CreateWebhook`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCreateWebhookRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **createWebhookRequest** | [**CreateWebhookRequest**](CreateWebhookRequest.md) |  |

### Return type

[**CreateWebhookResponse**](CreateWebhookResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteWebhook

> DeleteWebhook(ctx, orgId, id).Execute()



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
	id := "id_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.AuditAPI.DeleteWebhook(context.Background(), orgId, id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AuditAPI.DeleteWebhook``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteWebhookRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetAuditExport

> AuditExportJob GetAuditExport(ctx, orgId, jobId).Execute()



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
	jobId := "jobId_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AuditAPI.GetAuditExport(context.Background(), orgId, jobId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AuditAPI.GetAuditExport``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAuditExport`: AuditExportJob
	fmt.Fprintf(os.Stdout, "Response from `AuditAPI.GetAuditExport`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |
**jobId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetAuditExportRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



### Return type

[**AuditExportJob**](AuditExportJob.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListAuditEvents

> AuditEventsPage ListAuditEvents(ctx, orgId).EventType(eventType).Outcome(outcome).ResourceType(resourceType).ResourceId(resourceId).ResourceName(resourceName).DoorName(doorName).TargetId(targetId).ActorUserId(actorUserId).ActorGuestId(actorGuestId).ActorKind(actorKind).OrgScope(orgScope).CorrelationId(correlationId).OccurredAfter(occurredAfter).OccurredBefore(occurredBefore).Cursor(cursor).Limit(limit).Execute()



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
	eventType := "eventType_example" // string |  (optional)
	outcome := "outcome_example" // string |  (optional)
	resourceType := "resourceType_example" // string |  (optional)
	resourceId := "resourceId_example" // string |  (optional)
	resourceName := "resourceName_example" // string | Case-insensitive substring match on the resource's snapshotted display name (`details.resource_name`), e.g. `shelly`. (optional)
	doorName := "doorName_example" // string | Case-insensitive substring match on the snapshotted door name (`details.door_name`), e.g. `lobby`. Lets the Activity table answer \"who opened this door\" across every event that names it. (optional)
	targetId := "targetId_example" // string | Exact match against the snapshotted portal target ID (`details.public_portal_id`). (optional)
	actorUserId := "actorUserId_example" // string |  (optional)
	actorGuestId := "actorGuestId_example" // string | Filter to a single guest fingerprint. Disjoint from `actor_user_id`. (optional)
	actorKind := "actorKind_example" // string |  (optional)
	orgScope := "orgScope_example" // string | Organization scope for resource events. Descendants are included by default. (optional)
	correlationId := "correlationId_example" // string |  (optional)
	occurredAfter := "occurredAfter_example" // string | RFC 3339 lower bound (inclusive). Must be within the 30-day hot window. (optional)
	occurredBefore := "occurredBefore_example" // string | RFC 3339 upper bound (inclusive). (optional)
	cursor := "cursor_example" // string | Opaque cursor from a previous page's `next_cursor`. (optional)
	limit := int32(56) // int32 | Page size (default 50, max 200). (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AuditAPI.ListAuditEvents(context.Background(), orgId).EventType(eventType).Outcome(outcome).ResourceType(resourceType).ResourceId(resourceId).ResourceName(resourceName).DoorName(doorName).TargetId(targetId).ActorUserId(actorUserId).ActorGuestId(actorGuestId).ActorKind(actorKind).OrgScope(orgScope).CorrelationId(correlationId).OccurredAfter(occurredAfter).OccurredBefore(occurredBefore).Cursor(cursor).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AuditAPI.ListAuditEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListAuditEvents`: AuditEventsPage
	fmt.Fprintf(os.Stdout, "Response from `AuditAPI.ListAuditEvents`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListAuditEventsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **eventType** | **string** |  |
 **outcome** | **string** |  |
 **resourceType** | **string** |  |
 **resourceId** | **string** |  |
 **resourceName** | **string** | Case-insensitive substring match on the resource&#39;s snapshotted display name (&#x60;details.resource_name&#x60;), e.g. &#x60;shelly&#x60;. |
 **doorName** | **string** | Case-insensitive substring match on the snapshotted door name (&#x60;details.door_name&#x60;), e.g. &#x60;lobby&#x60;. Lets the Activity table answer \&quot;who opened this door\&quot; across every event that names it. |
 **targetId** | **string** | Exact match against the snapshotted portal target ID (&#x60;details.public_portal_id&#x60;). |
 **actorUserId** | **string** |  |
 **actorGuestId** | **string** | Filter to a single guest fingerprint. Disjoint from &#x60;actor_user_id&#x60;. |
 **actorKind** | **string** |  |
 **orgScope** | **string** | Organization scope for resource events. Descendants are included by default. |
 **correlationId** | **string** |  |
 **occurredAfter** | **string** | RFC 3339 lower bound (inclusive). Must be within the 30-day hot window. |
 **occurredBefore** | **string** | RFC 3339 upper bound (inclusive). |
 **cursor** | **string** | Opaque cursor from a previous page&#39;s &#x60;next_cursor&#x60;. |
 **limit** | **int32** | Page size (default 50, max 200). |

### Return type

[**AuditEventsPage**](AuditEventsPage.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListAuditExports

> []AuditExportJob ListAuditExports(ctx, orgId).Execute()



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
	resp, r, err := apiClient.AuditAPI.ListAuditExports(context.Background(), orgId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AuditAPI.ListAuditExports``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListAuditExports`: []AuditExportJob
	fmt.Fprintf(os.Stdout, "Response from `AuditAPI.ListAuditExports`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListAuditExportsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**[]AuditExportJob**](AuditExportJob.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListWebhooks

> []WebhookEndpoint ListWebhooks(ctx, orgId).Execute()



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
	resp, r, err := apiClient.AuditAPI.ListWebhooks(context.Background(), orgId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AuditAPI.ListWebhooks``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListWebhooks`: []WebhookEndpoint
	fmt.Fprintf(os.Stdout, "Response from `AuditAPI.ListWebhooks`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListWebhooksRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**[]WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## TestWebhook

> WebhookTestResponse TestWebhook(ctx, orgId, id).Execute()



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
	id := "id_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AuditAPI.TestWebhook(context.Background(), orgId, id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AuditAPI.TestWebhook``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `TestWebhook`: WebhookTestResponse
	fmt.Fprintf(os.Stdout, "Response from `AuditAPI.TestWebhook`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiTestWebhookRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



### Return type

[**WebhookTestResponse**](WebhookTestResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateWebhook

> WebhookEndpoint UpdateWebhook(ctx, orgId, id).UpdateWebhookRequest(updateWebhookRequest).Execute()



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
	id := "id_example" // string |
	updateWebhookRequest := *openapiclient.NewUpdateWebhookRequest() // UpdateWebhookRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AuditAPI.UpdateWebhook(context.Background(), orgId, id).UpdateWebhookRequest(updateWebhookRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AuditAPI.UpdateWebhook``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateWebhook`: WebhookEndpoint
	fmt.Fprintf(os.Stdout, "Response from `AuditAPI.UpdateWebhook`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateWebhookRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **updateWebhookRequest** | [**UpdateWebhookRequest**](UpdateWebhookRequest.md) |  |

### Return type

[**WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
