# \IntegrationsAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateIntegration**](IntegrationsAPI.md#CreateIntegration) | **Post** /integrations | Create an integration in an organization.
[**CreateIntegrationAccessInvite**](IntegrationsAPI.md#CreateIntegrationAccessInvite) | **Post** /integrations/{id}/access-invites | Create an access invite granting portal_open on one or more portals.
[**CreateIntegrationAccessPortal**](IntegrationsAPI.md#CreateIntegrationAccessPortal) | **Post** /integrations/{id}/access-portals | Create a public access portal for an integration.
[**DeleteAccessInvitePhoto**](IntegrationsAPI.md#DeleteAccessInvitePhoto) | **Delete** /integrations/{id}/access-invites/{invite_link_id}/photo |
[**DeleteIntegrationAccessInvite**](IntegrationsAPI.md#DeleteIntegrationAccessInvite) | **Delete** /integrations/{id}/access-invites/{invite_link_id} | Delete an access invite (permanently).
[**DeleteIntegrationAccessPortal**](IntegrationsAPI.md#DeleteIntegrationAccessPortal) | **Delete** /integrations/{id}/access-portals/{portal_id} | Delete a public access portal.
[**DismissIntegrationAccessInviteRenewalRequest**](IntegrationsAPI.md#DismissIntegrationAccessInviteRenewalRequest) | **Delete** /integrations/{id}/access-invites/{invite_link_id}/renewal-requests/{request_id} | Dismiss a pending renewal request for an access invite.
[**ExecuteIntegrationOp**](IntegrationsAPI.md#ExecuteIntegrationOp) | **Post** /integrations/{id}/ops/{op_id} | Execute a provider-specific op for an integration.
[**GetAccessInviteMessageMedia**](IntegrationsAPI.md#GetAccessInviteMessageMedia) | **Get** /integrations/{id}/access-invites/{invite_link_id}/message-media/{slot} |
[**GetAccessInvitePhoto**](IntegrationsAPI.md#GetAccessInvitePhoto) | **Get** /integrations/{id}/access-invites/{invite_link_id}/photo |
[**GetAccessPortalById**](IntegrationsAPI.md#GetAccessPortalById) | **Get** /integrations/access-portals/{portal_id} | Get an access portal by ID. Resolves the portal&#39;s building (integration) for direct links.
[**GetIntegration**](IntegrationsAPI.md#GetIntegration) | **Get** /integrations/{id} |
[**GetIntegrationDeviceMetadataSchema**](IntegrationsAPI.md#GetIntegrationDeviceMetadataSchema) | **Get** /integrations/{id}/device-metadata-schema | Get provider-specific JSON Schema for &#x60;device_metadata&#x60; for this integration (for generic UI labels).
[**GetIntegrationDiscoveredDevices**](IntegrationsAPI.md#GetIntegrationDiscoveredDevices) | **Get** /integrations/{id}/discovered-devices | List devices discovered from the upstream provider for this integration.
[**GetIntegrationProviderDefinition**](IntegrationsAPI.md#GetIntegrationProviderDefinition) | **Get** /integrations/provider-types/{provider_type}/definition | Get a provider definition (capabilities, actions, schemas) for UI/CLI.
[**GetIntegrationTransferPreview**](IntegrationsAPI.md#GetIntegrationTransferPreview) | **Get** /integrations/{id}/transfer-preview | Read-only preview of a transfer&#39;s impact.
[**GetSiteAccessDevices**](IntegrationsAPI.md#GetSiteAccessDevices) | **Get** /integrations/{id}/access-devices | Hardware inventory for a virtual_access site.
[**GetSiteAccessOverview**](IntegrationsAPI.md#GetSiteAccessOverview) | **Get** /integrations/{id}/access-overview | Counts and summaries for a virtual-access site overview.
[**GetSiteReadiness**](IntegrationsAPI.md#GetSiteReadiness) | **Get** /integrations/{id}/readiness | Configuration completeness for a virtual_access site.
[**GetTasmotaProvisioning**](IntegrationsAPI.md#GetTasmotaProvisioning) | **Get** /devices/{id}/tasmota/provisioning |
[**GetWaveshareProvisioningStatus**](IntegrationsAPI.md#GetWaveshareProvisioningStatus) | **Get** /integrations/{id}/provisioning/status | Get Waveshare provisioning status for an integration.
[**HardDeleteIntegration**](IntegrationsAPI.md#HardDeleteIntegration) | **Delete** /integrations/{id}/purge | Permanently delete (purge) an integration.
[**ListDeletedIntegrationAccessPortals**](IntegrationsAPI.md#ListDeletedIntegrationAccessPortals) | **Get** /integrations/{id}/access-portals/deleted | List soft-deleted public access portals for an integration (restore / trash view).
[**ListIntegrationAccessInviteRenewalRequests**](IntegrationsAPI.md#ListIntegrationAccessInviteRenewalRequests) | **Get** /integrations/{id}/access-invites/{invite_link_id}/renewal-requests | List pending renewal requests for an access invite.
[**ListIntegrationAccessInviteUsage**](IntegrationsAPI.md#ListIntegrationAccessInviteUsage) | **Get** /integrations/{id}/access-invites/{invite_link_id}/usage | &#x60;GET /integrations/{id}/access-invites/{invite_link_id}/usage&#x60;
[**ListIntegrationAccessInvites**](IntegrationsAPI.md#ListIntegrationAccessInvites) | **Get** /integrations/{id}/access-invites | List access invites for an integration.
[**ListIntegrationAccessPortals**](IntegrationsAPI.md#ListIntegrationAccessPortals) | **Get** /integrations/{id}/access-portals | List public access portals for an integration (for admin links to public portal pages).
[**ListIntegrationDoorUsage**](IntegrationsAPI.md#ListIntegrationDoorUsage) | **Get** /integrations/{id}/doors/{device_id}/usage | &#x60;GET /integrations/{id}/doors/{device_id}/usage&#x60;
[**ListIntegrationEntities**](IntegrationsAPI.md#ListIntegrationEntities) | **Get** /integrations/{id}/entities | List resource entities for devices belonging to an integration.
[**ListIntegrationOps**](IntegrationsAPI.md#ListIntegrationOps) | **Get** /integrations/{id}/ops | List provider-specific ops available for an integration.
[**ListIntegrationProviderTypes**](IntegrationsAPI.md#ListIntegrationProviderTypes) | **Get** /integrations/provider-types | List supported integration provider types (canonical values).
[**ListIntegrationUsers**](IntegrationsAPI.md#ListIntegrationUsers) | **Get** /integrations/{id}/integration-users | GET &#x60;/integrations/{id}/integration-users?device_id&#x3D;...&#x60;.
[**ListIntegrations**](IntegrationsAPI.md#ListIntegrations) | **Get** /integrations | List integrations for the organization context (X-Org).
[**PostAccessInviteMessageMedia**](IntegrationsAPI.md#PostAccessInviteMessageMedia) | **Post** /integrations/{id}/access-invites/{invite_link_id}/message-media |
[**PostAccessInvitePhoto**](IntegrationsAPI.md#PostAccessInvitePhoto) | **Post** /integrations/{id}/access-invites/{invite_link_id}/photo |
[**PostIntegrationTransfer**](IntegrationsAPI.md#PostIntegrationTransfer) | **Post** /integrations/{id}/transfer | Move or duplicate an integration (and its dependents) into another org.
[**PostPalgateProbeLinkedAccount**](IntegrationsAPI.md#PostPalgateProbeLinkedAccount) | **Post** /integration-setup/v1/palgate/probe-linked-account |
[**PostRefreshIntegrationDeviceChannelCounts**](IntegrationsAPI.md#PostRefreshIntegrationDeviceChannelCounts) | **Post** /integrations/{id}/discovered-devices/refresh-channel-counts | Explicitly refresh provider-reported channel counts on linked devices.
[**PostTasmotaProvisionLan**](IntegrationsAPI.md#PostTasmotaProvisionLan) | **Post** /devices/{id}/tasmota/provision-lan |
[**PostTasmotaRotateCredentials**](IntegrationsAPI.md#PostTasmotaRotateCredentials) | **Post** /devices/{id}/tasmota/rotate-credentials |
[**PostWaveshareProvisioningAck**](IntegrationsAPI.md#PostWaveshareProvisioningAck) | **Post** /integrations/{id}/provisioning/ack | Acknowledge that the device was reprovisioned (store the current connection config snapshot).
[**PreviewIntegrationAccessInvite**](IntegrationsAPI.md#PreviewIntegrationAccessInvite) | **Post** /integrations/{id}/access-invites/preview | Validate a desired invitation without creating physical access.
[**ReconcileIntegrationAccessInvite**](IntegrationsAPI.md#ReconcileIntegrationAccessInvite) | **Post** /integrations/{id}/access-invites/reconcile | Make the invitation state for an external record equal a desired state.
[**RegenerateIntegrationAccessInviteToken**](IntegrationsAPI.md#RegenerateIntegrationAccessInviteToken) | **Post** /integrations/{id}/access-invites/{invite_link_id}/regenerate-token | Regenerate invite token (returns new shareable link; old link stops working).
[**RestoreIntegration**](IntegrationsAPI.md#RestoreIntegration) | **Post** /integrations/{id}/restore | Restore a soft-deleted integration.
[**RestoreIntegrationAccessInvite**](IntegrationsAPI.md#RestoreIntegrationAccessInvite) | **Post** /integrations/{id}/access-invites/{invite_link_id}/restore | Restore (un-revoke) an access invite.
[**RestoreIntegrationAccessPortal**](IntegrationsAPI.md#RestoreIntegrationAccessPortal) | **Post** /integrations/{id}/access-portals/{portal_id}/restore | Restore a soft-deleted public access portal (reactivates its public_id / printed URL).
[**RevokeIntegrationAccessInvite**](IntegrationsAPI.md#RevokeIntegrationAccessInvite) | **Post** /integrations/{id}/access-invites/{invite_link_id}/revoke | Revoke an access invite.
[**UpdateIntegration**](IntegrationsAPI.md#UpdateIntegration) | **Put** /integrations/{id} | Update an integration.
[**UpdateIntegrationAccessInvite**](IntegrationsAPI.md#UpdateIntegrationAccessInvite) | **Put** /integrations/{id}/access-invites/{invite_link_id} | Update an access invite (portals, validity, max_uses). Only active invites can be updated.
[**UpdateIntegrationAccessPortal**](IntegrationsAPI.md#UpdateIntegrationAccessPortal) | **Put** /integrations/{id}/access-portals/{portal_id} | Update a public access portal.



## CreateIntegration

> IntegrationResponse CreateIntegration(ctx).XOrg(xOrg).CreateIntegrationRequest(createIntegrationRequest).IncludeDeleted(includeDeleted).Execute()

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.CreateIntegration(context.Background()).XOrg(xOrg).CreateIntegrationRequest(createIntegrationRequest).IncludeDeleted(includeDeleted).Execute()
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

### Return type

[**IntegrationResponse**](IntegrationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CreateIntegrationAccessInvite

> CreateAccessInviteResponse CreateIntegrationAccessInvite(ctx, id).XOrg(xOrg).CreateAccessInviteRequest(createAccessInviteRequest).IdempotencyKey(idempotencyKey).XCorrelationId(xCorrelationId).Execute()

Create an access invite granting portal_open on one or more portals.



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
	createAccessInviteRequest := *openapiclient.NewCreateAccessInviteRequest([]string{"PortalIds_example"}, []openapiclient.InviteScheduleEntryInput{*openapiclient.NewInviteScheduleEntryInput()}) // CreateAccessInviteRequest |
	idempotencyKey := "idempotencyKey_example" // string | Replay key; a retry with the same key and body returns the original result without creating a second invitation. (optional)
	xCorrelationId := "xCorrelationId_example" // string | Caller request id recorded with the invitation and its audit events. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.CreateIntegrationAccessInvite(context.Background(), id).XOrg(xOrg).CreateAccessInviteRequest(createAccessInviteRequest).IdempotencyKey(idempotencyKey).XCorrelationId(xCorrelationId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.CreateIntegrationAccessInvite``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateIntegrationAccessInvite`: CreateAccessInviteResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.CreateIntegrationAccessInvite`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCreateIntegrationAccessInviteRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **createAccessInviteRequest** | [**CreateAccessInviteRequest**](CreateAccessInviteRequest.md) |  |
 **idempotencyKey** | **string** | Replay key; a retry with the same key and body returns the original result without creating a second invitation. |
 **xCorrelationId** | **string** | Caller request id recorded with the invitation and its audit events. |

### Return type

[**CreateAccessInviteResponse**](CreateAccessInviteResponse.md)

### Authorization

[api_key](../README.md#api_key)

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteAccessInvitePhoto

> DeleteAccessInvitePhoto(ctx, id, inviteLinkId).XOrg(xOrg).Execute()



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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.IntegrationsAPI.DeleteAccessInvitePhoto(context.Background(), id, inviteLinkId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.DeleteAccessInvitePhoto``: %v\n", err)
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

Other parameters are passed through a pointer to a apiDeleteAccessInvitePhotoRequest struct via the builder pattern


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


## DeleteIntegrationAccessInvite

> DeleteIntegrationAccessInvite(ctx, id, inviteLinkId).XOrg(xOrg).XCorrelationId(xCorrelationId).Execute()

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
	xCorrelationId := "xCorrelationId_example" // string | Caller request id recorded on the audit event. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.IntegrationsAPI.DeleteIntegrationAccessInvite(context.Background(), id, inviteLinkId).XOrg(xOrg).XCorrelationId(xCorrelationId).Execute()
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
 **xCorrelationId** | **string** | Caller request id recorded on the audit event. |

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DismissIntegrationAccessInviteRenewalRequest

> DismissIntegrationAccessInviteRenewalRequest(ctx, id, inviteLinkId, requestId).XOrg(xOrg).Execute()

Dismiss a pending renewal request for an access invite.



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
	requestId := "requestId_example" // string |
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.IntegrationsAPI.DismissIntegrationAccessInviteRenewalRequest(context.Background(), id, inviteLinkId, requestId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.DismissIntegrationAccessInviteRenewalRequest``: %v\n", err)
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
**requestId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDismissIntegrationAccessInviteRenewalRequestRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



 **xOrg** | **string** |  |

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetAccessInviteMessageMedia

> ResourceImageUrlResponse GetAccessInviteMessageMedia(ctx, id, inviteLinkId, slot).XOrg(xOrg).Execute()



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
	slot := "slot_example" // string |
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.GetAccessInviteMessageMedia(context.Background(), id, inviteLinkId, slot).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetAccessInviteMessageMedia``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAccessInviteMessageMedia`: ResourceImageUrlResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.GetAccessInviteMessageMedia`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteLinkId** | **string** |  |
**slot** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetAccessInviteMessageMediaRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



 **xOrg** | **string** |  |

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


## GetAccessInvitePhoto

> ResourceImageUrlResponse GetAccessInvitePhoto(ctx, id, inviteLinkId).XOrg(xOrg).Execute()



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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.GetAccessInvitePhoto(context.Background(), id, inviteLinkId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetAccessInvitePhoto``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAccessInvitePhoto`: ResourceImageUrlResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.GetAccessInvitePhoto`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteLinkId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetAccessInvitePhotoRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetIntegration

> IntegrationResponse GetIntegration(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).Execute()



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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.GetIntegration(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).Execute()
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

### Return type

[**IntegrationResponse**](IntegrationResponse.md)

### Authorization

[api_key](../README.md#api_key)

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetIntegrationDiscoveredDevices

> interface{} GetIntegrationDiscoveredDevices(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).Execute()

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.GetIntegrationDiscoveredDevices(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).Execute()
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

### Return type

**interface{}**

### Authorization

[api_key](../README.md#api_key)

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetIntegrationTransferPreview

> TransferPreviewResponse GetIntegrationTransferPreview(ctx, id).XOrg(xOrg).Mode(mode).Execute()

Read-only preview of a transfer's impact.

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
	mode := "mode_example" // string | `move` (default) or `duplicate`; controls which side-effect warnings apply. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.GetIntegrationTransferPreview(context.Background(), id).XOrg(xOrg).Mode(mode).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetIntegrationTransferPreview``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetIntegrationTransferPreview`: TransferPreviewResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.GetIntegrationTransferPreview`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetIntegrationTransferPreviewRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **mode** | **string** | &#x60;move&#x60; (default) or &#x60;duplicate&#x60;; controls which side-effect warnings apply. |

### Return type

[**TransferPreviewResponse**](TransferPreviewResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetSiteAccessDevices

> SiteAccessDevicesResponse GetSiteAccessDevices(ctx, id).XOrg(xOrg).Execute()

Hardware inventory for a virtual_access site.

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
	resp, r, err := apiClient.IntegrationsAPI.GetSiteAccessDevices(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetSiteAccessDevices``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetSiteAccessDevices`: SiteAccessDevicesResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.GetSiteAccessDevices`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetSiteAccessDevicesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**SiteAccessDevicesResponse**](SiteAccessDevicesResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetSiteAccessOverview

> SiteAccessOverviewResponse GetSiteAccessOverview(ctx, id).XOrg(xOrg).Execute()

Counts and summaries for a virtual-access site overview.

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
	resp, r, err := apiClient.IntegrationsAPI.GetSiteAccessOverview(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetSiteAccessOverview``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetSiteAccessOverview`: SiteAccessOverviewResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.GetSiteAccessOverview`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetSiteAccessOverviewRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**SiteAccessOverviewResponse**](SiteAccessOverviewResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetSiteReadiness

> SiteReadiness GetSiteReadiness(ctx, id).XOrg(xOrg).Execute()

Configuration completeness for a virtual_access site.

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
	resp, r, err := apiClient.IntegrationsAPI.GetSiteReadiness(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetSiteReadiness``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetSiteReadiness`: SiteReadiness
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.GetSiteReadiness`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetSiteReadinessRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**SiteReadiness**](SiteReadiness.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetTasmotaProvisioning

> TasmotaProvisioningResponse GetTasmotaProvisioning(ctx, id).XOrg(xOrg).Execute()



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
	resp, r, err := apiClient.IntegrationsAPI.GetTasmotaProvisioning(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetTasmotaProvisioning``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetTasmotaProvisioning`: TasmotaProvisioningResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.GetTasmotaProvisioning`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetTasmotaProvisioningRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**TasmotaProvisioningResponse**](TasmotaProvisioningResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetWaveshareProvisioningStatus

> WaveshareProvisioningStatusResponse GetWaveshareProvisioningStatus(ctx, id).XOrg(xOrg).Execute()

Get Waveshare provisioning status for an integration.

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
	resp, r, err := apiClient.IntegrationsAPI.GetWaveshareProvisioningStatus(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.GetWaveshareProvisioningStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetWaveshareProvisioningStatus`: WaveshareProvisioningStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.GetWaveshareProvisioningStatus`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetWaveshareProvisioningStatusRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**WaveshareProvisioningStatusResponse**](WaveshareProvisioningStatusResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## HardDeleteIntegration

> IntegrationResponse HardDeleteIntegration(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).Execute()

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.HardDeleteIntegration(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).Execute()
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

### Return type

[**IntegrationResponse**](IntegrationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListDeletedIntegrationAccessPortals

> ListIntegrationAccessPortalsResponse ListDeletedIntegrationAccessPortals(ctx, id).XOrg(xOrg).Execute()

List soft-deleted public access portals for an integration (restore / trash view).

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
	resp, r, err := apiClient.IntegrationsAPI.ListDeletedIntegrationAccessPortals(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ListDeletedIntegrationAccessPortals``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListDeletedIntegrationAccessPortals`: ListIntegrationAccessPortalsResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.ListDeletedIntegrationAccessPortals`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListDeletedIntegrationAccessPortalsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**ListIntegrationAccessPortalsResponse**](ListIntegrationAccessPortalsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationAccessInviteRenewalRequests

> ListAccessInviteRenewalRequestsResponse ListIntegrationAccessInviteRenewalRequests(ctx, id, inviteLinkId).XOrg(xOrg).Execute()

List pending renewal requests for an access invite.

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.ListIntegrationAccessInviteRenewalRequests(context.Background(), id, inviteLinkId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ListIntegrationAccessInviteRenewalRequests``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListIntegrationAccessInviteRenewalRequests`: ListAccessInviteRenewalRequestsResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.ListIntegrationAccessInviteRenewalRequests`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteLinkId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListIntegrationAccessInviteRenewalRequestsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

### Return type

[**ListAccessInviteRenewalRequestsResponse**](ListAccessInviteRenewalRequestsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationAccessInviteUsage

> AccessUsageResponse ListIntegrationAccessInviteUsage(ctx, id, inviteLinkId).XOrg(xOrg).Execute()

`GET /integrations/{id}/access-invites/{invite_link_id}/usage`

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.ListIntegrationAccessInviteUsage(context.Background(), id, inviteLinkId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ListIntegrationAccessInviteUsage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListIntegrationAccessInviteUsage`: AccessUsageResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.ListIntegrationAccessInviteUsage`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteLinkId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListIntegrationAccessInviteUsageRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

### Return type

[**AccessUsageResponse**](AccessUsageResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationAccessInvites

> PaginatedResponseAccessInviteListItem ListIntegrationAccessInvites(ctx, id).XOrg(xOrg).Pagination(pagination).Kind(kind).ExternalSource(externalSource).ExternalRecordId(externalRecordId).State(state).PortalId(portalId).ValidFromAfter(validFromAfter).ValidToBefore(validToBefore).Execute()

List access invites for an integration.



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
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |
	kind := "kind_example" // string |  (optional)
	externalSource := "externalSource_example" // string | External system slug to filter on (`external_ref.source`). Served by the external-reference index, so an adapter can reconcile without keeping its own invite-id mapping. (optional)
	externalRecordId := "externalRecordId_example" // string | External record id to filter on. Requires `external_source`; the pair resolves to at most one live invitation. (optional)
	state := "state_example" // string | Computed state: `active`, `scheduled`, `expired`, `disabled`, or `revoked`. (optional)
	portalId := "portalId_example" // string | Only invitations granting this portal ULID. (optional)
	validFromAfter := "validFromAfter_example" // string | Only invitations whose window starts at or after this RFC 3339 instant. (optional)
	validToBefore := "validToBefore_example" // string | Only invitations whose window ends at or before this RFC 3339 instant. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.ListIntegrationAccessInvites(context.Background(), id).XOrg(xOrg).Pagination(pagination).Kind(kind).ExternalSource(externalSource).ExternalRecordId(externalRecordId).State(state).PortalId(portalId).ValidFromAfter(validFromAfter).ValidToBefore(validToBefore).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ListIntegrationAccessInvites``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListIntegrationAccessInvites`: PaginatedResponseAccessInviteListItem
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.ListIntegrationAccessInvites`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListIntegrationAccessInvitesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **pagination** | [**PaginationQuery**](PaginationQuery.md) |  |
 **kind** | **string** |  |
 **externalSource** | **string** | External system slug to filter on (&#x60;external_ref.source&#x60;). Served by the external-reference index, so an adapter can reconcile without keeping its own invite-id mapping. |
 **externalRecordId** | **string** | External record id to filter on. Requires &#x60;external_source&#x60;; the pair resolves to at most one live invitation. |
 **state** | **string** | Computed state: &#x60;active&#x60;, &#x60;scheduled&#x60;, &#x60;expired&#x60;, &#x60;disabled&#x60;, or &#x60;revoked&#x60;. |
 **portalId** | **string** | Only invitations granting this portal ULID. |
 **validFromAfter** | **string** | Only invitations whose window starts at or after this RFC 3339 instant. |
 **validToBefore** | **string** | Only invitations whose window ends at or before this RFC 3339 instant. |

### Return type

[**PaginatedResponseAccessInviteListItem**](PaginatedResponseAccessInviteListItem.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationAccessPortals

> PaginatedResponseAccessPortalListItem ListIntegrationAccessPortals(ctx, id).XOrg(xOrg).Execute()

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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
	// response from `ListIntegrationAccessPortals`: PaginatedResponseAccessPortalListItem
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

[**PaginatedResponseAccessPortalListItem**](PaginatedResponseAccessPortalListItem.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationDoorUsage

> AccessUsageResponse ListIntegrationDoorUsage(ctx, id, deviceId).XOrg(xOrg).Execute()

`GET /integrations/{id}/doors/{device_id}/usage`

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
	deviceId := "deviceId_example" // string |
	xOrg := "xOrg_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.ListIntegrationDoorUsage(context.Background(), id, deviceId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ListIntegrationDoorUsage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListIntegrationDoorUsage`: AccessUsageResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.ListIntegrationDoorUsage`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**deviceId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListIntegrationDoorUsageRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

### Return type

[**AccessUsageResponse**](AccessUsageResponse.md)

### Authorization

[api_key](../README.md#api_key)

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
	outputOptions := *openapiclient.NewMultiResourceOutputOptionsQuery() // MultiResourceOutputOptionsQuery |
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |
	entityType := "entityType_example" // string | Comma-separated entity types (e.g. `door,switch`). Omit for all types. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationOps

> ListIntegrationOps(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).Execute()

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.IntegrationsAPI.ListIntegrationOps(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).Execute()
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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListIntegrationUsers

> IntegrationUsersResponse ListIntegrationUsers(ctx, id).XOrg(xOrg).DeviceId(deviceId).Execute()

GET `/integrations/{id}/integration-users?device_id=...`.

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
	deviceId := "deviceId_example" // string | Provider device id (when the provider's `users_admin.list_users` op is scoped to a device). (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.ListIntegrationUsers(context.Background(), id).XOrg(xOrg).DeviceId(deviceId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ListIntegrationUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListIntegrationUsers`: IntegrationUsersResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.ListIntegrationUsers`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListIntegrationUsersRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **deviceId** | **string** | Provider device id (when the provider&#39;s &#x60;users_admin.list_users&#x60; op is scoped to a device). |

### Return type

[**IntegrationUsersResponse**](IntegrationUsersResponse.md)

### Authorization

[api_key](../README.md#api_key)

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
	outputOptions := *openapiclient.NewMultiResourceOutputOptionsQuery() // MultiResourceOutputOptionsQuery |
	pagination := *openapiclient.NewPaginationQuery() // PaginationQuery |
	providerType := "providerType_example" // string | Optional filter: only integrations with this provider_type (e.g. virtual_access). (optional)
	q := "q_example" // string | Optional case-insensitive substring match on integration `name` (localized JSON), same as devices list. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostAccessInviteMessageMedia

> AccessInviteMessageMediaResponse PostAccessInviteMessageMedia(ctx, id, inviteLinkId).XOrg(xOrg).Execute()



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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.PostAccessInviteMessageMedia(context.Background(), id, inviteLinkId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.PostAccessInviteMessageMedia``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostAccessInviteMessageMedia`: AccessInviteMessageMediaResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.PostAccessInviteMessageMedia`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteLinkId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostAccessInviteMessageMediaRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

### Return type

[**AccessInviteMessageMediaResponse**](AccessInviteMessageMediaResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostAccessInvitePhoto

> ResourceImageUrlResponse PostAccessInvitePhoto(ctx, id, inviteLinkId).XOrg(xOrg).Execute()



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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.PostAccessInvitePhoto(context.Background(), id, inviteLinkId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.PostAccessInvitePhoto``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostAccessInvitePhoto`: ResourceImageUrlResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.PostAccessInvitePhoto`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteLinkId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostAccessInvitePhotoRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

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


## PostIntegrationTransfer

> TransferIntegrationResponse PostIntegrationTransfer(ctx, id).XOrg(xOrg).TransferIntegrationRequest(transferIntegrationRequest).Execute()

Move or duplicate an integration (and its dependents) into another org.



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
	transferIntegrationRequest := *openapiclient.NewTransferIntegrationRequest("TargetOrgId_example") // TransferIntegrationRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.PostIntegrationTransfer(context.Background(), id).XOrg(xOrg).TransferIntegrationRequest(transferIntegrationRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.PostIntegrationTransfer``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostIntegrationTransfer`: TransferIntegrationResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.PostIntegrationTransfer`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostIntegrationTransferRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **transferIntegrationRequest** | [**TransferIntegrationRequest**](TransferIntegrationRequest.md) |  |

### Return type

[**TransferIntegrationResponse**](TransferIntegrationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostPalgateProbeLinkedAccount

> PalgateProbeLinkedAccountResponse PostPalgateProbeLinkedAccount(ctx).PalgateProbeLinkedAccountRequest(palgateProbeLinkedAccountRequest).Execute()



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
	palgateProbeLinkedAccountRequest := *openapiclient.NewPalgateProbeLinkedAccountRequest(interface{}(123), "DeviceId_example", "OrgId_example", interface{}(123)) // PalgateProbeLinkedAccountRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.PostPalgateProbeLinkedAccount(context.Background()).PalgateProbeLinkedAccountRequest(palgateProbeLinkedAccountRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.PostPalgateProbeLinkedAccount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostPalgateProbeLinkedAccount`: PalgateProbeLinkedAccountResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.PostPalgateProbeLinkedAccount`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostPalgateProbeLinkedAccountRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **palgateProbeLinkedAccountRequest** | [**PalgateProbeLinkedAccountRequest**](PalgateProbeLinkedAccountRequest.md) |  |

### Return type

[**PalgateProbeLinkedAccountResponse**](PalgateProbeLinkedAccountResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostRefreshIntegrationDeviceChannelCounts

> interface{} PostRefreshIntegrationDeviceChannelCounts(ctx, id).XOrg(xOrg).Execute()

Explicitly refresh provider-reported channel counts on linked devices.

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
	resp, r, err := apiClient.IntegrationsAPI.PostRefreshIntegrationDeviceChannelCounts(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.PostRefreshIntegrationDeviceChannelCounts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostRefreshIntegrationDeviceChannelCounts`: interface{}
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.PostRefreshIntegrationDeviceChannelCounts`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostRefreshIntegrationDeviceChannelCountsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

**interface{}**

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostTasmotaProvisionLan

> PostTasmotaProvisionLan(ctx, id).XOrg(xOrg).TasmotaLanProvisionRequest(tasmotaLanProvisionRequest).Execute()



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
	tasmotaLanProvisionRequest := *openapiclient.NewTasmotaLanProvisionRequest("DeviceHost_example") // TasmotaLanProvisionRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.IntegrationsAPI.PostTasmotaProvisionLan(context.Background(), id).XOrg(xOrg).TasmotaLanProvisionRequest(tasmotaLanProvisionRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.PostTasmotaProvisionLan``: %v\n", err)
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

Other parameters are passed through a pointer to a apiPostTasmotaProvisionLanRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **tasmotaLanProvisionRequest** | [**TasmotaLanProvisionRequest**](TasmotaLanProvisionRequest.md) |  |

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


## PostTasmotaRotateCredentials

> TasmotaProvisioningResponse PostTasmotaRotateCredentials(ctx, id).XOrg(xOrg).Execute()



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
	resp, r, err := apiClient.IntegrationsAPI.PostTasmotaRotateCredentials(context.Background(), id).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.PostTasmotaRotateCredentials``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostTasmotaRotateCredentials`: TasmotaProvisioningResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.PostTasmotaRotateCredentials`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostTasmotaRotateCredentialsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |

### Return type

[**TasmotaProvisioningResponse**](TasmotaProvisioningResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostWaveshareProvisioningAck

> IntegrationResponse PostWaveshareProvisioningAck(ctx, id).XOrg(xOrg).Body(body).Execute()

Acknowledge that the device was reprovisioned (store the current connection config snapshot).

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
	body := interface{}(987) // interface{} |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.PostWaveshareProvisioningAck(context.Background(), id).XOrg(xOrg).Body(body).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.PostWaveshareProvisioningAck``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostWaveshareProvisioningAck`: IntegrationResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.PostWaveshareProvisioningAck`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPostWaveshareProvisioningAckRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **body** | **interface{}** |  |

### Return type

[**IntegrationResponse**](IntegrationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PreviewIntegrationAccessInvite

> PreviewAccessInviteResponse PreviewIntegrationAccessInvite(ctx, id).XOrg(xOrg).CreateAccessInviteRequest(createAccessInviteRequest).XCorrelationId(xCorrelationId).Execute()

Validate a desired invitation without creating physical access.



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
	createAccessInviteRequest := *openapiclient.NewCreateAccessInviteRequest([]string{"PortalIds_example"}, []openapiclient.InviteScheduleEntryInput{*openapiclient.NewInviteScheduleEntryInput()}) // CreateAccessInviteRequest |
	xCorrelationId := "xCorrelationId_example" // string | Caller request id used for tracing this preview. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.PreviewIntegrationAccessInvite(context.Background(), id).XOrg(xOrg).CreateAccessInviteRequest(createAccessInviteRequest).XCorrelationId(xCorrelationId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.PreviewIntegrationAccessInvite``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PreviewIntegrationAccessInvite`: PreviewAccessInviteResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.PreviewIntegrationAccessInvite`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPreviewIntegrationAccessInviteRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **createAccessInviteRequest** | [**CreateAccessInviteRequest**](CreateAccessInviteRequest.md) |  |
 **xCorrelationId** | **string** | Caller request id used for tracing this preview. |

### Return type

[**PreviewAccessInviteResponse**](PreviewAccessInviteResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ReconcileIntegrationAccessInvite

> ReconcileAccessInviteResponse ReconcileIntegrationAccessInvite(ctx, id).XOrg(xOrg).ReconcileAccessInviteRequest(reconcileAccessInviteRequest).IdempotencyKey(idempotencyKey).XCorrelationId(xCorrelationId).Execute()

Make the invitation state for an external record equal a desired state.



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
	reconcileAccessInviteRequest := *openapiclient.NewReconcileAccessInviteRequest(*openapiclient.NewExternalReferenceInput("RecordId_example", "Source_example")) // ReconcileAccessInviteRequest |
	idempotencyKey := "idempotencyKey_example" // string | Replay key; a retry with the same key and body returns the original result. (optional)
	xCorrelationId := "xCorrelationId_example" // string | Caller request id recorded with the invitation and its audit events. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.ReconcileIntegrationAccessInvite(context.Background(), id).XOrg(xOrg).ReconcileAccessInviteRequest(reconcileAccessInviteRequest).IdempotencyKey(idempotencyKey).XCorrelationId(xCorrelationId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.ReconcileIntegrationAccessInvite``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ReconcileIntegrationAccessInvite`: ReconcileAccessInviteResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.ReconcileIntegrationAccessInvite`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiReconcileIntegrationAccessInviteRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOrg** | **string** |  |
 **reconcileAccessInviteRequest** | [**ReconcileAccessInviteRequest**](ReconcileAccessInviteRequest.md) |  |
 **idempotencyKey** | **string** | Replay key; a retry with the same key and body returns the original result. |
 **xCorrelationId** | **string** | Caller request id recorded with the invitation and its audit events. |

### Return type

[**ReconcileAccessInviteResponse**](ReconcileAccessInviteResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RegenerateIntegrationAccessInviteToken

> RegenerateAccessInviteTokenResponse RegenerateIntegrationAccessInviteToken(ctx, id, inviteLinkId).XOrg(xOrg).Execute()

Regenerate invite token (returns new shareable link; old link stops working).



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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.RegenerateIntegrationAccessInviteToken(context.Background(), id, inviteLinkId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.RegenerateIntegrationAccessInviteToken``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `RegenerateIntegrationAccessInviteToken`: RegenerateAccessInviteTokenResponse
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.RegenerateIntegrationAccessInviteToken`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteLinkId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiRegenerateIntegrationAccessInviteTokenRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

### Return type

[**RegenerateAccessInviteTokenResponse**](RegenerateAccessInviteTokenResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RestoreIntegration

> IntegrationResponse RestoreIntegration(ctx, id).XOrg(xOrg).IncludeDeleted(includeDeleted).Execute()

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.RestoreIntegration(context.Background(), id).XOrg(xOrg).IncludeDeleted(includeDeleted).Execute()
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

### Return type

[**IntegrationResponse**](IntegrationResponse.md)

### Authorization

[api_key](../README.md#api_key)

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RestoreIntegrationAccessPortal

> AccessPortalListItem RestoreIntegrationAccessPortal(ctx, id, portalId).XOrg(xOrg).Execute()

Restore a soft-deleted public access portal (reactivates its public_id / printed URL).

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.RestoreIntegrationAccessPortal(context.Background(), id, portalId).XOrg(xOrg).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.RestoreIntegrationAccessPortal``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `RestoreIntegrationAccessPortal`: AccessPortalListItem
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.RestoreIntegrationAccessPortal`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**portalId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiRestoreIntegrationAccessPortalRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |

### Return type

[**AccessPortalListItem**](AccessPortalListItem.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RevokeIntegrationAccessInvite

> AccessInviteListItem RevokeIntegrationAccessInvite(ctx, id, inviteLinkId).XOrg(xOrg).XCorrelationId(xCorrelationId).Execute()

Revoke an access invite.



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
	xCorrelationId := "xCorrelationId_example" // string | Caller request id recorded on the audit event. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.RevokeIntegrationAccessInvite(context.Background(), id, inviteLinkId).XOrg(xOrg).XCorrelationId(xCorrelationId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `IntegrationsAPI.RevokeIntegrationAccessInvite``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `RevokeIntegrationAccessInvite`: AccessInviteListItem
	fmt.Fprintf(os.Stdout, "Response from `IntegrationsAPI.RevokeIntegrationAccessInvite`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |
**inviteLinkId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiRevokeIntegrationAccessInviteRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **xOrg** | **string** |  |
 **xCorrelationId** | **string** | Caller request id recorded on the audit event. |

### Return type

[**AccessInviteListItem**](AccessInviteListItem.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateIntegration

> IntegrationResponse UpdateIntegration(ctx, id).XOrg(xOrg).UpdateIntegrationRequest(updateIntegrationRequest).IncludeDeleted(includeDeleted).Execute()

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.UpdateIntegration(context.Background(), id).XOrg(xOrg).UpdateIntegrationRequest(updateIntegrationRequest).IncludeDeleted(includeDeleted).Execute()
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

### Return type

[**IntegrationResponse**](IntegrationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateIntegrationAccessInvite

> AccessInviteListItem UpdateIntegrationAccessInvite(ctx, id, inviteLinkId).XOrg(xOrg).UpdateAccessInviteRequest(updateAccessInviteRequest).XCorrelationId(xCorrelationId).Execute()

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
	xCorrelationId := "xCorrelationId_example" // string | Caller request id recorded with the invitation and its audit events. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.IntegrationsAPI.UpdateIntegrationAccessInvite(context.Background(), id, inviteLinkId).XOrg(xOrg).UpdateAccessInviteRequest(updateAccessInviteRequest).XCorrelationId(xCorrelationId).Execute()
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
 **xCorrelationId** | **string** | Caller request id recorded with the invitation and its audit events. |

### Return type

[**AccessInviteListItem**](AccessInviteListItem.md)

### Authorization

[api_key](../README.md#api_key)

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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
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

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
