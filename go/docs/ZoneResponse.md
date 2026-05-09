# ZoneResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExternalId** | Pointer to **string** | External ID from the integration. | [optional]
**Id** | **string** | Unique identifier (ULID). |
**IntegrationId** | **string** | Integration this zone belongs to. |
**Metadata** | Pointer to **map[string]string** |  | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) | Zone name. |
**ParentZoneId** | Pointer to **string** | Parent zone for hierarchy. | [optional]
**CacheHit** | Pointer to **NullableBool** |  | [optional]
**CacheTtl** | Pointer to **NullableInt64** |  | [optional]
**CreatedAt** | Pointer to **NullableTime** |  | [optional]
**DeletedAt** | Pointer to **NullableTime** |  | [optional]
**HardDeleteAt** | Pointer to **NullableTime** |  | [optional]
**PurgeAt** | Pointer to **NullableTime** |  | [optional]
**UpdatedAt** | Pointer to **NullableTime** |  | [optional]

## Methods

### NewZoneResponse

`func NewZoneResponse(id string, integrationId string, name LocalizedString, ) *ZoneResponse`

NewZoneResponse instantiates a new ZoneResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewZoneResponseWithDefaults

`func NewZoneResponseWithDefaults() *ZoneResponse`

NewZoneResponseWithDefaults instantiates a new ZoneResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExternalId

`func (o *ZoneResponse) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *ZoneResponse) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *ZoneResponse) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.

### HasExternalId

`func (o *ZoneResponse) HasExternalId() bool`

HasExternalId returns a boolean if a field has been set.

### GetId

`func (o *ZoneResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ZoneResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ZoneResponse) SetId(v string)`

SetId sets Id field to given value.


### GetIntegrationId

`func (o *ZoneResponse) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *ZoneResponse) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *ZoneResponse) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### GetMetadata

`func (o *ZoneResponse) GetMetadata() map[string]string`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *ZoneResponse) GetMetadataOk() (*map[string]string, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *ZoneResponse) SetMetadata(v map[string]string)`

SetMetadata sets Metadata field to given value.

### HasMetadata

`func (o *ZoneResponse) HasMetadata() bool`

HasMetadata returns a boolean if a field has been set.

### GetName

`func (o *ZoneResponse) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ZoneResponse) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ZoneResponse) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetParentZoneId

`func (o *ZoneResponse) GetParentZoneId() string`

GetParentZoneId returns the ParentZoneId field if non-nil, zero value otherwise.

### GetParentZoneIdOk

`func (o *ZoneResponse) GetParentZoneIdOk() (*string, bool)`

GetParentZoneIdOk returns a tuple with the ParentZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetParentZoneId

`func (o *ZoneResponse) SetParentZoneId(v string)`

SetParentZoneId sets ParentZoneId field to given value.

### HasParentZoneId

`func (o *ZoneResponse) HasParentZoneId() bool`

HasParentZoneId returns a boolean if a field has been set.

### GetCacheHit

`func (o *ZoneResponse) GetCacheHit() bool`

GetCacheHit returns the CacheHit field if non-nil, zero value otherwise.

### GetCacheHitOk

`func (o *ZoneResponse) GetCacheHitOk() (*bool, bool)`

GetCacheHitOk returns a tuple with the CacheHit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCacheHit

`func (o *ZoneResponse) SetCacheHit(v bool)`

SetCacheHit sets CacheHit field to given value.

### HasCacheHit

`func (o *ZoneResponse) HasCacheHit() bool`

HasCacheHit returns a boolean if a field has been set.

### SetCacheHitNil

`func (o *ZoneResponse) SetCacheHitNil(b bool)`

 SetCacheHitNil sets the value for CacheHit to be an explicit nil

### UnsetCacheHit
`func (o *ZoneResponse) UnsetCacheHit()`

UnsetCacheHit ensures that no value is present for CacheHit, not even an explicit nil
### GetCacheTtl

`func (o *ZoneResponse) GetCacheTtl() int64`

