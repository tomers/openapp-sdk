# WifiCredentials

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Password** | Pointer to **NullableString** | Passphrase (&#x60;wpa_psk&#x60;) or enterprise password (&#x60;wpa_enterprise&#x60;). Absent for &#x60;open&#x60;. | [optional]
**Username** | Pointer to **NullableString** | Enterprise username. Present only for &#x60;wpa_enterprise&#x60;. | [optional]

## Methods

### NewWifiCredentials

`func NewWifiCredentials() *WifiCredentials`

NewWifiCredentials instantiates a new WifiCredentials object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewWifiCredentialsWithDefaults

`func NewWifiCredentialsWithDefaults() *WifiCredentials`

NewWifiCredentialsWithDefaults instantiates a new WifiCredentials object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPassword

`func (o *WifiCredentials) GetPassword() string`

GetPassword returns the Password field if non-nil, zero value otherwise.

### GetPasswordOk

`func (o *WifiCredentials) GetPasswordOk() (*string, bool)`

GetPasswordOk returns a tuple with the Password field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPassword

`func (o *WifiCredentials) SetPassword(v string)`

SetPassword sets Password field to given value.

### HasPassword

`func (o *WifiCredentials) HasPassword() bool`

HasPassword returns a boolean if a field has been set.

### SetPasswordNil

`func (o *WifiCredentials) SetPasswordNil(b bool)`

 SetPasswordNil sets the value for Password to be an explicit nil

### UnsetPassword
`func (o *WifiCredentials) UnsetPassword()`

UnsetPassword ensures that no value is present for Password, not even an explicit nil
### GetUsername

`func (o *WifiCredentials) GetUsername() string`

GetUsername returns the Username field if non-nil, zero value otherwise.

### GetUsernameOk

`func (o *WifiCredentials) GetUsernameOk() (*string, bool)`

GetUsernameOk returns a tuple with the Username field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsername

`func (o *WifiCredentials) SetUsername(v string)`

SetUsername sets Username field to given value.

### HasUsername

`func (o *WifiCredentials) HasUsername() bool`

HasUsername returns a boolean if a field has been set.

### SetUsernameNil

`func (o *WifiCredentials) SetUsernameNil(b bool)`

 SetUsernameNil sets the value for Username to be an explicit nil

### UnsetUsername
`func (o *WifiCredentials) UnsetUsername()`

UnsetUsername ensures that no value is present for Username, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
