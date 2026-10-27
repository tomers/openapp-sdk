# PaginatedResponseWifiNetworkResponseItemsInner

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

### NewPaginatedResponseWifiNetworkResponseItemsInner

`func NewPaginatedResponseWifiNetworkResponseItemsInner(createdAt time.Time, hidden bool, id string, orgId string, priority int32, security WifiSecurity, ssid string, updatedAt time.Time, hasCredentials bool, zoneIds []string, ) *PaginatedResponseWifiNetworkResponseItemsInner`

NewPaginatedResponseWifiNetworkResponseItemsInner instantiates a new PaginatedResponseWifiNetworkResponseItemsInner object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseWifiNetworkResponseItemsInnerWithDefaults

`func NewPaginatedResponseWifiNetworkResponseItemsInnerWithDefaults() *PaginatedResponseWifiNetworkResponseItemsInner`

NewPaginatedResponseWifiNetworkResponseItemsInnerWithDefaults instantiates a new PaginatedResponseWifiNetworkResponseItemsInner object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetHidden

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetHidden() bool`

GetHidden returns the Hidden field if non-nil, zero value otherwise.

### GetHiddenOk

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetHiddenOk() (*bool, bool)`

GetHiddenOk returns a tuple with the Hidden field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHidden

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) SetHidden(v bool)`

SetHidden sets Hidden field to given value.


### GetId

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) SetId(v string)`

SetId sets Id field to given value.


### GetNotes

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetNotes() string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetNotesOk() (*string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) SetNotes(v string)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### GetOrgId

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetPriority

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetPriority() int32`

GetPriority returns the Priority field if non-nil, zero value otherwise.

### GetPriorityOk

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetPriorityOk() (*int32, bool)`

GetPriorityOk returns a tuple with the Priority field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPriority

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) SetPriority(v int32)`

SetPriority sets Priority field to given value.


### GetSecurity

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetSecurity() WifiSecurity`

GetSecurity returns the Security field if non-nil, zero value otherwise.

### GetSecurityOk

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetSecurityOk() (*WifiSecurity, bool)`

GetSecurityOk returns a tuple with the Security field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSecurity

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) SetSecurity(v WifiSecurity)`

SetSecurity sets Security field to given value.


### GetSsid

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetSsid() string`

GetSsid returns the Ssid field if non-nil, zero value otherwise.

### GetSsidOk

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetSsidOk() (*string, bool)`

GetSsidOk returns a tuple with the Ssid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSsid

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) SetSsid(v string)`

SetSsid sets Ssid field to given value.


### GetUpdatedAt

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.


### GetHasCredentials

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetHasCredentials() bool`

GetHasCredentials returns the HasCredentials field if non-nil, zero value otherwise.

### GetHasCredentialsOk

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetHasCredentialsOk() (*bool, bool)`

GetHasCredentialsOk returns a tuple with the HasCredentials field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasCredentials

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) SetHasCredentials(v bool)`

SetHasCredentials sets HasCredentials field to given value.


### GetZoneIds

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetZoneIds() []string`

GetZoneIds returns the ZoneIds field if non-nil, zero value otherwise.

### GetZoneIdsOk

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) GetZoneIdsOk() (*[]string, bool)`

GetZoneIdsOk returns a tuple with the ZoneIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneIds

`func (o *PaginatedResponseWifiNetworkResponseItemsInner) SetZoneIds(v []string)`

SetZoneIds sets ZoneIds field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
