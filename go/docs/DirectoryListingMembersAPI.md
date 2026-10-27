# \DirectoryListingMembersAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**AddDirectoryListingMember**](DirectoryListingMembersAPI.md#AddDirectoryListingMember) | **Post** /entities/{id}/directory-listing-members | Add a member to a directory listing.
[**AddPerson**](DirectoryListingMembersAPI.md#AddPerson) | **Post** /integrations/{id}/people | Add a user to the site&#39;s people.
[**DeleteDirectoryListingMember**](DirectoryListingMembersAPI.md#DeleteDirectoryListingMember) | **Delete** /entities/{id}/directory-listing-members/{member_id} | Remove a member from a directory listing.
[**DeletePerson**](DirectoryListingMembersAPI.md#DeletePerson) | **Delete** /integrations/{id}/people/{user_id} | Remove a user from the site&#39;s people.
[**ListDirectoryListingMembers**](DirectoryListingMembersAPI.md#ListDirectoryListingMembers) | **Get** /entities/{id}/directory-listing-members | List members for a directory listing.
[**ListIntegrationDirectoryListingMembers**](DirectoryListingMembersAPI.md#ListIntegrationDirectoryListingMembers) | **Get** /integrations/{id}/directory-listing-members | Batch list members for multiple listing entities (integration-scoped).
[**ListPeople**](DirectoryListingMembersAPI.md#ListPeople) | **Get** /integrations/{id}/people | List the site&#39;s people and their directory listing assignments.
[**UpdateDirectoryListingMember**](DirectoryListingMembersAPI.md#UpdateDirectoryListingMember) | **Put** /entities/{id}/directory-listing-members/{member_id} | Update a member&#39;s role or receives_calls.
[**UpdatePerson**](DirectoryListingMembersAPI.md#UpdatePerson) | **Put** /integrations/{id}/people/{user_id} | Change a site person&#39;s role.



## AddDirectoryListingMember

> DirectoryListingMemberResponse AddDirectoryListingMember(ctx, id).XOrg(xOrg).AddDirectoryListingMemberPayload(addDirectoryListingMemberPayload).Execute()

Add a member to a directory listing.

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
	addDirectoryListingMemberPayload := *openapiclient.NewAddDirectoryListingMemberPayload("UserId_example") // AddDirectoryListingMemberPayload |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DirectoryListingMembersAPI.AddDirectoryListingMember(context.Background(), id).XOrg(xOrg).AddDirectoryListingMemberPayload(addDirectoryListingMemberPayload).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryListingMembersAPI.AddDirectoryListingMember``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `AddDirectoryListingMember`: DirectoryListingMemberResponse
	fmt.Fprintf(os.Stdout, "Response from `DirectoryListingMembersAPI.AddDirectoryListingMember`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiAddDirectoryListingMemberRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **addDirectoryListingMemberPayload** | [**AddDirectoryListingMemberPayload**](AddDirectoryListingMemberPayload.md) |  |

### Return type

[**DirectoryListingMemberResponse**](DirectoryListingMemberResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## AddPerson

> AddPerson(ctx, id).XOrg(xOrg).AddSitePersonPayload(addSitePersonPayload).Execute()

Add a user to the site's people.



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
	addSitePersonPayload := *openapiclient.NewAddSitePersonPayload("Role_example", "UserId_example") // AddSitePersonPayload |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.DirectoryListingMembersAPI.AddPerson(context.Background(), id).XOrg(xOrg).AddSitePersonPayload(addSitePersonPayload).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryListingMembersAPI.AddPerson``: %v\n", err)
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

Other parameters are passed through a pointer to a apiAddPersonRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **addSitePersonPayload** | [**AddSitePersonPayload**](AddSitePersonPayload.md) |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteDirectoryListingMember

> DeleteDirectoryListingMember(ctx, id, memberId).XOrg(xOrg).Execute()

Remove a member from a directory listing.

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
	memberId := "memberId_example" // string |
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.DirectoryListingMembersAPI.DeleteDirectoryListingMember(context.Background(), id, memberId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryListingMembersAPI.DeleteDirectoryListingMember``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**memberId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteDirectoryListingMemberRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeletePerson

> DeletePerson(ctx, id, userId).XOrg(xOrg).Execute()

Remove a user from the site's people.



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
	userId := "userId_example" // string |
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.DirectoryListingMembersAPI.DeletePerson(context.Background(), id, userId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryListingMembersAPI.DeletePerson``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**userId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeletePersonRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListDirectoryListingMembers

> []DirectoryListingMemberWithUserResponse ListDirectoryListingMembers(ctx, id).XOrg(xOrg).Execute()

List members for a directory listing.

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
	resp, r, err := apiClient.DirectoryListingMembersAPI.ListDirectoryListingMembers(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryListingMembersAPI.ListDirectoryListingMembers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListDirectoryListingMembers`: []DirectoryListingMemberWithUserResponse
	fmt.Fprintf(os.Stdout, "Response from `DirectoryListingMembersAPI.ListDirectoryListingMembers`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListDirectoryListingMembersRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**[]DirectoryListingMemberWithUserResponse**](DirectoryListingMemberWithUserResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationDirectoryListingMembers

> ListIntegrationDirectoryListingMembers(ctx, id).XOrg(xOrg).EntityIds(entityIds).Execute()

Batch list members for multiple listing entities (integration-scoped).

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
	entityIds := "entityIds_example" // string | Comma-separated entity IDs (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.DirectoryListingMembersAPI.ListIntegrationDirectoryListingMembers(context.Background(), id).XOrg(xOrg).EntityIds(entityIds).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryListingMembersAPI.ListIntegrationDirectoryListingMembers``: %v\n", err)
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

Other parameters are passed through a pointer to a apiListIntegrationDirectoryListingMembersRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **entityIds** | **string** | Comma-separated entity IDs |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListPeople

> PaginatedResponse ListPeople(ctx, id).XOrg(xOrg).Locale(locale).Q(q).Role(role).GroupId(groupId).ListingId(listingId).HasListing(hasListing).IncludeViaGroup(includeViaGroup).Limit(limit).Offset(offset).Execute()

List the site's people and their directory listing assignments.



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
	locale := "locale_example" // string | Preferred name locale for stable people ordering (optional)
	q := "q_example" // string | Case-insensitive match on name or email (optional)
	role := "role_example" // string | Site role: admin, resident, or technician (optional)
	groupId := "groupId_example" // string | Only people in this group (optional)
	listingId := "listingId_example" // string | Only people assigned to this listing (optional)
	hasListing := true // bool | Whether the person has a listing assignment (optional)
	includeViaGroup := true // bool | Include household members who have no site role (optional)
	limit := int32(56) // int32 | Page size (optional)
	offset := int32(56) // int32 | Rows to skip (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DirectoryListingMembersAPI.ListPeople(context.Background(), id).XOrg(xOrg).Locale(locale).Q(q).Role(role).GroupId(groupId).ListingId(listingId).HasListing(hasListing).IncludeViaGroup(includeViaGroup).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryListingMembersAPI.ListPeople``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListPeople`: PaginatedResponse
	fmt.Fprintf(os.Stdout, "Response from `DirectoryListingMembersAPI.ListPeople`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListPeopleRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **locale** | **string** | Preferred name locale for stable people ordering |
 **q** | **string** | Case-insensitive match on name or email |
 **role** | **string** | Site role: admin, resident, or technician |
 **groupId** | **string** | Only people in this group |
 **listingId** | **string** | Only people assigned to this listing |
 **hasListing** | **bool** | Whether the person has a listing assignment |
 **includeViaGroup** | **bool** | Include household members who have no site role |
 **limit** | **int32** | Page size |
 **offset** | **int32** | Rows to skip |

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


## UpdateDirectoryListingMember

> DirectoryListingMemberResponse UpdateDirectoryListingMember(ctx, id, memberId).XOrg(xOrg).UpdateDirectoryListingMemberPayload(updateDirectoryListingMemberPayload).Execute()

Update a member's role or receives_calls.

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
	memberId := "memberId_example" // string |
	xOrg := "xOrg_example" // string |
	updateDirectoryListingMemberPayload := *openapiclient.NewUpdateDirectoryListingMemberPayload() // UpdateDirectoryListingMemberPayload |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DirectoryListingMembersAPI.UpdateDirectoryListingMember(context.Background(), id, memberId).XOrg(xOrg).UpdateDirectoryListingMemberPayload(updateDirectoryListingMemberPayload).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryListingMembersAPI.UpdateDirectoryListingMember``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateDirectoryListingMember`: DirectoryListingMemberResponse
	fmt.Fprintf(os.Stdout, "Response from `DirectoryListingMembersAPI.UpdateDirectoryListingMember`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**memberId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateDirectoryListingMemberRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |
 **updateDirectoryListingMemberPayload** | [**UpdateDirectoryListingMemberPayload**](UpdateDirectoryListingMemberPayload.md) |  |

### Return type

[**DirectoryListingMemberResponse**](DirectoryListingMemberResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdatePerson

> SitePersonResponse UpdatePerson(ctx, id, userId).XOrg(xOrg).UpdateSitePersonPayload(updateSitePersonPayload).Execute()

Change a site person's role.



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
	userId := "userId_example" // string |
	xOrg := "xOrg_example" // string |
	updateSitePersonPayload := *openapiclient.NewUpdateSitePersonPayload("Role_example") // UpdateSitePersonPayload |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.DirectoryListingMembersAPI.UpdatePerson(context.Background(), id, userId).XOrg(xOrg).UpdateSitePersonPayload(updateSitePersonPayload).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DirectoryListingMembersAPI.UpdatePerson``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdatePerson`: SitePersonResponse
	fmt.Fprintf(os.Stdout, "Response from `DirectoryListingMembersAPI.UpdatePerson`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**userId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdatePersonRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |
 **updateSitePersonPayload** | [**UpdateSitePersonPayload**](UpdateSitePersonPayload.md) |  |

### Return type

[**SitePersonResponse**](SitePersonResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
