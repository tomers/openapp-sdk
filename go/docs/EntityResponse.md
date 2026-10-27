# EntityResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **time.Time** |  | [optional]
**DeletedAt** | Pointer to **time.Time** |  | [optional]
**HardDeleteAt** | Pointer to **time.Time** |  | [optional]
**PurgeAt** | Pointer to **time.Time** |  | [optional]
**UpdatedAt** | Pointer to **time.Time** |  | [optional]
**DeviceId** | **string** | Parent device ID. |
**EntityType** | [**EntityType**](EntityType.md) | Entity type (switch, light, sensor). |
**ExternalId** | Pointer to **string** | External ID from the integration. | [optional]
**Id** | **string** | Unique identifier (ULID). |
**Metadata** | Pointer to **map[string]string** |  | [optional]
**Name** | Pointer to **string** | Optional friendly name for the entity. | [optional]
**Zone** | Pointer to [**ZonePlacement**](ZonePlacement.md) | Zone placement: inherit the device&#39;s zone, name one explicitly, or none. | [optional]
**EffectiveZoneId** | Pointer to **NullableString** | Derived, read-only: the zone the entity is effectively in (&#x60;zone&#x60; resolved against the device zone), or null. | [optional]
**EntityMetadata** | Pointer to **map[string]string** |  | [optional]
**EntryKind** | Pointer to **NullableString** | Derived, read-only entry kind for switchable entries on Virtual Access devices. Omitted when the entity is not an entry or the device has no Virtual Access config. | [optional]
**State** | Pointer to **interface{}** |  | [optional]
**ZoneDoorId** | Pointer to **NullableString** | Derived, read-only: the door that lists this entity and owns its zone. Present exactly when &#x60;zone_source&#x60; is &#x60;door&#x60;, and also when that door has no zone. | [optional]
**ZoneSource** | Pointer to [**NullableZoneSource**](ZoneSource.md) | Derived, read-only: &#x60;entity&#x60; when the zone is explicit, &#x60;device&#x60; when inherited, &#x60;door&#x60; when a door that lists this entity owns the zone, null when there is no effective zone. | [optional]

## Methods

### NewEntityResponse

`func NewEntityResponse(deviceId string, entityType EntityType, id string, ) *EntityResponse`

NewEntityResponse instantiates a new EntityResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewEntityResponseWithDefaults

`func NewEntityResponseWithDefaults() *EntityResponse`

NewEntityResponseWithDefaults instantiates a new EntityResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *EntityResponse) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *EntityResponse) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *EntityResponse) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *EntityResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetDeletedAt

`func (o *EntityResponse) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *EntityResponse) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *EntityResponse) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *EntityResponse) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetHardDeleteAt

`func (o *EntityResponse) GetHardDeleteAt() time.Time`

GetHardDeleteAt returns the HardDeleteAt field if non-nil, zero value otherwise.

### GetHardDeleteAtOk

`func (o *EntityResponse) GetHardDeleteAtOk() (*time.Time, bool)`

GetHardDeleteAtOk returns a tuple with the HardDeleteAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHardDeleteAt

`func (o *EntityResponse) SetHardDeleteAt(v time.Time)`

SetHardDeleteAt sets HardDeleteAt field to given value.

### HasHardDeleteAt

`func (o *EntityResponse) HasHardDeleteAt() bool`

HasHardDeleteAt returns a boolean if a field has been set.

### GetPurgeAt

`func (o *EntityResponse) GetPurgeAt() time.Time`

GetPurgeAt returns the PurgeAt field if non-nil, zero value otherwise.

### GetPurgeAtOk

`func (o *EntityResponse) GetPurgeAtOk() (*time.Time, bool)`

GetPurgeAtOk returns a tuple with the PurgeAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurgeAt

`func (o *EntityResponse) SetPurgeAt(v time.Time)`

SetPurgeAt sets PurgeAt field to given value.

### HasPurgeAt

`func (o *EntityResponse) HasPurgeAt() bool`

HasPurgeAt returns a boolean if a field has been set.

### GetUpdatedAt

`func (o *EntityResponse) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *EntityResponse) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *EntityResponse) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *EntityResponse) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### GetDeviceId

