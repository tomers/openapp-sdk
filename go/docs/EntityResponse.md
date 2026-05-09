# EntityResponse

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

### GetZoneId

`func (o *EntityResponse) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *EntityResponse) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *EntityResponse) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.

### HasZoneId

`func (o *EntityResponse) HasZoneId() bool`

HasZoneId returns a boolean if a field has been set.

### GetCacheHit

`func (o *EntityResponse) GetCacheHit() bool`

GetCacheHit returns the CacheHit field if non-nil, zero value otherwise.

### GetCacheHitOk

`func (o *EntityResponse) GetCacheHitOk() (*bool, bool)`

GetCacheHitOk returns a tuple with the CacheHit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCacheHit

`func (o *EntityResponse) SetCacheHit(v bool)`

SetCacheHit sets CacheHit field to given value.

### HasCacheHit

`func (o *EntityResponse) HasCacheHit() bool`

HasCacheHit returns a boolean if a field has been set.

### SetCacheHitNil

`func (o *EntityResponse) SetCacheHitNil(b bool)`

 SetCacheHitNil sets the value for CacheHit to be an explicit nil

### UnsetCacheHit
`func (o *EntityResponse) UnsetCacheHit()`

UnsetCacheHit ensures that no value is present for CacheHit, not even an explicit nil
### GetCacheTtl

`func (o *EntityResponse) GetCacheTtl() int64`

GetCacheTtl returns the CacheTtl field if non-nil, zero value otherwise.

### GetCacheTtlOk

`func (o *EntityResponse) GetCacheTtlOk() (*int64, bool)`

GetCacheTtlOk returns a tuple with the CacheTtl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCacheTtl

`func (o *EntityResponse) SetCacheTtl(v int64)`

SetCacheTtl sets CacheTtl field to given value.

### HasCacheTtl

`func (o *EntityResponse) HasCacheTtl() bool`

HasCacheTtl returns a boolean if a field has been set.

### SetCacheTtlNil

`func (o *EntityResponse) SetCacheTtlNil(b bool)`

 SetCacheTtlNil sets the value for CacheTtl to be an explicit nil

### UnsetCacheTtl
`func (o *EntityResponse) UnsetCacheTtl()`

UnsetCacheTtl ensures that no value is present for CacheTtl, not even an explicit nil
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

### SetCreatedAtNil

`func (o *EntityResponse) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *EntityResponse) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
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

### SetDeletedAtNil

`func (o *EntityResponse) SetDeletedAtNil(b bool)`

 SetDeletedAtNil sets the value for DeletedAt to be an explicit nil

### UnsetDeletedAt
`func (o *EntityResponse) UnsetDeletedAt()`

UnsetDeletedAt ensures that no value is present for DeletedAt, not even an explicit nil
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

### SetHardDeleteAtNil

`func (o *EntityResponse) SetHardDeleteAtNil(b bool)`

 SetHardDeleteAtNil sets the value for HardDeleteAt to be an explicit nil

### UnsetHardDeleteAt
`func (o *EntityResponse) UnsetHardDeleteAt()`

UnsetHardDeleteAt ensures that no value is present for HardDeleteAt, not even an explicit nil
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

### SetPurgeAtNil

`func (o *EntityResponse) SetPurgeAtNil(b bool)`

 SetPurgeAtNil sets the value for PurgeAt to be an explicit nil

### UnsetPurgeAt
`func (o *EntityResponse) UnsetPurgeAt()`

UnsetPurgeAt ensures that no value is present for PurgeAt, not even an explicit nil
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

### SetUpdatedAtNil

`func (o *EntityResponse) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *EntityResponse) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil
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

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
