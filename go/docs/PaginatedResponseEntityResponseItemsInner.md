# PaginatedResponseEntityResponseItemsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DeviceId** | **string** | Parent device ID. |
**EntityType** | [**EntityType**](EntityType.md) | Entity type (switch, light, sensor). |
**ExternalId** | Pointer to **string** | External ID from the integration. | [optional]
**Id** | **string** | Unique identifier (ULID). |
**Metadata** | Pointer to **map[string]string** |  | [optional]
**Name** | Pointer to **string** | Optional friendly name for the entity. | [optional]
**ZoneId** | Pointer to **string** | Optional zone this entity belongs to. | [optional]
**CacheHit** | Pointer to **NullableBool** |  | [optional]
**CacheTtl** | Pointer to **NullableInt64** |  | [optional]
**CreatedAt** | Pointer to **NullableTime** |  | [optional]
**DeletedAt** | Pointer to **NullableTime** |  | [optional]
**HardDeleteAt** | Pointer to **NullableTime** |  | [optional]
**PurgeAt** | Pointer to **NullableTime** |  | [optional]
**UpdatedAt** | Pointer to **NullableTime** |  | [optional]
**EntityMetadata** | Pointer to **map[string]string** |  | [optional]
**State** | Pointer to **interface{}** |  | [optional]

## Methods

### NewPaginatedResponseEntityResponseItemsInner

`func NewPaginatedResponseEntityResponseItemsInner(deviceId string, entityType EntityType, id string, ) *PaginatedResponseEntityResponseItemsInner`

NewPaginatedResponseEntityResponseItemsInner instantiates a new PaginatedResponseEntityResponseItemsInner object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseEntityResponseItemsInnerWithDefaults

`func NewPaginatedResponseEntityResponseItemsInnerWithDefaults() *PaginatedResponseEntityResponseItemsInner`

NewPaginatedResponseEntityResponseItemsInnerWithDefaults instantiates a new PaginatedResponseEntityResponseItemsInner object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeviceId

`func (o *PaginatedResponseEntityResponseItemsInner) GetDeviceId() string`

GetDeviceId returns the DeviceId field if non-nil, zero value otherwise.

### GetDeviceIdOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetDeviceIdOk() (*string, bool)`

GetDeviceIdOk returns a tuple with the DeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceId

`func (o *PaginatedResponseEntityResponseItemsInner) SetDeviceId(v string)`

SetDeviceId sets DeviceId field to given value.


### GetEntityType

`func (o *PaginatedResponseEntityResponseItemsInner) GetEntityType() EntityType`

GetEntityType returns the EntityType field if non-nil, zero value otherwise.

### GetEntityTypeOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetEntityTypeOk() (*EntityType, bool)`

GetEntityTypeOk returns a tuple with the EntityType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityType

`func (o *PaginatedResponseEntityResponseItemsInner) SetEntityType(v EntityType)`

SetEntityType sets EntityType field to given value.


### GetExternalId

`func (o *PaginatedResponseEntityResponseItemsInner) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *PaginatedResponseEntityResponseItemsInner) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.

### HasExternalId

`func (o *PaginatedResponseEntityResponseItemsInner) HasExternalId() bool`

HasExternalId returns a boolean if a field has been set.

### GetId

`func (o *PaginatedResponseEntityResponseItemsInner) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PaginatedResponseEntityResponseItemsInner) SetId(v string)`

SetId sets Id field to given value.


### GetMetadata

`func (o *PaginatedResponseEntityResponseItemsInner) GetMetadata() map[string]string`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetMetadataOk() (*map[string]string, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *PaginatedResponseEntityResponseItemsInner) SetMetadata(v map[string]string)`

SetMetadata sets Metadata field to given value.

### HasMetadata

`func (o *PaginatedResponseEntityResponseItemsInner) HasMetadata() bool`

HasMetadata returns a boolean if a field has been set.

### GetName

