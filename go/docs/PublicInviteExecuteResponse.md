# PublicInviteExecuteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DoorAutoCloseDuration** | Pointer to **NullableInt64** |  | [optional]
**LightsAutoOffDuration** | Pointer to **map[string]int64** | Map of light entity ULID to auto-off duration in seconds. Serializes as JSON object with string keys. | [optional]
**Message** | Pointer to **NullableString** |  | [optional]
**Ok** | **bool** |  |

## Methods

### NewPublicInviteExecuteResponse

`func NewPublicInviteExecuteResponse(ok bool, ) *PublicInviteExecuteResponse`

NewPublicInviteExecuteResponse instantiates a new PublicInviteExecuteResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicInviteExecuteResponseWithDefaults

`func NewPublicInviteExecuteResponseWithDefaults() *PublicInviteExecuteResponse`

NewPublicInviteExecuteResponseWithDefaults instantiates a new PublicInviteExecuteResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDoorAutoCloseDuration

`func (o *PublicInviteExecuteResponse) GetDoorAutoCloseDuration() int64`

GetDoorAutoCloseDuration returns the DoorAutoCloseDuration field if non-nil, zero value otherwise.

### GetDoorAutoCloseDurationOk

`func (o *PublicInviteExecuteResponse) GetDoorAutoCloseDurationOk() (*int64, bool)`

GetDoorAutoCloseDurationOk returns a tuple with the DoorAutoCloseDuration field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorAutoCloseDuration

`func (o *PublicInviteExecuteResponse) SetDoorAutoCloseDuration(v int64)`

SetDoorAutoCloseDuration sets DoorAutoCloseDuration field to given value.

### HasDoorAutoCloseDuration

`func (o *PublicInviteExecuteResponse) HasDoorAutoCloseDuration() bool`

HasDoorAutoCloseDuration returns a boolean if a field has been set.

### SetDoorAutoCloseDurationNil

`func (o *PublicInviteExecuteResponse) SetDoorAutoCloseDurationNil(b bool)`

 SetDoorAutoCloseDurationNil sets the value for DoorAutoCloseDuration to be an explicit nil

### UnsetDoorAutoCloseDuration
`func (o *PublicInviteExecuteResponse) UnsetDoorAutoCloseDuration()`

UnsetDoorAutoCloseDuration ensures that no value is present for DoorAutoCloseDuration, not even an explicit nil
### GetLightsAutoOffDuration

`func (o *PublicInviteExecuteResponse) GetLightsAutoOffDuration() map[string]int64`

GetLightsAutoOffDuration returns the LightsAutoOffDuration field if non-nil, zero value otherwise.

### GetLightsAutoOffDurationOk

`func (o *PublicInviteExecuteResponse) GetLightsAutoOffDurationOk() (*map[string]int64, bool)`

GetLightsAutoOffDurationOk returns a tuple with the LightsAutoOffDuration field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLightsAutoOffDuration

`func (o *PublicInviteExecuteResponse) SetLightsAutoOffDuration(v map[string]int64)`

SetLightsAutoOffDuration sets LightsAutoOffDuration field to given value.

### HasLightsAutoOffDuration

`func (o *PublicInviteExecuteResponse) HasLightsAutoOffDuration() bool`

HasLightsAutoOffDuration returns a boolean if a field has been set.

### GetMessage

`func (o *PublicInviteExecuteResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *PublicInviteExecuteResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *PublicInviteExecuteResponse) SetMessage(v string)`

SetMessage sets Message field to given value.

### HasMessage

`func (o *PublicInviteExecuteResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### SetMessageNil

`func (o *PublicInviteExecuteResponse) SetMessageNil(b bool)`

 SetMessageNil sets the value for Message to be an explicit nil

### UnsetMessage
`func (o *PublicInviteExecuteResponse) UnsetMessage()`

UnsetMessage ensures that no value is present for Message, not even an explicit nil
### GetOk

`func (o *PublicInviteExecuteResponse) GetOk() bool`

GetOk returns the Ok field if non-nil, zero value otherwise.

### GetOkOk

`func (o *PublicInviteExecuteResponse) GetOkOk() (*bool, bool)`

GetOkOk returns a tuple with the Ok field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOk

`func (o *PublicInviteExecuteResponse) SetOk(v bool)`

SetOk sets Ok field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
