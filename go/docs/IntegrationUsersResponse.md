# IntegrationUsersResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DirectoryLimited** | Pointer to **NullableBool** | When true, the upstream provider did not return the full user directory (e.g. linked credentials lack upstream admin rights). Provider-agnostic; see provider-specific endpoints for role details. | [optional]
**IntegrationId** | **string** |  |
**Items** | [**[]IntegrationUserItem**](IntegrationUserItem.md) |  |
**UnmanagedCount** | **int32** | Count of vendor users not linked to OpenApp (status &#x60;unlinked&#x60;). Informational only. |

## Methods

### NewIntegrationUsersResponse

`func NewIntegrationUsersResponse(integrationId string, items []IntegrationUserItem, unmanagedCount int32, ) *IntegrationUsersResponse`

NewIntegrationUsersResponse instantiates a new IntegrationUsersResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewIntegrationUsersResponseWithDefaults

`func NewIntegrationUsersResponseWithDefaults() *IntegrationUsersResponse`

NewIntegrationUsersResponseWithDefaults instantiates a new IntegrationUsersResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDirectoryLimited

`func (o *IntegrationUsersResponse) GetDirectoryLimited() bool`

GetDirectoryLimited returns the DirectoryLimited field if non-nil, zero value otherwise.

### GetDirectoryLimitedOk

`func (o *IntegrationUsersResponse) GetDirectoryLimitedOk() (*bool, bool)`

GetDirectoryLimitedOk returns a tuple with the DirectoryLimited field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDirectoryLimited

`func (o *IntegrationUsersResponse) SetDirectoryLimited(v bool)`

SetDirectoryLimited sets DirectoryLimited field to given value.

### HasDirectoryLimited

`func (o *IntegrationUsersResponse) HasDirectoryLimited() bool`

HasDirectoryLimited returns a boolean if a field has been set.

### SetDirectoryLimitedNil

`func (o *IntegrationUsersResponse) SetDirectoryLimitedNil(b bool)`

 SetDirectoryLimitedNil sets the value for DirectoryLimited to be an explicit nil

### UnsetDirectoryLimited
`func (o *IntegrationUsersResponse) UnsetDirectoryLimited()`

UnsetDirectoryLimited ensures that no value is present for DirectoryLimited, not even an explicit nil
### GetIntegrationId

`func (o *IntegrationUsersResponse) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *IntegrationUsersResponse) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *IntegrationUsersResponse) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### GetItems

`func (o *IntegrationUsersResponse) GetItems() []IntegrationUserItem`

GetItems returns the Items field if non-nil, zero value otherwise.

### GetItemsOk

`func (o *IntegrationUsersResponse) GetItemsOk() (*[]IntegrationUserItem, bool)`

GetItemsOk returns a tuple with the Items field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetItems

`func (o *IntegrationUsersResponse) SetItems(v []IntegrationUserItem)`

SetItems sets Items field to given value.


### GetUnmanagedCount

`func (o *IntegrationUsersResponse) GetUnmanagedCount() int32`

GetUnmanagedCount returns the UnmanagedCount field if non-nil, zero value otherwise.

### GetUnmanagedCountOk

`func (o *IntegrationUsersResponse) GetUnmanagedCountOk() (*int32, bool)`

GetUnmanagedCountOk returns a tuple with the UnmanagedCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnmanagedCount

`func (o *IntegrationUsersResponse) SetUnmanagedCount(v int32)`

SetUnmanagedCount sets UnmanagedCount field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
