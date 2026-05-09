# \UsersAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**AddRoles**](UsersAPI.md#AddRoles) | **Post** /users/{id}/roles | Add roles to a user. Requires users:create (or admin) in each org where roles are added.
[**CreateUser**](UsersAPI.md#CreateUser) | **Post** /users | Create a user in the organization context.
[**DeleteRoles**](UsersAPI.md#DeleteRoles) | **Delete** /users/{id}/roles | Delete roles from a user. Requires users:create (or admin) in each org where roles are deleted.
[**DeleteUser**](UsersAPI.md#DeleteUser) | **Delete** /users/{id} | Soft-delete a user.
[**GetUser**](UsersAPI.md#GetUser) | **Get** /users/{id} | Get a user by ID.
[**HardDeleteUser**](UsersAPI.md#HardDeleteUser) | **Delete** /users/{id}/purge | Permanently delete (purge) a user.
[**SearchUsers**](UsersAPI.md#SearchUsers) | **Get** /users/search | Search users by name/email. Requires &#x60;users:list&#x60; in the org (&#x60;X-Org&#x60; or &#x60;org_id&#x60;), or on the root org when &#x60;scope&#x3D;all&#x60;.
[**UpdateUser**](UsersAPI.md#UpdateUser) | **Put** /users/{id} | Update a user by ID.



## AddRoles

> UserResponse AddRoles(ctx, id).XOrg(xOrg).RequestBody(requestBody).Execute()

Add roles to a user. Requires users:create (or admin) in each org where roles are added.

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
	requestBody := map[string][]string{"key": []string{"Inner_example"}} // map[string][]string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.UsersAPI.AddRoles(context.Background(), id).XOrg(xOrg).RequestBody(requestBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `UsersAPI.AddRoles``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `AddRoles`: UserResponse
	fmt.Fprintf(os.Stdout, "Response from `UsersAPI.AddRoles`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiAddRolesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **requestBody** | **map[string][]string** |  |

### Return type

[**UserResponse**](UserResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CreateUser

> UserResponse CreateUser(ctx).XOrg(xOrg).CreateUserRequest(createUserRequest).IncludeDeleted(includeDeleted).OnlyDeleted(onlyDeleted).IncludeMetadata(includeMetadata).Execute()

Create a user in the organization context.

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
	createUserRequest := *openapiclient.NewCreateUserRequest("Email_example", *openapiclient.NewLocalizedString(map[string]string{"key": "Inner_example"})) // CreateUserRequest |
	includeDeleted := true // bool |  (optional)
	onlyDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.UsersAPI.CreateUser(context.Background()).XOrg(xOrg).CreateUserRequest(createUserRequest).IncludeDeleted(includeDeleted).OnlyDeleted(onlyDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `UsersAPI.CreateUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateUser`: UserResponse
	fmt.Fprintf(os.Stdout, "Response from `UsersAPI.CreateUser`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCreateUserRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **createUserRequest** | [**CreateUserRequest**](CreateUserRequest.md) |  |
 **includeDeleted** | **bool** |  |
 **onlyDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**UserResponse**](UserResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteRoles

> UserResponse DeleteRoles(ctx, id).XOrg(xOrg).RequestBody(requestBody).Execute()

Delete roles from a user. Requires users:create (or admin) in each org where roles are deleted.

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
	requestBody := map[string][]string{"key": []string{"Inner_example"}} // map[string][]string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.UsersAPI.DeleteRoles(context.Background(), id).XOrg(xOrg).RequestBody(requestBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `UsersAPI.DeleteRoles``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteRoles`: UserResponse
	fmt.Fprintf(os.Stdout, "Response from `UsersAPI.DeleteRoles`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteRolesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **requestBody** | **map[string][]string** |  |

### Return type

[**UserResponse**](UserResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteUser

> UserResponse DeleteUser(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Soft-delete a user.

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
	resp, r, err := apiClient.UsersAPI.DeleteUser(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `UsersAPI.DeleteUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteUser`: UserResponse
	fmt.Fprintf(os.Stdout, "Response from `UsersAPI.DeleteUser`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteUserRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**UserResponse**](UserResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetUser

> UserResponse GetUser(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Get a user by ID.

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
	resp, r, err := apiClient.UsersAPI.GetUser(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `UsersAPI.GetUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetUser`: UserResponse
	fmt.Fprintf(os.Stdout, "Response from `UsersAPI.GetUser`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetUserRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**UserResponse**](UserResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## HardDeleteUser

> UserResponse HardDeleteUser(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Permanently delete (purge) a user.

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
	resp, r, err := apiClient.UsersAPI.HardDeleteUser(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `UsersAPI.HardDeleteUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `HardDeleteUser`: UserResponse
	fmt.Fprintf(os.Stdout, "Response from `UsersAPI.HardDeleteUser`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiHardDeleteUserRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**UserResponse**](UserResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## SearchUsers

> PaginatedResponse SearchUsers(ctx).Q(q).OrgId(orgId).Scope(scope).Limit(limit).Offset(offset).ExcludeIds(excludeIds).Execute()

Search users by name/email. Requires `users:list` in the org (`X-Org` or `org_id`), or on the root org when `scope=all`.

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
	q := "q_example" // string | Search text (ILIKE on name and email) (optional)
	orgId := "orgId_example" // string | Filter by org subtree (ignored when `scope=all`). (optional)
	scope := "scope_example" // string | `all`: search every user in the database (requires `users:list` on the root org). Otherwise org-scoped. (optional)
	limit := int32(56) // int32 |  (optional)
	offset := int32(56) // int32 |  (optional)
	excludeIds := "excludeIds_example" // string | Comma-separated user IDs to exclude (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.UsersAPI.SearchUsers(context.Background()).Q(q).OrgId(orgId).Scope(scope).Limit(limit).Offset(offset).ExcludeIds(excludeIds).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `UsersAPI.SearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `SearchUsers`: PaginatedResponse
	fmt.Fprintf(os.Stdout, "Response from `UsersAPI.SearchUsers`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiSearchUsersRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **q** | **string** | Search text (ILIKE on name and email) |
 **orgId** | **string** | Filter by org subtree (ignored when &#x60;scope&#x3D;all&#x60;). |
 **scope** | **string** | &#x60;all&#x60;: search every user in the database (requires &#x60;users:list&#x60; on the root org). Otherwise org-scoped. |
 **limit** | **int32** |  |
 **offset** | **int32** |  |
 **excludeIds** | **string** | Comma-separated user IDs to exclude |

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


## UpdateUser

> UserResponse UpdateUser(ctx, id).XOrg(xOrg).UpdateUserRequest(updateUserRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Update a user by ID.

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
	updateUserRequest := *openapiclient.NewUpdateUserRequest() // UpdateUserRequest |
	includeDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.UsersAPI.UpdateUser(context.Background(), id).XOrg(xOrg).UpdateUserRequest(updateUserRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `UsersAPI.UpdateUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateUser`: UserResponse
	fmt.Fprintf(os.Stdout, "Response from `UsersAPI.UpdateUser`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateUserRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **updateUserRequest** | [**UpdateUserRequest**](UpdateUserRequest.md) |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**UserResponse**](UserResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
