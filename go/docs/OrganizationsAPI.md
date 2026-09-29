# \OrganizationsAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GetOrgPrivacy**](OrganizationsAPI.md#GetOrgPrivacy) | **Get** /orgs/{id}/privacy |
[**PatchOrgPrivacy**](OrganizationsAPI.md#PatchOrgPrivacy) | **Patch** /orgs/{id}/privacy |



## GetOrgPrivacy

> OrgPrivacySettingsResponse GetOrgPrivacy(ctx, id).Execute()



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
	resp, r, err := apiClient.OrganizationsAPI.GetOrgPrivacy(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrganizationsAPI.GetOrgPrivacy``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOrgPrivacy`: OrgPrivacySettingsResponse
	fmt.Fprintf(os.Stdout, "Response from `OrganizationsAPI.GetOrgPrivacy`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetOrgPrivacyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**OrgPrivacySettingsResponse**](OrgPrivacySettingsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PatchOrgPrivacy

> OrgPrivacySettingsResponse PatchOrgPrivacy(ctx, id).PatchOrgPrivacySettingsRequest(patchOrgPrivacySettingsRequest).Execute()



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
	patchOrgPrivacySettingsRequest := *openapiclient.NewPatchOrgPrivacySettingsRequest(int32(123)) // PatchOrgPrivacySettingsRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.OrganizationsAPI.PatchOrgPrivacy(context.Background(), id).PatchOrgPrivacySettingsRequest(patchOrgPrivacySettingsRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OrganizationsAPI.PatchOrgPrivacy``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PatchOrgPrivacy`: OrgPrivacySettingsResponse
	fmt.Fprintf(os.Stdout, "Response from `OrganizationsAPI.PatchOrgPrivacy`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPatchOrgPrivacyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **patchOrgPrivacySettingsRequest** | [**PatchOrgPrivacySettingsRequest**](PatchOrgPrivacySettingsRequest.md) |  |

### Return type

[**OrgPrivacySettingsResponse**](OrgPrivacySettingsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
