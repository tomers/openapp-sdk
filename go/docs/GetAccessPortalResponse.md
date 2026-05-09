# GetAccessPortalResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **string** |  | [optional]
**DeviceExternalId** | Pointer to **string** | Device external_id (same integration): portal links to virtual_access_portal device by standard external identity. | [optional]
**Id** | **string** |  |
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**PublicId** | **string** |  |
**UpdatedAt** | Pointer to **string** |  | [optional]
**IntegrationId** | **string** |  |

## Methods

### NewGetAccessPortalResponse

`func NewGetAccessPortalResponse(id string, name LocalizedString, publicId string, integrationId string, ) *GetAccessPortalResponse`

NewGetAccessPortalResponse instantiates a new GetAccessPortalResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewGetAccessPortalResponseWithDefaults

`func NewGetAccessPortalResponseWithDefaults() *GetAccessPortalResponse`

NewGetAccessPortalResponseWithDefaults instantiates a new GetAccessPortalResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *GetAccessPortalResponse) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *GetAccessPortalResponse) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *GetAccessPortalResponse) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *GetAccessPortalResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetDeviceExternalId

`func (o *GetAccessPortalResponse) GetDeviceExternalId() string`

GetDeviceExternalId returns the DeviceExternalId field if non-nil, zero value otherwise.

### GetDeviceExternalIdOk

`func (o *GetAccessPortalResponse) GetDeviceExternalIdOk() (*string, bool)`

GetDeviceExternalIdOk returns a tuple with the DeviceExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceExternalId

`func (o *GetAccessPortalResponse) SetDeviceExternalId(v string)`

SetDeviceExternalId sets DeviceExternalId field to given value.

### HasDeviceExternalId

`func (o *GetAccessPortalResponse) HasDeviceExternalId() bool`

HasDeviceExternalId returns a boolean if a field has been set.

### GetId

`func (o *GetAccessPortalResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *GetAccessPortalResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *GetAccessPortalResponse) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *GetAccessPortalResponse) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *GetAccessPortalResponse) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *GetAccessPortalResponse) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetPublicId

`func (o *GetAccessPortalResponse) GetPublicId() string`

GetPublicId returns the PublicId field if non-nil, zero value otherwise.

### GetPublicIdOk

`func (o *GetAccessPortalResponse) GetPublicIdOk() (*string, bool)`

GetPublicIdOk returns a tuple with the PublicId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicId

`func (o *GetAccessPortalResponse) SetPublicId(v string)`

SetPublicId sets PublicId field to given value.


### GetUpdatedAt

`func (o *GetAccessPortalResponse) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *GetAccessPortalResponse) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *GetAccessPortalResponse) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *GetAccessPortalResponse) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### GetIntegrationId

`func (o *GetAccessPortalResponse) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *GetAccessPortalResponse) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *GetAccessPortalResponse) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
