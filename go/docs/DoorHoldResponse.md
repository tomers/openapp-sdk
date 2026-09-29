# DoorHoldResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Hold** | Pointer to [**NullableHoldView**](HoldView.md) |  | [optional]
**IncludeLights** | **bool** |  |
**Lights** | [**[]EntityHoldItem**](EntityHoldItem.md) |  |
**Mixed** | **bool** |  |
**Mode** | Pointer to [**NullableHoldMode**](HoldMode.md) |  | [optional]
**Openers** | [**[]EntityHoldItem**](EntityHoldItem.md) |  |
**Timezone** | **string** | Organization IANA timezone used when authoring a new recurring hold. |

## Methods

### NewDoorHoldResponse

`func NewDoorHoldResponse(includeLights bool, lights []EntityHoldItem, mixed bool, openers []EntityHoldItem, timezone string, ) *DoorHoldResponse`

NewDoorHoldResponse instantiates a new DoorHoldResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDoorHoldResponseWithDefaults

`func NewDoorHoldResponseWithDefaults() *DoorHoldResponse`

NewDoorHoldResponseWithDefaults instantiates a new DoorHoldResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetHold

`func (o *DoorHoldResponse) GetHold() HoldView`

GetHold returns the Hold field if non-nil, zero value otherwise.

### GetHoldOk

`func (o *DoorHoldResponse) GetHoldOk() (*HoldView, bool)`

GetHoldOk returns a tuple with the Hold field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHold

`func (o *DoorHoldResponse) SetHold(v HoldView)`

SetHold sets Hold field to given value.

### HasHold

`func (o *DoorHoldResponse) HasHold() bool`

HasHold returns a boolean if a field has been set.

### SetHoldNil

`func (o *DoorHoldResponse) SetHoldNil(b bool)`

 SetHoldNil sets the value for Hold to be an explicit nil

### UnsetHold
`func (o *DoorHoldResponse) UnsetHold()`

UnsetHold ensures that no value is present for Hold, not even an explicit nil
### GetIncludeLights

`func (o *DoorHoldResponse) GetIncludeLights() bool`

GetIncludeLights returns the IncludeLights field if non-nil, zero value otherwise.

### GetIncludeLightsOk

`func (o *DoorHoldResponse) GetIncludeLightsOk() (*bool, bool)`

GetIncludeLightsOk returns a tuple with the IncludeLights field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeLights

`func (o *DoorHoldResponse) SetIncludeLights(v bool)`

SetIncludeLights sets IncludeLights field to given value.


### GetLights

`func (o *DoorHoldResponse) GetLights() []EntityHoldItem`

GetLights returns the Lights field if non-nil, zero value otherwise.

### GetLightsOk

`func (o *DoorHoldResponse) GetLightsOk() (*[]EntityHoldItem, bool)`

GetLightsOk returns a tuple with the Lights field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLights

`func (o *DoorHoldResponse) SetLights(v []EntityHoldItem)`

SetLights sets Lights field to given value.


### GetMixed

`func (o *DoorHoldResponse) GetMixed() bool`

GetMixed returns the Mixed field if non-nil, zero value otherwise.

### GetMixedOk

`func (o *DoorHoldResponse) GetMixedOk() (*bool, bool)`

GetMixedOk returns a tuple with the Mixed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMixed

`func (o *DoorHoldResponse) SetMixed(v bool)`

SetMixed sets Mixed field to given value.


### GetMode

`func (o *DoorHoldResponse) GetMode() HoldMode`

GetMode returns the Mode field if non-nil, zero value otherwise.

### GetModeOk

`func (o *DoorHoldResponse) GetModeOk() (*HoldMode, bool)`

GetModeOk returns a tuple with the Mode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMode

`func (o *DoorHoldResponse) SetMode(v HoldMode)`

SetMode sets Mode field to given value.

### HasMode

`func (o *DoorHoldResponse) HasMode() bool`

HasMode returns a boolean if a field has been set.

### SetModeNil

`func (o *DoorHoldResponse) SetModeNil(b bool)`

 SetModeNil sets the value for Mode to be an explicit nil

### UnsetMode
`func (o *DoorHoldResponse) UnsetMode()`

UnsetMode ensures that no value is present for Mode, not even an explicit nil
### GetOpeners

`func (o *DoorHoldResponse) GetOpeners() []EntityHoldItem`

GetOpeners returns the Openers field if non-nil, zero value otherwise.

### GetOpenersOk

`func (o *DoorHoldResponse) GetOpenersOk() (*[]EntityHoldItem, bool)`

GetOpenersOk returns a tuple with the Openers field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpeners

`func (o *DoorHoldResponse) SetOpeners(v []EntityHoldItem)`

SetOpeners sets Openers field to given value.


### GetTimezone

`func (o *DoorHoldResponse) GetTimezone() string`

GetTimezone returns the Timezone field if non-nil, zero value otherwise.

### GetTimezoneOk

`func (o *DoorHoldResponse) GetTimezoneOk() (*string, bool)`

GetTimezoneOk returns a tuple with the Timezone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimezone

`func (o *DoorHoldResponse) SetTimezone(v string)`

SetTimezone sets Timezone field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
