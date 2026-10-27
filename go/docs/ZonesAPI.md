# \ZonesAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateZone**](ZonesAPI.md#CreateZone) | **Post** /zones | Create a zone in the &#x60;X-Org&#x60; organization.
[**DeleteZone**](ZonesAPI.md#DeleteZone) | **Delete** /zones/{id} | Soft-delete a zone.
[**GetZone**](ZonesAPI.md#GetZone) | **Get** /zones/{id} | Get a zone by ID.
[**GetZoneSummary**](ZonesAPI.md#GetZoneSummary) | **Get** /zones/{id}/summary | Room view: the zone, its path, children, and placement counts.
[**GetZoneTree**](ZonesAPI.md#GetZoneTree) | **Get** /zones/tree | The org zone hierarchy with direct device and effective-entity counts.
[**HardDeleteZone**](ZonesAPI.md#HardDeleteZone) | **Delete** /zones/{id}/purge | Permanently delete (purge) a zone.
[**ListZones**](ZonesAPI.md#ListZones) | **Get** /zones | List zones in the &#x60;X-Org&#x60; organization subtree.
[**MergeZones**](ZonesAPI.md#MergeZones) | **Post** /zones/{id}/merge | Merge zones into the target zone.
[**RestoreZone**](ZonesAPI.md#RestoreZone) | **Post** /zones/{id}/restore | Restore a soft-deleted zone. References detached on delete are not reattached.
[**UpdateZone**](ZonesAPI.md#UpdateZone) | **Put** /zones/{id} | Update a zone (rename and/or reparent).



## CreateZone

> ZoneResponse CreateZone(ctx).XOrg(xOrg).CreateZoneRequest(createZoneRequest).Execute()

Create a zone in the `X-Org` organization.



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
	createZoneRequest := *openapiclient.NewCreateZoneRequest(*openapiclient.NewLocalizedString(map[string]string{"key": "Inner_example"})) // CreateZoneRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.ZonesAPI.CreateZone(context.Background()).XOrg(xOrg).CreateZoneRequest(createZoneRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ZonesAPI.CreateZone``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateZone`: ZoneResponse
	fmt.Fprintf(os.Stdout, "Response from `ZonesAPI.CreateZone`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCreateZoneRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **createZoneRequest** | [**CreateZoneRequest**](CreateZoneRequest.md) |  |

### Return type

[**ZoneResponse**](ZoneResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteZone

> ZoneResponse DeleteZone(ctx, id).XOrg(xOrg).Cascade(cascade).Execute()

Soft-delete a zone.



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
	cascade := "cascade_example" // string | `detach` clears every reference (children become roots, devices lose the zone, explicit entities become `mode=none`) before deleting. Without it a referenced zone gives 409. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.ZonesAPI.DeleteZone(context.Background(), id).XOrg(xOrg).Cascade(cascade).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ZonesAPI.DeleteZone``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteZone`: ZoneResponse
	fmt.Fprintf(os.Stdout, "Response from `ZonesAPI.DeleteZone`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteZoneRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **cascade** | **string** | &#x60;detach&#x60; clears every reference (children become roots, devices lose the zone, explicit entities become &#x60;mode&#x3D;none&#x60;) before deleting. Without it a referenced zone gives 409. |

### Return type

[**ZoneResponse**](ZoneResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetZone

> ZoneResponse GetZone(ctx, id).XOrg(xOrg).Execute()

Get a zone by ID.

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
	resp, r, err := apiClient.ZonesAPI.GetZone(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ZonesAPI.GetZone``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetZone`: ZoneResponse
	fmt.Fprintf(os.Stdout, "Response from `ZonesAPI.GetZone`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetZoneRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**ZoneResponse**](ZoneResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetZoneSummary

> ZoneSummaryResponse GetZoneSummary(ctx, id).XOrg(xOrg).Execute()

Room view: the zone, its path, children, and placement counts.

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
	resp, r, err := apiClient.ZonesAPI.GetZoneSummary(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ZonesAPI.GetZoneSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetZoneSummary`: ZoneSummaryResponse
	fmt.Fprintf(os.Stdout, "Response from `ZonesAPI.GetZoneSummary`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetZoneSummaryRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**ZoneSummaryResponse**](ZoneSummaryResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetZoneTree

> ZoneTreeResponse GetZoneTree(ctx).XOrg(xOrg).SiteId(siteId).Execute()

The org zone hierarchy with direct device and effective-entity counts.

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
	siteId := "siteId_example" // string | Only zones that are members of this Access site (`virtual_access` integration id). (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.ZonesAPI.GetZoneTree(context.Background()).XOrg(xOrg).SiteId(siteId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ZonesAPI.GetZoneTree``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetZoneTree`: ZoneTreeResponse
	fmt.Fprintf(os.Stdout, "Response from `ZonesAPI.GetZoneTree`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiGetZoneTreeRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **siteId** | **string** | Only zones that are members of this Access site (&#x60;virtual_access&#x60; integration id). |

### Return type

[**ZoneTreeResponse**](ZoneTreeResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## HardDeleteZone

> ZoneResponse HardDeleteZone(ctx, id).XOrg(xOrg).Execute()

Permanently delete (purge) a zone.

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
	resp, r, err := apiClient.ZonesAPI.HardDeleteZone(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ZonesAPI.HardDeleteZone``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `HardDeleteZone`: ZoneResponse
	fmt.Fprintf(os.Stdout, "Response from `ZonesAPI.HardDeleteZone`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiHardDeleteZoneRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**ZoneResponse**](ZoneResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListZones

> PaginatedResponseZoneResponse ListZones(ctx).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).ParentZoneId(parentZoneId).Root(root).Q(q).SiteId(siteId).Execute()

List zones in the `X-Org` organization subtree.

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
	outputOptions := *openapiclient.NewListZonesOutputOptionsParameter() // ListZonesOutputOptionsParameter |
	pagination := *openapiclient.NewListZonesPaginationParameter() // ListZonesPaginationParameter |
	parentZoneId := "parentZoneId_example" // string | Only direct children of this zone. (optional)
	root := true // bool | Only root zones. Cannot be combined with `parent_zone_id`. (optional)
	q := "q_example" // string | Case-insensitive substring match on the localized name. (optional)
	siteId := "siteId_example" // string | Only zones that are members of this Access site (`virtual_access` integration id). (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.ZonesAPI.ListZones(context.Background()).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).ParentZoneId(parentZoneId).Root(root).Q(q).SiteId(siteId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ZonesAPI.ListZones``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListZones`: PaginatedResponseZoneResponse
	fmt.Fprintf(os.Stdout, "Response from `ZonesAPI.ListZones`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListZonesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **outputOptions** | [**ListZonesOutputOptionsParameter**](ListZonesOutputOptionsParameter.md) |  |
 **pagination** | [**ListZonesPaginationParameter**](ListZonesPaginationParameter.md) |  |
 **parentZoneId** | **string** | Only direct children of this zone. |
 **root** | **bool** | Only root zones. Cannot be combined with &#x60;parent_zone_id&#x60;. |
 **q** | **string** | Case-insensitive substring match on the localized name. |
 **siteId** | **string** | Only zones that are members of this Access site (&#x60;virtual_access&#x60; integration id). |

### Return type

[**PaginatedResponseZoneResponse**](PaginatedResponseZoneResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## MergeZones

> MergeZonesResponse MergeZones(ctx, id).XOrg(xOrg).MergeZonesRequest(mergeZonesRequest).Execute()

Merge zones into the target zone.



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
	mergeZonesRequest := *openapiclient.NewMergeZonesRequest([]string{"SourceZoneIds_example"}) // MergeZonesRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.ZonesAPI.MergeZones(context.Background(), id).XOrg(xOrg).MergeZonesRequest(mergeZonesRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ZonesAPI.MergeZones``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `MergeZones`: MergeZonesResponse
	fmt.Fprintf(os.Stdout, "Response from `ZonesAPI.MergeZones`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiMergeZonesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **mergeZonesRequest** | [**MergeZonesRequest**](MergeZonesRequest.md) |  |

### Return type

[**MergeZonesResponse**](MergeZonesResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RestoreZone

> ZoneResponse RestoreZone(ctx, id).XOrg(xOrg).Execute()

Restore a soft-deleted zone. References detached on delete are not reattached.

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
	resp, r, err := apiClient.ZonesAPI.RestoreZone(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ZonesAPI.RestoreZone``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `RestoreZone`: ZoneResponse
	fmt.Fprintf(os.Stdout, "Response from `ZonesAPI.RestoreZone`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiRestoreZoneRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**ZoneResponse**](ZoneResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateZone

> ZoneResponse UpdateZone(ctx, id).XOrg(xOrg).UpdateZoneRequest(updateZoneRequest).Execute()

Update a zone (rename and/or reparent).



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
	updateZoneRequest := *openapiclient.NewUpdateZoneRequest() // UpdateZoneRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.ZonesAPI.UpdateZone(context.Background(), id).XOrg(xOrg).UpdateZoneRequest(updateZoneRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ZonesAPI.UpdateZone``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateZone`: ZoneResponse
	fmt.Fprintf(os.Stdout, "Response from `ZonesAPI.UpdateZone`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateZoneRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **updateZoneRequest** | [**UpdateZoneRequest**](UpdateZoneRequest.md) |  |

### Return type

[**ZoneResponse**](ZoneResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
