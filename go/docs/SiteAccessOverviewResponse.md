# SiteAccessOverviewResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Devices** | Pointer to [**NullableSiteAccessOverviewDevices**](SiteAccessOverviewDevices.md) | Missing when the requester cannot list devices. | [optional]
**DirectoryDeviceId** | Pointer to **NullableString** |  | [optional]
**DoorsCount** | **int64** |  |
**Integration** | [**Integration**](Integration.md) | Sanitized integration data used by the setup checklist. |
**InvitationsCount** | **int64** |  |
**ListingsCount** | **int64** |  |
**PeopleCount** | **int64** |  |
**PoliciesCount** | Pointer to **NullableInt64** | Missing when the requester cannot list policies. | [optional]
**Portals** | [**[]SiteAccessOverviewPortal**](SiteAccessOverviewPortal.md) | Portal-to-door links are used to scope readiness issues on the hub. |
**PortalsCount** | **int64** |  |
**VisibleListingsCount** | **int64** |  |

## Methods

### NewSiteAccessOverviewResponse

`func NewSiteAccessOverviewResponse(doorsCount int64, integration Integration, invitationsCount int64, listingsCount int64, peopleCount int64, portals []SiteAccessOverviewPortal, portalsCount int64, visibleListingsCount int64, ) *SiteAccessOverviewResponse`

NewSiteAccessOverviewResponse instantiates a new SiteAccessOverviewResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSiteAccessOverviewResponseWithDefaults

`func NewSiteAccessOverviewResponseWithDefaults() *SiteAccessOverviewResponse`

NewSiteAccessOverviewResponseWithDefaults instantiates a new SiteAccessOverviewResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDevices

`func (o *SiteAccessOverviewResponse) GetDevices() SiteAccessOverviewDevices`

GetDevices returns the Devices field if non-nil, zero value otherwise.

### GetDevicesOk

`func (o *SiteAccessOverviewResponse) GetDevicesOk() (*SiteAccessOverviewDevices, bool)`

GetDevicesOk returns a tuple with the Devices field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDevices

`func (o *SiteAccessOverviewResponse) SetDevices(v SiteAccessOverviewDevices)`

SetDevices sets Devices field to given value.

### HasDevices

`func (o *SiteAccessOverviewResponse) HasDevices() bool`

HasDevices returns a boolean if a field has been set.

### SetDevicesNil

`func (o *SiteAccessOverviewResponse) SetDevicesNil(b bool)`

 SetDevicesNil sets the value for Devices to be an explicit nil

### UnsetDevices
`func (o *SiteAccessOverviewResponse) UnsetDevices()`

UnsetDevices ensures that no value is present for Devices, not even an explicit nil
### GetDirectoryDeviceId

`func (o *SiteAccessOverviewResponse) GetDirectoryDeviceId() string`

GetDirectoryDeviceId returns the DirectoryDeviceId field if non-nil, zero value otherwise.

### GetDirectoryDeviceIdOk

`func (o *SiteAccessOverviewResponse) GetDirectoryDeviceIdOk() (*string, bool)`

GetDirectoryDeviceIdOk returns a tuple with the DirectoryDeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDirectoryDeviceId

`func (o *SiteAccessOverviewResponse) SetDirectoryDeviceId(v string)`

SetDirectoryDeviceId sets DirectoryDeviceId field to given value.

### HasDirectoryDeviceId

`func (o *SiteAccessOverviewResponse) HasDirectoryDeviceId() bool`

HasDirectoryDeviceId returns a boolean if a field has been set.

### SetDirectoryDeviceIdNil

`func (o *SiteAccessOverviewResponse) SetDirectoryDeviceIdNil(b bool)`

 SetDirectoryDeviceIdNil sets the value for DirectoryDeviceId to be an explicit nil

### UnsetDirectoryDeviceId
`func (o *SiteAccessOverviewResponse) UnsetDirectoryDeviceId()`

UnsetDirectoryDeviceId ensures that no value is present for DirectoryDeviceId, not even an explicit nil
### GetDoorsCount

`func (o *SiteAccessOverviewResponse) GetDoorsCount() int64`

GetDoorsCount returns the DoorsCount field if non-nil, zero value otherwise.

### GetDoorsCountOk

`func (o *SiteAccessOverviewResponse) GetDoorsCountOk() (*int64, bool)`

GetDoorsCountOk returns a tuple with the DoorsCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorsCount

`func (o *SiteAccessOverviewResponse) SetDoorsCount(v int64)`

SetDoorsCount sets DoorsCount field to given value.


### GetIntegration

`func (o *SiteAccessOverviewResponse) GetIntegration() Integration`

GetIntegration returns the Integration field if non-nil, zero value otherwise.

### GetIntegrationOk

