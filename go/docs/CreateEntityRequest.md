# CreateEntityRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ChannelIndex** | Pointer to **NullableInt32** |  | [optional]
**DeviceId** | **string** |  |
**EntityType** | **string** |  |
**ExternalId** | Pointer to **NullableString** |  | [optional]
**Metadata** | Pointer to **map[string]interface{}** | Arbitrary, persisted entity metadata (integration-specific).  For example, MQTT entities can store: - &#x60;command_topic&#x60; - &#x60;toggle_payload&#x60;  Values are converted to strings (non-strings are JSON-stringified) to match &#x60;domain::Metadata&#x60;. | [optional]
**Name** | Pointer to **NullableString** | Optional friendly name for the entity. | [optional]
**ZoneId** | Pointer to **NullableString** |  | [optional]

## Methods

### NewCreateEntityRequest

`func NewCreateEntityRequest(deviceId string, entityType string, ) *CreateEntityRequest`

NewCreateEntityRequest instantiates a new CreateEntityRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateEntityRequestWithDefaults

`func NewCreateEntityRequestWithDefaults() *CreateEntityRequest`

NewCreateEntityRequestWithDefaults instantiates a new CreateEntityRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetChannelIndex

`func (o *CreateEntityRequest) GetChannelIndex() int32`

GetChannelIndex returns the ChannelIndex field if non-nil, zero value otherwise.

### GetChannelIndexOk

`func (o *CreateEntityRequest) GetChannelIndexOk() (*int32, bool)`

GetChannelIndexOk returns a tuple with the ChannelIndex field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChannelIndex

`func (o *CreateEntityRequest) SetChannelIndex(v int32)`

SetChannelIndex sets ChannelIndex field to given value.

### HasChannelIndex

`func (o *CreateEntityRequest) HasChannelIndex() bool`

HasChannelIndex returns a boolean if a field has been set.

### SetChannelIndexNil

`func (o *CreateEntityRequest) SetChannelIndexNil(b bool)`

 SetChannelIndexNil sets the value for ChannelIndex to be an explicit nil

### UnsetChannelIndex
`func (o *CreateEntityRequest) UnsetChannelIndex()`

UnsetChannelIndex ensures that no value is present for ChannelIndex, not even an explicit nil
### GetDeviceId

`func (o *CreateEntityRequest) GetDeviceId() string`

GetDeviceId returns the DeviceId field if non-nil, zero value otherwise.

### GetDeviceIdOk

`func (o *CreateEntityRequest) GetDeviceIdOk() (*string, bool)`

GetDeviceIdOk returns a tuple with the DeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceId

`func (o *CreateEntityRequest) SetDeviceId(v string)`

SetDeviceId sets DeviceId field to given value.


### GetEntityType

`func (o *CreateEntityRequest) GetEntityType() string`

GetEntityType returns the EntityType field if non-nil, zero value otherwise.

### GetEntityTypeOk

`func (o *CreateEntityRequest) GetEntityTypeOk() (*string, bool)`

GetEntityTypeOk returns a tuple with the EntityType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityType

`func (o *CreateEntityRequest) SetEntityType(v string)`

SetEntityType sets EntityType field to given value.


### GetExternalId

`func (o *CreateEntityRequest) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *CreateEntityRequest) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *CreateEntityRequest) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.

### HasExternalId

`func (o *CreateEntityRequest) HasExternalId() bool`

HasExternalId returns a boolean if a field has been set.

### SetExternalIdNil

`func (o *CreateEntityRequest) SetExternalIdNil(b bool)`

 SetExternalIdNil sets the value for ExternalId to be an explicit nil

### UnsetExternalId
`func (o *CreateEntityRequest) UnsetExternalId()`

UnsetExternalId ensures that no value is present for ExternalId, not even an explicit nil
### GetMetadata

`func (o *CreateEntityRequest) GetMetadata() map[string]interface{}`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *CreateEntityRequest) GetMetadataOk() (*map[string]interface{}, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *CreateEntityRequest) SetMetadata(v map[string]interface{})`

SetMetadata sets Metadata field to given value.

### HasMetadata

`func (o *CreateEntityRequest) HasMetadata() bool`

HasMetadata returns a boolean if a field has been set.

### GetName

`func (o *CreateEntityRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateEntityRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateEntityRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *CreateEntityRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *CreateEntityRequest) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *CreateEntityRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetZoneId

`func (o *CreateEntityRequest) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *CreateEntityRequest) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *CreateEntityRequest) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.

### HasZoneId

`func (o *CreateEntityRequest) HasZoneId() bool`

HasZoneId returns a boolean if a field has been set.

### SetZoneIdNil

`func (o *CreateEntityRequest) SetZoneIdNil(b bool)`

 SetZoneIdNil sets the value for ZoneId to be an explicit nil

### UnsetZoneId
`func (o *CreateEntityRequest) UnsetZoneId()`

UnsetZoneId ensures that no value is present for ZoneId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
