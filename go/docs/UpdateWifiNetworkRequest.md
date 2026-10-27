# UpdateWifiNetworkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Credentials** | Pointer to [**NullableWifiCredentials**](WifiCredentials.md) | Omit to keep the stored credentials, which is only allowed while &#x60;security&#x60; is unchanged. Changing &#x60;security&#x60; to a type that needs credentials requires them here. | [optional]
**Hidden** | Pointer to **bool** |  | [optional]
**Notes** | Pointer to **NullableString** |  | [optional]
**Priority** | Pointer to **int32** |  | [optional]
**Security** | [**WifiSecurity**](WifiSecurity.md) |  |
**Ssid** | **string** |  |

## Methods

### NewUpdateWifiNetworkRequest

`func NewUpdateWifiNetworkRequest(security WifiSecurity, ssid string, ) *UpdateWifiNetworkRequest`

NewUpdateWifiNetworkRequest instantiates a new UpdateWifiNetworkRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateWifiNetworkRequestWithDefaults

`func NewUpdateWifiNetworkRequestWithDefaults() *UpdateWifiNetworkRequest`

NewUpdateWifiNetworkRequestWithDefaults instantiates a new UpdateWifiNetworkRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCredentials

`func (o *UpdateWifiNetworkRequest) GetCredentials() WifiCredentials`

GetCredentials returns the Credentials field if non-nil, zero value otherwise.

### GetCredentialsOk

`func (o *UpdateWifiNetworkRequest) GetCredentialsOk() (*WifiCredentials, bool)`

GetCredentialsOk returns a tuple with the Credentials field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCredentials

`func (o *UpdateWifiNetworkRequest) SetCredentials(v WifiCredentials)`

SetCredentials sets Credentials field to given value.

### HasCredentials

`func (o *UpdateWifiNetworkRequest) HasCredentials() bool`

HasCredentials returns a boolean if a field has been set.

### SetCredentialsNil

`func (o *UpdateWifiNetworkRequest) SetCredentialsNil(b bool)`

 SetCredentialsNil sets the value for Credentials to be an explicit nil

### UnsetCredentials
`func (o *UpdateWifiNetworkRequest) UnsetCredentials()`

UnsetCredentials ensures that no value is present for Credentials, not even an explicit nil
### GetHidden

`func (o *UpdateWifiNetworkRequest) GetHidden() bool`

GetHidden returns the Hidden field if non-nil, zero value otherwise.

### GetHiddenOk

`func (o *UpdateWifiNetworkRequest) GetHiddenOk() (*bool, bool)`

GetHiddenOk returns a tuple with the Hidden field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHidden

`func (o *UpdateWifiNetworkRequest) SetHidden(v bool)`

SetHidden sets Hidden field to given value.

### HasHidden

`func (o *UpdateWifiNetworkRequest) HasHidden() bool`

HasHidden returns a boolean if a field has been set.

### GetNotes

`func (o *UpdateWifiNetworkRequest) GetNotes() string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *UpdateWifiNetworkRequest) GetNotesOk() (*string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *UpdateWifiNetworkRequest) SetNotes(v string)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *UpdateWifiNetworkRequest) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *UpdateWifiNetworkRequest) SetNotesNil(b bool)`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *UpdateWifiNetworkRequest) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil
### GetPriority

`func (o *UpdateWifiNetworkRequest) GetPriority() int32`

GetPriority returns the Priority field if non-nil, zero value otherwise.

### GetPriorityOk

`func (o *UpdateWifiNetworkRequest) GetPriorityOk() (*int32, bool)`

GetPriorityOk returns a tuple with the Priority field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPriority

`func (o *UpdateWifiNetworkRequest) SetPriority(v int32)`

SetPriority sets Priority field to given value.

### HasPriority

`func (o *UpdateWifiNetworkRequest) HasPriority() bool`

HasPriority returns a boolean if a field has been set.

### GetSecurity

`func (o *UpdateWifiNetworkRequest) GetSecurity() WifiSecurity`

GetSecurity returns the Security field if non-nil, zero value otherwise.

### GetSecurityOk

`func (o *UpdateWifiNetworkRequest) GetSecurityOk() (*WifiSecurity, bool)`

GetSecurityOk returns a tuple with the Security field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSecurity

`func (o *UpdateWifiNetworkRequest) SetSecurity(v WifiSecurity)`

SetSecurity sets Security field to given value.


### GetSsid

`func (o *UpdateWifiNetworkRequest) GetSsid() string`

GetSsid returns the Ssid field if non-nil, zero value otherwise.

### GetSsidOk

`func (o *UpdateWifiNetworkRequest) GetSsidOk() (*string, bool)`

GetSsidOk returns a tuple with the Ssid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSsid

`func (o *UpdateWifiNetworkRequest) SetSsid(v string)`

SetSsid sets Ssid field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
