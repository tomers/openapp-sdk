# DirectoryConfigResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AllowedListingKinds** | **[]string** |  |
**DeviceId** | **string** |  |
**DisabledMessage** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**NamingPatterns** | **map[string]string** |  |
**State** | **string** |  |

## Methods

### NewDirectoryConfigResponse

`func NewDirectoryConfigResponse(allowedListingKinds []string, deviceId string, namingPatterns map[string]string, state string, ) *DirectoryConfigResponse`

NewDirectoryConfigResponse instantiates a new DirectoryConfigResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDirectoryConfigResponseWithDefaults

`func NewDirectoryConfigResponseWithDefaults() *DirectoryConfigResponse`

NewDirectoryConfigResponseWithDefaults instantiates a new DirectoryConfigResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAllowedListingKinds

`func (o *DirectoryConfigResponse) GetAllowedListingKinds() []string`

GetAllowedListingKinds returns the AllowedListingKinds field if non-nil, zero value otherwise.

### GetAllowedListingKindsOk

`func (o *DirectoryConfigResponse) GetAllowedListingKindsOk() (*[]string, bool)`

GetAllowedListingKindsOk returns a tuple with the AllowedListingKinds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedListingKinds

`func (o *DirectoryConfigResponse) SetAllowedListingKinds(v []string)`

SetAllowedListingKinds sets AllowedListingKinds field to given value.


### GetDeviceId

`func (o *DirectoryConfigResponse) GetDeviceId() string`

GetDeviceId returns the DeviceId field if non-nil, zero value otherwise.

### GetDeviceIdOk

`func (o *DirectoryConfigResponse) GetDeviceIdOk() (*string, bool)`

GetDeviceIdOk returns a tuple with the DeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceId

`func (o *DirectoryConfigResponse) SetDeviceId(v string)`

SetDeviceId sets DeviceId field to given value.


### GetDisabledMessage

`func (o *DirectoryConfigResponse) GetDisabledMessage() LocalizedString`

GetDisabledMessage returns the DisabledMessage field if non-nil, zero value otherwise.

### GetDisabledMessageOk

`func (o *DirectoryConfigResponse) GetDisabledMessageOk() (*LocalizedString, bool)`

GetDisabledMessageOk returns a tuple with the DisabledMessage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisabledMessage

`func (o *DirectoryConfigResponse) SetDisabledMessage(v LocalizedString)`

SetDisabledMessage sets DisabledMessage field to given value.

### HasDisabledMessage

`func (o *DirectoryConfigResponse) HasDisabledMessage() bool`

HasDisabledMessage returns a boolean if a field has been set.

### SetDisabledMessageNil

`func (o *DirectoryConfigResponse) SetDisabledMessageNil(b bool)`

 SetDisabledMessageNil sets the value for DisabledMessage to be an explicit nil

### UnsetDisabledMessage
`func (o *DirectoryConfigResponse) UnsetDisabledMessage()`

UnsetDisabledMessage ensures that no value is present for DisabledMessage, not even an explicit nil
### GetNamingPatterns

`func (o *DirectoryConfigResponse) GetNamingPatterns() map[string]string`

GetNamingPatterns returns the NamingPatterns field if non-nil, zero value otherwise.

### GetNamingPatternsOk

`func (o *DirectoryConfigResponse) GetNamingPatternsOk() (*map[string]string, bool)`

GetNamingPatternsOk returns a tuple with the NamingPatterns field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNamingPatterns

`func (o *DirectoryConfigResponse) SetNamingPatterns(v map[string]string)`

SetNamingPatterns sets NamingPatterns field to given value.


### GetState

`func (o *DirectoryConfigResponse) GetState() string`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *DirectoryConfigResponse) GetStateOk() (*string, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *DirectoryConfigResponse) SetState(v string)`

SetState sets State field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
