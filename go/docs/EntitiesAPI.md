# \EntitiesAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateEntity**](EntitiesAPI.md#CreateEntity) | **Post** /entities | Create an entity on a device.
[**DeleteEntity**](EntitiesAPI.md#DeleteEntity) | **Delete** /entities/{id} | Soft-delete an entity.
[**ExecuteEntityAction**](EntitiesAPI.md#ExecuteEntityAction) | **Post** /entities/{id}/actions/{action_id} | Execute an entity action using the integration engine.
[**GetDeviceEntityMetadataDefinition**](EntitiesAPI.md#GetDeviceEntityMetadataDefinition) | **Get** /devices/{device_id}/entities/metadata-definition | Get provider-specific JSON schema for &#x60;entity_metadata&#x60;, for creating a new entity on a device.
[**GetEntity**](EntitiesAPI.md#GetEntity) | **Get** /entities/{id} | Get an entity by ID.
[**GetEntityMetadataDefinition**](EntitiesAPI.md#GetEntityMetadataDefinition) | **Get** /entities/{id}/metadata-definition | Get provider-specific JSON schema for &#x60;entity_metadata&#x60;, for a given entity.
[**HardDeleteEntity**](EntitiesAPI.md#HardDeleteEntity) | **Delete** /entities/{id}/purge | Permanently delete (purge) an entity.
[**ListDeviceApartmentFloors**](EntitiesAPI.md#ListDeviceApartmentFloors) | **Get** /devices/{device_id}/apartment-floors |
[**ListDeviceEntities**](EntitiesAPI.md#ListDeviceEntities) | **Get** /devices/{device_id}/entities | List entities for a device.
[**ListEntities**](EntitiesAPI.md#ListEntities) | **Get** /entities |
[**PatchEntity**](EntitiesAPI.md#PatchEntity) | **Patch** /entities/{id} | Partially update entity metadata (shallow merge). Other fields (&#x60;name&#x60;, etc.) are unchanged.
[**RestoreEntity**](EntitiesAPI.md#RestoreEntity) | **Post** /entities/{id}/restore | Restore a soft-deleted entity.
[**UpdateEntity**](EntitiesAPI.md#UpdateEntity) | **Put** /entities/{id} | Update an entity.



## CreateEntity

> EntityResponse CreateEntity(ctx).XOrg(xOrg).CreateEntityRequest(createEntityRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Create an entity on a device.

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
	createEntityRequest := *openapiclient.NewCreateEntityRequest("DeviceId_example", "EntityType_example") // CreateEntityRequest |
	includeDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EntitiesAPI.CreateEntity(context.Background()).XOrg(xOrg).CreateEntityRequest(createEntityRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.CreateEntity``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateEntity`: EntityResponse
	fmt.Fprintf(os.Stdout, "Response from `EntitiesAPI.CreateEntity`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCreateEntityRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **createEntityRequest** | [**CreateEntityRequest**](CreateEntityRequest.md) |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**EntityResponse**](EntityResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteEntity

> EntityResponse DeleteEntity(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Soft-delete an entity.

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
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EntitiesAPI.DeleteEntity(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.DeleteEntity``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteEntity`: EntityResponse
	fmt.Fprintf(os.Stdout, "Response from `EntitiesAPI.DeleteEntity`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteEntityRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**EntityResponse**](EntityResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ExecuteEntityAction

> ExecuteEntityAction(ctx, id, actionId).Body(body).Execute()

Execute an entity action using the integration engine.

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
	actionId := "actionId_example" // string |
	body := interface{}(987) // interface{} |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.EntitiesAPI.ExecuteEntityAction(context.Background(), id, actionId).Body(body).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.ExecuteEntityAction``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**actionId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiExecuteEntityActionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **body** | **interface{}** |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetDeviceEntityMetadataDefinition

> EntityMetadataDefinitionResponse GetDeviceEntityMetadataDefinition(ctx, deviceId).XOrg(xOrg).EntityType(entityType).Execute()

Get provider-specific JSON schema for `entity_metadata`, for creating a new entity on a device.

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
	deviceId := "deviceId_example" // string |
	xOrg := "xOrg_example" // string |
	entityType := "entityType_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EntitiesAPI.GetDeviceEntityMetadataDefinition(context.Background(), deviceId).XOrg(xOrg).EntityType(entityType).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.GetDeviceEntityMetadataDefinition``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetDeviceEntityMetadataDefinition`: EntityMetadataDefinitionResponse
	fmt.Fprintf(os.Stdout, "Response from `EntitiesAPI.GetDeviceEntityMetadataDefinition`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**deviceId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetDeviceEntityMetadataDefinitionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **entityType** | **string** |  |

### Return type

[**EntityMetadataDefinitionResponse**](EntityMetadataDefinitionResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetEntity

> EntityResponse GetEntity(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Get an entity by ID.

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
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EntitiesAPI.GetEntity(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.GetEntity``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetEntity`: EntityResponse
	fmt.Fprintf(os.Stdout, "Response from `EntitiesAPI.GetEntity`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetEntityRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**EntityResponse**](EntityResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetEntityMetadataDefinition

> EntityMetadataDefinitionResponse GetEntityMetadataDefinition(ctx, id).XOrg(xOrg).Execute()

Get provider-specific JSON schema for `entity_metadata`, for a given entity.

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EntitiesAPI.GetEntityMetadataDefinition(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.GetEntityMetadataDefinition``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetEntityMetadataDefinition`: EntityMetadataDefinitionResponse
	fmt.Fprintf(os.Stdout, "Response from `EntitiesAPI.GetEntityMetadataDefinition`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetEntityMetadataDefinitionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**EntityMetadataDefinitionResponse**](EntityMetadataDefinitionResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## HardDeleteEntity

> EntityResponse HardDeleteEntity(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Permanently delete (purge) an entity.

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
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EntitiesAPI.HardDeleteEntity(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.HardDeleteEntity``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `HardDeleteEntity`: EntityResponse
	fmt.Fprintf(os.Stdout, "Response from `EntitiesAPI.HardDeleteEntity`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiHardDeleteEntityRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**EntityResponse**](EntityResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListDeviceApartmentFloors

> ApartmentFloorListResponse ListDeviceApartmentFloors(ctx, deviceId).XOrg(xOrg).Execute()



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
	deviceId := "deviceId_example" // string |
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EntitiesAPI.ListDeviceApartmentFloors(context.Background(), deviceId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.ListDeviceApartmentFloors``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListDeviceApartmentFloors`: ApartmentFloorListResponse
	fmt.Fprintf(os.Stdout, "Response from `EntitiesAPI.ListDeviceApartmentFloors`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**deviceId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListDeviceApartmentFloorsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**ApartmentFloorListResponse**](ApartmentFloorListResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListDeviceEntities

> PaginatedResponse ListDeviceEntities(ctx, deviceId).XOrg(xOrg).Pagination(pagination).IncludeDeleted(includeDeleted).OnlyDeleted(onlyDeleted).IncludeMetadata(includeMetadata).EntityType(entityType).Execute()

List entities for a device.

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
	deviceId := "deviceId_example" // string |
	xOrg := "xOrg_example" // string |
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |
	includeDeleted := true // bool |  (optional)
	onlyDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)
	entityType := "entityType_example" // string | When set, only entities of this type are returned (e.g. `apartment`). (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EntitiesAPI.ListDeviceEntities(context.Background(), deviceId).XOrg(xOrg).Pagination(pagination).IncludeDeleted(includeDeleted).OnlyDeleted(onlyDeleted).IncludeMetadata(includeMetadata).EntityType(entityType).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.ListDeviceEntities``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListDeviceEntities`: PaginatedResponse
	fmt.Fprintf(os.Stdout, "Response from `EntitiesAPI.ListDeviceEntities`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**deviceId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListDeviceEntitiesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **pagination** | [**PaginationQuery**](PaginationQuery.md) |  |
 **includeDeleted** | **bool** |  |
 **onlyDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |
 **entityType** | **string** | When set, only entities of this type are returned (e.g. &#x60;apartment&#x60;). |

### Return type

[**PaginatedResponse**](PaginatedResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListEntities

> PaginatedResponse ListEntities(ctx).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).ZoneId(zoneId).Execute()



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
	outputOptions := *openapiclient.NewMultiResourceOutputOptionsQuery(false, false, false) // MultiResourceOutputOptionsQuery |
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |
	zoneId := "zoneId_example" // string | Optional filter: only entities in this zone. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EntitiesAPI.ListEntities(context.Background()).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).ZoneId(zoneId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.ListEntities``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListEntities`: PaginatedResponse
	fmt.Fprintf(os.Stdout, "Response from `EntitiesAPI.ListEntities`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListEntitiesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **outputOptions** | [**MultiResourceOutputOptionsQuery**](MultiResourceOutputOptionsQuery.md) |  |
 **pagination** | [**PaginationQuery**](PaginationQuery.md) |  |
 **zoneId** | **string** | Optional filter: only entities in this zone. |

### Return type

[**PaginatedResponse**](PaginatedResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PatchEntity

> EntityResponse PatchEntity(ctx, id).XOrg(xOrg).PatchEntityRequest(patchEntityRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Partially update entity metadata (shallow merge). Other fields (`name`, etc.) are unchanged.

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
	patchEntityRequest := *openapiclient.NewPatchEntityRequest(map[string]interface{}{"key": interface{}(123)}) // PatchEntityRequest |
	includeDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EntitiesAPI.PatchEntity(context.Background(), id).XOrg(xOrg).PatchEntityRequest(patchEntityRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.PatchEntity``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PatchEntity`: EntityResponse
	fmt.Fprintf(os.Stdout, "Response from `EntitiesAPI.PatchEntity`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPatchEntityRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **patchEntityRequest** | [**PatchEntityRequest**](PatchEntityRequest.md) |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**EntityResponse**](EntityResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RestoreEntity

> EntityResponse RestoreEntity(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Restore a soft-deleted entity.

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
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EntitiesAPI.RestoreEntity(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.RestoreEntity``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `RestoreEntity`: EntityResponse
	fmt.Fprintf(os.Stdout, "Response from `EntitiesAPI.RestoreEntity`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiRestoreEntityRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**EntityResponse**](EntityResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateEntity

> EntityResponse UpdateEntity(ctx, id).XOrg(xOrg).UpdateEntityRequest(updateEntityRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Update an entity.

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
	updateEntityRequest := *openapiclient.NewUpdateEntityRequest() // UpdateEntityRequest |
	includeDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.EntitiesAPI.UpdateEntity(context.Background(), id).XOrg(xOrg).UpdateEntityRequest(updateEntityRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EntitiesAPI.UpdateEntity``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateEntity`: EntityResponse
	fmt.Fprintf(os.Stdout, "Response from `EntitiesAPI.UpdateEntity`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateEntityRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **updateEntityRequest** | [**UpdateEntityRequest**](UpdateEntityRequest.md) |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**EntityResponse**](EntityResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
