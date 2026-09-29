# CreateAccessPortalRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DeviceId** | Pointer to **NullableString** | Door device id (virtual_access_portal device in this integration). Links the portal to a door. | [optional]
**DirectoryId** | Pointer to **NullableString** | Directory device id (virtual_access_directory in this integration). | [optional]
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

### GetDeviceId

`func (o *CreateAccessPortalRequest) GetDeviceId() string`

GetDeviceId returns the DeviceId field if non-nil, zero value otherwise.

### GetDeviceIdOk

`func (o *CreateAccessPortalRequest) GetDeviceIdOk() (*string, bool)`

GetDeviceIdOk returns a tuple with the DeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceId

`func (o *CreateAccessPortalRequest) SetDeviceId(v string)`

SetDeviceId sets DeviceId field to given value.

### HasDeviceId

`func (o *CreateAccessPortalRequest) HasDeviceId() bool`

HasDeviceId returns a boolean if a field has been set.

### SetDeviceIdNil

`func (o *CreateAccessPortalRequest) SetDeviceIdNil(b bool)`

 SetDeviceIdNil sets the value for DeviceId to be an explicit nil

### UnsetDeviceId
`func (o *CreateAccessPortalRequest) UnsetDeviceId()`

UnsetDeviceId ensures that no value is present for DeviceId, not even an explicit nil
### GetDirectoryId

`func (o *CreateAccessPortalRequest) GetDirectoryId() string`

GetDirectoryId returns the DirectoryId field if non-nil, zero value otherwise.

### GetDirectoryIdOk

`func (o *CreateAccessPortalRequest) GetDirectoryIdOk() (*string, bool)`

GetDirectoryIdOk returns a tuple with the DirectoryId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDirectoryId

`func (o *CreateAccessPortalRequest) SetDirectoryId(v string)`

SetDirectoryId sets DirectoryId field to given value.

### HasDirectoryId

`func (o *CreateAccessPortalRequest) HasDirectoryId() bool`

HasDirectoryId returns a boolean if a field has been set.

### SetDirectoryIdNil

`func (o *CreateAccessPortalRequest) SetDirectoryIdNil(b bool)`

 SetDirectoryIdNil sets the value for DirectoryId to be an explicit nil

### UnsetDirectoryId
`func (o *CreateAccessPortalRequest) UnsetDirectoryId()`

UnsetDirectoryId ensures that no value is present for DirectoryId, not even an explicit nil
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
