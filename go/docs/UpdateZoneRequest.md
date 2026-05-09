# UpdateZoneRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExternalId** | Pointer to **NullableString** |  | [optional]
**Name** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**ParentZoneId** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUpdateZoneRequest

`func NewUpdateZoneRequest() *UpdateZoneRequest`

NewUpdateZoneRequest instantiates a new UpdateZoneRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateZoneRequestWithDefaults

`func NewUpdateZoneRequestWithDefaults() *UpdateZoneRequest`

NewUpdateZoneRequestWithDefaults instantiates a new UpdateZoneRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExternalId

`func (o *UpdateZoneRequest) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *UpdateZoneRequest) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *UpdateZoneRequest) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.

### HasExternalId

`func (o *UpdateZoneRequest) HasExternalId() bool`

HasExternalId returns a boolean if a field has been set.

### SetExternalIdNil

`func (o *UpdateZoneRequest) SetExternalIdNil(b bool)`

 SetExternalIdNil sets the value for ExternalId to be an explicit nil

### UnsetExternalId
`func (o *UpdateZoneRequest) UnsetExternalId()`

UnsetExternalId ensures that no value is present for ExternalId, not even an explicit nil
### GetName

`func (o *UpdateZoneRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UpdateZoneRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UpdateZoneRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.

### HasName

`func (o *UpdateZoneRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *UpdateZoneRequest) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *UpdateZoneRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetParentZoneId

`func (o *UpdateZoneRequest) GetParentZoneId() string`

GetParentZoneId returns the ParentZoneId field if non-nil, zero value otherwise.

### GetParentZoneIdOk

`func (o *UpdateZoneRequest) GetParentZoneIdOk() (*string, bool)`

GetParentZoneIdOk returns a tuple with the ParentZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetParentZoneId

`func (o *UpdateZoneRequest) SetParentZoneId(v string)`

SetParentZoneId sets ParentZoneId field to given value.

### HasParentZoneId

`func (o *UpdateZoneRequest) HasParentZoneId() bool`

HasParentZoneId returns a boolean if a field has been set.

### SetParentZoneIdNil

`func (o *UpdateZoneRequest) SetParentZoneIdNil(b bool)`

 SetParentZoneIdNil sets the value for ParentZoneId to be an explicit nil

### UnsetParentZoneId
`func (o *UpdateZoneRequest) UnsetParentZoneId()`

UnsetParentZoneId ensures that no value is present for ParentZoneId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
