# VirtualAccessOpenResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DoorAutoCloseDuration** | Pointer to **NullableInt64** |  | [optional]
**DoorOpenDurationSeconds** | Pointer to **NullableInt64** |  | [optional]
**LightsAutoOffDuration** | Pointer to **map[string]int64** | Map of light entity ULID to auto-off duration in seconds. | [optional]
**Ok** | **bool** |  |

## Methods

### NewVirtualAccessOpenResponse

`func NewVirtualAccessOpenResponse(ok bool, ) *VirtualAccessOpenResponse`

NewVirtualAccessOpenResponse instantiates a new VirtualAccessOpenResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewVirtualAccessOpenResponseWithDefaults

`func NewVirtualAccessOpenResponseWithDefaults() *VirtualAccessOpenResponse`

NewVirtualAccessOpenResponseWithDefaults instantiates a new VirtualAccessOpenResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDoorAutoCloseDuration

`func (o *VirtualAccessOpenResponse) GetDoorAutoCloseDuration() int64`

GetDoorAutoCloseDuration returns the DoorAutoCloseDuration field if non-nil, zero value otherwise.

### GetDoorAutoCloseDurationOk

`func (o *VirtualAccessOpenResponse) GetDoorAutoCloseDurationOk() (*int64, bool)`

GetDoorAutoCloseDurationOk returns a tuple with the DoorAutoCloseDuration field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorAutoCloseDuration

`func (o *VirtualAccessOpenResponse) SetDoorAutoCloseDuration(v int64)`

SetDoorAutoCloseDuration sets DoorAutoCloseDuration field to given value.

### HasDoorAutoCloseDuration

`func (o *VirtualAccessOpenResponse) HasDoorAutoCloseDuration() bool`

HasDoorAutoCloseDuration returns a boolean if a field has been set.

### SetDoorAutoCloseDurationNil

`func (o *VirtualAccessOpenResponse) SetDoorAutoCloseDurationNil(b bool)`

 SetDoorAutoCloseDurationNil sets the value for DoorAutoCloseDuration to be an explicit nil

### UnsetDoorAutoCloseDuration
`func (o *VirtualAccessOpenResponse) UnsetDoorAutoCloseDuration()`

UnsetDoorAutoCloseDuration ensures that no value is present for DoorAutoCloseDuration, not even an explicit nil
### GetDoorOpenDurationSeconds

`func (o *VirtualAccessOpenResponse) GetDoorOpenDurationSeconds() int64`

GetDoorOpenDurationSeconds returns the DoorOpenDurationSeconds field if non-nil, zero value otherwise.

### GetDoorOpenDurationSecondsOk

`func (o *VirtualAccessOpenResponse) GetDoorOpenDurationSecondsOk() (*int64, bool)`

GetDoorOpenDurationSecondsOk returns a tuple with the DoorOpenDurationSeconds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorOpenDurationSeconds

`func (o *VirtualAccessOpenResponse) SetDoorOpenDurationSeconds(v int64)`

SetDoorOpenDurationSeconds sets DoorOpenDurationSeconds field to given value.

### HasDoorOpenDurationSeconds

`func (o *VirtualAccessOpenResponse) HasDoorOpenDurationSeconds() bool`

HasDoorOpenDurationSeconds returns a boolean if a field has been set.

### SetDoorOpenDurationSecondsNil

`func (o *VirtualAccessOpenResponse) SetDoorOpenDurationSecondsNil(b bool)`

 SetDoorOpenDurationSecondsNil sets the value for DoorOpenDurationSeconds to be an explicit nil

### UnsetDoorOpenDurationSeconds
`func (o *VirtualAccessOpenResponse) UnsetDoorOpenDurationSeconds()`

UnsetDoorOpenDurationSeconds ensures that no value is present for DoorOpenDurationSeconds, not even an explicit nil
### GetLightsAutoOffDuration

`func (o *VirtualAccessOpenResponse) GetLightsAutoOffDuration() map[string]int64`

GetLightsAutoOffDuration returns the LightsAutoOffDuration field if non-nil, zero value otherwise.

### GetLightsAutoOffDurationOk

`func (o *VirtualAccessOpenResponse) GetLightsAutoOffDurationOk() (*map[string]int64, bool)`

GetLightsAutoOffDurationOk returns a tuple with the LightsAutoOffDuration field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLightsAutoOffDuration

`func (o *VirtualAccessOpenResponse) SetLightsAutoOffDuration(v map[string]int64)`

SetLightsAutoOffDuration sets LightsAutoOffDuration field to given value.

### HasLightsAutoOffDuration

`func (o *VirtualAccessOpenResponse) HasLightsAutoOffDuration() bool`

HasLightsAutoOffDuration returns a boolean if a field has been set.

### GetOk

`func (o *VirtualAccessOpenResponse) GetOk() bool`

GetOk returns the Ok field if non-nil, zero value otherwise.

### GetOkOk

`func (o *VirtualAccessOpenResponse) GetOkOk() (*bool, bool)`

GetOkOk returns a tuple with the Ok field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOk

`func (o *VirtualAccessOpenResponse) SetOk(v bool)`

SetOk sets Ok field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
