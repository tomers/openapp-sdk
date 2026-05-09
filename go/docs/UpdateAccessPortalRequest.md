# UpdateAccessPortalRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DeviceExternalId** | Pointer to **NullableString** | Device external_id. Must be a virtual_access_portal device in this integration. Use null to unlink. | [optional]
**Name** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]

## Methods

### NewUpdateAccessPortalRequest

`func NewUpdateAccessPortalRequest() *UpdateAccessPortalRequest`

NewUpdateAccessPortalRequest instantiates a new UpdateAccessPortalRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateAccessPortalRequestWithDefaults

`func NewUpdateAccessPortalRequestWithDefaults() *UpdateAccessPortalRequest`

NewUpdateAccessPortalRequestWithDefaults instantiates a new UpdateAccessPortalRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeviceExternalId

`func (o *UpdateAccessPortalRequest) GetDeviceExternalId() string`

GetDeviceExternalId returns the DeviceExternalId field if non-nil, zero value otherwise.

### GetDeviceExternalIdOk

`func (o *UpdateAccessPortalRequest) GetDeviceExternalIdOk() (*string, bool)`

GetDeviceExternalIdOk returns a tuple with the DeviceExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceExternalId

`func (o *UpdateAccessPortalRequest) SetDeviceExternalId(v string)`

SetDeviceExternalId sets DeviceExternalId field to given value.

### HasDeviceExternalId

`func (o *UpdateAccessPortalRequest) HasDeviceExternalId() bool`

HasDeviceExternalId returns a boolean if a field has been set.

### SetDeviceExternalIdNil

`func (o *UpdateAccessPortalRequest) SetDeviceExternalIdNil(b bool)`

 SetDeviceExternalIdNil sets the value for DeviceExternalId to be an explicit nil

### UnsetDeviceExternalId
`func (o *UpdateAccessPortalRequest) UnsetDeviceExternalId()`

UnsetDeviceExternalId ensures that no value is present for DeviceExternalId, not even an explicit nil
### GetName

`func (o *UpdateAccessPortalRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UpdateAccessPortalRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UpdateAccessPortalRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.

### HasName

`func (o *UpdateAccessPortalRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *UpdateAccessPortalRequest) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *UpdateAccessPortalRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
