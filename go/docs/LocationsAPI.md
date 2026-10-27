# \LocationsAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateOrgLocation**](LocationsAPI.md#CreateOrgLocation) | **Post** /orgs/{id}/locations |
[**DeleteLocation**](LocationsAPI.md#DeleteLocation) | **Delete** /locations/{id} |
[**DeleteLocationImage**](LocationsAPI.md#DeleteLocationImage) | **Delete** /locations/{id}/image |
[**GetAccessInviteLocationBinding**](LocationsAPI.md#GetAccessInviteLocationBinding) | **Get** /access-invites/{id}/location-binding |
[**GetAccessPortalLocationBinding**](LocationsAPI.md#GetAccessPortalLocationBinding) | **Get** /access-portals/{id}/location-binding |
[**GetDeviceLocationBinding**](LocationsAPI.md#GetDeviceLocationBinding) | **Get** /devices/{id}/location-binding |
[**GetIntegrationLocationBinding**](LocationsAPI.md#GetIntegrationLocationBinding) | **Get** /integrations/{id}/location-binding |
[**GetLocation**](LocationsAPI.md#GetLocation) | **Get** /locations/{id} |
[**GetLocationImage**](LocationsAPI.md#GetLocationImage) | **Get** /locations/{id}/image |
[**GetOrgLocationBinding**](LocationsAPI.md#GetOrgLocationBinding) | **Get** /orgs/{id}/location-binding |
[**GetZoneLocationBinding**](LocationsAPI.md#GetZoneLocationBinding) | **Get** /zones/{id}/location-binding |
[**ListIntegrationLocations**](LocationsAPI.md#ListIntegrationLocations) | **Get** /integrations/{id}/locations |
[**ListOrgLocations**](LocationsAPI.md#ListOrgLocations) | **Get** /orgs/{id}/locations |
[**PatchLocation**](LocationsAPI.md#PatchLocation) | **Patch** /locations/{id} |
[**PostLocationImage**](LocationsAPI.md#PostLocationImage) | **Post** /locations/{id}/image |
[**PutAccessInviteLocationBinding**](LocationsAPI.md#PutAccessInviteLocationBinding) | **Put** /access-invites/{id}/location-binding |
[**PutAccessPortalLocationBinding**](LocationsAPI.md#PutAccessPortalLocationBinding) | **Put** /access-portals/{id}/location-binding |
[**PutDeviceLocationBinding**](LocationsAPI.md#PutDeviceLocationBinding) | **Put** /devices/{id}/location-binding |
[**PutIntegrationLocationBinding**](LocationsAPI.md#PutIntegrationLocationBinding) | **Put** /integrations/{id}/location-binding |
[**PutOrgLocationBinding**](LocationsAPI.md#PutOrgLocationBinding) | **Put** /orgs/{id}/location-binding |
[**PutZoneLocationBinding**](LocationsAPI.md#PutZoneLocationBinding) | **Put** /zones/{id}/location-binding |



## CreateOrgLocation

> LocationResponse CreateOrgLocation(ctx, id).CreateLocationRequest(createLocationRequest).Execute()





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
	createLocationRequest := *openapiclient.NewCreateLocationRequest(openapiclient.LocationKind("address")) // CreateLocationRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LocationsAPI.CreateOrgLocation(context.Background(), id).CreateLocationRequest(createLocationRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.CreateOrgLocation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateOrgLocation`: LocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.CreateOrgLocation`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCreateOrgLocationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **createLocationRequest** | [**CreateLocationRequest**](CreateLocationRequest.md) |  |

### Return type

[**LocationResponse**](LocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteLocation

> DeleteLocation(ctx, id).Execute()





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
	r, err := apiClient.LocationsAPI.DeleteLocation(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.DeleteLocation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteLocationRequest struct via the builder pattern


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


## DeleteLocationImage

> DeleteLocationImage(ctx, id).Execute()





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
	r, err := apiClient.LocationsAPI.DeleteLocationImage(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.DeleteLocationImage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteLocationImageRequest struct via the builder pattern


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


## GetAccessInviteLocationBinding

> ResolvedLocationResponse GetAccessInviteLocationBinding(ctx, id).Execute()





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
	resp, r, err := apiClient.LocationsAPI.GetAccessInviteLocationBinding(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.GetAccessInviteLocationBinding``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAccessInviteLocationBinding`: ResolvedLocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.GetAccessInviteLocationBinding`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetAccessInviteLocationBindingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ResolvedLocationResponse**](ResolvedLocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetAccessPortalLocationBinding

> ResolvedLocationResponse GetAccessPortalLocationBinding(ctx, id).Execute()





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
	resp, r, err := apiClient.LocationsAPI.GetAccessPortalLocationBinding(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.GetAccessPortalLocationBinding``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAccessPortalLocationBinding`: ResolvedLocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.GetAccessPortalLocationBinding`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetAccessPortalLocationBindingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ResolvedLocationResponse**](ResolvedLocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetDeviceLocationBinding

> ResolvedLocationResponse GetDeviceLocationBinding(ctx, id).Execute()





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
	resp, r, err := apiClient.LocationsAPI.GetDeviceLocationBinding(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.GetDeviceLocationBinding``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetDeviceLocationBinding`: ResolvedLocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.GetDeviceLocationBinding`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetDeviceLocationBindingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ResolvedLocationResponse**](ResolvedLocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetIntegrationLocationBinding

> ResolvedLocationResponse GetIntegrationLocationBinding(ctx, id).Execute()





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
	resp, r, err := apiClient.LocationsAPI.GetIntegrationLocationBinding(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.GetIntegrationLocationBinding``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetIntegrationLocationBinding`: ResolvedLocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.GetIntegrationLocationBinding`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetIntegrationLocationBindingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ResolvedLocationResponse**](ResolvedLocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetLocation

> LocationResponse GetLocation(ctx, id).Execute()



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
	resp, r, err := apiClient.LocationsAPI.GetLocation(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.GetLocation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetLocation`: LocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.GetLocation`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetLocationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**LocationResponse**](LocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetLocationImage

> ResourceImageUrlResponse GetLocationImage(ctx, id).Execute()





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
	resp, r, err := apiClient.LocationsAPI.GetLocationImage(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.GetLocationImage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetLocationImage`: ResourceImageUrlResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.GetLocationImage`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetLocationImageRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ResourceImageUrlResponse**](ResourceImageUrlResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetOrgLocationBinding

> ResolvedLocationResponse GetOrgLocationBinding(ctx, id).Execute()





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
	resp, r, err := apiClient.LocationsAPI.GetOrgLocationBinding(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.GetOrgLocationBinding``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOrgLocationBinding`: ResolvedLocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.GetOrgLocationBinding`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetOrgLocationBindingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ResolvedLocationResponse**](ResolvedLocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetZoneLocationBinding

> ResolvedLocationResponse GetZoneLocationBinding(ctx, id).Execute()





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
	resp, r, err := apiClient.LocationsAPI.GetZoneLocationBinding(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.GetZoneLocationBinding``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetZoneLocationBinding`: ResolvedLocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.GetZoneLocationBinding`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetZoneLocationBindingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ResolvedLocationResponse**](ResolvedLocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationLocations

> PaginatedResponseLocationListItemResponse ListIntegrationLocations(ctx, id).Pagination(pagination).Search(search).Execute()



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
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |
	search := "search_example" // string |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LocationsAPI.ListIntegrationLocations(context.Background(), id).Pagination(pagination).Search(search).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.ListIntegrationLocations``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListIntegrationLocations`: PaginatedResponseLocationListItemResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.ListIntegrationLocations`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListIntegrationLocationsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **pagination** | [**PaginationQuery**](PaginationQuery.md) |  |
 **search** | **string** |  |

### Return type

[**PaginatedResponseLocationListItemResponse**](PaginatedResponseLocationListItemResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListOrgLocations

> PaginatedResponseLocationListItemResponse ListOrgLocations(ctx, id).Pagination(pagination).Search(search).Execute()



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
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |
	search := "search_example" // string |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LocationsAPI.ListOrgLocations(context.Background(), id).Pagination(pagination).Search(search).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.ListOrgLocations``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListOrgLocations`: PaginatedResponseLocationListItemResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.ListOrgLocations`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListOrgLocationsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **pagination** | [**PaginationQuery**](PaginationQuery.md) |  |
 **search** | **string** |  |

### Return type

[**PaginatedResponseLocationListItemResponse**](PaginatedResponseLocationListItemResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PatchLocation

> LocationResponse PatchLocation(ctx, id).PatchLocationRequest(patchLocationRequest).Execute()





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
	patchLocationRequest := *openapiclient.NewPatchLocationRequest() // PatchLocationRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LocationsAPI.PatchLocation(context.Background(), id).PatchLocationRequest(patchLocationRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.PatchLocation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PatchLocation`: LocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.PatchLocation`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPatchLocationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **patchLocationRequest** | [**PatchLocationRequest**](PatchLocationRequest.md) |  |

### Return type

[**LocationResponse**](LocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostLocationImage

> ResourceImageUrlResponse PostLocationImage(ctx, id).File(file).Execute()





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
	file := os.NewFile(1234, "some_file") // *os.File | Image file bytes (JPEG, PNG, or WebP).

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LocationsAPI.PostLocationImage(context.Background(), id).File(file).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.PostLocationImage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostLocationImage`: ResourceImageUrlResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.PostLocationImage`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostLocationImageRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **file** | ***os.File** | Image file bytes (JPEG, PNG, or WebP). |

### Return type

[**ResourceImageUrlResponse**](ResourceImageUrlResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutAccessInviteLocationBinding

> ResolvedLocationResponse PutAccessInviteLocationBinding(ctx, id).PutLocationBindingRequest(putLocationBindingRequest).Execute()





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
	putLocationBindingRequest := *openapiclient.NewPutLocationBindingRequest(openapiclient.LocationBindingMode("inherit")) // PutLocationBindingRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LocationsAPI.PutAccessInviteLocationBinding(context.Background(), id).PutLocationBindingRequest(putLocationBindingRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.PutAccessInviteLocationBinding``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutAccessInviteLocationBinding`: ResolvedLocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.PutAccessInviteLocationBinding`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPutAccessInviteLocationBindingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **putLocationBindingRequest** | [**PutLocationBindingRequest**](PutLocationBindingRequest.md) |  |

### Return type

[**ResolvedLocationResponse**](ResolvedLocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutAccessPortalLocationBinding

> ResolvedLocationResponse PutAccessPortalLocationBinding(ctx, id).PutLocationBindingRequest(putLocationBindingRequest).Execute()





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
	putLocationBindingRequest := *openapiclient.NewPutLocationBindingRequest(openapiclient.LocationBindingMode("inherit")) // PutLocationBindingRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LocationsAPI.PutAccessPortalLocationBinding(context.Background(), id).PutLocationBindingRequest(putLocationBindingRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.PutAccessPortalLocationBinding``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutAccessPortalLocationBinding`: ResolvedLocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.PutAccessPortalLocationBinding`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPutAccessPortalLocationBindingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **putLocationBindingRequest** | [**PutLocationBindingRequest**](PutLocationBindingRequest.md) |  |

### Return type

[**ResolvedLocationResponse**](ResolvedLocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutDeviceLocationBinding

> ResolvedLocationResponse PutDeviceLocationBinding(ctx, id).PutLocationBindingRequest(putLocationBindingRequest).Execute()





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
	putLocationBindingRequest := *openapiclient.NewPutLocationBindingRequest(openapiclient.LocationBindingMode("inherit")) // PutLocationBindingRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LocationsAPI.PutDeviceLocationBinding(context.Background(), id).PutLocationBindingRequest(putLocationBindingRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.PutDeviceLocationBinding``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutDeviceLocationBinding`: ResolvedLocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.PutDeviceLocationBinding`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPutDeviceLocationBindingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **putLocationBindingRequest** | [**PutLocationBindingRequest**](PutLocationBindingRequest.md) |  |

### Return type

[**ResolvedLocationResponse**](ResolvedLocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutIntegrationLocationBinding

> ResolvedLocationResponse PutIntegrationLocationBinding(ctx, id).PutLocationBindingRequest(putLocationBindingRequest).Execute()





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
	putLocationBindingRequest := *openapiclient.NewPutLocationBindingRequest(openapiclient.LocationBindingMode("inherit")) // PutLocationBindingRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LocationsAPI.PutIntegrationLocationBinding(context.Background(), id).PutLocationBindingRequest(putLocationBindingRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.PutIntegrationLocationBinding``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutIntegrationLocationBinding`: ResolvedLocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.PutIntegrationLocationBinding`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPutIntegrationLocationBindingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **putLocationBindingRequest** | [**PutLocationBindingRequest**](PutLocationBindingRequest.md) |  |

### Return type

[**ResolvedLocationResponse**](ResolvedLocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutOrgLocationBinding

> ResolvedLocationResponse PutOrgLocationBinding(ctx, id).PutLocationBindingRequest(putLocationBindingRequest).Execute()





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
	putLocationBindingRequest := *openapiclient.NewPutLocationBindingRequest(openapiclient.LocationBindingMode("inherit")) // PutLocationBindingRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LocationsAPI.PutOrgLocationBinding(context.Background(), id).PutLocationBindingRequest(putLocationBindingRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.PutOrgLocationBinding``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutOrgLocationBinding`: ResolvedLocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.PutOrgLocationBinding`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPutOrgLocationBindingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **putLocationBindingRequest** | [**PutLocationBindingRequest**](PutLocationBindingRequest.md) |  |

### Return type

[**ResolvedLocationResponse**](ResolvedLocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PutZoneLocationBinding

> ResolvedLocationResponse PutZoneLocationBinding(ctx, id).PutLocationBindingRequest(putLocationBindingRequest).Execute()





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
	putLocationBindingRequest := *openapiclient.NewPutLocationBindingRequest(openapiclient.LocationBindingMode("inherit")) // PutLocationBindingRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.LocationsAPI.PutZoneLocationBinding(context.Background(), id).PutLocationBindingRequest(putLocationBindingRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `LocationsAPI.PutZoneLocationBinding``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PutZoneLocationBinding`: ResolvedLocationResponse
	fmt.Fprintf(os.Stdout, "Response from `LocationsAPI.PutZoneLocationBinding`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPutZoneLocationBindingRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **putLocationBindingRequest** | [**PutLocationBindingRequest**](PutLocationBindingRequest.md) |  |

### Return type

[**ResolvedLocationResponse**](ResolvedLocationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
