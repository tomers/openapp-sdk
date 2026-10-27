# \OrgsAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateOrg**](OrgsAPI.md#CreateOrg) | **Post** /orgs | Create an organization (body: name, description?, parent_id?).
[**DeleteOrg**](OrgsAPI.md#DeleteOrg) | **Delete** /orgs/{id} | Soft-delete an organization.
[**GetOrg**](OrgsAPI.md#GetOrg) | **Get** /orgs/{id} | Get an organization by ID.
[**GetOrgDefaultLocale**](OrgsAPI.md#GetOrgDefaultLocale) | **Get** /orgs/{id}/default-locale | Get the org&#39;s default message language.
[**GetOrgPermissions**](OrgsAPI.md#GetOrgPermissions) | **Get** /orgs/{id}/permissions | Get the current user&#39;s permissions for an organization.
[**HardDeleteOrg**](OrgsAPI.md#HardDeleteOrg) | **Delete** /orgs/{id}/purge | Permanently delete (purge) an organization.
[**ListOrgAncestors**](OrgsAPI.md#ListOrgAncestors) | **Get** /orgs/{id}/ancestors | Visible ancestor chain of an org, ordered root-most first (excludes the org itself).
[**ListOrgChildren**](OrgsAPI.md#ListOrgChildren) | **Get** /orgs/children | List the direct children of an org (or the visible-forest roots) for lazy tree expansion.
[**ListOrgUsers**](OrgsAPI.md#ListOrgUsers) | **Get** /orgs/{org_id}/users | List users in an organization (and optionally descendant orgs).
[**ListOrgs**](OrgsAPI.md#ListOrgs) | **Get** /orgs | List organizations the user has access to.
[**RestoreOrg**](OrgsAPI.md#RestoreOrg) | **Post** /orgs/{id}/restore | Restore a soft-deleted organization.
[**SearchOrgs**](OrgsAPI.md#SearchOrgs) | **Get** /orgs/search | Free-text search across the orgs visible to the caller (flat, paginated).
[**SetOrgDefaultLocale**](OrgsAPI.md#SetOrgDefaultLocale) | **Patch** /orgs/{id}/default-locale | Set or clear the org&#39;s default message language.
[**UpdateOrg**](OrgsAPI.md#UpdateOrg) | **Put** /orgs/{id} | Update an organization (body: name?, description?, parent_id?).



## CreateOrg

> OrganizationResponse CreateOrg(ctx).CreateOrganizationRequest(createOrganizationRequest).IncludeDeleted(includeDeleted).Execute()

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.CreateOrg(context.Background()).CreateOrganizationRequest(createOrganizationRequest).IncludeDeleted(includeDeleted).Execute()
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

### Return type

[**OrganizationResponse**](OrganizationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteOrg

> OrganizationResponse DeleteOrg(ctx, id).IncludeDeleted(includeDeleted).Execute()

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.DeleteOrg(context.Background(), id).IncludeDeleted(includeDeleted).Execute()
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

### Return type

[**OrganizationResponse**](OrganizationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetOrg

> OrganizationResponse GetOrg(ctx, id).IncludeDeleted(includeDeleted).Execute()

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.GetOrg(context.Background(), id).IncludeDeleted(includeDeleted).Execute()
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

### Return type

[**OrganizationResponse**](OrganizationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetOrgDefaultLocale

> OrgDefaultLocaleResponse GetOrgDefaultLocale(ctx, id).Execute()

Get the org's default message language.

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
	resp, r, err := apiClient.OrgsAPI.GetOrgDefaultLocale(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.GetOrgDefaultLocale``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOrgDefaultLocale`: OrgDefaultLocaleResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.GetOrgDefaultLocale`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetOrgDefaultLocaleRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**OrgDefaultLocaleResponse**](OrgDefaultLocaleResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetOrgPermissions

> OrgPermissionsResponse GetOrgPermissions(ctx, id).Execute()

Get the current user's permissions for an organization.



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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## HardDeleteOrg

> OrganizationResponse HardDeleteOrg(ctx, id).IncludeDeleted(includeDeleted).Execute()

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.HardDeleteOrg(context.Background(), id).IncludeDeleted(includeDeleted).Execute()
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

### Return type

[**OrganizationResponse**](OrganizationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListOrgAncestors

> []OrganizationResponse ListOrgAncestors(ctx, id).Execute()

Visible ancestor chain of an org, ordered root-most first (excludes the org itself).



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
	id := "id_example" // string | Organization ID

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.ListOrgAncestors(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.ListOrgAncestors``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListOrgAncestors`: []OrganizationResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.ListOrgAncestors`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Organization ID |

### Other Parameters

Other parameters are passed through a pointer to a apiListOrgAncestorsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**[]OrganizationResponse**](OrganizationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListOrgChildren

> PaginatedResponse ListOrgChildren(ctx).OutputOptions(outputOptions).Pagination(pagination).ParentId(parentId).Execute()

List the direct children of an org (or the visible-forest roots) for lazy tree expansion.



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
	outputOptions := *openapiclient.NewMultiResourceOutputOptionsQuery() // MultiResourceOutputOptionsQuery |
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |
	parentId := "parentId_example" // string | Parent org id. When omitted, returns the roots of the caller's visible org forest. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.ListOrgChildren(context.Background()).OutputOptions(outputOptions).Pagination(pagination).ParentId(parentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.ListOrgChildren``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListOrgChildren`: PaginatedResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.ListOrgChildren`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListOrgChildrenRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **outputOptions** | [**MultiResourceOutputOptionsQuery**](MultiResourceOutputOptionsQuery.md) |  |
 **pagination** | [**PaginationQuery**](PaginationQuery.md) |  |
 **parentId** | **string** | Parent org id. When omitted, returns the roots of the caller&#39;s visible org forest. |

### Return type

[**PaginatedResponse**](PaginatedResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListOrgUsers

> PaginatedResponse ListOrgUsers(ctx, orgId).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).Recursive(recursive).Q(q).Role(role).Execute()

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
	outputOptions := *openapiclient.NewMultiResourceOutputOptionsQuery() // MultiResourceOutputOptionsQuery |
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |
	recursive := true // bool |  (optional)
	q := "q_example" // string | Case-insensitive substring of name or email. (optional)
	role := "role_example" // string | Membership role that must be held in the listed org subtree. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.ListOrgUsers(context.Background(), orgId).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).Recursive(recursive).Q(q).Role(role).Execute()
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
 **q** | **string** | Case-insensitive substring of name or email. |
 **role** | **string** | Membership role that must be held in the listed org subtree. |

### Return type

[**PaginatedResponse**](PaginatedResponse.md)

### Authorization

[api_key](../README.md#api_key)

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
	outputOptions := *openapiclient.NewMultiResourceOutputOptionsQuery() // MultiResourceOutputOptionsQuery |
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RestoreOrg

> OrganizationResponse RestoreOrg(ctx, id).IncludeDeleted(includeDeleted).Execute()

Restore a soft-deleted organization.

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.RestoreOrg(context.Background(), id).IncludeDeleted(includeDeleted).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.RestoreOrg``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `RestoreOrg`: OrganizationResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.RestoreOrg`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiRestoreOrgRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **includeDeleted** | **bool** |  |

### Return type

[**OrganizationResponse**](OrganizationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## SearchOrgs

> PaginatedResponse SearchOrgs(ctx).Q(q).OutputOptions(outputOptions).Pagination(pagination).Execute()

Free-text search across the orgs visible to the caller (flat, paginated).



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
	q := "q_example" // string | Free-text query matched as a case-insensitive substring of the org name, or as an exact (case-insensitive) org ID.
	outputOptions := *openapiclient.NewMultiResourceOutputOptionsQuery() // MultiResourceOutputOptionsQuery |
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.SearchOrgs(context.Background()).Q(q).OutputOptions(outputOptions).Pagination(pagination).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.SearchOrgs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `SearchOrgs`: PaginatedResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.SearchOrgs`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiSearchOrgsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **q** | **string** | Free-text query matched as a case-insensitive substring of the org name, or as an exact (case-insensitive) org ID. |
 **outputOptions** | [**MultiResourceOutputOptionsQuery**](MultiResourceOutputOptionsQuery.md) |  |
 **pagination** | [**PaginationQuery**](PaginationQuery.md) |  |

### Return type

[**PaginatedResponse**](PaginatedResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## SetOrgDefaultLocale

> OrgDefaultLocaleResponse SetOrgDefaultLocale(ctx, id).SetOrgDefaultLocaleRequest(setOrgDefaultLocaleRequest).Execute()

Set or clear the org's default message language.

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
	setOrgDefaultLocaleRequest := *openapiclient.NewSetOrgDefaultLocaleRequest() // SetOrgDefaultLocaleRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.SetOrgDefaultLocale(context.Background(), id).SetOrgDefaultLocaleRequest(setOrgDefaultLocaleRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrgsAPI.SetOrgDefaultLocale``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `SetOrgDefaultLocale`: OrgDefaultLocaleResponse
	fmt.Fprintf(os.Stdout, "Response from `OrgsAPI.SetOrgDefaultLocale`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiSetOrgDefaultLocaleRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **setOrgDefaultLocaleRequest** | [**SetOrgDefaultLocaleRequest**](SetOrgDefaultLocaleRequest.md) |  |

### Return type

[**OrgDefaultLocaleResponse**](OrgDefaultLocaleResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateOrg

> OrganizationResponse UpdateOrg(ctx, id).UpdateOrganizationRequest(updateOrganizationRequest).IncludeDeleted(includeDeleted).Execute()

Update an organization (body: name?, description?, parent_id?).



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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrgsAPI.UpdateOrg(context.Background(), id).UpdateOrganizationRequest(updateOrganizationRequest).IncludeDeleted(includeDeleted).Execute()
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

### Return type

[**OrganizationResponse**](OrganizationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
