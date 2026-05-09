# \OrgsAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateOrg**](OrgsAPI.md#CreateOrg) | **Post** /orgs | Create an organization (body: name, description?, parent_id?).
[**DeleteOrg**](OrgsAPI.md#DeleteOrg) | **Delete** /orgs/{id} | Soft-delete an organization.
[**GetOrg**](OrgsAPI.md#GetOrg) | **Get** /orgs/{id} | Get an organization by ID.
[**GetOrgPermissions**](OrgsAPI.md#GetOrgPermissions) | **Get** /orgs/{id}/permissions | Get current user&#39;s permissions for an organization.
[**HardDeleteOrg**](OrgsAPI.md#HardDeleteOrg) | **Delete** /orgs/{id}/purge | Permanently delete (purge) an organization.
[**ListOrgUsers**](OrgsAPI.md#ListOrgUsers) | **Get** /orgs/{org_id}/users | List users in an organization (and optionally descendant orgs).
[**ListOrgs**](OrgsAPI.md#ListOrgs) | **Get** /orgs | List organizations the user has access to.
[**UpdateOrg**](OrgsAPI.md#UpdateOrg) | **Put** /orgs/{id} | Update an organization (body: name?, description?).



## CreateOrg

> OrganizationResponse CreateOrg(ctx).CreateOrganizationRequest(createOrganizationRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Create an organization (body: name, description?, parent_id?).

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
	createOrganizationRequest := *openapiclient.NewCreateOrganizationRequest(*openapiclient.NewLocalizedString(map[string]string{"key": "Inner_example"})) // CreateOrganizationRequest |
	includeDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.CreateOrg(context.Background()).CreateOrganizationRequest(createOrganizationRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.CreateOrg``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateOrg`: OrganizationResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.CreateOrg`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCreateOrgRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createOrganizationRequest** | [**CreateOrganizationRequest**](CreateOrganizationRequest.md) |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**OrganizationResponse**](OrganizationResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteOrg

> OrganizationResponse DeleteOrg(ctx, id).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Soft-delete an organization.

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
	includeDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.DeleteOrg(context.Background(), id).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.DeleteOrg``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteOrg`: OrganizationResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.DeleteOrg`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteOrgRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**OrganizationResponse**](OrganizationResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetOrg

> OrganizationResponse GetOrg(ctx, id).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Get an organization by ID.

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
	includeDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.GetOrg(context.Background(), id).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.GetOrg``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOrg`: OrganizationResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.GetOrg`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetOrgRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**OrganizationResponse**](OrganizationResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetOrgPermissions

> OrgPermissionsResponse GetOrgPermissions(ctx, id).Execute()

Get current user's permissions for an organization.

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.GetOrgPermissions(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.GetOrgPermissions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOrgPermissions`: OrgPermissionsResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.GetOrgPermissions`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetOrgPermissionsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**OrgPermissionsResponse**](OrgPermissionsResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## HardDeleteOrg

> OrganizationResponse HardDeleteOrg(ctx, id).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Permanently delete (purge) an organization.

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
	includeDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.HardDeleteOrg(context.Background(), id).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.HardDeleteOrg``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `HardDeleteOrg`: OrganizationResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.HardDeleteOrg`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiHardDeleteOrgRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**OrganizationResponse**](OrganizationResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListOrgUsers

> PaginatedResponse ListOrgUsers(ctx, orgId).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).Recursive(recursive).Execute()

List users in an organization (and optionally descendant orgs).

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
	xOrg := "xOrg_example" // string |
	outputOptions := *openapiclient.NewMultiResourceOutputOptionsQuery(false, false, false) // MultiResourceOutputOptionsQuery |
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |
	recursive := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.ListOrgUsers(context.Background(), orgId).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).Recursive(recursive).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.ListOrgUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListOrgUsers`: PaginatedResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.ListOrgUsers`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListOrgUsersRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **outputOptions** | [**MultiResourceOutputOptionsQuery**](MultiResourceOutputOptionsQuery.md) |  |
 **pagination** | [**PaginationQuery**](PaginationQuery.md) |  |
 **recursive** | **bool** |  |

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


## ListOrgs

> PaginatedResponse ListOrgs(ctx).OutputOptions(outputOptions).Pagination(pagination).Execute()

List organizations the user has access to.

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
	outputOptions := *openapiclient.NewMultiResourceOutputOptionsQuery(false, false, false) // MultiResourceOutputOptionsQuery |
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.ListOrgs(context.Background()).OutputOptions(outputOptions).Pagination(pagination).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.ListOrgs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListOrgs`: PaginatedResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.ListOrgs`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListOrgsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **outputOptions** | [**MultiResourceOutputOptionsQuery**](MultiResourceOutputOptionsQuery.md) |  |
 **pagination** | [**PaginationQuery**](PaginationQuery.md) |  |

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


## UpdateOrg

> OrganizationResponse UpdateOrg(ctx, id).UpdateOrganizationRequest(updateOrganizationRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Update an organization (body: name?, description?).

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
	updateOrganizationRequest := *openapiclient.NewUpdateOrganizationRequest() // UpdateOrganizationRequest |
	includeDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.UpdateOrg(context.Background(), id).UpdateOrganizationRequest(updateOrganizationRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.UpdateOrg``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateOrg`: OrganizationResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.UpdateOrg`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateOrgRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **updateOrganizationRequest** | [**UpdateOrganizationRequest**](UpdateOrganizationRequest.md) |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**OrganizationResponse**](OrganizationResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
