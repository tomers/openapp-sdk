# SiteAccessWifiNetwork

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Covers** | [**[]SiteAccessWifiCover**](SiteAccessWifiCover.md) | The site zones this network applies to, and how. |
**HasCredentials** | **bool** | False for &#x60;open&#x60; networks. Credentials are never part of this response. |
**Hidden** | **bool** |  |
**Id** | **string** |  |
**Notes** | Pointer to **NullableString** |  | [optional]
**Priority** | **int32** |  |
**Security** | [**WifiSecurity**](WifiSecurity.md) |  |
**Ssid** | **string** |  |

## Methods

### NewSiteAccessWifiNetwork

`func NewSiteAccessWifiNetwork(covers []SiteAccessWifiCover, hasCredentials bool, hidden bool, id string, priority int32, security WifiSecurity, ssid string, ) *SiteAccessWifiNetwork`

NewSiteAccessWifiNetwork instantiates a new SiteAccessWifiNetwork object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSiteAccessWifiNetworkWithDefaults

`func NewSiteAccessWifiNetworkWithDefaults() *SiteAccessWifiNetwork`

NewSiteAccessWifiNetworkWithDefaults instantiates a new SiteAccessWifiNetwork object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCovers

`func (o *SiteAccessWifiNetwork) GetCovers() []SiteAccessWifiCover`

GetCovers returns the Covers field if non-nil, zero value otherwise.

### GetCoversOk

`func (o *SiteAccessWifiNetwork) GetCoversOk() (*[]SiteAccessWifiCover, bool)`

GetCoversOk returns a tuple with the Covers field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCovers

`func (o *SiteAccessWifiNetwork) SetCovers(v []SiteAccessWifiCover)`

SetCovers sets Covers field to given value.


### GetHasCredentials

`func (o *SiteAccessWifiNetwork) GetHasCredentials() bool`

GetHasCredentials returns the HasCredentials field if non-nil, zero value otherwise.

### GetHasCredentialsOk

`func (o *SiteAccessWifiNetwork) GetHasCredentialsOk() (*bool, bool)`

GetHasCredentialsOk returns a tuple with the HasCredentials field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasCredentials

`func (o *SiteAccessWifiNetwork) SetHasCredentials(v bool)`

SetHasCredentials sets HasCredentials field to given value.


### GetHidden

`func (o *SiteAccessWifiNetwork) GetHidden() bool`

GetHidden returns the Hidden field if non-nil, zero value otherwise.

### GetHiddenOk

`func (o *SiteAccessWifiNetwork) GetHiddenOk() (*bool, bool)`

GetHiddenOk returns a tuple with the Hidden field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHidden

`func (o *SiteAccessWifiNetwork) SetHidden(v bool)`

SetHidden sets Hidden field to given value.


### GetId

`func (o *SiteAccessWifiNetwork) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *SiteAccessWifiNetwork) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *SiteAccessWifiNetwork) SetId(v string)`

SetId sets Id field to given value.


### GetNotes

`func (o *SiteAccessWifiNetwork) GetNotes() string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *SiteAccessWifiNetwork) GetNotesOk() (*string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *SiteAccessWifiNetwork) SetNotes(v string)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *SiteAccessWifiNetwork) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *SiteAccessWifiNetwork) SetNotesNil(b bool)`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *SiteAccessWifiNetwork) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil
### GetPriority

`func (o *SiteAccessWifiNetwork) GetPriority() int32`

GetPriority returns the Priority field if non-nil, zero value otherwise.

### GetPriorityOk

`func (o *SiteAccessWifiNetwork) GetPriorityOk() (*int32, bool)`

GetPriorityOk returns a tuple with the Priority field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPriority

`func (o *SiteAccessWifiNetwork) SetPriority(v int32)`

SetPriority sets Priority field to given value.


### GetSecurity

`func (o *SiteAccessWifiNetwork) GetSecurity() WifiSecurity`

GetSecurity returns the Security field if non-nil, zero value otherwise.

### GetSecurityOk

`func (o *SiteAccessWifiNetwork) GetSecurityOk() (*WifiSecurity, bool)`

GetSecurityOk returns a tuple with the Security field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSecurity

`func (o *SiteAccessWifiNetwork) SetSecurity(v WifiSecurity)`

SetSecurity sets Security field to given value.


### GetSsid

`func (o *SiteAccessWifiNetwork) GetSsid() string`

GetSsid returns the Ssid field if non-nil, zero value otherwise.

### GetSsidOk

`func (o *SiteAccessWifiNetwork) GetSsidOk() (*string, bool)`

GetSsidOk returns a tuple with the Ssid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSsid

`func (o *SiteAccessWifiNetwork) SetSsid(v string)`

SetSsid sets Ssid field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