`func (o *EntityResponse) GetDeviceId() string`

GetDeviceId returns the DeviceId field if non-nil, zero value otherwise.

### GetDeviceIdOk

`func (o *EntityResponse) GetDeviceIdOk() (*string, bool)`

GetDeviceIdOk returns a tuple with the DeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceId

`func (o *EntityResponse) SetDeviceId(v string)`

SetDeviceId sets DeviceId field to given value.


### GetEntityType

`func (o *EntityResponse) GetEntityType() EntityType`

GetEntityType returns the EntityType field if non-nil, zero value otherwise.

### GetEntityTypeOk

`func (o *EntityResponse) GetEntityTypeOk() (*EntityType, bool)`

GetEntityTypeOk returns a tuple with the EntityType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityType

`func (o *EntityResponse) SetEntityType(v EntityType)`

SetEntityType sets EntityType field to given value.


### GetExternalId

`func (o *EntityResponse) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *EntityResponse) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *EntityResponse) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.

### HasExternalId

`func (o *EntityResponse) HasExternalId() bool`

HasExternalId returns a boolean if a field has been set.

### GetId

`func (o *EntityResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *EntityResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *EntityResponse) SetId(v string)`

SetId sets Id field to given value.


### GetMetadata

`func (o *EntityResponse) GetMetadata() map[string]string`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *EntityResponse) GetMetadataOk() (*map[string]string, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *EntityResponse) SetMetadata(v map[string]string)`

SetMetadata sets Metadata field to given value.

### HasMetadata

`func (o *EntityResponse) HasMetadata() bool`

HasMetadata returns a boolean if a field has been set.

### GetName

`func (o *EntityResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *EntityResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *EntityResponse) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *EntityResponse) HasName() bool`

HasName returns a boolean if a field has been set.

### GetZone

`func (o *EntityResponse) GetZone() ZonePlacement`

GetZone returns the Zone field if non-nil, zero value otherwise.

### GetZoneOk

`func (o *EntityResponse) GetZoneOk() (*ZonePlacement, bool)`

GetZoneOk returns a tuple with the Zone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZone

`func (o *EntityResponse) SetZone(v ZonePlacement)`

SetZone sets Zone field to given value.

### HasZone

`func (o *EntityResponse) HasZone() bool`

HasZone returns a boolean if a field has been set.

### GetEffectiveZoneId

`func (o *EntityResponse) GetEffectiveZoneId() string`

GetEffectiveZoneId returns the EffectiveZoneId field if non-nil, zero value otherwise.

### GetEffectiveZoneIdOk

`func (o *EntityResponse) GetEffectiveZoneIdOk() (*string, bool)`

GetEffectiveZoneIdOk returns a tuple with the EffectiveZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEffectiveZoneId

`func (o *EntityResponse) SetEffectiveZoneId(v string)`

SetEffectiveZoneId sets EffectiveZoneId field to given value.

### HasEffectiveZoneId

`func (o *EntityResponse) HasEffectiveZoneId() bool`

HasEffectiveZoneId returns a boolean if a field has been set.

### SetEffectiveZoneIdNil

`func (o *EntityResponse) SetEffectiveZoneIdNil(b bool)`

 SetEffectiveZoneIdNil sets the value for EffectiveZoneId to be an explicit nil

### UnsetEffectiveZoneId
`func (o *EntityResponse) UnsetEffectiveZoneId()`

UnsetEffectiveZoneId ensures that no value is present for EffectiveZoneId, not even an explicit nil
### GetEntityMetadata

`func (o *EntityResponse) GetEntityMetadata() map[string]string`

GetEntityMetadata returns the EntityMetadata field if non-nil, zero value otherwise.

### GetEntityMetadataOk

