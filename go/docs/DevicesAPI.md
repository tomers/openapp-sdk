# \DevicesAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateDevice**](DevicesAPI.md#CreateDevice) | **Post** /devices | Create a device in an organization.
[**DeleteDevice**](DevicesAPI.md#DeleteDevice) | **Delete** /devices/{id} | Soft-delete a device.
[**GetDevice**](DevicesAPI.md#GetDevice) | **Get** /devices/{id} | Get a device by ID.
[**GetDeviceHold**](DevicesAPI.md#GetDeviceHold) | **Get** /devices/{id}/hold |
[**GetDeviceMetadataDefinition**](DevicesAPI.md#GetDeviceMetadataDefinition) | **Get** /devices/{id}/metadata-definition | Get provider-specific JSON schema for &#x60;device_metadata&#x60;, for a given device.
[**GetDoorRestrictions**](DevicesAPI.md#GetDoorRestrictions) | **Get** /devices/{id}/door-restrictions | GET /devices/{id}/door-restrictions — list listing entity IDs allowed to open this door. Empty &#x3D; no restrictions (all building members can open). Only for virtual_access_portal devices.
[**HardDeleteDevice**](DevicesAPI.md#HardDeleteDevice) | **Delete** /devices/{id}/purge | Permanently delete (purge) a device.
[**ListDevices**](DevicesAPI.md#ListDevices) | **Get** /devices | List devices for the organization context (X-Org).
[**PostRefreshDeviceChannelCount**](DevicesAPI.md#PostRefreshDeviceChannelCount) | **Post** /devices/{id}/channel-count/refresh | Refresh the provider-reported channel count for one linked device.
[**PostVirtualAccessOpen**](DevicesAPI.md#PostVirtualAccessOpen) | **Post** /devices/{id}/virtual-access/open | POST /devices/{id}/virtual-access/open — open all configured openers on a portal device.
[**PutDeviceHold**](DevicesAPI.md#PutDeviceHold) | **Put** /devices/{id}/hold |
[**PutDoorRestrictions**](DevicesAPI.md#PutDoorRestrictions) | **Put** /devices/{id}/door-restrictions | PUT /devices/{id}/door-restrictions — set which listings can open this door. Empty array &#x3D; no restrictions (all building residents). Only for virtual_access_portal devices.
[**RestoreDevice**](DevicesAPI.md#RestoreDevice) | **Post** /devices/{id}/restore | Restore a soft-deleted device.
[**UpdateDevice**](DevicesAPI.md#UpdateDevice) | **Put** /devices/{id} | Update a device.



## CreateDevice

> DeviceResponse CreateDevice(ctx).XOrg(xOrg).CreateDeviceRequest(createDeviceRequest).IncludeDeleted(includeDeleted).Execute()

Create a device in an organization.



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
	xOrg := "xOrg_example" // string | Organization context (required)
	createDeviceRequest := *openapiclient.NewCreateDeviceRequest("IntegrationId_example", *openapiclient.NewLocalizedString(map[string]string{"key": "Inner_example"}), "OrgId_example") // CreateDeviceRequest |
	includeDeleted := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DevicesAPI.CreateDevice(context.Background()).XOrg(xOrg).CreateDeviceRequest(createDeviceRequest).IncludeDeleted(includeDeleted).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.CreateDevice``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateDevice`: DeviceResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.CreateDevice`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCreateDeviceRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** | Organization context (required) |
 **createDeviceRequest** | [**CreateDeviceRequest**](CreateDeviceRequest.md) |  |
 **includeDeleted** | **bool** |  |

### Return type

[**DeviceResponse**](DeviceResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteDevice

> DeviceResponse DeleteDevice(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).Recursive(recursive).Execute()

Soft-delete a device.

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
	xOrg := "xOrg_example" // string |
	includeDeleted := true // bool |  (optional)
	recursive := true // bool | If true, delete (or purge) all device entities first. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DevicesAPI.DeleteDevice(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).Recursive(recursive).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.DeleteDevice``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteDevice`: DeviceResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.DeleteDevice`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteDeviceRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **recursive** | **bool** | If true, delete (or purge) all device entities first. |

### Return type

[**DeviceResponse**](DeviceResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetDevice

> DeviceResponse GetDevice(ctx, id).XOrg(xOrg).OutputOptions(outputOptions).IncludeStale(includeStale).Execute()

Get a device by ID.

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
	id := "id_example" // string | Device ULID
	xOrg := "xOrg_example" // string |
	outputOptions := *openapiclient.NewSingleResourceOutputOptionsQuery() // SingleResourceOutputOptionsQuery |
	includeStale := true // bool | When true, include `stale` if the integration provider supports stale device detection. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DevicesAPI.GetDevice(context.Background(), id).XOrg(xOrg).OutputOptions(outputOptions).IncludeStale(includeStale).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.GetDevice``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetDevice`: DeviceResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.GetDevice`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Device ULID |

### Other Parameters

Other parameters are passed through a pointer to a apiGetDeviceRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **outputOptions** | [**SingleResourceOutputOptionsQuery**](SingleResourceOutputOptionsQuery.md) |  |
 **includeStale** | **bool** | When true, include &#x60;stale&#x60; if the integration provider supports stale device detection. |

### Return type

[**DeviceResponse**](DeviceResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetDeviceHold

> DoorHoldResponse GetDeviceHold(ctx, id).Execute()



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
	resp, r, err := apiClient.DevicesAPI.GetDeviceHold(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.GetDeviceHold``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetDeviceHold`: DoorHoldResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.GetDeviceHold`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetDeviceHoldRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**DoorHoldResponse**](DoorHoldResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetDeviceMetadataDefinition

> DeviceMetadataDefinitionResponse GetDeviceMetadataDefinition(ctx, id).XOrg(xOrg).Execute()

Get provider-specific JSON schema for `device_metadata`, for a given device.

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
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DevicesAPI.GetDeviceMetadataDefinition(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.GetDeviceMetadataDefinition``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetDeviceMetadataDefinition`: DeviceMetadataDefinitionResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.GetDeviceMetadataDefinition`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetDeviceMetadataDefinitionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**DeviceMetadataDefinitionResponse**](DeviceMetadataDefinitionResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetDoorRestrictions

> DoorRestrictionsResponse GetDoorRestrictions(ctx, id).Execute()

GET /devices/{id}/door-restrictions — list listing entity IDs allowed to open this door. Empty = no restrictions (all building members can open). Only for virtual_access_portal devices.

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
	resp, r, err := apiClient.DevicesAPI.GetDoorRestrictions(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.GetDoorRestrictions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetDoorRestrictions`: DoorRestrictionsResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.GetDoorRestrictions`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetDoorRestrictionsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**DoorRestrictionsResponse**](DoorRestrictionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## HardDeleteDevice

> DeviceResponse HardDeleteDevice(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).Recursive(recursive).Execute()

Permanently delete (purge) a device.

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
	xOrg := "xOrg_example" // string |
	includeDeleted := true // bool |  (optional)
	recursive := true // bool | If true, delete (or purge) all device entities first. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DevicesAPI.HardDeleteDevice(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).Recursive(recursive).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.HardDeleteDevice``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `HardDeleteDevice`: DeviceResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.HardDeleteDevice`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiHardDeleteDeviceRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **recursive** | **bool** | If true, delete (or purge) all device entities first. |

### Return type

[**DeviceResponse**](DeviceResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListDevices

> []DeviceResponse ListDevices(ctx).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).IntegrationId(integrationId).DeviceKind(deviceKind).ExternalId(externalId).Q(q).Sort(sort).Locale(locale).HasExternalId(hasExternalId).HasGo2rtcChannel(hasGo2rtcChannel).IncludeStale(includeStale).Execute()

List devices for the organization context (X-Org).

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
	xOrg := "xOrg_example" // string |
	outputOptions := *openapiclient.NewMultiResourceOutputOptionsQuery() // MultiResourceOutputOptionsQuery |
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |
	integrationId := "integrationId_example" // string | Optional filter: only devices belonging to this integration. (optional)
	deviceKind := "deviceKind_example" // string | Optional filter: only devices with metadata.kind equal to this value (e.g. virtual_access_portal). Requires integration_id. Filtering done at SQL level. (optional)
	externalId := "externalId_example" // string | Optional filter: only devices with this external_id. Requires integration_id and device_kind. Returns at most 1 device. Filtering done at SQL level. (optional)
	q := "q_example" // string | Case-insensitive substring match on localized device name (JSON). Best-effort when `integration_id` is set (SQL ILIKE). (optional)
	sort := "sort_example" // string | Server-side ordering for org-wide lists: `name:asc`, `name:desc`, `created_at:asc`, `created_at:desc`. Ignored when `integration_id` is set. Defaults to insertion order when omitted. (optional)
	locale := "locale_example" // string | Locale used to resolve the localized `name` when sorting by name (e.g. `en`). Defaults to `en`. (optional)
	hasExternalId := true // bool | When true, only devices with a non-empty `external_id`. Requires `integration_id`. (optional)
	hasGo2rtcChannel := true // bool | When true, only devices whose metadata JSON has a non-empty `channel` (go2rtc cameras). Requires `integration_id`. (optional)
	includeStale := true // bool | When true with `integration_id`, include `stale` per device when the provider supports it. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DevicesAPI.ListDevices(context.Background()).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).IntegrationId(integrationId).DeviceKind(deviceKind).ExternalId(externalId).Q(q).Sort(sort).Locale(locale).HasExternalId(hasExternalId).HasGo2rtcChannel(hasGo2rtcChannel).IncludeStale(includeStale).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.ListDevices``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListDevices`: []DeviceResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.ListDevices`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListDevicesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **outputOptions** | [**MultiResourceOutputOptionsQuery**](MultiResourceOutputOptionsQuery.md) |  |
 **pagination** | [**PaginationQuery**](PaginationQuery.md) |  |
 **integrationId** | **string** | Optional filter: only devices belonging to this integration. |
 **deviceKind** | **string** | Optional filter: only devices with metadata.kind equal to this value (e.g. virtual_access_portal). Requires integration_id. Filtering done at SQL level. |
 **externalId** | **string** | Optional filter: only devices with this external_id. Requires integration_id and device_kind. Returns at most 1 device. Filtering done at SQL level. |
 **q** | **string** | Case-insensitive substring match on localized device name (JSON). Best-effort when &#x60;integration_id&#x60; is set (SQL ILIKE). |
 **sort** | **string** | Server-side ordering for org-wide lists: &#x60;name:asc&#x60;, &#x60;name:desc&#x60;, &#x60;created_at:asc&#x60;, &#x60;created_at:desc&#x60;. Ignored when &#x60;integration_id&#x60; is set. Defaults to insertion order when omitted. |
 **locale** | **string** | Locale used to resolve the localized &#x60;name&#x60; when sorting by name (e.g. &#x60;en&#x60;). Defaults to &#x60;en&#x60;. |
 **hasExternalId** | **bool** | When true, only devices with a non-empty &#x60;external_id&#x60;. Requires &#x60;integration_id&#x60;. |
 **hasGo2rtcChannel** | **bool** | When true, only devices whose metadata JSON has a non-empty &#x60;channel&#x60; (go2rtc cameras). Requires &#x60;integration_id&#x60;. |
 **includeStale** | **bool** | When true with &#x60;integration_id&#x60;, include &#x60;stale&#x60; per device when the provider supports it. |

### Return type

[**[]DeviceResponse**](DeviceResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostRefreshDeviceChannelCount

> interface{} PostRefreshDeviceChannelCount(ctx, id).XOrg(xOrg).Execute()

Refresh the provider-reported channel count for one linked device.

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
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DevicesAPI.PostRefreshDeviceChannelCount(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.PostRefreshDeviceChannelCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostRefreshDeviceChannelCount`: interface{}
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.PostRefreshDeviceChannelCount`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostRefreshDeviceChannelCountRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

**interface{}**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostVirtualAccessOpen

> VirtualAccessOpenResponse PostVirtualAccessOpen(ctx, id).Execute()

POST /devices/{id}/virtual-access/open — open all configured openers on a portal device.

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
	resp, r, err := apiClient.DevicesAPI.PostVirtualAccessOpen(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.PostVirtualAccessOpen``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostVirtualAccessOpen`: VirtualAccessOpenResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.PostVirtualAccessOpen`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostVirtualAccessOpenRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**VirtualAccessOpenResponse**](VirtualAccessOpenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutDeviceHold

> DoorHoldResponse PutDeviceHold(ctx, id).PutHoldRequest(putHoldRequest).Execute()



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
	putHoldRequest := *openapiclient.NewPutHoldRequest("Mode_example") // PutHoldRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DevicesAPI.PutDeviceHold(context.Background(), id).PutHoldRequest(putHoldRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.PutDeviceHold``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutDeviceHold`: DoorHoldResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.PutDeviceHold`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPutDeviceHoldRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **putHoldRequest** | [**PutHoldRequest**](PutHoldRequest.md) |  |

### Return type

[**DoorHoldResponse**](DoorHoldResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutDoorRestrictions

> DoorRestrictionsResponse PutDoorRestrictions(ctx, id).SetDoorRestrictionsPayload(setDoorRestrictionsPayload).Execute()

PUT /devices/{id}/door-restrictions — set which listings can open this door. Empty array = no restrictions (all building residents). Only for virtual_access_portal devices.

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
	setDoorRestrictionsPayload := *openapiclient.NewSetDoorRestrictionsPayload([]string{"ListingEntityIds_example"}) // SetDoorRestrictionsPayload |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DevicesAPI.PutDoorRestrictions(context.Background(), id).SetDoorRestrictionsPayload(setDoorRestrictionsPayload).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.PutDoorRestrictions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutDoorRestrictions`: DoorRestrictionsResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.PutDoorRestrictions`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPutDoorRestrictionsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **setDoorRestrictionsPayload** | [**SetDoorRestrictionsPayload**](SetDoorRestrictionsPayload.md) |  |

### Return type

[**DoorRestrictionsResponse**](DoorRestrictionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RestoreDevice

> DeviceResponse RestoreDevice(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).Execute()

Restore a soft-deleted device.

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
	xOrg := "xOrg_example" // string |
	includeDeleted := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DevicesAPI.RestoreDevice(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.RestoreDevice``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `RestoreDevice`: DeviceResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.RestoreDevice`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiRestoreDeviceRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |

### Return type

[**DeviceResponse**](DeviceResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateDevice

> DeviceResponse UpdateDevice(ctx, id).XOrg(xOrg).UpdateDeviceRequest(updateDeviceRequest).IncludeDeleted(includeDeleted).Execute()

Update a device.

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
	xOrg := "xOrg_example" // string |
	updateDeviceRequest := *openapiclient.NewUpdateDeviceRequest() // UpdateDeviceRequest |
	includeDeleted := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DevicesAPI.UpdateDevice(context.Background(), id).XOrg(xOrg).UpdateDeviceRequest(updateDeviceRequest).IncludeDeleted(includeDeleted).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DevicesAPI.UpdateDevice``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateDevice`: DeviceResponse
	fmt.Fprintf(os.Stdout, "Response from `DevicesAPI.UpdateDevice`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateDeviceRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **updateDeviceRequest** | [**UpdateDeviceRequest**](UpdateDeviceRequest.md) |  |
 **includeDeleted** | **bool** |  |

### Return type

[**DeviceResponse**](DeviceResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
