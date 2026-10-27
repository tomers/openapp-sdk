# SiteAccessWifiCover

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ViaZoneId** | Pointer to **NullableString** | The ancestor zone the network is linked to. Absent when the network is linked to &#x60;zone_id&#x60; itself. | [optional]
**ViaZoneName** | Pointer to [**NullableLocalizedString**](LocalizedString.md) | Name of &#x60;via_zone_id&#x60;. | [optional]
**ZoneId** | **string** | A site zone (one that holds doors) the network applies to. |

## Methods

### NewSiteAccessWifiCover

`func NewSiteAccessWifiCover(zoneId string, ) *SiteAccessWifiCover`

NewSiteAccessWifiCover instantiates a new SiteAccessWifiCover object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSiteAccessWifiCoverWithDefaults

`func NewSiteAccessWifiCoverWithDefaults() *SiteAccessWifiCover`

NewSiteAccessWifiCoverWithDefaults instantiates a new SiteAccessWifiCover object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetViaZoneId

`func (o *SiteAccessWifiCover) GetViaZoneId() string`

GetViaZoneId returns the ViaZoneId field if non-nil, zero value otherwise.

### GetViaZoneIdOk

`func (o *SiteAccessWifiCover) GetViaZoneIdOk() (*string, bool)`

GetViaZoneIdOk returns a tuple with the ViaZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetViaZoneId

`func (o *SiteAccessWifiCover) SetViaZoneId(v string)`

SetViaZoneId sets ViaZoneId field to given value.

### HasViaZoneId

`func (o *SiteAccessWifiCover) HasViaZoneId() bool`

HasViaZoneId returns a boolean if a field has been set.

### SetViaZoneIdNil

`func (o *SiteAccessWifiCover) SetViaZoneIdNil(b bool)`

 SetViaZoneIdNil sets the value for ViaZoneId to be an explicit nil

### UnsetViaZoneId
`func (o *SiteAccessWifiCover) UnsetViaZoneId()`

UnsetViaZoneId ensures that no value is present for ViaZoneId, not even an explicit nil
### GetViaZoneName

`func (o *SiteAccessWifiCover) GetViaZoneName() LocalizedString`

GetViaZoneName returns the ViaZoneName field if non-nil, zero value otherwise.

### GetViaZoneNameOk

`func (o *SiteAccessWifiCover) GetViaZoneNameOk() (*LocalizedString, bool)`

GetViaZoneNameOk returns a tuple with the ViaZoneName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetViaZoneName

`func (o *SiteAccessWifiCover) SetViaZoneName(v LocalizedString)`

SetViaZoneName sets ViaZoneName field to given value.

### HasViaZoneName

`func (o *SiteAccessWifiCover) HasViaZoneName() bool`

HasViaZoneName returns a boolean if a field has been set.

### SetViaZoneNameNil

`func (o *SiteAccessWifiCover) SetViaZoneNameNil(b bool)`

 SetViaZoneNameNil sets the value for ViaZoneName to be an explicit nil

### UnsetViaZoneName
`func (o *SiteAccessWifiCover) UnsetViaZoneName()`

UnsetViaZoneName ensures that no value is present for ViaZoneName, not even an explicit nil
### GetZoneId

`func (o *SiteAccessWifiCover) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *SiteAccessWifiCover) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *SiteAccessWifiCover) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
