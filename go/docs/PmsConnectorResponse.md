# PmsConnectorResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**Id** | **string** |  |
**Name** | **string** |  |
**Provider** | **string** |  |
**WebhookUrlPath** | **string** |  |

## Methods

### NewPmsConnectorResponse

`func NewPmsConnectorResponse(id string, name string, provider string, webhookUrlPath string, ) *PmsConnectorResponse`

NewPmsConnectorResponse instantiates a new PmsConnectorResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPmsConnectorResponseWithDefaults

`func NewPmsConnectorResponseWithDefaults() *PmsConnectorResponse`

NewPmsConnectorResponseWithDefaults instantiates a new PmsConnectorResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *PmsConnectorResponse) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *PmsConnectorResponse) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *PmsConnectorResponse) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *PmsConnectorResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *PmsConnectorResponse) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *PmsConnectorResponse) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetId

`func (o *PmsConnectorResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PmsConnectorResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PmsConnectorResponse) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *PmsConnectorResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PmsConnectorResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PmsConnectorResponse) SetName(v string)`

SetName sets Name field to given value.


### GetProvider

`func (o *PmsConnectorResponse) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *PmsConnectorResponse) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *PmsConnectorResponse) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetWebhookUrlPath

`func (o *PmsConnectorResponse) GetWebhookUrlPath() string`

GetWebhookUrlPath returns the WebhookUrlPath field if non-nil, zero value otherwise.

### GetWebhookUrlPathOk

`func (o *PmsConnectorResponse) GetWebhookUrlPathOk() (*string, bool)`

GetWebhookUrlPathOk returns a tuple with the WebhookUrlPath field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWebhookUrlPath

`func (o *PmsConnectorResponse) SetWebhookUrlPath(v string)`

SetWebhookUrlPath sets WebhookUrlPath field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
