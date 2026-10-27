# SetWifiNetworkZonesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ZoneIds** | **[]string** | The complete set of zones the network covers. Each must be a live zone of the network&#39;s org. |

## Methods

### NewSetWifiNetworkZonesRequest

`func NewSetWifiNetworkZonesRequest(zoneIds []string, ) *SetWifiNetworkZonesRequest`

NewSetWifiNetworkZonesRequest instantiates a new SetWifiNetworkZonesRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSetWifiNetworkZonesRequestWithDefaults

`func NewSetWifiNetworkZonesRequestWithDefaults() *SetWifiNetworkZonesRequest`

NewSetWifiNetworkZonesRequestWithDefaults instantiates a new SetWifiNetworkZonesRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetZoneIds

`func (o *SetWifiNetworkZonesRequest) GetZoneIds() []string`

GetZoneIds returns the ZoneIds field if non-nil, zero value otherwise.

### GetZoneIdsOk

`func (o *SetWifiNetworkZonesRequest) GetZoneIdsOk() (*[]string, bool)`

GetZoneIdsOk returns a tuple with the ZoneIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneIds

`func (o *SetWifiNetworkZonesRequest) SetZoneIds(v []string)`

SetZoneIds sets ZoneIds field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
