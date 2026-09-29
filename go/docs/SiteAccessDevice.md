# SiteAccessDevice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**Deleted** | **bool** | True when the device is soft-deleted but still referenced by a door. |
**DeviceId** | **string** |  |
**Doors** | [**[]SiteAccessDeviceDoor**](SiteAccessDeviceDoor.md) | Doors this device serves in this role (union of &#x60;entities[].doors&#x60;). |
**Entities** | [**[]SiteAccessDeviceEntity**](SiteAccessDeviceEntity.md) | Entities on this device playing this section&#39;s role. Empty for cameras. |
**ExternalId** | Pointer to **NullableString** |  | [optional]
**IntegrationId** | **string** | Owning integration — usually not the site&#39;s own integration. |
**Name** | [**LocalizedString**](LocalizedString.md) |  |

## Methods

### NewSiteAccessDevice

`func NewSiteAccessDevice(deleted bool, deviceId string, doors []SiteAccessDeviceDoor, entities []SiteAccessDeviceEntity, integrationId string, name LocalizedString, ) *SiteAccessDevice`

NewSiteAccessDevice instantiates a new SiteAccessDevice object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSiteAccessDeviceWithDefaults

`func NewSiteAccessDeviceWithDefaults() *SiteAccessDevice`

NewSiteAccessDeviceWithDefaults instantiates a new SiteAccessDevice object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *SiteAccessDevice) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *SiteAccessDevice) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *SiteAccessDevice) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *SiteAccessDevice) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *SiteAccessDevice) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *SiteAccessDevice) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetDeleted

`func (o *SiteAccessDevice) GetDeleted() bool`

GetDeleted returns the Deleted field if non-nil, zero value otherwise.

### GetDeletedOk

`func (o *SiteAccessDevice) GetDeletedOk() (*bool, bool)`

GetDeletedOk returns a tuple with the Deleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeleted

`func (o *SiteAccessDevice) SetDeleted(v bool)`

SetDeleted sets Deleted field to given value.


### GetDeviceId

`func (o *SiteAccessDevice) GetDeviceId() string`

GetDeviceId returns the DeviceId field if non-nil, zero value otherwise.

### GetDeviceIdOk

`func (o *SiteAccessDevice) GetDeviceIdOk() (*string, bool)`

GetDeviceIdOk returns a tuple with the DeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceId

`func (o *SiteAccessDevice) SetDeviceId(v string)`

SetDeviceId sets DeviceId field to given value.


### GetDoors

`func (o *SiteAccessDevice) GetDoors() []SiteAccessDeviceDoor`

GetDoors returns the Doors field if non-nil, zero value otherwise.

### GetDoorsOk

`func (o *SiteAccessDevice) GetDoorsOk() (*[]SiteAccessDeviceDoor, bool)`

GetDoorsOk returns a tuple with the Doors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoors

`func (o *SiteAccessDevice) SetDoors(v []SiteAccessDeviceDoor)`

SetDoors sets Doors field to given value.


### GetEntities

`func (o *SiteAccessDevice) GetEntities() []SiteAccessDeviceEntity`

GetEntities returns the Entities field if non-nil, zero value otherwise.

### GetEntitiesOk

`func (o *SiteAccessDevice) GetEntitiesOk() (*[]SiteAccessDeviceEntity, bool)`

GetEntitiesOk returns a tuple with the Entities field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntities

`func (o *SiteAccessDevice) SetEntities(v []SiteAccessDeviceEntity)`

SetEntities sets Entities field to given value.


### GetExternalId

`func (o *SiteAccessDevice) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *SiteAccessDevice) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *SiteAccessDevice) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.

### HasExternalId

`func (o *SiteAccessDevice) HasExternalId() bool`

HasExternalId returns a boolean if a field has been set.

### SetExternalIdNil

`func (o *SiteAccessDevice) SetExternalIdNil(b bool)`

 SetExternalIdNil sets the value for ExternalId to be an explicit nil

### UnsetExternalId
`func (o *SiteAccessDevice) UnsetExternalId()`

UnsetExternalId ensures that no value is present for ExternalId, not even an explicit nil
### GetIntegrationId

`func (o *SiteAccessDevice) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *SiteAccessDevice) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *SiteAccessDevice) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### GetName

`func (o *SiteAccessDevice) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *SiteAccessDevice) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *SiteAccessDevice) SetName(v LocalizedString)`

SetName sets Name field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
