# \WiFiNetworksAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateWifiNetwork**](WiFiNetworksAPI.md#CreateWifiNetwork) | **Post** /wifi-networks | Create a Wi-Fi network in the &#x60;X-Org&#x60; organization.
[**DeleteWifiNetwork**](WiFiNetworksAPI.md#DeleteWifiNetwork) | **Delete** /wifi-networks/{id} | Delete a Wi-Fi network.
[**GetWifiNetwork**](WiFiNetworksAPI.md#GetWifiNetwork) | **Get** /wifi-networks/{id} | Get a Wi-Fi network by ID.
[**GetWifiNetworkCredentials**](WiFiNetworksAPI.md#GetWifiNetworkCredentials) | **Get** /wifi-networks/{id}/credentials | Reveal the credentials of a Wi-Fi network.
[**ListWifiNetworks**](WiFiNetworksAPI.md#ListWifiNetworks) | **Get** /wifi-networks | List the Wi-Fi networks of the &#x60;X-Org&#x60; organization.
[**SetWifiNetworkZones**](WiFiNetworksAPI.md#SetWifiNetworkZones) | **Put** /wifi-networks/{id}/zones | Replace the zones a Wi-Fi network covers.
[**UpdateWifiNetwork**](WiFiNetworksAPI.md#UpdateWifiNetwork) | **Put** /wifi-networks/{id} | Update a Wi-Fi network.



## CreateWifiNetwork

> WifiNetworkResponse CreateWifiNetwork(ctx).XOrg(xOrg).CreateWifiNetworkRequest(createWifiNetworkRequest).Execute()

Create a Wi-Fi network in the `X-Org` organization.



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
	createWifiNetworkRequest := *openapiclient.NewCreateWifiNetworkRequest(openapiclient.WifiSecurity("open"), "Ssid_example") // CreateWifiNetworkRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.WiFiNetworksAPI.CreateWifiNetwork(context.Background()).XOrg(xOrg).CreateWifiNetworkRequest(createWifiNetworkRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `WiFiNetworksAPI.CreateWifiNetwork``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateWifiNetwork`: WifiNetworkResponse
	fmt.Fprintf(os.Stdout, "Response from `WiFiNetworksAPI.CreateWifiNetwork`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCreateWifiNetworkRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **createWifiNetworkRequest** | [**CreateWifiNetworkRequest**](CreateWifiNetworkRequest.md) |  |

### Return type

[**WifiNetworkResponse**](WifiNetworkResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteWifiNetwork

> WifiNetworkResponse DeleteWifiNetwork(ctx, id).Execute()

Delete a Wi-Fi network.



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
	id := "id_example" // string | Wi-Fi network ID

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.WiFiNetworksAPI.DeleteWifiNetwork(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `WiFiNetworksAPI.DeleteWifiNetwork``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteWifiNetwork`: WifiNetworkResponse
	fmt.Fprintf(os.Stdout, "Response from `WiFiNetworksAPI.DeleteWifiNetwork`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Wi-Fi network ID |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteWifiNetworkRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**WifiNetworkResponse**](WifiNetworkResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetWifiNetwork

> WifiNetworkResponse GetWifiNetwork(ctx, id).Execute()

Get a Wi-Fi network by ID.



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
	id := "id_example" // string | Wi-Fi network ID

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.WiFiNetworksAPI.GetWifiNetwork(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `WiFiNetworksAPI.GetWifiNetwork``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetWifiNetwork`: WifiNetworkResponse
	fmt.Fprintf(os.Stdout, "Response from `WiFiNetworksAPI.GetWifiNetwork`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Wi-Fi network ID |

### Other Parameters

Other parameters are passed through a pointer to a apiGetWifiNetworkRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**WifiNetworkResponse**](WifiNetworkResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetWifiNetworkCredentials

> WifiCredentials GetWifiNetworkCredentials(ctx, id).Execute()

Reveal the credentials of a Wi-Fi network.



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
	id := "id_example" // string | Wi-Fi network ID

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.WiFiNetworksAPI.GetWifiNetworkCredentials(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `WiFiNetworksAPI.GetWifiNetworkCredentials``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetWifiNetworkCredentials`: WifiCredentials
	fmt.Fprintf(os.Stdout, "Response from `WiFiNetworksAPI.GetWifiNetworkCredentials`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Wi-Fi network ID |

### Other Parameters

Other parameters are passed through a pointer to a apiGetWifiNetworkCredentialsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**WifiCredentials**](WifiCredentials.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListWifiNetworks

> PaginatedResponseWifiNetworkResponse ListWifiNetworks(ctx).XOrg(xOrg).Limit(limit).Offset(offset).CoversZoneId(coversZoneId).Execute()

List the Wi-Fi networks of the `X-Org` organization.



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
	limit := int32(50) // int32 | Number of items per page. Default from config, max 200. (optional)
	offset := int32(0) // int32 | Number of items to skip. Default 0. (optional)
	coversZoneId := "coversZoneId_example" // string | Only networks that cover this zone: linked to it or to any zone above it. A network on a building covers its floors and rooms. The zone must be a live zone of the `X-Org` organization. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.WiFiNetworksAPI.ListWifiNetworks(context.Background()).XOrg(xOrg).Limit(limit).Offset(offset).CoversZoneId(coversZoneId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `WiFiNetworksAPI.ListWifiNetworks``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListWifiNetworks`: PaginatedResponseWifiNetworkResponse
	fmt.Fprintf(os.Stdout, "Response from `WiFiNetworksAPI.ListWifiNetworks`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListWifiNetworksRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **limit** | **int32** | Number of items per page. Default from config, max 200. |
 **offset** | **int32** | Number of items to skip. Default 0. |
 **coversZoneId** | **string** | Only networks that cover this zone: linked to it or to any zone above it. A network on a building covers its floors and rooms. The zone must be a live zone of the &#x60;X-Org&#x60; organization. |

### Return type

[**PaginatedResponseWifiNetworkResponse**](PaginatedResponseWifiNetworkResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## SetWifiNetworkZones

> WifiNetworkResponse SetWifiNetworkZones(ctx, id).SetWifiNetworkZonesRequest(setWifiNetworkZonesRequest).Execute()

Replace the zones a Wi-Fi network covers.



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
	id := "id_example" // string | Wi-Fi network ID
	setWifiNetworkZonesRequest := *openapiclient.NewSetWifiNetworkZonesRequest([]string{"ZoneIds_example"}) // SetWifiNetworkZonesRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.WiFiNetworksAPI.SetWifiNetworkZones(context.Background(), id).SetWifiNetworkZonesRequest(setWifiNetworkZonesRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `WiFiNetworksAPI.SetWifiNetworkZones``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `SetWifiNetworkZones`: WifiNetworkResponse
	fmt.Fprintf(os.Stdout, "Response from `WiFiNetworksAPI.SetWifiNetworkZones`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Wi-Fi network ID |

### Other Parameters

Other parameters are passed through a pointer to a apiSetWifiNetworkZonesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **setWifiNetworkZonesRequest** | [**SetWifiNetworkZonesRequest**](SetWifiNetworkZonesRequest.md) |  |

### Return type

[**WifiNetworkResponse**](WifiNetworkResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateWifiNetwork

> WifiNetworkResponse UpdateWifiNetwork(ctx, id).UpdateWifiNetworkRequest(updateWifiNetworkRequest).Execute()

Update a Wi-Fi network.



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
	id := "id_example" // string | Wi-Fi network ID
	updateWifiNetworkRequest := *openapiclient.NewUpdateWifiNetworkRequest(openapiclient.WifiSecurity("open"), "Ssid_example") // UpdateWifiNetworkRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.WiFiNetworksAPI.UpdateWifiNetwork(context.Background(), id).UpdateWifiNetworkRequest(updateWifiNetworkRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `WiFiNetworksAPI.UpdateWifiNetwork``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateWifiNetwork`: WifiNetworkResponse
	fmt.Fprintf(os.Stdout, "Response from `WiFiNetworksAPI.UpdateWifiNetwork`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** | Wi-Fi network ID |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateWifiNetworkRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **updateWifiNetworkRequest** | [**UpdateWifiNetworkRequest**](UpdateWifiNetworkRequest.md) |  |

### Return type

[**WifiNetworkResponse**](WifiNetworkResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
