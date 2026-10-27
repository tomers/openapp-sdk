# \AgentsAPI

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateAgent**](AgentsAPI.md#CreateAgent) | **Post** /agents |
[**CreateAgentCredential**](AgentsAPI.md#CreateAgentCredential) | **Post** /agents/{id}/credentials |
[**CreateAlias**](AgentsAPI.md#CreateAlias) | **Post** /orgs/{org_id}/aliases |
[**CreatePmsConnector**](AgentsAPI.md#CreatePmsConnector) | **Post** /orgs/{org_id}/connectors/pms |
[**CreateSkill**](AgentsAPI.md#CreateSkill) | **Post** /orgs/{org_id}/skills |
[**DeleteAlias**](AgentsAPI.md#DeleteAlias) | **Delete** /orgs/{org_id}/aliases/{id} |
[**DeletePmsConnector**](AgentsAPI.md#DeletePmsConnector) | **Delete** /orgs/{org_id}/connectors/pms/{connector_id} | Delete a PMS connector and its signing secret. Deliveries signed with that secret are rejected from then on; this is how a leaked secret is retired (create a new connector and reconfigure the PMS).
[**GetAgent**](AgentsAPI.md#GetAgent) | **Get** /agents/{id} |
[**ListAgents**](AgentsAPI.md#ListAgents) | **Get** /agents |
[**ListAliases**](AgentsAPI.md#ListAliases) | **Get** /orgs/{org_id}/aliases |
[**ListPmsConnectors**](AgentsAPI.md#ListPmsConnectors) | **Get** /orgs/{org_id}/connectors/pms |
[**ListSkills**](AgentsAPI.md#ListSkills) | **Get** /orgs/{org_id}/skills |
[**McpJsonrpc**](AgentsAPI.md#McpJsonrpc) | **Post** /mcp |
[**OauthProtectedResource**](AgentsAPI.md#OauthProtectedResource) | **Get** /.well-known/oauth-protected-resource |
[**PmsInbound**](AgentsAPI.md#PmsInbound) | **Post** /pms/{connector_id}/inbound | Receive a PMS event. Unauthenticated at the gateway; each request is authenticated by an HMAC-SHA256 signature made with the connector&#39;s signing secret.
[**ResolveOrgQuery**](AgentsAPI.md#ResolveOrgQuery) | **Post** /orgs/{org_id}/resolve |
[**RevokeAgent**](AgentsAPI.md#RevokeAgent) | **Delete** /agents/{id} |
[**UpdateAgent**](AgentsAPI.md#UpdateAgent) | **Patch** /agents/{id} |



## CreateAgent

> AgentResponse CreateAgent(ctx).CreateAgentRequest(createAgentRequest).Execute()



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
	createAgentRequest := *openapiclient.NewCreateAgentRequest("Name_example") // CreateAgentRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AgentsAPI.CreateAgent(context.Background()).CreateAgentRequest(createAgentRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.CreateAgent``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateAgent`: AgentResponse
	fmt.Fprintf(os.Stdout, "Response from `AgentsAPI.CreateAgent`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCreateAgentRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createAgentRequest** | [**CreateAgentRequest**](CreateAgentRequest.md) |  |

### Return type

[**AgentResponse**](AgentResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CreateAgentCredential

> CreateAgentCredentialResponse CreateAgentCredential(ctx, id).CreateAgentCredentialRequest(createAgentCredentialRequest).Execute()



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
	createAgentCredentialRequest := *openapiclient.NewCreateAgentCredentialRequest() // CreateAgentCredentialRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AgentsAPI.CreateAgentCredential(context.Background(), id).CreateAgentCredentialRequest(createAgentCredentialRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.CreateAgentCredential``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateAgentCredential`: CreateAgentCredentialResponse
	fmt.Fprintf(os.Stdout, "Response from `AgentsAPI.CreateAgentCredential`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCreateAgentCredentialRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **createAgentCredentialRequest** | [**CreateAgentCredentialRequest**](CreateAgentCredentialRequest.md) |  |

### Return type

[**CreateAgentCredentialResponse**](CreateAgentCredentialResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CreateAlias

> AliasResponse CreateAlias(ctx, orgId).CreateAliasRequest(createAliasRequest).Execute()



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
	createAliasRequest := *openapiclient.NewCreateAliasRequest("Alias_example", "ResourceId_example", "ResourceType_example") // CreateAliasRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AgentsAPI.CreateAlias(context.Background(), orgId).CreateAliasRequest(createAliasRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.CreateAlias``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateAlias`: AliasResponse
	fmt.Fprintf(os.Stdout, "Response from `AgentsAPI.CreateAlias`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCreateAliasRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **createAliasRequest** | [**CreateAliasRequest**](CreateAliasRequest.md) |  |

### Return type

[**AliasResponse**](AliasResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CreatePmsConnector

> CreatePmsConnectorResponse CreatePmsConnector(ctx, orgId).CreatePmsConnectorRequest(createPmsConnectorRequest).Execute()



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
	createPmsConnectorRequest := *openapiclient.NewCreatePmsConnectorRequest("Name_example") // CreatePmsConnectorRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AgentsAPI.CreatePmsConnector(context.Background(), orgId).CreatePmsConnectorRequest(createPmsConnectorRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.CreatePmsConnector``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreatePmsConnector`: CreatePmsConnectorResponse
	fmt.Fprintf(os.Stdout, "Response from `AgentsAPI.CreatePmsConnector`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCreatePmsConnectorRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **createPmsConnectorRequest** | [**CreatePmsConnectorRequest**](CreatePmsConnectorRequest.md) |  |

### Return type

[**CreatePmsConnectorResponse**](CreatePmsConnectorResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CreateSkill

> SkillResponse CreateSkill(ctx, orgId).CreateSkillRequest(createSkillRequest).Execute()



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
	createSkillRequest := *openapiclient.NewCreateSkillRequest("AgentId_example", "Name_example", []openapiclient.SkillStep{*openapiclient.NewSkillStep("Tool_example")}, []string{"TriggerEventTypes_example"}) // CreateSkillRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AgentsAPI.CreateSkill(context.Background(), orgId).CreateSkillRequest(createSkillRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.CreateSkill``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateSkill`: SkillResponse
	fmt.Fprintf(os.Stdout, "Response from `AgentsAPI.CreateSkill`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiCreateSkillRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **createSkillRequest** | [**CreateSkillRequest**](CreateSkillRequest.md) |  |

### Return type

[**SkillResponse**](SkillResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DeleteAlias

> DeleteAlias(ctx, orgId, id).Execute()



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
	id := "id_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.AgentsAPI.DeleteAlias(context.Background(), orgId, id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.DeleteAlias``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteAliasRequest struct via the builder pattern


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


## DeletePmsConnector

> DeletePmsConnector(ctx, orgId, connectorId).Execute()

Delete a PMS connector and its signing secret. Deliveries signed with that secret are rejected from then on; this is how a leaked secret is retired (create a new connector and reconfigure the PMS).

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
	connectorId := "connectorId_example" // string |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.AgentsAPI.DeletePmsConnector(context.Background(), orgId, connectorId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.DeletePmsConnector``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |
**connectorId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiDeletePmsConnectorRequest struct via the builder pattern


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


## GetAgent

> AgentResponse GetAgent(ctx, id).Execute()



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
	resp, r, err := apiClient.AgentsAPI.GetAgent(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.GetAgent``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAgent`: AgentResponse
	fmt.Fprintf(os.Stdout, "Response from `AgentsAPI.GetAgent`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiGetAgentRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**AgentResponse**](AgentResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListAgents

> PaginatedResponseAgentResponse ListAgents(ctx).Limit(limit).Offset(offset).Execute()



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
	limit := int32(50) // int32 | Number of items per page. Default from config, max 200. (optional)
	offset := int32(0) // int32 | Number of items to skip. Default 0. (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AgentsAPI.ListAgents(context.Background()).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.ListAgents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListAgents`: PaginatedResponseAgentResponse
	fmt.Fprintf(os.Stdout, "Response from `AgentsAPI.ListAgents`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListAgentsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int32** | Number of items per page. Default from config, max 200. |
 **offset** | **int32** | Number of items to skip. Default 0. |

### Return type

[**PaginatedResponseAgentResponse**](PaginatedResponseAgentResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListAliases

> ListAliases(ctx, orgId).ResourceType(resourceType).ResourceId(resourceId).Execute()



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
	resourceType := "resourceType_example" // string |  (optional)
	resourceId := "resourceId_example" // string |  (optional)

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.AgentsAPI.ListAliases(context.Background(), orgId).ResourceType(resourceType).ResourceId(resourceId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.ListAliases``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListAliasesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **resourceType** | **string** |  |
 **resourceId** | **string** |  |

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


## ListPmsConnectors

> []PmsConnectorResponse ListPmsConnectors(ctx, orgId).Execute()



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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AgentsAPI.ListPmsConnectors(context.Background(), orgId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.ListPmsConnectors``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListPmsConnectors`: []PmsConnectorResponse
	fmt.Fprintf(os.Stdout, "Response from `AgentsAPI.ListPmsConnectors`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListPmsConnectorsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**[]PmsConnectorResponse**](PmsConnectorResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListSkills

> ListSkills(ctx, orgId).Execute()



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

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.AgentsAPI.ListSkills(context.Background(), orgId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.ListSkills``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiListSkillsRequest struct via the builder pattern


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


## McpJsonrpc

> JsonRpcResponse McpJsonrpc(ctx).Body(body).Execute()



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
	body := interface{}(987) // interface{} |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AgentsAPI.McpJsonrpc(context.Background()).Body(body).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.McpJsonrpc``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `McpJsonrpc`: JsonRpcResponse
	fmt.Fprintf(os.Stdout, "Response from `AgentsAPI.McpJsonrpc`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiMcpJsonrpcRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | **interface{}** |  |

### Return type

[**JsonRpcResponse**](JsonRpcResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## OauthProtectedResource

> OauthProtectedResource(ctx).Execute()



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
	r, err := apiClient.AgentsAPI.OauthProtectedResource(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.OauthProtectedResource``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiOauthProtectedResourceRequest struct via the builder pattern


### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PmsInbound

> PmsInbound(ctx, connectorId).XOpenAppTimestamp(xOpenAppTimestamp).XOpenAppSignature(xOpenAppSignature).PmsInboundPayload(pmsInboundPayload).Execute()

Receive a PMS event. Unauthenticated at the gateway; each request is authenticated by an HMAC-SHA256 signature made with the connector's signing secret.



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
	connectorId := "connectorId_example" // string |
	xOpenAppTimestamp := "xOpenAppTimestamp_example" // string | Unix time (seconds) the request was signed
	xOpenAppSignature := "xOpenAppSignature_example" // string | `sha256=<hex HMAC-SHA256 of \"{timestamp}.{body}\">`
	pmsInboundPayload := *openapiclient.NewPmsInboundPayload("Event_example") // PmsInboundPayload |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	r, err := apiClient.AgentsAPI.PmsInbound(context.Background(), connectorId).XOpenAppTimestamp(xOpenAppTimestamp).XOpenAppSignature(xOpenAppSignature).PmsInboundPayload(pmsInboundPayload).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.PmsInbound``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**connectorId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiPmsInboundRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **xOpenAppTimestamp** | **string** | Unix time (seconds) the request was signed |
 **xOpenAppSignature** | **string** | &#x60;sha256&#x3D;&lt;hex HMAC-SHA256 of \&quot;{timestamp}.{body}\&quot;&gt;&#x60; |
 **pmsInboundPayload** | [**PmsInboundPayload**](PmsInboundPayload.md) |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ResolveOrgQuery

> ResolveResponse ResolveOrgQuery(ctx, orgId).ResolveRequest(resolveRequest).Execute()



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
	resolveRequest := *openapiclient.NewResolveRequest("Query_example") // ResolveRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AgentsAPI.ResolveOrgQuery(context.Background(), orgId).ResolveRequest(resolveRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.ResolveOrgQuery``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ResolveOrgQuery`: ResolveResponse
	fmt.Fprintf(os.Stdout, "Response from `AgentsAPI.ResolveOrgQuery`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**orgId** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiResolveOrgQueryRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **resolveRequest** | [**ResolveRequest**](ResolveRequest.md) |  |

### Return type

[**ResolveResponse**](ResolveResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RevokeAgent

> RevokeAgent(ctx, id).Execute()



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
	r, err := apiClient.AgentsAPI.RevokeAgent(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.RevokeAgent``: %v\n", err)
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

Other parameters are passed through a pointer to a apiRevokeAgentRequest struct via the builder pattern


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


## UpdateAgent

> AgentResponse UpdateAgent(ctx, id).UpdateAgentRequest(updateAgentRequest).Execute()



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
	updateAgentRequest := *openapiclient.NewUpdateAgentRequest() // UpdateAgentRequest |

	apiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080_openapp_example_secret")
	if err != nil {
		fmt.Fprintf(os.Stderr, "NewAPIClient: %v\n", err)
		os.Exit(1)
	}
	defer apiClient.Close()
	resp, r, err := apiClient.AgentsAPI.UpdateAgent(context.Background(), id).UpdateAgentRequest(updateAgentRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AgentsAPI.UpdateAgent``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateAgent`: AgentResponse
	fmt.Fprintf(os.Stdout, "Response from `AgentsAPI.UpdateAgent`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **string** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateAgentRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **updateAgentRequest** | [**UpdateAgentRequest**](UpdateAgentRequest.md) |  |

### Return type

[**AgentResponse**](AgentResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