`func (o *EntityResponse) GetEntityMetadataOk() (*map[string]string, bool)`

GetEntityMetadataOk returns a tuple with the EntityMetadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityMetadata

`func (o *EntityResponse) SetEntityMetadata(v map[string]string)`

SetEntityMetadata sets EntityMetadata field to given value.

### HasEntityMetadata

`func (o *EntityResponse) HasEntityMetadata() bool`

HasEntityMetadata returns a boolean if a field has been set.

### GetEntryKind

`func (o *EntityResponse) GetEntryKind() string`

GetEntryKind returns the EntryKind field if non-nil, zero value otherwise.

### GetEntryKindOk

`func (o *EntityResponse) GetEntryKindOk() (*string, bool)`

GetEntryKindOk returns a tuple with the EntryKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntryKind

`func (o *EntityResponse) SetEntryKind(v string)`

SetEntryKind sets EntryKind field to given value.

### HasEntryKind

`func (o *EntityResponse) HasEntryKind() bool`

HasEntryKind returns a boolean if a field has been set.

### SetEntryKindNil

`func (o *EntityResponse) SetEntryKindNil(b bool)`

 SetEntryKindNil sets the value for EntryKind to be an explicit nil

### UnsetEntryKind
`func (o *EntityResponse) UnsetEntryKind()`

UnsetEntryKind ensures that no value is present for EntryKind, not even an explicit nil
### GetState

`func (o *EntityResponse) GetState() interface{}`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *EntityResponse) GetStateOk() (*interface{}, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *EntityResponse) SetState(v interface{})`

SetState sets State field to given value.

### HasState

`func (o *EntityResponse) HasState() bool`

HasState returns a boolean if a field has been set.

### SetStateNil

`func (o *EntityResponse) SetStateNil(b bool)`

 SetStateNil sets the value for State to be an explicit nil

### UnsetState
`func (o *EntityResponse) UnsetState()`

UnsetState ensures that no value is present for State, not even an explicit nil
### GetZoneDoorId

`func (o *EntityResponse) GetZoneDoorId() string`

GetZoneDoorId returns the ZoneDoorId field if non-nil, zero value otherwise.

### GetZoneDoorIdOk

`func (o *EntityResponse) GetZoneDoorIdOk() (*string, bool)`

GetZoneDoorIdOk returns a tuple with the ZoneDoorId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneDoorId

`func (o *EntityResponse) SetZoneDoorId(v string)`

SetZoneDoorId sets ZoneDoorId field to given value.

### HasZoneDoorId

`func (o *EntityResponse) HasZoneDoorId() bool`

HasZoneDoorId returns a boolean if a field has been set.

### SetZoneDoorIdNil

`func (o *EntityResponse) SetZoneDoorIdNil(b bool)`

 SetZoneDoorIdNil sets the value for ZoneDoorId to be an explicit nil

### UnsetZoneDoorId
`func (o *EntityResponse) UnsetZoneDoorId()`

UnsetZoneDoorId ensures that no value is present for ZoneDoorId, not even an explicit nil
### GetZoneSource

`func (o *EntityResponse) GetZoneSource() ZoneSource`

GetZoneSource returns the ZoneSource field if non-nil, zero value otherwise.

### GetZoneSourceOk

`func (o *EntityResponse) GetZoneSourceOk() (*ZoneSource, bool)`

GetZoneSourceOk returns a tuple with the ZoneSource field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneSource

`func (o *EntityResponse) SetZoneSource(v ZoneSource)`

SetZoneSource sets ZoneSource field to given value.

### HasZoneSource

`func (o *EntityResponse) HasZoneSource() bool`

HasZoneSource returns a boolean if a field has been set.

### SetZoneSourceNil

`func (o *EntityResponse) SetZoneSourceNil(b bool)`

 SetZoneSourceNil sets the value for ZoneSource to be an explicit nil

### UnsetZoneSource
`func (o *EntityResponse) UnsetZoneSource()`

UnsetZoneSource ensures that no value is present for ZoneSource, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
