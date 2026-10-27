# MergeZonesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**MovedChildZones** | **int64** |  |
**MovedDevices** | **int64** |  |
**MovedEntities** | **int64** |  |
**MovedProviderLinks** | **int64** |  |
**Zone** | [**ZoneResponse**](ZoneResponse.md) |  |

## Methods

### NewMergeZonesResponse

`func NewMergeZonesResponse(movedChildZones int64, movedDevices int64, movedEntities int64, movedProviderLinks int64, zone ZoneResponse, ) *MergeZonesResponse`

NewMergeZonesResponse instantiates a new MergeZonesResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMergeZonesResponseWithDefaults

`func NewMergeZonesResponseWithDefaults() *MergeZonesResponse`

NewMergeZonesResponseWithDefaults instantiates a new MergeZonesResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMovedChildZones

`func (o *MergeZonesResponse) GetMovedChildZones() int64`

GetMovedChildZones returns the MovedChildZones field if non-nil, zero value otherwise.

### GetMovedChildZonesOk

`func (o *MergeZonesResponse) GetMovedChildZonesOk() (*int64, bool)`

GetMovedChildZonesOk returns a tuple with the MovedChildZones field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMovedChildZones

`func (o *MergeZonesResponse) SetMovedChildZones(v int64)`

SetMovedChildZones sets MovedChildZones field to given value.


### GetMovedDevices

`func (o *MergeZonesResponse) GetMovedDevices() int64`

GetMovedDevices returns the MovedDevices field if non-nil, zero value otherwise.

### GetMovedDevicesOk

`func (o *MergeZonesResponse) GetMovedDevicesOk() (*int64, bool)`

GetMovedDevicesOk returns a tuple with the MovedDevices field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMovedDevices

`func (o *MergeZonesResponse) SetMovedDevices(v int64)`

SetMovedDevices sets MovedDevices field to given value.


### GetMovedEntities

`func (o *MergeZonesResponse) GetMovedEntities() int64`

GetMovedEntities returns the MovedEntities field if non-nil, zero value otherwise.

### GetMovedEntitiesOk

`func (o *MergeZonesResponse) GetMovedEntitiesOk() (*int64, bool)`

GetMovedEntitiesOk returns a tuple with the MovedEntities field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMovedEntities

`func (o *MergeZonesResponse) SetMovedEntities(v int64)`

SetMovedEntities sets MovedEntities field to given value.


### GetMovedProviderLinks

`func (o *MergeZonesResponse) GetMovedProviderLinks() int64`

GetMovedProviderLinks returns the MovedProviderLinks field if non-nil, zero value otherwise.

### GetMovedProviderLinksOk

`func (o *MergeZonesResponse) GetMovedProviderLinksOk() (*int64, bool)`

GetMovedProviderLinksOk returns a tuple with the MovedProviderLinks field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMovedProviderLinks

`func (o *MergeZonesResponse) SetMovedProviderLinks(v int64)`

SetMovedProviderLinks sets MovedProviderLinks field to given value.


### GetZone

`func (o *MergeZonesResponse) GetZone() ZoneResponse`

GetZone returns the Zone field if non-nil, zero value otherwise.

### GetZoneOk

`func (o *MergeZonesResponse) GetZoneOk() (*ZoneResponse, bool)`

GetZoneOk returns a tuple with the Zone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZone

`func (o *MergeZonesResponse) SetZone(v ZoneResponse)`

SetZone sets Zone field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
