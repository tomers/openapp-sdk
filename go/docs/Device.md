# Device

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DeviceMetadata** | Pointer to **interface{}** |  | [optional]
**ExternalId** | Pointer to **NullableString** | External ID from the integration provider. | [optional]
**Id** | **string** | Unique identifier (ULID). |
**IntegrationId** | **string** | Owning integration (e.g. Shelly Cloud account/connection). |
**Metadata** | Pointer to **map[string]string** |  | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) | Human-readable name. |
**OrgId** | **string** | Organization that owns this device. |

## Methods

### NewDevice

`func NewDevice(id string, integrationId string, name LocalizedString, orgId string, ) *Device`

NewDevice instantiates a new Device object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDeviceWithDefaults

`func NewDeviceWithDefaults() *Device`

NewDeviceWithDefaults instantiates a new Device object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeviceMetadata

`func (o *Device) GetDeviceMetadata() interface{}`

GetDeviceMetadata returns the DeviceMetadata field if non-nil, zero value otherwise.

### GetDeviceMetadataOk

`func (o *Device) GetDeviceMetadataOk() (*interface{}, bool)`

GetDeviceMetadataOk returns a tuple with the DeviceMetadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceMetadata

`func (o *Device) SetDeviceMetadata(v interface{})`

SetDeviceMetadata sets DeviceMetadata field to given value.

### HasDeviceMetadata

`func (o *Device) HasDeviceMetadata() bool`

HasDeviceMetadata returns a boolean if a field has been set.

### SetDeviceMetadataNil

`func (o *Device) SetDeviceMetadataNil(b bool)`

 SetDeviceMetadataNil sets the value for DeviceMetadata to be an explicit nil

### UnsetDeviceMetadata
`func (o *Device) UnsetDeviceMetadata()`

UnsetDeviceMetadata ensures that no value is present for DeviceMetadata, not even an explicit nil
### GetExternalId

`func (o *Device) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *Device) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *Device) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.

### HasExternalId

`func (o *Device) HasExternalId() bool`

HasExternalId returns a boolean if a field has been set.

### SetExternalIdNil

`func (o *Device) SetExternalIdNil(b bool)`

 SetExternalIdNil sets the value for ExternalId to be an explicit nil

### UnsetExternalId
`func (o *Device) UnsetExternalId()`

UnsetExternalId ensures that no value is present for ExternalId, not even an explicit nil
### GetId

`func (o *Device) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *Device) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *Device) SetId(v string)`

SetId sets Id field to given value.


### GetIntegrationId

`func (o *Device) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *Device) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *Device) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### GetMetadata

`func (o *Device) GetMetadata() map[string]string`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *Device) GetMetadataOk() (*map[string]string, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *Device) SetMetadata(v map[string]string)`

SetMetadata sets Metadata field to given value.

### HasMetadata

`func (o *Device) HasMetadata() bool`

HasMetadata returns a boolean if a field has been set.

### GetName

`func (o *Device) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *Device) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *Device) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetOrgId

`func (o *Device) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *Device) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *Device) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
