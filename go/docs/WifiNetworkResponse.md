# WifiNetworkResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | **time.Time** |  |
**Hidden** | **bool** | The network does not broadcast its SSID. |
**Id** | **string** |  |
**Notes** | Pointer to **string** |  | [optional]
**OrgId** | **string** |  |
**Priority** | **int32** | Lower values are listed first. |
**Security** | [**WifiSecurity**](WifiSecurity.md) |  |
**Ssid** | **string** |  |
**UpdatedAt** | **time.Time** |  |
**HasCredentials** | **bool** | False for &#x60;open&#x60; networks. |
**ZoneIds** | **[]string** | Zones this network covers. Invitations to doors in these zones offer the network. |

## Methods

### NewWifiNetworkResponse

`func NewWifiNetworkResponse(createdAt time.Time, hidden bool, id string, orgId string, priority int32, security WifiSecurity, ssid string, updatedAt time.Time, hasCredentials bool, zoneIds []string, ) *WifiNetworkResponse`

NewWifiNetworkResponse instantiates a new WifiNetworkResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewWifiNetworkResponseWithDefaults

`func NewWifiNetworkResponseWithDefaults() *WifiNetworkResponse`

NewWifiNetworkResponseWithDefaults instantiates a new WifiNetworkResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *WifiNetworkResponse) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *WifiNetworkResponse) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *WifiNetworkResponse) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetHidden

`func (o *WifiNetworkResponse) GetHidden() bool`

GetHidden returns the Hidden field if non-nil, zero value otherwise.

### GetHiddenOk

`func (o *WifiNetworkResponse) GetHiddenOk() (*bool, bool)`

GetHiddenOk returns a tuple with the Hidden field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHidden

`func (o *WifiNetworkResponse) SetHidden(v bool)`

SetHidden sets Hidden field to given value.


### GetId

`func (o *WifiNetworkResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *WifiNetworkResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *WifiNetworkResponse) SetId(v string)`

SetId sets Id field to given value.


### GetNotes

`func (o *WifiNetworkResponse) GetNotes() string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *WifiNetworkResponse) GetNotesOk() (*string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *WifiNetworkResponse) SetNotes(v string)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *WifiNetworkResponse) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### GetOrgId

`func (o *WifiNetworkResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *WifiNetworkResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *WifiNetworkResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetPriority

`func (o *WifiNetworkResponse) GetPriority() int32`

GetPriority returns the Priority field if non-nil, zero value otherwise.

### GetPriorityOk

`func (o *WifiNetworkResponse) GetPriorityOk() (*int32, bool)`

GetPriorityOk returns a tuple with the Priority field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPriority

`func (o *WifiNetworkResponse) SetPriority(v int32)`

SetPriority sets Priority field to given value.


### GetSecurity

`func (o *WifiNetworkResponse) GetSecurity() WifiSecurity`

GetSecurity returns the Security field if non-nil, zero value otherwise.

### GetSecurityOk

`func (o *WifiNetworkResponse) GetSecurityOk() (*WifiSecurity, bool)`

GetSecurityOk returns a tuple with the Security field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSecurity

`func (o *WifiNetworkResponse) SetSecurity(v WifiSecurity)`

SetSecurity sets Security field to given value.


### GetSsid

`func (o *WifiNetworkResponse) GetSsid() string`

GetSsid returns the Ssid field if non-nil, zero value otherwise.

### GetSsidOk

`func (o *WifiNetworkResponse) GetSsidOk() (*string, bool)`

GetSsidOk returns a tuple with the Ssid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSsid

`func (o *WifiNetworkResponse) SetSsid(v string)`

SetSsid sets Ssid field to given value.


### GetUpdatedAt

`func (o *WifiNetworkResponse) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *WifiNetworkResponse) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *WifiNetworkResponse) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.


### GetHasCredentials

`func (o *WifiNetworkResponse) GetHasCredentials() bool`

GetHasCredentials returns the HasCredentials field if non-nil, zero value otherwise.

### GetHasCredentialsOk

`func (o *WifiNetworkResponse) GetHasCredentialsOk() (*bool, bool)`

GetHasCredentialsOk returns a tuple with the HasCredentials field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasCredentials

`func (o *WifiNetworkResponse) SetHasCredentials(v bool)`

SetHasCredentials sets HasCredentials field to given value.


### GetZoneIds

`func (o *WifiNetworkResponse) GetZoneIds() []string`

GetZoneIds returns the ZoneIds field if non-nil, zero value otherwise.

### GetZoneIdsOk

`func (o *WifiNetworkResponse) GetZoneIdsOk() (*[]string, bool)`

GetZoneIdsOk returns a tuple with the ZoneIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneIds

`func (o *WifiNetworkResponse) SetZoneIds(v []string)`

SetZoneIds sets ZoneIds field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
