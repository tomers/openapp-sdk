# \VirtualAccessAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**PostVirtualAccessDirectoryListingEnrich**](VirtualAccessAPI.md#PostVirtualAccessDirectoryListingEnrich) | **Post** /virtual-access/directory-listing-enrich |



## PostVirtualAccessDirectoryListingEnrich

> map[string]interface{} PostVirtualAccessDirectoryListingEnrich(ctx).XOrg(xOrg).Images(images).IntegrationId(integrationId).Execute()





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
	images := []*os.File{"TODO"} // []*os.File | One or more door / intercom photos.
	integrationId := "integrationId_example" // string | Virtual access integration ULID.

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.VirtualAccessAPI.PostVirtualAccessDirectoryListingEnrich(context.Background()).XOrg(xOrg).Images(images).IntegrationId(integrationId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `VirtualAccessAPI.PostVirtualAccessDirectoryListingEnrich``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostVirtualAccessDirectoryListingEnrich`: map[string]interface{}
	fmt.Fprintf(os.Stdout, "Response from `VirtualAccessAPI.PostVirtualAccessDirectoryListingEnrich`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostVirtualAccessDirectoryListingEnrichRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **images** | **[]*os.File** | One or more door / intercom photos. |
 **integrationId** | **string** | Virtual access integration ULID. |

### Return type

**map[string]interface{}**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