GetCacheTtl returns the CacheTtl field if non-nil, zero value otherwise.

### GetCacheTtlOk

`func (o *ZoneResponse) GetCacheTtlOk() (*int64, bool)`

GetCacheTtlOk returns a tuple with the CacheTtl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCacheTtl

`func (o *ZoneResponse) SetCacheTtl(v int64)`

SetCacheTtl sets CacheTtl field to given value.

### HasCacheTtl

`func (o *ZoneResponse) HasCacheTtl() bool`

HasCacheTtl returns a boolean if a field has been set.

### SetCacheTtlNil

`func (o *ZoneResponse) SetCacheTtlNil(b bool)`

 SetCacheTtlNil sets the value for CacheTtl to be an explicit nil

### UnsetCacheTtl
`func (o *ZoneResponse) UnsetCacheTtl()`

UnsetCacheTtl ensures that no value is present for CacheTtl, not even an explicit nil
### GetCreatedAt

`func (o *ZoneResponse) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ZoneResponse) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ZoneResponse) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *ZoneResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *ZoneResponse) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *ZoneResponse) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetDeletedAt

`func (o *ZoneResponse) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *ZoneResponse) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *ZoneResponse) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *ZoneResponse) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### SetDeletedAtNil

`func (o *ZoneResponse) SetDeletedAtNil(b bool)`

 SetDeletedAtNil sets the value for DeletedAt to be an explicit nil

### UnsetDeletedAt
`func (o *ZoneResponse) UnsetDeletedAt()`

UnsetDeletedAt ensures that no value is present for DeletedAt, not even an explicit nil
### GetHardDeleteAt

`func (o *ZoneResponse) GetHardDeleteAt() time.Time`

GetHardDeleteAt returns the HardDeleteAt field if non-nil, zero value otherwise.

### GetHardDeleteAtOk

`func (o *ZoneResponse) GetHardDeleteAtOk() (*time.Time, bool)`

GetHardDeleteAtOk returns a tuple with the HardDeleteAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHardDeleteAt

`func (o *ZoneResponse) SetHardDeleteAt(v time.Time)`

SetHardDeleteAt sets HardDeleteAt field to given value.

### HasHardDeleteAt

`func (o *ZoneResponse) HasHardDeleteAt() bool`

HasHardDeleteAt returns a boolean if a field has been set.

### SetHardDeleteAtNil

`func (o *ZoneResponse) SetHardDeleteAtNil(b bool)`

 SetHardDeleteAtNil sets the value for HardDeleteAt to be an explicit nil

### UnsetHardDeleteAt
`func (o *ZoneResponse) UnsetHardDeleteAt()`

UnsetHardDeleteAt ensures that no value is present for HardDeleteAt, not even an explicit nil
### GetPurgeAt

`func (o *ZoneResponse) GetPurgeAt() time.Time`

GetPurgeAt returns the PurgeAt field if non-nil, zero value otherwise.

### GetPurgeAtOk

`func (o *ZoneResponse) GetPurgeAtOk() (*time.Time, bool)`

GetPurgeAtOk returns a tuple with the PurgeAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurgeAt

`func (o *ZoneResponse) SetPurgeAt(v time.Time)`

SetPurgeAt sets PurgeAt field to given value.

### HasPurgeAt

`func (o *ZoneResponse) HasPurgeAt() bool`

HasPurgeAt returns a boolean if a field has been set.

### SetPurgeAtNil

`func (o *ZoneResponse) SetPurgeAtNil(b bool)`

 SetPurgeAtNil sets the value for PurgeAt to be an explicit nil

### UnsetPurgeAt
`func (o *ZoneResponse) UnsetPurgeAt()`

UnsetPurgeAt ensures that no value is present for PurgeAt, not even an explicit nil
### GetUpdatedAt

`func (o *ZoneResponse) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *ZoneResponse) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *ZoneResponse) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *ZoneResponse) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *ZoneResponse) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *ZoneResponse) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
