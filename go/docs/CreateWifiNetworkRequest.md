# CreateWifiNetworkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Credentials** | Pointer to [**NullableWifiCredentials**](WifiCredentials.md) | Required for &#x60;wpa_psk&#x60; (password) and &#x60;wpa_enterprise&#x60; (username and password); rejected for &#x60;open&#x60;. | [optional]
**Hidden** | Pointer to **bool** |  | [optional]
**Notes** | Pointer to **NullableString** |  | [optional]
**Priority** | Pointer to **int32** | Lower values are listed first in text messages. Defaults to 0. | [optional]
**Security** | [**WifiSecurity**](WifiSecurity.md) |  |
**Ssid** | **string** |  |
**ZoneIds** | Pointer to **[]string** | Zones the network covers. Each must be a live zone of the &#x60;X-Org&#x60; organization. | [optional]

## Methods

### NewCreateWifiNetworkRequest

`func NewCreateWifiNetworkRequest(security WifiSecurity, ssid string, ) *CreateWifiNetworkRequest`

NewCreateWifiNetworkRequest instantiates a new CreateWifiNetworkRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateWifiNetworkRequestWithDefaults

`func NewCreateWifiNetworkRequestWithDefaults() *CreateWifiNetworkRequest`

NewCreateWifiNetworkRequestWithDefaults instantiates a new CreateWifiNetworkRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCredentials

`func (o *CreateWifiNetworkRequest) GetCredentials() WifiCredentials`

GetCredentials returns the Credentials field if non-nil, zero value otherwise.

### GetCredentialsOk

`func (o *CreateWifiNetworkRequest) GetCredentialsOk() (*WifiCredentials, bool)`

GetCredentialsOk returns a tuple with the Credentials field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCredentials

`func (o *CreateWifiNetworkRequest) SetCredentials(v WifiCredentials)`

SetCredentials sets Credentials field to given value.

### HasCredentials

`func (o *CreateWifiNetworkRequest) HasCredentials() bool`

HasCredentials returns a boolean if a field has been set.

### SetCredentialsNil

`func (o *CreateWifiNetworkRequest) SetCredentialsNil(b bool)`

 SetCredentialsNil sets the value for Credentials to be an explicit nil

### UnsetCredentials
`func (o *CreateWifiNetworkRequest) UnsetCredentials()`

UnsetCredentials ensures that no value is present for Credentials, not even an explicit nil
### GetHidden

`func (o *CreateWifiNetworkRequest) GetHidden() bool`

GetHidden returns the Hidden field if non-nil, zero value otherwise.

### GetHiddenOk

`func (o *CreateWifiNetworkRequest) GetHiddenOk() (*bool, bool)`

GetHiddenOk returns a tuple with the Hidden field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHidden

`func (o *CreateWifiNetworkRequest) SetHidden(v bool)`

SetHidden sets Hidden field to given value.

### HasHidden

`func (o *CreateWifiNetworkRequest) HasHidden() bool`

HasHidden returns a boolean if a field has been set.

### GetNotes

`func (o *CreateWifiNetworkRequest) GetNotes() string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *CreateWifiNetworkRequest) GetNotesOk() (*string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *CreateWifiNetworkRequest) SetNotes(v string)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *CreateWifiNetworkRequest) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *CreateWifiNetworkRequest) SetNotesNil(b bool)`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *CreateWifiNetworkRequest) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil
### GetPriority

`func (o *CreateWifiNetworkRequest) GetPriority() int32`

GetPriority returns the Priority field if non-nil, zero value otherwise.

### GetPriorityOk

`func (o *CreateWifiNetworkRequest) GetPriorityOk() (*int32, bool)`

GetPriorityOk returns a tuple with the Priority field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPriority

`func (o *CreateWifiNetworkRequest) SetPriority(v int32)`

SetPriority sets Priority field to given value.

### HasPriority

`func (o *CreateWifiNetworkRequest) HasPriority() bool`

HasPriority returns a boolean if a field has been set.

### GetSecurity

`func (o *CreateWifiNetworkRequest) GetSecurity() WifiSecurity`

GetSecurity returns the Security field if non-nil, zero value otherwise.

### GetSecurityOk

`func (o *CreateWifiNetworkRequest) GetSecurityOk() (*WifiSecurity, bool)`

GetSecurityOk returns a tuple with the Security field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSecurity

`func (o *CreateWifiNetworkRequest) SetSecurity(v WifiSecurity)`

SetSecurity sets Security field to given value.


### GetSsid

`func (o *CreateWifiNetworkRequest) GetSsid() string`

GetSsid returns the Ssid field if non-nil, zero value otherwise.

### GetSsidOk

`func (o *CreateWifiNetworkRequest) GetSsidOk() (*string, bool)`

GetSsidOk returns a tuple with the Ssid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSsid

`func (o *CreateWifiNetworkRequest) SetSsid(v string)`

SetSsid sets Ssid field to given value.


### GetZoneIds

`func (o *CreateWifiNetworkRequest) GetZoneIds() []string`

GetZoneIds returns the ZoneIds field if non-nil, zero value otherwise.

### GetZoneIdsOk

`func (o *CreateWifiNetworkRequest) GetZoneIdsOk() (*[]string, bool)`

GetZoneIdsOk returns a tuple with the ZoneIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneIds

`func (o *CreateWifiNetworkRequest) SetZoneIds(v []string)`

SetZoneIds sets ZoneIds field to given value.

### HasZoneIds

`func (o *CreateWifiNetworkRequest) HasZoneIds() bool`

HasZoneIds returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
