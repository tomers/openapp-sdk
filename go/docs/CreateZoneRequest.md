# CreateZoneRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExternalId** | Pointer to **NullableString** |  | [optional]
**IntegrationId** | **string** |  |
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**ParentZoneId** | Pointer to **NullableString** |  | [optional]

## Methods

### NewCreateZoneRequest

`func NewCreateZoneRequest(integrationId string, name LocalizedString, ) *CreateZoneRequest`

NewCreateZoneRequest instantiates a new CreateZoneRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateZoneRequestWithDefaults

`func NewCreateZoneRequestWithDefaults() *CreateZoneRequest`

NewCreateZoneRequestWithDefaults instantiates a new CreateZoneRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExternalId

`func (o *CreateZoneRequest) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *CreateZoneRequest) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *CreateZoneRequest) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.

### HasExternalId

`func (o *CreateZoneRequest) HasExternalId() bool`

HasExternalId returns a boolean if a field has been set.

### SetExternalIdNil

`func (o *CreateZoneRequest) SetExternalIdNil(b bool)`

 SetExternalIdNil sets the value for ExternalId to be an explicit nil

### UnsetExternalId
`func (o *CreateZoneRequest) UnsetExternalId()`

UnsetExternalId ensures that no value is present for ExternalId, not even an explicit nil
### GetIntegrationId

`func (o *CreateZoneRequest) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *CreateZoneRequest) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *CreateZoneRequest) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### GetName

`func (o *CreateZoneRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateZoneRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateZoneRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetParentZoneId

`func (o *CreateZoneRequest) GetParentZoneId() string`

GetParentZoneId returns the ParentZoneId field if non-nil, zero value otherwise.

### GetParentZoneIdOk

`func (o *CreateZoneRequest) GetParentZoneIdOk() (*string, bool)`

GetParentZoneIdOk returns a tuple with the ParentZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetParentZoneId

`func (o *CreateZoneRequest) SetParentZoneId(v string)`

SetParentZoneId sets ParentZoneId field to given value.

### HasParentZoneId

`func (o *CreateZoneRequest) HasParentZoneId() bool`

HasParentZoneId returns a boolean if a field has been set.

### SetParentZoneIdNil

`func (o *CreateZoneRequest) SetParentZoneIdNil(b bool)`

 SetParentZoneIdNil sets the value for ParentZoneId to be an explicit nil

### UnsetParentZoneId
`func (o *CreateZoneRequest) UnsetParentZoneId()`

UnsetParentZoneId ensures that no value is present for ParentZoneId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
