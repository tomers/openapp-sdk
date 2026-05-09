# CreateAccessPortalResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DeviceExternalId** | Pointer to **NullableString** |  | [optional]
**Id** | **string** |  |
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**PublicId** | **string** |  |

## Methods

### NewCreateAccessPortalResponse

`func NewCreateAccessPortalResponse(id string, name LocalizedString, publicId string, ) *CreateAccessPortalResponse`

NewCreateAccessPortalResponse instantiates a new CreateAccessPortalResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateAccessPortalResponseWithDefaults

`func NewCreateAccessPortalResponseWithDefaults() *CreateAccessPortalResponse`

NewCreateAccessPortalResponseWithDefaults instantiates a new CreateAccessPortalResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeviceExternalId

`func (o *CreateAccessPortalResponse) GetDeviceExternalId() string`

GetDeviceExternalId returns the DeviceExternalId field if non-nil, zero value otherwise.

### GetDeviceExternalIdOk

`func (o *CreateAccessPortalResponse) GetDeviceExternalIdOk() (*string, bool)`

GetDeviceExternalIdOk returns a tuple with the DeviceExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceExternalId

`func (o *CreateAccessPortalResponse) SetDeviceExternalId(v string)`

SetDeviceExternalId sets DeviceExternalId field to given value.

### HasDeviceExternalId

`func (o *CreateAccessPortalResponse) HasDeviceExternalId() bool`

HasDeviceExternalId returns a boolean if a field has been set.

### SetDeviceExternalIdNil

`func (o *CreateAccessPortalResponse) SetDeviceExternalIdNil(b bool)`

 SetDeviceExternalIdNil sets the value for DeviceExternalId to be an explicit nil

### UnsetDeviceExternalId
`func (o *CreateAccessPortalResponse) UnsetDeviceExternalId()`

UnsetDeviceExternalId ensures that no value is present for DeviceExternalId, not even an explicit nil
### GetId

`func (o *CreateAccessPortalResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *CreateAccessPortalResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *CreateAccessPortalResponse) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *CreateAccessPortalResponse) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateAccessPortalResponse) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateAccessPortalResponse) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetPublicId

`func (o *CreateAccessPortalResponse) GetPublicId() string`

GetPublicId returns the PublicId field if non-nil, zero value otherwise.

### GetPublicIdOk

`func (o *CreateAccessPortalResponse) GetPublicIdOk() (*string, bool)`

GetPublicIdOk returns a tuple with the PublicId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicId

`func (o *CreateAccessPortalResponse) SetPublicId(v string)`

SetPublicId sets PublicId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
