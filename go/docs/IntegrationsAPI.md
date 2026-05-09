# \IntegrationsAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateIntegration**](IntegrationsAPI.md#CreateIntegration) | **Post** /integrations | Create an integration in an organization.
[**CreateIntegrationAccessPortal**](IntegrationsAPI.md#CreateIntegrationAccessPortal) | **Post** /integrations/{id}/access-portals | Create a public access portal for an integration.
[**DeleteIntegrationAccessInvite**](IntegrationsAPI.md#DeleteIntegrationAccessInvite) | **Delete** /integrations/{id}/access-invites/{invite_link_id} | Delete an access invite (permanently).
[**DeleteIntegrationAccessPortal**](IntegrationsAPI.md#DeleteIntegrationAccessPortal) | **Delete** /integrations/{id}/access-portals/{portal_id} | Delete a public access portal.
[**ExecuteIntegrationOp**](IntegrationsAPI.md#ExecuteIntegrationOp) | **Post** /integrations/{id}/ops/{op_id} | Execute a provider-specific op for an integration.
[**GetAccessPortalById**](IntegrationsAPI.md#GetAccessPortalById) | **Get** /integrations/access-portals/{portal_id} | Get an access portal by ID. Resolves the portal&#39;s building (integration) for direct links.
[**GetIntegration**](IntegrationsAPI.md#GetIntegration) | **Get** /integrations/{id} |
[**GetIntegrationDeviceMetadataSchema**](IntegrationsAPI.md#GetIntegrationDeviceMetadataSchema) | **Get** /integrations/{id}/device-metadata-schema | Get provider-specific JSON Schema for &#x60;device_metadata&#x60; for this integration (for generic UI labels).
[**GetIntegrationDiscoveredDevices**](IntegrationsAPI.md#GetIntegrationDiscoveredDevices) | **Get** /integrations/{id}/discovered-devices | List devices discovered from the upstream provider for this integration.
[**GetIntegrationProviderDefinition**](IntegrationsAPI.md#GetIntegrationProviderDefinition) | **Get** /integrations/provider-types/{provider_type}/definition | Get a provider definition (capabilities, actions, schemas) for UI/CLI.
[**HardDeleteIntegration**](IntegrationsAPI.md#HardDeleteIntegration) | **Delete** /integrations/{id}/purge | Permanently delete (purge) an integration.
[**ListIntegrationAccessPortals**](IntegrationsAPI.md#ListIntegrationAccessPortals) | **Get** /integrations/{id}/access-portals | List public access portals for an integration (for admin links to public portal pages).
[**ListIntegrationEntities**](IntegrationsAPI.md#ListIntegrationEntities) | **Get** /integrations/{id}/entities | List resource entities for devices belonging to an integration.
[**ListIntegrationOps**](IntegrationsAPI.md#ListIntegrationOps) | **Get** /integrations/{id}/ops | List provider-specific ops available for an integration.
[**ListIntegrationProviderTypes**](IntegrationsAPI.md#ListIntegrationProviderTypes) | **Get** /integrations/provider-types | List supported integration provider types (canonical values).
[**ListIntegrations**](IntegrationsAPI.md#ListIntegrations) | **Get** /integrations | List integrations for the organization context (X-Org).
[**RestoreIntegration**](IntegrationsAPI.md#RestoreIntegration) | **Post** /integrations/{id}/restore | Restore a soft-deleted integration.
[**RestoreIntegrationAccessInvite**](IntegrationsAPI.md#RestoreIntegrationAccessInvite) | **Post** /integrations/{id}/access-invites/{invite_link_id}/restore | Restore (un-revoke) an access invite.
[**UpdateIntegration**](IntegrationsAPI.md#UpdateIntegration) | **Put** /integrations/{id} | Update an integration.
[**UpdateIntegrationAccessInvite**](IntegrationsAPI.md#UpdateIntegrationAccessInvite) | **Put** /integrations/{id}/access-invites/{invite_link_id} | Update an access invite (portals, validity, max_uses). Only active invites can be updated.
[**UpdateIntegrationAccessPortal**](IntegrationsAPI.md#UpdateIntegrationAccessPortal) | **Put** /integrations/{id}/access-portals/{portal_id} | Update a public access portal.



## CreateIntegration

> IntegrationResponse CreateIntegration(ctx).XOrg(xOrg).CreateIntegrationRequest(createIntegrationRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Create an integration in an organization.

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
	createIntegrationRequest := *openapiclient.NewCreateIntegrationRequest("OrgId_example", "ProviderType_example") // CreateIntegrationRequest |
	includeDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.CreateIntegration(context.Background()).XOrg(xOrg).CreateIntegrationRequest(createIntegrationRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.CreateIntegration``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateIntegration`: IntegrationResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.CreateIntegration`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCreateIntegrationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **createIntegrationRequest** | [**CreateIntegrationRequest**](CreateIntegrationRequest.md) |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**IntegrationResponse**](IntegrationResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CreateIntegrationAccessPortal

> CreateAccessPortalResponse CreateIntegrationAccessPortal(ctx, id).XOrg(xOrg).CreateAccessPortalRequest(createAccessPortalRequest).Execute()

Create a public access portal for an integration.

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
	createAccessPortalRequest := *openapiclient.NewCreateAccessPortalRequest(*openapiclient.NewLocalizedString(map[string]string{"key": "Inner_example"})) // CreateAccessPortalRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.CreateIntegrationAccessPortal(context.Background(), id).XOrg(xOrg).CreateAccessPortalRequest(createAccessPortalRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.CreateIntegrationAccessPortal``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateIntegrationAccessPortal`: CreateAccessPortalResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.CreateIntegrationAccessPortal`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCreateIntegrationAccessPortalRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **createAccessPortalRequest** | [**CreateAccessPortalRequest**](CreateAccessPortalRequest.md) |  |

### Return type

[**CreateAccessPortalResponse**](CreateAccessPortalResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteIntegrationAccessInvite

> DeleteIntegrationAccessInvite(ctx, id, inviteLinkId).XOrg(xOrg).Execute()

Delete an access invite (permanently).

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
	inviteLinkId := "inviteLinkId_example" // string |
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.IntegrationsAPI.DeleteIntegrationAccessInvite(context.Background(), id, inviteLinkId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.DeleteIntegrationAccessInvite``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteLinkId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteIntegrationAccessInviteRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteIntegrationAccessPortal

> DeleteIntegrationAccessPortal(ctx, id, portalId).XOrg(xOrg).Execute()

Delete a public access portal.

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
	portalId := "portalId_example" // string |
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.IntegrationsAPI.DeleteIntegrationAccessPortal(context.Background(), id, portalId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.DeleteIntegrationAccessPortal``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**portalId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteIntegrationAccessPortalRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ExecuteIntegrationOp

> ExecuteIntegrationOp(ctx, id, opId).XOrg(xOrg).Body(body).Execute()

Execute a provider-specific op for an integration.

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
	opId := "opId_example" // string |
	xOrg := "xOrg_example" // string |
	body := interface{}(987) // interface{} |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.IntegrationsAPI.ExecuteIntegrationOp(context.Background(), id, opId).XOrg(xOrg).Body(body).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ExecuteIntegrationOp``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**opId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiExecuteIntegrationOpRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |
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


## GetAccessPortalById

> GetAccessPortalResponse GetAccessPortalById(ctx, portalId).XOrg(xOrg).Execute()

Get an access portal by ID. Resolves the portal's building (integration) for direct links.

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
	portalId := "portalId_example" // string |
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.GetAccessPortalById(context.Background(), portalId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetAccessPortalById``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAccessPortalById`: GetAccessPortalResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.GetAccessPortalById`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**portalId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetAccessPortalByIdRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**GetAccessPortalResponse**](GetAccessPortalResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetIntegration

> IntegrationResponse GetIntegration(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()



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
	resp, r, err := apiClient.IntegrationsAPI.GetIntegration(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetIntegration``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetIntegration`: IntegrationResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.GetIntegration`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetIntegrationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**IntegrationResponse**](IntegrationResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetIntegrationDeviceMetadataSchema

> IntegrationDeviceMetadataSchemaResponse GetIntegrationDeviceMetadataSchema(ctx, id).XOrg(xOrg).Execute()

Get provider-specific JSON Schema for `device_metadata` for this integration (for generic UI labels).

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
	resp, r, err := apiClient.IntegrationsAPI.GetIntegrationDeviceMetadataSchema(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetIntegrationDeviceMetadataSchema``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetIntegrationDeviceMetadataSchema`: IntegrationDeviceMetadataSchemaResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.GetIntegrationDeviceMetadataSchema`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetIntegrationDeviceMetadataSchemaRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**IntegrationDeviceMetadataSchemaResponse**](IntegrationDeviceMetadataSchemaResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetIntegrationDiscoveredDevices

> interface{} GetIntegrationDiscoveredDevices(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

List devices discovered from the upstream provider for this integration.

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
	resp, r, err := apiClient.IntegrationsAPI.GetIntegrationDiscoveredDevices(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetIntegrationDiscoveredDevices``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetIntegrationDiscoveredDevices`: interface{}
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.GetIntegrationDiscoveredDevices`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetIntegrationDiscoveredDevicesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

**interface{}**

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetIntegrationProviderDefinition

> GetIntegrationProviderDefinition(ctx, providerType).Execute()

Get a provider definition (capabilities, actions, schemas) for UI/CLI.

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
	providerType := "providerType_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.IntegrationsAPI.GetIntegrationProviderDefinition(context.Background(), providerType).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetIntegrationProviderDefinition``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**providerType** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetIntegrationProviderDefinitionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## HardDeleteIntegration

> IntegrationResponse HardDeleteIntegration(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Permanently delete (purge) an integration.

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
	resp, r, err := apiClient.IntegrationsAPI.HardDeleteIntegration(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.HardDeleteIntegration``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `HardDeleteIntegration`: IntegrationResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.HardDeleteIntegration`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiHardDeleteIntegrationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**IntegrationResponse**](IntegrationResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationAccessPortals

> ListIntegrationAccessPortalsResponse ListIntegrationAccessPortals(ctx, id).XOrg(xOrg).Execute()

List public access portals for an integration (for admin links to public portal pages).

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
	resp, r, err := apiClient.IntegrationsAPI.ListIntegrationAccessPortals(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ListIntegrationAccessPortals``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListIntegrationAccessPortals`: ListIntegrationAccessPortalsResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.ListIntegrationAccessPortals`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListIntegrationAccessPortalsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**ListIntegrationAccessPortalsResponse**](ListIntegrationAccessPortalsResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationEntities

> PaginatedResponseEntityResponse ListIntegrationEntities(ctx, id).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).EntityType(entityType).Execute()

List resource entities for devices belonging to an integration.

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
	outputOptions := *openapiclient.NewMultiResourceOutputOptionsQuery(false, false, false) // MultiResourceOutputOptionsQuery |
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |
	entityType := "entityType_example" // string | Comma-separated entity types (e.g. `door,switch`). Omit for all types. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.ListIntegrationEntities(context.Background(), id).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).EntityType(entityType).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ListIntegrationEntities``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListIntegrationEntities`: PaginatedResponseEntityResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.ListIntegrationEntities`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListIntegrationEntitiesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **outputOptions** | [**MultiResourceOutputOptionsQuery**](MultiResourceOutputOptionsQuery.md) |  |
 **pagination** | [**PaginationQuery**](PaginationQuery.md) |  |
 **entityType** | **string** | Comma-separated entity types (e.g. &#x60;door,switch&#x60;). Omit for all types. |

### Return type

[**PaginatedResponseEntityResponse**](PaginatedResponseEntityResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationOps

> ListIntegrationOps(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

List provider-specific ops available for an integration.

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
	r, err := apiClient.IntegrationsAPI.ListIntegrationOps(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ListIntegrationOps``: %v\n", err)
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

Other parameters are passed through a pointer to a apiListIntegrationOpsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationProviderTypes

> map[string]map[string]string ListIntegrationProviderTypes(ctx).Execute()

List supported integration provider types (canonical values).

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.ListIntegrationProviderTypes(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ListIntegrationProviderTypes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListIntegrationProviderTypes`: map[string]map[string]string
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.ListIntegrationProviderTypes`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiListIntegrationProviderTypesRequest struct via the builder pattern


### Return type

[**map[string]map[string]string**](map.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrations

> PaginatedResponse ListIntegrations(ctx).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).ProviderType(providerType).Q(q).Execute()

List integrations for the organization context (X-Org).

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
	providerType := "providerType_example" // string | Optional filter: only integrations with this provider_type (e.g. virtual_access). (optional)
	q := "q_example" // string | Optional case-insensitive substring match on integration `name` (localized JSON), same as devices list. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.ListIntegrations(context.Background()).XOrg(xOrg).OutputOptions(outputOptions).Pagination(pagination).ProviderType(providerType).Q(q).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ListIntegrations``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListIntegrations`: PaginatedResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.ListIntegrations`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListIntegrationsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xOrg** | **string** |  |
 **outputOptions** | [**MultiResourceOutputOptionsQuery**](MultiResourceOutputOptionsQuery.md) |  |
 **pagination** | [**PaginationQuery**](PaginationQuery.md) |  |
 **providerType** | **string** | Optional filter: only integrations with this provider_type (e.g. virtual_access). |
 **q** | **string** | Optional case-insensitive substring match on integration &#x60;name&#x60; (localized JSON), same as devices list. |

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


## RestoreIntegration

> IntegrationResponse RestoreIntegration(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Restore a soft-deleted integration.

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
	resp, r, err := apiClient.IntegrationsAPI.RestoreIntegration(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.RestoreIntegration``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `RestoreIntegration`: IntegrationResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.RestoreIntegration`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiRestoreIntegrationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**IntegrationResponse**](IntegrationResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RestoreIntegrationAccessInvite

> AccessInviteListItem RestoreIntegrationAccessInvite(ctx, id, inviteLinkId).XOrg(xOrg).Execute()

Restore (un-revoke) an access invite.

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
	inviteLinkId := "inviteLinkId_example" // string |
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.RestoreIntegrationAccessInvite(context.Background(), id, inviteLinkId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.RestoreIntegrationAccessInvite``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `RestoreIntegrationAccessInvite`: AccessInviteListItem
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.RestoreIntegrationAccessInvite`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteLinkId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiRestoreIntegrationAccessInviteRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

### Return type

[**AccessInviteListItem**](AccessInviteListItem.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateIntegration

> IntegrationResponse UpdateIntegration(ctx, id).XOrg(xOrg).UpdateIntegrationRequest(updateIntegrationRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()

Update an integration.

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
	updateIntegrationRequest := *openapiclient.NewUpdateIntegrationRequest() // UpdateIntegrationRequest |
	includeDeleted := true // bool |  (optional)
	includeMetadata := true // bool |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.UpdateIntegration(context.Background(), id).XOrg(xOrg).UpdateIntegrationRequest(updateIntegrationRequest).IncludeDeleted(includeDeleted).IncludeMetadata(includeMetadata).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.UpdateIntegration``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateIntegration`: IntegrationResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.UpdateIntegration`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateIntegrationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **updateIntegrationRequest** | [**UpdateIntegrationRequest**](UpdateIntegrationRequest.md) |  |
 **includeDeleted** | **bool** |  |
 **includeMetadata** | **bool** |  |

### Return type

[**IntegrationResponse**](IntegrationResponse.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateIntegrationAccessInvite

> AccessInviteListItem UpdateIntegrationAccessInvite(ctx, id, inviteLinkId).XOrg(xOrg).UpdateAccessInviteRequest(updateAccessInviteRequest).Execute()

Update an access invite (portals, validity, max_uses). Only active invites can be updated.

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
	inviteLinkId := "inviteLinkId_example" // string |
	xOrg := "xOrg_example" // string |
	updateAccessInviteRequest := *openapiclient.NewUpdateAccessInviteRequest() // UpdateAccessInviteRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.UpdateIntegrationAccessInvite(context.Background(), id, inviteLinkId).XOrg(xOrg).UpdateAccessInviteRequest(updateAccessInviteRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.UpdateIntegrationAccessInvite``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateIntegrationAccessInvite`: AccessInviteListItem
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.UpdateIntegrationAccessInvite`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteLinkId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateIntegrationAccessInviteRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |
 **updateAccessInviteRequest** | [**UpdateAccessInviteRequest**](UpdateAccessInviteRequest.md) |  |

### Return type

[**AccessInviteListItem**](AccessInviteListItem.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateIntegrationAccessPortal

> AccessPortalListItem UpdateIntegrationAccessPortal(ctx, id, portalId).XOrg(xOrg).UpdateAccessPortalRequest(updateAccessPortalRequest).Execute()

Update a public access portal.

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
	portalId := "portalId_example" // string |
	xOrg := "xOrg_example" // string |
	updateAccessPortalRequest := *openapiclient.NewUpdateAccessPortalRequest() // UpdateAccessPortalRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.UpdateIntegrationAccessPortal(context.Background(), id, portalId).XOrg(xOrg).UpdateAccessPortalRequest(updateAccessPortalRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.UpdateIntegrationAccessPortal``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateIntegrationAccessPortal`: AccessPortalListItem
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.UpdateIntegrationAccessPortal`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**portalId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateIntegrationAccessPortalRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |
 **updateAccessPortalRequest** | [**UpdateAccessPortalRequest**](UpdateAccessPortalRequest.md) |  |

### Return type

[**AccessPortalListItem**](AccessPortalListItem.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
