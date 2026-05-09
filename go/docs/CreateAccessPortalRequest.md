# CreateAccessPortalRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DeviceExternalId** | Pointer to **NullableString** | Device external_id (virtual_access_portal device in this integration). Standard way to link portal to device. | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) | Portal display name (localized). Accepts string or object e.g. { \&quot;en\&quot;: \&quot;Lobby\&quot;, \&quot;he\&quot;: \&quot;לובי\&quot; }. |

## Methods

### NewCreateAccessPortalRequest

`func NewCreateAccessPortalRequest(name LocalizedString, ) *CreateAccessPortalRequest`

NewCreateAccessPortalRequest instantiates a new CreateAccessPortalRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateAccessPortalRequestWithDefaults

`func NewCreateAccessPortalRequestWithDefaults() *CreateAccessPortalRequest`

NewCreateAccessPortalRequestWithDefaults instantiates a new CreateAccessPortalRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeviceExternalId

`func (o *CreateAccessPortalRequest) GetDeviceExternalId() string`

GetDeviceExternalId returns the DeviceExternalId field if non-nil, zero value otherwise.

### GetDeviceExternalIdOk

`func (o *CreateAccessPortalRequest) GetDeviceExternalIdOk() (*string, bool)`

GetDeviceExternalIdOk returns a tuple with the DeviceExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceExternalId

`func (o *CreateAccessPortalRequest) SetDeviceExternalId(v string)`

SetDeviceExternalId sets DeviceExternalId field to given value.

### HasDeviceExternalId

`func (o *CreateAccessPortalRequest) HasDeviceExternalId() bool`

HasDeviceExternalId returns a boolean if a field has been set.

### SetDeviceExternalIdNil

`func (o *CreateAccessPortalRequest) SetDeviceExternalIdNil(b bool)`

 SetDeviceExternalIdNil sets the value for DeviceExternalId to be an explicit nil

### UnsetDeviceExternalId
`func (o *CreateAccessPortalRequest) UnsetDeviceExternalId()`

UnsetDeviceExternalId ensures that no value is present for DeviceExternalId, not even an explicit nil
### GetName

`func (o *CreateAccessPortalRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateAccessPortalRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateAccessPortalRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
