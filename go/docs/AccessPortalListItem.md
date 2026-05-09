# AccessPortalListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**DeviceExternalId** | Pointer to **NullableString** | Device external_id (same integration): portal links to virtual_access_portal device by standard external identity. | [optional]
**Id** | **string** |  |
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**PublicId** | **string** |  |
**UpdatedAt** | Pointer to **NullableString** |  | [optional]

## Methods

### NewAccessPortalListItem

`func NewAccessPortalListItem(id string, name LocalizedString, publicId string, ) *AccessPortalListItem`

NewAccessPortalListItem instantiates a new AccessPortalListItem object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAccessPortalListItemWithDefaults

`func NewAccessPortalListItemWithDefaults() *AccessPortalListItem`

NewAccessPortalListItemWithDefaults instantiates a new AccessPortalListItem object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *AccessPortalListItem) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *AccessPortalListItem) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *AccessPortalListItem) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *AccessPortalListItem) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *AccessPortalListItem) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *AccessPortalListItem) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetDeviceExternalId

`func (o *AccessPortalListItem) GetDeviceExternalId() string`

GetDeviceExternalId returns the DeviceExternalId field if non-nil, zero value otherwise.

### GetDeviceExternalIdOk

`func (o *AccessPortalListItem) GetDeviceExternalIdOk() (*string, bool)`

GetDeviceExternalIdOk returns a tuple with the DeviceExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceExternalId

`func (o *AccessPortalListItem) SetDeviceExternalId(v string)`

SetDeviceExternalId sets DeviceExternalId field to given value.

### HasDeviceExternalId

`func (o *AccessPortalListItem) HasDeviceExternalId() bool`

HasDeviceExternalId returns a boolean if a field has been set.

### SetDeviceExternalIdNil

`func (o *AccessPortalListItem) SetDeviceExternalIdNil(b bool)`

 SetDeviceExternalIdNil sets the value for DeviceExternalId to be an explicit nil

### UnsetDeviceExternalId
`func (o *AccessPortalListItem) UnsetDeviceExternalId()`

UnsetDeviceExternalId ensures that no value is present for DeviceExternalId, not even an explicit nil
### GetId

`func (o *AccessPortalListItem) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *AccessPortalListItem) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *AccessPortalListItem) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *AccessPortalListItem) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *AccessPortalListItem) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *AccessPortalListItem) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetPublicId

`func (o *AccessPortalListItem) GetPublicId() string`

GetPublicId returns the PublicId field if non-nil, zero value otherwise.

### GetPublicIdOk

`func (o *AccessPortalListItem) GetPublicIdOk() (*string, bool)`

GetPublicIdOk returns a tuple with the PublicId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicId

`func (o *AccessPortalListItem) SetPublicId(v string)`

SetPublicId sets PublicId field to given value.


### GetUpdatedAt

`func (o *AccessPortalListItem) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *AccessPortalListItem) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *AccessPortalListItem) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *AccessPortalListItem) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *AccessPortalListItem) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *AccessPortalListItem) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
