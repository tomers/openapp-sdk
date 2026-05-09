# DeviceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DeviceMetadata** | Pointer to **interface{}** |  | [optional]
**ExternalId** | Pointer to **string** | External ID from the integration provider. | [optional]
**Id** | **string** | Unique identifier (ULID). |
**IntegrationId** | **string** | Owning integration (e.g. Shelly Cloud account/connection). |
**Metadata** | Pointer to **map[string]string** |  | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) | Human-readable name. |
**OrgId** | **string** | Organization that owns this device. |
**CacheHit** | Pointer to **NullableBool** |  | [optional]
**CacheTtl** | Pointer to **NullableInt64** |  | [optional]
**CreatedAt** | Pointer to **NullableTime** |  | [optional]
**DeletedAt** | Pointer to **NullableTime** |  | [optional]
**HardDeleteAt** | Pointer to **NullableTime** |  | [optional]
**PurgeAt** | Pointer to **NullableTime** |  | [optional]
**UpdatedAt** | Pointer to **NullableTime** |  | [optional]
**Stale** | Pointer to **NullableBool** | Present when &#x60;include_stale&#x3D;true&#x60; was requested and the provider supports stale detection. | [optional]

## Methods

### NewDeviceResponse

`func NewDeviceResponse(id string, integrationId string, name LocalizedString, orgId string, ) *DeviceResponse`

NewDeviceResponse instantiates a new DeviceResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDeviceResponseWithDefaults

`func NewDeviceResponseWithDefaults() *DeviceResponse`

NewDeviceResponseWithDefaults instantiates a new DeviceResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeviceMetadata

`func (o *DeviceResponse) GetDeviceMetadata() interface{}`

GetDeviceMetadata returns the DeviceMetadata field if non-nil, zero value otherwise.

### GetDeviceMetadataOk

`func (o *DeviceResponse) GetDeviceMetadataOk() (*interface{}, bool)`

GetDeviceMetadataOk returns a tuple with the DeviceMetadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceMetadata

`func (o *DeviceResponse) SetDeviceMetadata(v interface{})`

SetDeviceMetadata sets DeviceMetadata field to given value.

### HasDeviceMetadata

`func (o *DeviceResponse) HasDeviceMetadata() bool`

HasDeviceMetadata returns a boolean if a field has been set.

### SetDeviceMetadataNil

`func (o *DeviceResponse) SetDeviceMetadataNil(b bool)`

 SetDeviceMetadataNil sets the value for DeviceMetadata to be an explicit nil

### UnsetDeviceMetadata
`func (o *DeviceResponse) UnsetDeviceMetadata()`

UnsetDeviceMetadata ensures that no value is present for DeviceMetadata, not even an explicit nil
### GetExternalId

`func (o *DeviceResponse) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *DeviceResponse) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *DeviceResponse) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.

### HasExternalId

`func (o *DeviceResponse) HasExternalId() bool`

HasExternalId returns a boolean if a field has been set.

### GetId

`func (o *DeviceResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *DeviceResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *DeviceResponse) SetId(v string)`

SetId sets Id field to given value.


### GetIntegrationId

`func (o *DeviceResponse) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *DeviceResponse) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *DeviceResponse) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### GetMetadata

`func (o *DeviceResponse) GetMetadata() map[string]string`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *DeviceResponse) GetMetadataOk() (*map[string]string, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *DeviceResponse) SetMetadata(v map[string]string)`

SetMetadata sets Metadata field to given value.

### HasMetadata

`func (o *DeviceResponse) HasMetadata() bool`

HasMetadata returns a boolean if a field has been set.

### GetName

`func (o *DeviceResponse) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *DeviceResponse) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *DeviceResponse) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetOrgId

