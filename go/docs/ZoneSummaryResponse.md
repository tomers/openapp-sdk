# ZoneSummaryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Children** | [**[]Zone**](Zone.md) |  |
**DescendantEntityCount** | **int64** |  |
**DeviceCount** | **int64** |  |
**EntityCount** | **int64** |  |
**Path** | [**[]ZonePathItem**](ZonePathItem.md) | Ancestors root first, including the zone itself. |
**Zone** | [**ZoneResponse**](ZoneResponse.md) |  |

## Methods

### NewZoneSummaryResponse

`func NewZoneSummaryResponse(children []Zone, descendantEntityCount int64, deviceCount int64, entityCount int64, path []ZonePathItem, zone ZoneResponse, ) *ZoneSummaryResponse`

NewZoneSummaryResponse instantiates a new ZoneSummaryResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewZoneSummaryResponseWithDefaults

`func NewZoneSummaryResponseWithDefaults() *ZoneSummaryResponse`

NewZoneSummaryResponseWithDefaults instantiates a new ZoneSummaryResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetChildren

`func (o *ZoneSummaryResponse) GetChildren() []Zone`

GetChildren returns the Children field if non-nil, zero value otherwise.

### GetChildrenOk

`func (o *ZoneSummaryResponse) GetChildrenOk() (*[]Zone, bool)`

GetChildrenOk returns a tuple with the Children field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChildren

`func (o *ZoneSummaryResponse) SetChildren(v []Zone)`

SetChildren sets Children field to given value.


### GetDescendantEntityCount

`func (o *ZoneSummaryResponse) GetDescendantEntityCount() int64`

GetDescendantEntityCount returns the DescendantEntityCount field if non-nil, zero value otherwise.

### GetDescendantEntityCountOk

`func (o *ZoneSummaryResponse) GetDescendantEntityCountOk() (*int64, bool)`

GetDescendantEntityCountOk returns a tuple with the DescendantEntityCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDescendantEntityCount

`func (o *ZoneSummaryResponse) SetDescendantEntityCount(v int64)`

SetDescendantEntityCount sets DescendantEntityCount field to given value.


### GetDeviceCount

`func (o *ZoneSummaryResponse) GetDeviceCount() int64`

GetDeviceCount returns the DeviceCount field if non-nil, zero value otherwise.

### GetDeviceCountOk

`func (o *ZoneSummaryResponse) GetDeviceCountOk() (*int64, bool)`

GetDeviceCountOk returns a tuple with the DeviceCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceCount

`func (o *ZoneSummaryResponse) SetDeviceCount(v int64)`

SetDeviceCount sets DeviceCount field to given value.


### GetEntityCount

`func (o *ZoneSummaryResponse) GetEntityCount() int64`

GetEntityCount returns the EntityCount field if non-nil, zero value otherwise.

### GetEntityCountOk

`func (o *ZoneSummaryResponse) GetEntityCountOk() (*int64, bool)`

GetEntityCountOk returns a tuple with the EntityCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityCount

`func (o *ZoneSummaryResponse) SetEntityCount(v int64)`

SetEntityCount sets EntityCount field to given value.


### GetPath

`func (o *ZoneSummaryResponse) GetPath() []ZonePathItem`

GetPath returns the Path field if non-nil, zero value otherwise.

### GetPathOk

`func (o *ZoneSummaryResponse) GetPathOk() (*[]ZonePathItem, bool)`

GetPathOk returns a tuple with the Path field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPath

`func (o *ZoneSummaryResponse) SetPath(v []ZonePathItem)`

SetPath sets Path field to given value.


### GetZone

`func (o *ZoneSummaryResponse) GetZone() ZoneResponse`

GetZone returns the Zone field if non-nil, zero value otherwise.

### GetZoneOk

`func (o *ZoneSummaryResponse) GetZoneOk() (*ZoneResponse, bool)`

GetZoneOk returns a tuple with the Zone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZone

`func (o *ZoneSummaryResponse) SetZone(v ZoneResponse)`

SetZone sets Zone field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
