# UpdateEntityRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ChannelIndex** | Pointer to **NullableInt32** |  | [optional]
**EntityType** | Pointer to **NullableString** |  | [optional]
**ExternalId** | Pointer to **NullableString** |  | [optional]
**Metadata** | Pointer to **map[string]interface{}** | Update (replace) persisted entity metadata.  - &#x60;None&#x60;: no change - &#x60;Some(None)&#x60;: clear metadata - &#x60;Some(Some(map))&#x60;: replace with provided map | [optional]
**Name** | Pointer to **NullableString** | Update entity name.  - &#x60;None&#x60;: no change - &#x60;Some(None)&#x60;: clear - &#x60;Some(Some(name))&#x60;: set (trimmed; empty becomes clear) | [optional]
**ZoneId** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUpdateEntityRequest

`func NewUpdateEntityRequest() *UpdateEntityRequest`

NewUpdateEntityRequest instantiates a new UpdateEntityRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateEntityRequestWithDefaults

`func NewUpdateEntityRequestWithDefaults() *UpdateEntityRequest`

NewUpdateEntityRequestWithDefaults instantiates a new UpdateEntityRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetChannelIndex

`func (o *UpdateEntityRequest) GetChannelIndex() int32`

GetChannelIndex returns the ChannelIndex field if non-nil, zero value otherwise.

### GetChannelIndexOk

`func (o *UpdateEntityRequest) GetChannelIndexOk() (*int32, bool)`

GetChannelIndexOk returns a tuple with the ChannelIndex field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChannelIndex

`func (o *UpdateEntityRequest) SetChannelIndex(v int32)`

SetChannelIndex sets ChannelIndex field to given value.

### HasChannelIndex

`func (o *UpdateEntityRequest) HasChannelIndex() bool`

HasChannelIndex returns a boolean if a field has been set.

### SetChannelIndexNil

`func (o *UpdateEntityRequest) SetChannelIndexNil(b bool)`

 SetChannelIndexNil sets the value for ChannelIndex to be an explicit nil

### UnsetChannelIndex
`func (o *UpdateEntityRequest) UnsetChannelIndex()`

UnsetChannelIndex ensures that no value is present for ChannelIndex, not even an explicit nil
### GetEntityType

`func (o *UpdateEntityRequest) GetEntityType() string`

GetEntityType returns the EntityType field if non-nil, zero value otherwise.

### GetEntityTypeOk

`func (o *UpdateEntityRequest) GetEntityTypeOk() (*string, bool)`

GetEntityTypeOk returns a tuple with the EntityType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityType

`func (o *UpdateEntityRequest) SetEntityType(v string)`

SetEntityType sets EntityType field to given value.

### HasEntityType

`func (o *UpdateEntityRequest) HasEntityType() bool`

HasEntityType returns a boolean if a field has been set.

### SetEntityTypeNil

`func (o *UpdateEntityRequest) SetEntityTypeNil(b bool)`

 SetEntityTypeNil sets the value for EntityType to be an explicit nil

### UnsetEntityType
`func (o *UpdateEntityRequest) UnsetEntityType()`

UnsetEntityType ensures that no value is present for EntityType, not even an explicit nil
### GetExternalId

`func (o *UpdateEntityRequest) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *UpdateEntityRequest) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *UpdateEntityRequest) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.

### HasExternalId

`func (o *UpdateEntityRequest) HasExternalId() bool`

HasExternalId returns a boolean if a field has been set.

### SetExternalIdNil

`func (o *UpdateEntityRequest) SetExternalIdNil(b bool)`

 SetExternalIdNil sets the value for ExternalId to be an explicit nil

### UnsetExternalId
`func (o *UpdateEntityRequest) UnsetExternalId()`

UnsetExternalId ensures that no value is present for ExternalId, not even an explicit nil
### GetMetadata

`func (o *UpdateEntityRequest) GetMetadata() map[string]interface{}`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *UpdateEntityRequest) GetMetadataOk() (*map[string]interface{}, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *UpdateEntityRequest) SetMetadata(v map[string]interface{})`

SetMetadata sets Metadata field to given value.

### HasMetadata

`func (o *UpdateEntityRequest) HasMetadata() bool`

HasMetadata returns a boolean if a field has been set.

### GetName

`func (o *UpdateEntityRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UpdateEntityRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UpdateEntityRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *UpdateEntityRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *UpdateEntityRequest) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *UpdateEntityRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetZoneId

`func (o *UpdateEntityRequest) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *UpdateEntityRequest) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *UpdateEntityRequest) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.

### HasZoneId

`func (o *UpdateEntityRequest) HasZoneId() bool`

HasZoneId returns a boolean if a field has been set.

### SetZoneIdNil

`func (o *UpdateEntityRequest) SetZoneIdNil(b bool)`

 SetZoneIdNil sets the value for ZoneId to be an explicit nil

### UnsetZoneId
`func (o *UpdateEntityRequest) UnsetZoneId()`

UnsetZoneId ensures that no value is present for ZoneId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