`func (o *PaginatedResponseEntityResponseItemsInner) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PaginatedResponseEntityResponseItemsInner) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *PaginatedResponseEntityResponseItemsInner) HasName() bool`

HasName returns a boolean if a field has been set.

### GetZoneId

`func (o *PaginatedResponseEntityResponseItemsInner) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *PaginatedResponseEntityResponseItemsInner) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.

### HasZoneId

`func (o *PaginatedResponseEntityResponseItemsInner) HasZoneId() bool`

HasZoneId returns a boolean if a field has been set.

### GetCacheHit

`func (o *PaginatedResponseEntityResponseItemsInner) GetCacheHit() bool`

GetCacheHit returns the CacheHit field if non-nil, zero value otherwise.

### GetCacheHitOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetCacheHitOk() (*bool, bool)`

GetCacheHitOk returns a tuple with the CacheHit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCacheHit

`func (o *PaginatedResponseEntityResponseItemsInner) SetCacheHit(v bool)`

SetCacheHit sets CacheHit field to given value.

### HasCacheHit

`func (o *PaginatedResponseEntityResponseItemsInner) HasCacheHit() bool`

HasCacheHit returns a boolean if a field has been set.

### SetCacheHitNil

`func (o *PaginatedResponseEntityResponseItemsInner) SetCacheHitNil(b bool)`

 SetCacheHitNil sets the value for CacheHit to be an explicit nil

### UnsetCacheHit
`func (o *PaginatedResponseEntityResponseItemsInner) UnsetCacheHit()`

UnsetCacheHit ensures that no value is present for CacheHit, not even an explicit nil
### GetCacheTtl

`func (o *PaginatedResponseEntityResponseItemsInner) GetCacheTtl() int64`

GetCacheTtl returns the CacheTtl field if non-nil, zero value otherwise.

### GetCacheTtlOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetCacheTtlOk() (*int64, bool)`

GetCacheTtlOk returns a tuple with the CacheTtl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCacheTtl

`func (o *PaginatedResponseEntityResponseItemsInner) SetCacheTtl(v int64)`

SetCacheTtl sets CacheTtl field to given value.

### HasCacheTtl

`func (o *PaginatedResponseEntityResponseItemsInner) HasCacheTtl() bool`

HasCacheTtl returns a boolean if a field has been set.

### SetCacheTtlNil

`func (o *PaginatedResponseEntityResponseItemsInner) SetCacheTtlNil(b bool)`

 SetCacheTtlNil sets the value for CacheTtl to be an explicit nil

### UnsetCacheTtl
`func (o *PaginatedResponseEntityResponseItemsInner) UnsetCacheTtl()`

UnsetCacheTtl ensures that no value is present for CacheTtl, not even an explicit nil
### GetCreatedAt

`func (o *PaginatedResponseEntityResponseItemsInner) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *PaginatedResponseEntityResponseItemsInner) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *PaginatedResponseEntityResponseItemsInner) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *PaginatedResponseEntityResponseItemsInner) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *PaginatedResponseEntityResponseItemsInner) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetDeletedAt

`func (o *PaginatedResponseEntityResponseItemsInner) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *PaginatedResponseEntityResponseItemsInner) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *PaginatedResponseEntityResponseItemsInner) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### SetDeletedAtNil

`func (o *PaginatedResponseEntityResponseItemsInner) SetDeletedAtNil(b bool)`

 SetDeletedAtNil sets the value for DeletedAt to be an explicit nil

### UnsetDeletedAt
`func (o *PaginatedResponseEntityResponseItemsInner) UnsetDeletedAt()`

UnsetDeletedAt ensures that no value is present for DeletedAt, not even an explicit nil
### GetHardDeleteAt

`func (o *PaginatedResponseEntityResponseItemsInner) GetHardDeleteAt() time.Time`

GetHardDeleteAt returns the HardDeleteAt field if non-nil, zero value otherwise.

### GetHardDeleteAtOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetHardDeleteAtOk() (*time.Time, bool)`

