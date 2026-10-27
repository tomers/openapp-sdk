# PublicInviteWifiNetwork

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CredentialsUnavailable** | Pointer to **bool** | The credentials could not be read right now. The network is listed so the guest knows it exists and can ask the host; the door itself opens without Wi-Fi. | [optional]
**Hidden** | **bool** | The network does not broadcast its SSID. |
**Id** | **string** |  |
**Notes** | Pointer to **NullableString** | Where the network reaches, for example \&quot;lobby only\&quot;. | [optional]
**Password** | Pointer to **NullableString** | Passphrase (&#x60;wpa_psk&#x60;) or enterprise password. Absent for open networks. | [optional]
**Security** | [**WifiSecurity**](WifiSecurity.md) |  |
**Ssid** | **string** |  |
**Username** | Pointer to **NullableString** | Enterprise username (&#x60;wpa_enterprise&#x60; only). | [optional]

## Methods

### NewPublicInviteWifiNetwork

`func NewPublicInviteWifiNetwork(hidden bool, id string, security WifiSecurity, ssid string, ) *PublicInviteWifiNetwork`

NewPublicInviteWifiNetwork instantiates a new PublicInviteWifiNetwork object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicInviteWifiNetworkWithDefaults

`func NewPublicInviteWifiNetworkWithDefaults() *PublicInviteWifiNetwork`

NewPublicInviteWifiNetworkWithDefaults instantiates a new PublicInviteWifiNetwork object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCredentialsUnavailable

`func (o *PublicInviteWifiNetwork) GetCredentialsUnavailable() bool`

GetCredentialsUnavailable returns the CredentialsUnavailable field if non-nil, zero value otherwise.

### GetCredentialsUnavailableOk

`func (o *PublicInviteWifiNetwork) GetCredentialsUnavailableOk() (*bool, bool)`

GetCredentialsUnavailableOk returns a tuple with the CredentialsUnavailable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCredentialsUnavailable

`func (o *PublicInviteWifiNetwork) SetCredentialsUnavailable(v bool)`

SetCredentialsUnavailable sets CredentialsUnavailable field to given value.

### HasCredentialsUnavailable

`func (o *PublicInviteWifiNetwork) HasCredentialsUnavailable() bool`

HasCredentialsUnavailable returns a boolean if a field has been set.

### GetHidden

`func (o *PublicInviteWifiNetwork) GetHidden() bool`

GetHidden returns the Hidden field if non-nil, zero value otherwise.

### GetHiddenOk

`func (o *PublicInviteWifiNetwork) GetHiddenOk() (*bool, bool)`

GetHiddenOk returns a tuple with the Hidden field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHidden

`func (o *PublicInviteWifiNetwork) SetHidden(v bool)`

SetHidden sets Hidden field to given value.


### GetId

`func (o *PublicInviteWifiNetwork) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PublicInviteWifiNetwork) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PublicInviteWifiNetwork) SetId(v string)`

SetId sets Id field to given value.


### GetNotes

`func (o *PublicInviteWifiNetwork) GetNotes() string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *PublicInviteWifiNetwork) GetNotesOk() (*string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *PublicInviteWifiNetwork) SetNotes(v string)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *PublicInviteWifiNetwork) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *PublicInviteWifiNetwork) SetNotesNil(b bool)`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *PublicInviteWifiNetwork) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil
### GetPassword

`func (o *PublicInviteWifiNetwork) GetPassword() string`

GetPassword returns the Password field if non-nil, zero value otherwise.

### GetPasswordOk

`func (o *PublicInviteWifiNetwork) GetPasswordOk() (*string, bool)`

GetPasswordOk returns a tuple with the Password field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPassword

`func (o *PublicInviteWifiNetwork) SetPassword(v string)`

SetPassword sets Password field to given value.

### HasPassword

`func (o *PublicInviteWifiNetwork) HasPassword() bool`

HasPassword returns a boolean if a field has been set.

### SetPasswordNil

`func (o *PublicInviteWifiNetwork) SetPasswordNil(b bool)`

 SetPasswordNil sets the value for Password to be an explicit nil

### UnsetPassword
`func (o *PublicInviteWifiNetwork) UnsetPassword()`

UnsetPassword ensures that no value is present for Password, not even an explicit nil
### GetSecurity

`func (o *PublicInviteWifiNetwork) GetSecurity() WifiSecurity`

GetSecurity returns the Security field if non-nil, zero value otherwise.

### GetSecurityOk

`func (o *PublicInviteWifiNetwork) GetSecurityOk() (*WifiSecurity, bool)`

GetSecurityOk returns a tuple with the Security field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSecurity

`func (o *PublicInviteWifiNetwork) SetSecurity(v WifiSecurity)`

SetSecurity sets Security field to given value.


### GetSsid

`func (o *PublicInviteWifiNetwork) GetSsid() string`

GetSsid returns the Ssid field if non-nil, zero value otherwise.

### GetSsidOk

`func (o *PublicInviteWifiNetwork) GetSsidOk() (*string, bool)`

GetSsidOk returns a tuple with the Ssid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSsid

`func (o *PublicInviteWifiNetwork) SetSsid(v string)`

SetSsid sets Ssid field to given value.


### GetUsername

`func (o *PublicInviteWifiNetwork) GetUsername() string`

GetUsername returns the Username field if non-nil, zero value otherwise.

### GetUsernameOk

`func (o *PublicInviteWifiNetwork) GetUsernameOk() (*string, bool)`

GetUsernameOk returns a tuple with the Username field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsername

`func (o *PublicInviteWifiNetwork) SetUsername(v string)`

SetUsername sets Username field to given value.

### HasUsername

`func (o *PublicInviteWifiNetwork) HasUsername() bool`

HasUsername returns a boolean if a field has been set.

### SetUsernameNil

`func (o *PublicInviteWifiNetwork) SetUsernameNil(b bool)`

 SetUsernameNil sets the value for Username to be an explicit nil

### UnsetUsername
`func (o *PublicInviteWifiNetwork) UnsetUsername()`

UnsetUsername ensures that no value is present for Username, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
