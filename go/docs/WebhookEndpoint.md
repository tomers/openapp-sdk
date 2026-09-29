# WebhookEndpoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**Description** | Pointer to **NullableString** |  | [optional]
**Enabled** | **bool** |  |
**EventTypes** | Pointer to **[]string** |  | [optional]
**Id** | **string** |  |
**LastDeliveryAt** | Pointer to **NullableString** |  | [optional]
**LastDeliveryStatus** | Pointer to **NullableString** |  | [optional]
**OrgId** | **string** |  |
**UpdatedAt** | Pointer to **NullableString** |  | [optional]
**Url** | **string** |  |

## Methods

### NewWebhookEndpoint

`func NewWebhookEndpoint(enabled bool, id string, orgId string, url string, ) *WebhookEndpoint`

NewWebhookEndpoint instantiates a new WebhookEndpoint object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewWebhookEndpointWithDefaults

`func NewWebhookEndpointWithDefaults() *WebhookEndpoint`

NewWebhookEndpointWithDefaults instantiates a new WebhookEndpoint object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *WebhookEndpoint) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *WebhookEndpoint) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *WebhookEndpoint) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *WebhookEndpoint) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *WebhookEndpoint) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *WebhookEndpoint) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetDescription

`func (o *WebhookEndpoint) GetDescription() string`

GetDescription returns the Description field if non-nil, zero value otherwise.

### GetDescriptionOk

`func (o *WebhookEndpoint) GetDescriptionOk() (*string, bool)`

GetDescriptionOk returns a tuple with the Description field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDescription

`func (o *WebhookEndpoint) SetDescription(v string)`

SetDescription sets Description field to given value.

### HasDescription

`func (o *WebhookEndpoint) HasDescription() bool`

HasDescription returns a boolean if a field has been set.

### SetDescriptionNil

`func (o *WebhookEndpoint) SetDescriptionNil(b bool)`

 SetDescriptionNil sets the value for Description to be an explicit nil

### UnsetDescription
`func (o *WebhookEndpoint) UnsetDescription()`

UnsetDescription ensures that no value is present for Description, not even an explicit nil
### GetEnabled

`func (o *WebhookEndpoint) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *WebhookEndpoint) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *WebhookEndpoint) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.


### GetEventTypes

`func (o *WebhookEndpoint) GetEventTypes() []string`

GetEventTypes returns the EventTypes field if non-nil, zero value otherwise.

### GetEventTypesOk

`func (o *WebhookEndpoint) GetEventTypesOk() (*[]string, bool)`

GetEventTypesOk returns a tuple with the EventTypes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEventTypes

`func (o *WebhookEndpoint) SetEventTypes(v []string)`

SetEventTypes sets EventTypes field to given value.

### HasEventTypes

`func (o *WebhookEndpoint) HasEventTypes() bool`

HasEventTypes returns a boolean if a field has been set.

### SetEventTypesNil

`func (o *WebhookEndpoint) SetEventTypesNil(b bool)`

 SetEventTypesNil sets the value for EventTypes to be an explicit nil

### UnsetEventTypes
`func (o *WebhookEndpoint) UnsetEventTypes()`

UnsetEventTypes ensures that no value is present for EventTypes, not even an explicit nil
### GetId

`func (o *WebhookEndpoint) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *WebhookEndpoint) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *WebhookEndpoint) SetId(v string)`

SetId sets Id field to given value.


### GetLastDeliveryAt

`func (o *WebhookEndpoint) GetLastDeliveryAt() string`

GetLastDeliveryAt returns the LastDeliveryAt field if non-nil, zero value otherwise.

### GetLastDeliveryAtOk

`func (o *WebhookEndpoint) GetLastDeliveryAtOk() (*string, bool)`

GetLastDeliveryAtOk returns a tuple with the LastDeliveryAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastDeliveryAt

`func (o *WebhookEndpoint) SetLastDeliveryAt(v string)`

SetLastDeliveryAt sets LastDeliveryAt field to given value.

### HasLastDeliveryAt

`func (o *WebhookEndpoint) HasLastDeliveryAt() bool`

HasLastDeliveryAt returns a boolean if a field has been set.

### SetLastDeliveryAtNil

`func (o *WebhookEndpoint) SetLastDeliveryAtNil(b bool)`

 SetLastDeliveryAtNil sets the value for LastDeliveryAt to be an explicit nil

### UnsetLastDeliveryAt
`func (o *WebhookEndpoint) UnsetLastDeliveryAt()`

UnsetLastDeliveryAt ensures that no value is present for LastDeliveryAt, not even an explicit nil
### GetLastDeliveryStatus

`func (o *WebhookEndpoint) GetLastDeliveryStatus() string`

GetLastDeliveryStatus returns the LastDeliveryStatus field if non-nil, zero value otherwise.

### GetLastDeliveryStatusOk

`func (o *WebhookEndpoint) GetLastDeliveryStatusOk() (*string, bool)`

GetLastDeliveryStatusOk returns a tuple with the LastDeliveryStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastDeliveryStatus

`func (o *WebhookEndpoint) SetLastDeliveryStatus(v string)`

SetLastDeliveryStatus sets LastDeliveryStatus field to given value.

### HasLastDeliveryStatus

`func (o *WebhookEndpoint) HasLastDeliveryStatus() bool`

HasLastDeliveryStatus returns a boolean if a field has been set.

### SetLastDeliveryStatusNil

`func (o *WebhookEndpoint) SetLastDeliveryStatusNil(b bool)`

 SetLastDeliveryStatusNil sets the value for LastDeliveryStatus to be an explicit nil

### UnsetLastDeliveryStatus
`func (o *WebhookEndpoint) UnsetLastDeliveryStatus()`

UnsetLastDeliveryStatus ensures that no value is present for LastDeliveryStatus, not even an explicit nil
### GetOrgId

`func (o *WebhookEndpoint) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *WebhookEndpoint) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *WebhookEndpoint) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetUpdatedAt

`func (o *WebhookEndpoint) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *WebhookEndpoint) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *WebhookEndpoint) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *WebhookEndpoint) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *WebhookEndpoint) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *WebhookEndpoint) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil
### GetUrl

`func (o *WebhookEndpoint) GetUrl() string`

GetUrl returns the Url field if non-nil, zero value otherwise.

### GetUrlOk

`func (o *WebhookEndpoint) GetUrlOk() (*string, bool)`

GetUrlOk returns a tuple with the Url field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUrl

`func (o *WebhookEndpoint) SetUrl(v string)`

SetUrl sets Url field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
