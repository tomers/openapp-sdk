# CreateWebhookResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **string** |  | [optional]
**Description** | Pointer to **string** |  | [optional]
**Enabled** | **bool** |  |
**EventTypes** | Pointer to **[]string** |  | [optional]
**Id** | **string** |  |
**LastDeliveryAt** | Pointer to **string** |  | [optional]
**LastDeliveryStatus** | Pointer to **string** |  | [optional]
**OrgId** | **string** |  |
**UpdatedAt** | Pointer to **string** |  | [optional]
**Url** | **string** |  |
**SigningSecret** | **string** | The HMAC signing secret, returned exactly once at creation. Store it securely; it is used to verify the &#x60;X-OpenApp-Signature&#x60; header on every delivery and is never returned again. |

## Methods

### NewCreateWebhookResponse

`func NewCreateWebhookResponse(enabled bool, id string, orgId string, url string, signingSecret string, ) *CreateWebhookResponse`

NewCreateWebhookResponse instantiates a new CreateWebhookResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateWebhookResponseWithDefaults

`func NewCreateWebhookResponseWithDefaults() *CreateWebhookResponse`

NewCreateWebhookResponseWithDefaults instantiates a new CreateWebhookResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *CreateWebhookResponse) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *CreateWebhookResponse) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *CreateWebhookResponse) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *CreateWebhookResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetDescription

`func (o *CreateWebhookResponse) GetDescription() string`

GetDescription returns the Description field if non-nil, zero value otherwise.

### GetDescriptionOk

`func (o *CreateWebhookResponse) GetDescriptionOk() (*string, bool)`

GetDescriptionOk returns a tuple with the Description field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDescription

`func (o *CreateWebhookResponse) SetDescription(v string)`

SetDescription sets Description field to given value.

### HasDescription

`func (o *CreateWebhookResponse) HasDescription() bool`

HasDescription returns a boolean if a field has been set.

### GetEnabled

`func (o *CreateWebhookResponse) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *CreateWebhookResponse) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *CreateWebhookResponse) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.


### GetEventTypes

`func (o *CreateWebhookResponse) GetEventTypes() []string`

GetEventTypes returns the EventTypes field if non-nil, zero value otherwise.

### GetEventTypesOk

`func (o *CreateWebhookResponse) GetEventTypesOk() (*[]string, bool)`

GetEventTypesOk returns a tuple with the EventTypes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEventTypes

`func (o *CreateWebhookResponse) SetEventTypes(v []string)`

SetEventTypes sets EventTypes field to given value.

### HasEventTypes

`func (o *CreateWebhookResponse) HasEventTypes() bool`

HasEventTypes returns a boolean if a field has been set.

### GetId

`func (o *CreateWebhookResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *CreateWebhookResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *CreateWebhookResponse) SetId(v string)`

SetId sets Id field to given value.


### GetLastDeliveryAt

`func (o *CreateWebhookResponse) GetLastDeliveryAt() string`

GetLastDeliveryAt returns the LastDeliveryAt field if non-nil, zero value otherwise.

### GetLastDeliveryAtOk

`func (o *CreateWebhookResponse) GetLastDeliveryAtOk() (*string, bool)`

GetLastDeliveryAtOk returns a tuple with the LastDeliveryAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastDeliveryAt

`func (o *CreateWebhookResponse) SetLastDeliveryAt(v string)`

SetLastDeliveryAt sets LastDeliveryAt field to given value.

### HasLastDeliveryAt

`func (o *CreateWebhookResponse) HasLastDeliveryAt() bool`

HasLastDeliveryAt returns a boolean if a field has been set.

### GetLastDeliveryStatus

`func (o *CreateWebhookResponse) GetLastDeliveryStatus() string`

GetLastDeliveryStatus returns the LastDeliveryStatus field if non-nil, zero value otherwise.

### GetLastDeliveryStatusOk

`func (o *CreateWebhookResponse) GetLastDeliveryStatusOk() (*string, bool)`

GetLastDeliveryStatusOk returns a tuple with the LastDeliveryStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastDeliveryStatus

`func (o *CreateWebhookResponse) SetLastDeliveryStatus(v string)`

SetLastDeliveryStatus sets LastDeliveryStatus field to given value.

### HasLastDeliveryStatus

`func (o *CreateWebhookResponse) HasLastDeliveryStatus() bool`

HasLastDeliveryStatus returns a boolean if a field has been set.

### GetOrgId

`func (o *CreateWebhookResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *CreateWebhookResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *CreateWebhookResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetUpdatedAt

`func (o *CreateWebhookResponse) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *CreateWebhookResponse) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *CreateWebhookResponse) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *CreateWebhookResponse) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### GetUrl

`func (o *CreateWebhookResponse) GetUrl() string`

GetUrl returns the Url field if non-nil, zero value otherwise.

### GetUrlOk

`func (o *CreateWebhookResponse) GetUrlOk() (*string, bool)`

GetUrlOk returns a tuple with the Url field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUrl

`func (o *CreateWebhookResponse) SetUrl(v string)`

SetUrl sets Url field to given value.


### GetSigningSecret

`func (o *CreateWebhookResponse) GetSigningSecret() string`

GetSigningSecret returns the SigningSecret field if non-nil, zero value otherwise.

### GetSigningSecretOk

`func (o *CreateWebhookResponse) GetSigningSecretOk() (*string, bool)`

GetSigningSecretOk returns a tuple with the SigningSecret field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSigningSecret

`func (o *CreateWebhookResponse) SetSigningSecret(v string)`

SetSigningSecret sets SigningSecret field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
