# SiteAccessZonesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DoorsWithoutZone** | [**[]SiteAccessZoneDoor**](SiteAccessZoneDoor.md) | Doors with no zone. Their invitations offer no Wi-Fi and no zone policy binds them. |
**WifiNetworks** | Pointer to [**[]SiteAccessWifiNetwork**](SiteAccessWifiNetwork.md) | Networks that apply to the site&#39;s doors. Omitted when the requester lacks &#x60;zones:read&#x60;. | [optional]
**Zones** | Pointer to [**[]SiteAccessZone**](SiteAccessZone.md) | Zones that hold the site&#39;s doors. Omitted when the requester lacks &#x60;zones:read&#x60;. | [optional]

## Methods

### NewSiteAccessZonesResponse

`func NewSiteAccessZonesResponse(doorsWithoutZone []SiteAccessZoneDoor, ) *SiteAccessZonesResponse`

NewSiteAccessZonesResponse instantiates a new SiteAccessZonesResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSiteAccessZonesResponseWithDefaults

`func NewSiteAccessZonesResponseWithDefaults() *SiteAccessZonesResponse`

NewSiteAccessZonesResponseWithDefaults instantiates a new SiteAccessZonesResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDoorsWithoutZone

`func (o *SiteAccessZonesResponse) GetDoorsWithoutZone() []SiteAccessZoneDoor`

GetDoorsWithoutZone returns the DoorsWithoutZone field if non-nil, zero value otherwise.

### GetDoorsWithoutZoneOk

`func (o *SiteAccessZonesResponse) GetDoorsWithoutZoneOk() (*[]SiteAccessZoneDoor, bool)`

GetDoorsWithoutZoneOk returns a tuple with the DoorsWithoutZone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorsWithoutZone

`func (o *SiteAccessZonesResponse) SetDoorsWithoutZone(v []SiteAccessZoneDoor)`

SetDoorsWithoutZone sets DoorsWithoutZone field to given value.


### GetWifiNetworks

`func (o *SiteAccessZonesResponse) GetWifiNetworks() []SiteAccessWifiNetwork`

GetWifiNetworks returns the WifiNetworks field if non-nil, zero value otherwise.

### GetWifiNetworksOk

`func (o *SiteAccessZonesResponse) GetWifiNetworksOk() (*[]SiteAccessWifiNetwork, bool)`

GetWifiNetworksOk returns a tuple with the WifiNetworks field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWifiNetworks

`func (o *SiteAccessZonesResponse) SetWifiNetworks(v []SiteAccessWifiNetwork)`

SetWifiNetworks sets WifiNetworks field to given value.

### HasWifiNetworks

`func (o *SiteAccessZonesResponse) HasWifiNetworks() bool`

HasWifiNetworks returns a boolean if a field has been set.

### SetWifiNetworksNil

`func (o *SiteAccessZonesResponse) SetWifiNetworksNil(b bool)`

 SetWifiNetworksNil sets the value for WifiNetworks to be an explicit nil

### UnsetWifiNetworks
`func (o *SiteAccessZonesResponse) UnsetWifiNetworks()`

UnsetWifiNetworks ensures that no value is present for WifiNetworks, not even an explicit nil
### GetZones

`func (o *SiteAccessZonesResponse) GetZones() []SiteAccessZone`

GetZones returns the Zones field if non-nil, zero value otherwise.

### GetZonesOk

`func (o *SiteAccessZonesResponse) GetZonesOk() (*[]SiteAccessZone, bool)`

GetZonesOk returns a tuple with the Zones field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZones

`func (o *SiteAccessZonesResponse) SetZones(v []SiteAccessZone)`

SetZones sets Zones field to given value.

### HasZones

`func (o *SiteAccessZonesResponse) HasZones() bool`

HasZones returns a boolean if a field has been set.

### SetZonesNil

`func (o *SiteAccessZonesResponse) SetZonesNil(b bool)`

 SetZonesNil sets the value for Zones to be an explicit nil

### UnsetZones
`func (o *SiteAccessZonesResponse) UnsetZones()`

UnsetZones ensures that no value is present for Zones, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