GetHardDeleteAtOk returns a tuple with the HardDeleteAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHardDeleteAt

`func (o *PaginatedResponseEntityResponseItemsInner) SetHardDeleteAt(v time.Time)`

SetHardDeleteAt sets HardDeleteAt field to given value.

### HasHardDeleteAt

`func (o *PaginatedResponseEntityResponseItemsInner) HasHardDeleteAt() bool`

HasHardDeleteAt returns a boolean if a field has been set.

### SetHardDeleteAtNil

`func (o *PaginatedResponseEntityResponseItemsInner) SetHardDeleteAtNil(b bool)`

 SetHardDeleteAtNil sets the value for HardDeleteAt to be an explicit nil

### UnsetHardDeleteAt
`func (o *PaginatedResponseEntityResponseItemsInner) UnsetHardDeleteAt()`

UnsetHardDeleteAt ensures that no value is present for HardDeleteAt, not even an explicit nil
### GetPurgeAt

`func (o *PaginatedResponseEntityResponseItemsInner) GetPurgeAt() time.Time`

GetPurgeAt returns the PurgeAt field if non-nil, zero value otherwise.

### GetPurgeAtOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetPurgeAtOk() (*time.Time, bool)`

GetPurgeAtOk returns a tuple with the PurgeAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurgeAt

`func (o *PaginatedResponseEntityResponseItemsInner) SetPurgeAt(v time.Time)`

SetPurgeAt sets PurgeAt field to given value.

### HasPurgeAt

`func (o *PaginatedResponseEntityResponseItemsInner) HasPurgeAt() bool`

HasPurgeAt returns a boolean if a field has been set.

### SetPurgeAtNil

`func (o *PaginatedResponseEntityResponseItemsInner) SetPurgeAtNil(b bool)`

 SetPurgeAtNil sets the value for PurgeAt to be an explicit nil

### UnsetPurgeAt
`func (o *PaginatedResponseEntityResponseItemsInner) UnsetPurgeAt()`

UnsetPurgeAt ensures that no value is present for PurgeAt, not even an explicit nil
### GetUpdatedAt

`func (o *PaginatedResponseEntityResponseItemsInner) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *PaginatedResponseEntityResponseItemsInner) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *PaginatedResponseEntityResponseItemsInner) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *PaginatedResponseEntityResponseItemsInner) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *PaginatedResponseEntityResponseItemsInner) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil
### GetEntityMetadata

`func (o *PaginatedResponseEntityResponseItemsInner) GetEntityMetadata() map[string]string`

GetEntityMetadata returns the EntityMetadata field if non-nil, zero value otherwise.

### GetEntityMetadataOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetEntityMetadataOk() (*map[string]string, bool)`

GetEntityMetadataOk returns a tuple with the EntityMetadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityMetadata

`func (o *PaginatedResponseEntityResponseItemsInner) SetEntityMetadata(v map[string]string)`

SetEntityMetadata sets EntityMetadata field to given value.

### HasEntityMetadata

`func (o *PaginatedResponseEntityResponseItemsInner) HasEntityMetadata() bool`

HasEntityMetadata returns a boolean if a field has been set.

### GetState

`func (o *PaginatedResponseEntityResponseItemsInner) GetState() interface{}`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *PaginatedResponseEntityResponseItemsInner) GetStateOk() (*interface{}, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *PaginatedResponseEntityResponseItemsInner) SetState(v interface{})`

SetState sets State field to given value.

### HasState

`func (o *PaginatedResponseEntityResponseItemsInner) HasState() bool`

HasState returns a boolean if a field has been set.

### SetStateNil

`func (o *PaginatedResponseEntityResponseItemsInner) SetStateNil(b bool)`

 SetStateNil sets the value for State to be an explicit nil

### UnsetState
`func (o *PaginatedResponseEntityResponseItemsInner) UnsetState()`

UnsetState ensures that no value is present for State, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
