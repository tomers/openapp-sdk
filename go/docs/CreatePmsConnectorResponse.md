# CreatePmsConnectorResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **string** |  |
**Name** | **string** |  |
**Provider** | **string** |  |
**SigningSecret** | **string** | HMAC-SHA256 signing key, returned exactly once. The PMS signs each delivery with it (see &#x60;POST /pms/{connector_id}/inbound&#x60;); it is never returned again. |
**WebhookUrlPath** | **string** |  |

## Methods

### NewCreatePmsConnectorResponse

`func NewCreatePmsConnectorResponse(id string, name string, provider string, signingSecret string, webhookUrlPath string, ) *CreatePmsConnectorResponse`

NewCreatePmsConnectorResponse instantiates a new CreatePmsConnectorResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreatePmsConnectorResponseWithDefaults

`func NewCreatePmsConnectorResponseWithDefaults() *CreatePmsConnectorResponse`

NewCreatePmsConnectorResponseWithDefaults instantiates a new CreatePmsConnectorResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *CreatePmsConnectorResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *CreatePmsConnectorResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *CreatePmsConnectorResponse) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *CreatePmsConnectorResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreatePmsConnectorResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreatePmsConnectorResponse) SetName(v string)`

SetName sets Name field to given value.


### GetProvider

`func (o *CreatePmsConnectorResponse) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *CreatePmsConnectorResponse) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *CreatePmsConnectorResponse) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetSigningSecret

`func (o *CreatePmsConnectorResponse) GetSigningSecret() string`

GetSigningSecret returns the SigningSecret field if non-nil, zero value otherwise.

### GetSigningSecretOk

`func (o *CreatePmsConnectorResponse) GetSigningSecretOk() (*string, bool)`

GetSigningSecretOk returns a tuple with the SigningSecret field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSigningSecret

`func (o *CreatePmsConnectorResponse) SetSigningSecret(v string)`

SetSigningSecret sets SigningSecret field to given value.


### GetWebhookUrlPath

`func (o *CreatePmsConnectorResponse) GetWebhookUrlPath() string`

GetWebhookUrlPath returns the WebhookUrlPath field if non-nil, zero value otherwise.

### GetWebhookUrlPathOk

`func (o *CreatePmsConnectorResponse) GetWebhookUrlPathOk() (*string, bool)`

GetWebhookUrlPathOk returns a tuple with the WebhookUrlPath field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWebhookUrlPath

`func (o *CreatePmsConnectorResponse) SetWebhookUrlPath(v string)`

SetWebhookUrlPath sets WebhookUrlPath field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
