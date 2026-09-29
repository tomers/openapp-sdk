# \AIAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**PostAiTranslateLocaleContent**](AIAPI.md#PostAiTranslateLocaleContent) | **Post** /ai/translate-locale-content |



## PostAiTranslateLocaleContent

> TranslateLocaleContentResponse PostAiTranslateLocaleContent(ctx).XOrg(xOrg).TranslateLocaleContentRequest(translateLocaleContentRequest).Execute()





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
	translateLocaleContentRequest := *openapiclient.NewTranslateLocaleContentRequest("Body_example", "SourceLocale_example", []string{"TargetLocales_example"}, "Title_example") // TranslateLocaleContentRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AIAPI.PostAiTranslateLocaleContent(context.Background()).XOrg(xOrg).TranslateLocaleContentRequest(translateLocaleContentRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AIAPI.PostAiTranslateLocaleContent``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostAiTranslateLocaleContent`: TranslateLocaleContentResponse
	fmt.Fprintf(os.Stdout, "Response from `AIAPI.PostAiTranslateLocaleContent`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostAiTranslateLocaleContentRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **translateLocaleContentRequest** | [**TranslateLocaleContentRequest**](TranslateLocaleContentRequest.md) |  |

### Return type

[**TranslateLocaleContentResponse**](TranslateLocaleContentResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