`func (o *DeviceResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *DeviceResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *DeviceResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetCacheHit

`func (o *DeviceResponse) GetCacheHit() bool`

GetCacheHit returns the CacheHit field if non-nil, zero value otherwise.

### GetCacheHitOk

`func (o *DeviceResponse) GetCacheHitOk() (*bool, bool)`

GetCacheHitOk returns a tuple with the CacheHit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCacheHit

`func (o *DeviceResponse) SetCacheHit(v bool)`

SetCacheHit sets CacheHit field to given value.

### HasCacheHit

`func (o *DeviceResponse) HasCacheHit() bool`

HasCacheHit returns a boolean if a field has been set.

### SetCacheHitNil

`func (o *DeviceResponse) SetCacheHitNil(b bool)`

 SetCacheHitNil sets the value for CacheHit to be an explicit nil

### UnsetCacheHit
`func (o *DeviceResponse) UnsetCacheHit()`

UnsetCacheHit ensures that no value is present for CacheHit, not even an explicit nil
### GetCacheTtl

`func (o *DeviceResponse) GetCacheTtl() int64`

GetCacheTtl returns the CacheTtl field if non-nil, zero value otherwise.

### GetCacheTtlOk

`func (o *DeviceResponse) GetCacheTtlOk() (*int64, bool)`

GetCacheTtlOk returns a tuple with the CacheTtl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCacheTtl

`func (o *DeviceResponse) SetCacheTtl(v int64)`

SetCacheTtl sets CacheTtl field to given value.

### HasCacheTtl

`func (o *DeviceResponse) HasCacheTtl() bool`

HasCacheTtl returns a boolean if a field has been set.

### SetCacheTtlNil

`func (o *DeviceResponse) SetCacheTtlNil(b bool)`

 SetCacheTtlNil sets the value for CacheTtl to be an explicit nil

### UnsetCacheTtl
`func (o *DeviceResponse) UnsetCacheTtl()`

UnsetCacheTtl ensures that no value is present for CacheTtl, not even an explicit nil
### GetCreatedAt

`func (o *DeviceResponse) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *DeviceResponse) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *DeviceResponse) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *DeviceResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *DeviceResponse) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *DeviceResponse) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetDeletedAt

`func (o *DeviceResponse) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *DeviceResponse) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *DeviceResponse) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *DeviceResponse) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### SetDeletedAtNil

`func (o *DeviceResponse) SetDeletedAtNil(b bool)`

 SetDeletedAtNil sets the value for DeletedAt to be an explicit nil

### UnsetDeletedAt
`func (o *DeviceResponse) UnsetDeletedAt()`

UnsetDeletedAt ensures that no value is present for DeletedAt, not even an explicit nil
### GetHardDeleteAt

`func (o *DeviceResponse) GetHardDeleteAt() time.Time`

GetHardDeleteAt returns the HardDeleteAt field if non-nil, zero value otherwise.

### GetHardDeleteAtOk

`func (o *DeviceResponse) GetHardDeleteAtOk() (*time.Time, bool)`

GetHardDeleteAtOk returns a tuple with the HardDeleteAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHardDeleteAt

`func (o *DeviceResponse) SetHardDeleteAt(v time.Time)`

SetHardDeleteAt sets HardDeleteAt field to given value.

### HasHardDeleteAt

`func (o *DeviceResponse) HasHardDeleteAt() bool`

HasHardDeleteAt returns a boolean if a field has been set.

### SetHardDeleteAtNil

`func (o *DeviceResponse) SetHardDeleteAtNil(b bool)`

 SetHardDeleteAtNil sets the value for HardDeleteAt to be an explicit nil

### UnsetHardDeleteAt
`func (o *DeviceResponse) UnsetHardDeleteAt()`

UnsetHardDeleteAt ensures that no value is present for HardDeleteAt, not even an explicit nil
### GetPurgeAt

`func (o *DeviceResponse) GetPurgeAt() time.Time`

GetPurgeAt returns the PurgeAt field if non-nil, zero value otherwise.

### GetPurgeAtOk

`func (o *DeviceResponse) GetPurgeAtOk() (*time.Time, bool)`

GetPurgeAtOk returns a tuple with the PurgeAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurgeAt

`func (o *DeviceResponse) SetPurgeAt(v time.Time)`

SetPurgeAt sets PurgeAt field to given value.

### HasPurgeAt

`func (o *DeviceResponse) HasPurgeAt() bool`

HasPurgeAt returns a boolean if a field has been set.

### SetPurgeAtNil

`func (o *DeviceResponse) SetPurgeAtNil(b bool)`

 SetPurgeAtNil sets the value for PurgeAt to be an explicit nil

### UnsetPurgeAt
`func (o *DeviceResponse) UnsetPurgeAt()`

UnsetPurgeAt ensures that no value is present for PurgeAt, not even an explicit nil
### GetUpdatedAt

`func (o *DeviceResponse) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *DeviceResponse) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *DeviceResponse) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *DeviceResponse) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *DeviceResponse) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *DeviceResponse) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil
### GetStale

`func (o *DeviceResponse) GetStale() bool`

GetStale returns the Stale field if non-nil, zero value otherwise.

### GetStaleOk

`func (o *DeviceResponse) GetStaleOk() (*bool, bool)`

GetStaleOk returns a tuple with the Stale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStale

`func (o *DeviceResponse) SetStale(v bool)`

SetStale sets Stale field to given value.

### HasStale

`func (o *DeviceResponse) HasStale() bool`

HasStale returns a boolean if a field has been set.

### SetStaleNil

`func (o *DeviceResponse) SetStaleNil(b bool)`

 SetStaleNil sets the value for Stale to be an explicit nil

### UnsetStale
`func (o *DeviceResponse) UnsetStale()`

UnsetStale ensures that no value is present for Stale, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
