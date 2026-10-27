# WifiNetwork

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | **time.Time** |  |
**Hidden** | **bool** | The network does not broadcast its SSID. |
**Id** | **string** |  |
**Notes** | Pointer to **NullableString** |  | [optional]
**OrgId** | **string** |  |
**Priority** | **int32** | Lower values are listed first. |
**Security** | [**WifiSecurity**](WifiSecurity.md) |  |
**Ssid** | **string** |  |
**UpdatedAt** | **time.Time** |  |

## Methods

### NewWifiNetwork

`func NewWifiNetwork(createdAt time.Time, hidden bool, id string, orgId string, priority int32, security WifiSecurity, ssid string, updatedAt time.Time, ) *WifiNetwork`

NewWifiNetwork instantiates a new WifiNetwork object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewWifiNetworkWithDefaults

`func NewWifiNetworkWithDefaults() *WifiNetwork`

NewWifiNetworkWithDefaults instantiates a new WifiNetwork object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *WifiNetwork) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *WifiNetwork) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *WifiNetwork) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetHidden

`func (o *WifiNetwork) GetHidden() bool`

GetHidden returns the Hidden field if non-nil, zero value otherwise.

### GetHiddenOk

`func (o *WifiNetwork) GetHiddenOk() (*bool, bool)`

GetHiddenOk returns a tuple with the Hidden field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHidden

`func (o *WifiNetwork) SetHidden(v bool)`

SetHidden sets Hidden field to given value.


### GetId

`func (o *WifiNetwork) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *WifiNetwork) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *WifiNetwork) SetId(v string)`

SetId sets Id field to given value.


### GetNotes

`func (o *WifiNetwork) GetNotes() string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *WifiNetwork) GetNotesOk() (*string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *WifiNetwork) SetNotes(v string)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *WifiNetwork) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *WifiNetwork) SetNotesNil(b bool)`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *WifiNetwork) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil
### GetOrgId

`func (o *WifiNetwork) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *WifiNetwork) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *WifiNetwork) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetPriority

`func (o *WifiNetwork) GetPriority() int32`

GetPriority returns the Priority field if non-nil, zero value otherwise.

### GetPriorityOk

`func (o *WifiNetwork) GetPriorityOk() (*int32, bool)`

GetPriorityOk returns a tuple with the Priority field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPriority

`func (o *WifiNetwork) SetPriority(v int32)`

SetPriority sets Priority field to given value.


### GetSecurity

`func (o *WifiNetwork) GetSecurity() WifiSecurity`

GetSecurity returns the Security field if non-nil, zero value otherwise.

### GetSecurityOk

`func (o *WifiNetwork) GetSecurityOk() (*WifiSecurity, bool)`

GetSecurityOk returns a tuple with the Security field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSecurity

`func (o *WifiNetwork) SetSecurity(v WifiSecurity)`

SetSecurity sets Security field to given value.


### GetSsid

`func (o *WifiNetwork) GetSsid() string`

GetSsid returns the Ssid field if non-nil, zero value otherwise.

### GetSsidOk

`func (o *WifiNetwork) GetSsidOk() (*string, bool)`

GetSsidOk returns a tuple with the Ssid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSsid

`func (o *WifiNetwork) SetSsid(v string)`

SetSsid sets Ssid field to given value.


### GetUpdatedAt

`func (o *WifiNetwork) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *WifiNetwork) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *WifiNetwork) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
