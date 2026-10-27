# DeviceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **time.Time** |  | [optional]
**DeletedAt** | Pointer to **time.Time** |  | [optional]
**HardDeleteAt** | Pointer to **time.Time** |  | [optional]
**PurgeAt** | Pointer to **time.Time** |  | [optional]
**UpdatedAt** | Pointer to **time.Time** |  | [optional]
**DeviceMetadata** | Pointer to **interface{}** |  | [optional]
**ExternalId** | Pointer to **string** | External ID from the integration provider. | [optional]
**Id** | **string** | Unique identifier (ULID). |
**IntegrationId** | **string** | Owning integration (e.g. Shelly Cloud account/connection). |
**Name** | [**LocalizedString**](LocalizedString.md) | Human-readable name. |
**OrgId** | **string** | Organization that owns this device. |
**ZoneId** | Pointer to **string** | Zone the device is placed in (same org). Entities with &#x60;zone.mode &#x3D; inherit&#x60; take it. | [optional]
**CredentialsSet** | **[]string** | JSON Pointers into &#x60;device_metadata&#x60; of write-only credential fields (schema &#x60;format: password&#x60;) that currently hold a value. Credential values are never returned here; an update that omits them keeps the stored value and &#x60;null&#x60; clears it. |
**IsPrimaryInstance** | Pointer to **NullableBool** | Read-only, computed by the server: true when this device record is the primary instance for its physical hardware, when the same hardware has more than one device record. Never accepted in a request body. | [optional]
**Stale** | Pointer to **NullableBool** | Present when &#x60;include_stale&#x3D;true&#x60; was requested and the provider supports stale detection. | [optional]

## Methods

### NewDeviceResponse

`func NewDeviceResponse(id string, integrationId string, name LocalizedString, orgId string, credentialsSet []string, ) *DeviceResponse`

NewDeviceResponse instantiates a new DeviceResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDeviceResponseWithDefaults

`func NewDeviceResponseWithDefaults() *DeviceResponse`

NewDeviceResponseWithDefaults instantiates a new DeviceResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

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


### GetZoneId

`func (o *DeviceResponse) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *DeviceResponse) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *DeviceResponse) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.

### HasZoneId

`func (o *DeviceResponse) HasZoneId() bool`

HasZoneId returns a boolean if a field has been set.

### GetCredentialsSet

`func (o *DeviceResponse) GetCredentialsSet() []string`

GetCredentialsSet returns the CredentialsSet field if non-nil, zero value otherwise.

### GetCredentialsSetOk

`func (o *DeviceResponse) GetCredentialsSetOk() (*[]string, bool)`

GetCredentialsSetOk returns a tuple with the CredentialsSet field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCredentialsSet

`func (o *DeviceResponse) SetCredentialsSet(v []string)`

SetCredentialsSet sets CredentialsSet field to given value.


### GetIsPrimaryInstance

`func (o *DeviceResponse) GetIsPrimaryInstance() bool`

GetIsPrimaryInstance returns the IsPrimaryInstance field if non-nil, zero value otherwise.

### GetIsPrimaryInstanceOk

`func (o *DeviceResponse) GetIsPrimaryInstanceOk() (*bool, bool)`

GetIsPrimaryInstanceOk returns a tuple with the IsPrimaryInstance field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsPrimaryInstance

`func (o *DeviceResponse) SetIsPrimaryInstance(v bool)`

SetIsPrimaryInstance sets IsPrimaryInstance field to given value.

### HasIsPrimaryInstance

`func (o *DeviceResponse) HasIsPrimaryInstance() bool`

HasIsPrimaryInstance returns a boolean if a field has been set.

### SetIsPrimaryInstanceNil

`func (o *DeviceResponse) SetIsPrimaryInstanceNil(b bool)`

 SetIsPrimaryInstanceNil sets the value for IsPrimaryInstance to be an explicit nil

### UnsetIsPrimaryInstance
`func (o *DeviceResponse) UnsetIsPrimaryInstance()`

UnsetIsPrimaryInstance ensures that no value is present for IsPrimaryInstance, not even an explicit nil
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