`func (o *SiteAccessOverviewResponse) GetIntegrationOk() (*Integration, bool)`

GetIntegrationOk returns a tuple with the Integration field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegration

`func (o *SiteAccessOverviewResponse) SetIntegration(v Integration)`

SetIntegration sets Integration field to given value.


### GetInvitationsCount

`func (o *SiteAccessOverviewResponse) GetInvitationsCount() int64`

GetInvitationsCount returns the InvitationsCount field if non-nil, zero value otherwise.

### GetInvitationsCountOk

`func (o *SiteAccessOverviewResponse) GetInvitationsCountOk() (*int64, bool)`

GetInvitationsCountOk returns a tuple with the InvitationsCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvitationsCount

`func (o *SiteAccessOverviewResponse) SetInvitationsCount(v int64)`

SetInvitationsCount sets InvitationsCount field to given value.


### GetListingsCount

`func (o *SiteAccessOverviewResponse) GetListingsCount() int64`

GetListingsCount returns the ListingsCount field if non-nil, zero value otherwise.

### GetListingsCountOk

`func (o *SiteAccessOverviewResponse) GetListingsCountOk() (*int64, bool)`

GetListingsCountOk returns a tuple with the ListingsCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListingsCount

`func (o *SiteAccessOverviewResponse) SetListingsCount(v int64)`

SetListingsCount sets ListingsCount field to given value.


### GetPeopleCount

`func (o *SiteAccessOverviewResponse) GetPeopleCount() int64`

GetPeopleCount returns the PeopleCount field if non-nil, zero value otherwise.

### GetPeopleCountOk

`func (o *SiteAccessOverviewResponse) GetPeopleCountOk() (*int64, bool)`

GetPeopleCountOk returns a tuple with the PeopleCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeopleCount

`func (o *SiteAccessOverviewResponse) SetPeopleCount(v int64)`

SetPeopleCount sets PeopleCount field to given value.


### GetPoliciesCount

`func (o *SiteAccessOverviewResponse) GetPoliciesCount() int64`

GetPoliciesCount returns the PoliciesCount field if non-nil, zero value otherwise.

### GetPoliciesCountOk

`func (o *SiteAccessOverviewResponse) GetPoliciesCountOk() (*int64, bool)`

GetPoliciesCountOk returns a tuple with the PoliciesCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPoliciesCount

`func (o *SiteAccessOverviewResponse) SetPoliciesCount(v int64)`

SetPoliciesCount sets PoliciesCount field to given value.

### HasPoliciesCount

`func (o *SiteAccessOverviewResponse) HasPoliciesCount() bool`

HasPoliciesCount returns a boolean if a field has been set.

### SetPoliciesCountNil

`func (o *SiteAccessOverviewResponse) SetPoliciesCountNil(b bool)`

 SetPoliciesCountNil sets the value for PoliciesCount to be an explicit nil

### UnsetPoliciesCount
`func (o *SiteAccessOverviewResponse) UnsetPoliciesCount()`

UnsetPoliciesCount ensures that no value is present for PoliciesCount, not even an explicit nil
### GetPortals

`func (o *SiteAccessOverviewResponse) GetPortals() []SiteAccessOverviewPortal`

GetPortals returns the Portals field if non-nil, zero value otherwise.

### GetPortalsOk

`func (o *SiteAccessOverviewResponse) GetPortalsOk() (*[]SiteAccessOverviewPortal, bool)`

GetPortalsOk returns a tuple with the Portals field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPortals

`func (o *SiteAccessOverviewResponse) SetPortals(v []SiteAccessOverviewPortal)`

SetPortals sets Portals field to given value.


### GetPortalsCount

`func (o *SiteAccessOverviewResponse) GetPortalsCount() int64`

GetPortalsCount returns the PortalsCount field if non-nil, zero value otherwise.

### GetPortalsCountOk

`func (o *SiteAccessOverviewResponse) GetPortalsCountOk() (*int64, bool)`

GetPortalsCountOk returns a tuple with the PortalsCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPortalsCount

`func (o *SiteAccessOverviewResponse) SetPortalsCount(v int64)`

SetPortalsCount sets PortalsCount field to given value.


### GetVisibleListingsCount

`func (o *SiteAccessOverviewResponse) GetVisibleListingsCount() int64`

GetVisibleListingsCount returns the VisibleListingsCount field if non-nil, zero value otherwise.

### GetVisibleListingsCountOk

`func (o *SiteAccessOverviewResponse) GetVisibleListingsCountOk() (*int64, bool)`

GetVisibleListingsCountOk returns a tuple with the VisibleListingsCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVisibleListingsCount

`func (o *SiteAccessOverviewResponse) SetVisibleListingsCount(v int64)`

SetVisibleListingsCount sets VisibleListingsCount field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
