# InviteScheduleSnapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ActiveWindow** | Pointer to [**NullableInviteWindow**](InviteWindow.md) |  | [optional]
**IsRecurring** | **bool** |  |
**NextWindow** | Pointer to [**NullableInviteWindow**](InviteWindow.md) |  | [optional]
**SeriesEnd** | Pointer to [**NullableInviteRecurrenceSeriesEnd**](InviteRecurrenceSeriesEnd.md) |  | [optional]
**SlotDurationSeconds** | **int64** |  |

## Methods

### NewInviteScheduleSnapshot

`func NewInviteScheduleSnapshot(isRecurring bool, slotDurationSeconds int64, ) *InviteScheduleSnapshot`

NewInviteScheduleSnapshot instantiates a new InviteScheduleSnapshot object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInviteScheduleSnapshotWithDefaults

`func NewInviteScheduleSnapshotWithDefaults() *InviteScheduleSnapshot`

NewInviteScheduleSnapshotWithDefaults instantiates a new InviteScheduleSnapshot object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetActiveWindow

`func (o *InviteScheduleSnapshot) GetActiveWindow() InviteWindow`

GetActiveWindow returns the ActiveWindow field if non-nil, zero value otherwise.

### GetActiveWindowOk

`func (o *InviteScheduleSnapshot) GetActiveWindowOk() (*InviteWindow, bool)`

GetActiveWindowOk returns a tuple with the ActiveWindow field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActiveWindow

`func (o *InviteScheduleSnapshot) SetActiveWindow(v InviteWindow)`

SetActiveWindow sets ActiveWindow field to given value.

### HasActiveWindow

`func (o *InviteScheduleSnapshot) HasActiveWindow() bool`

HasActiveWindow returns a boolean if a field has been set.

### SetActiveWindowNil

`func (o *InviteScheduleSnapshot) SetActiveWindowNil(b bool)`

 SetActiveWindowNil sets the value for ActiveWindow to be an explicit nil

### UnsetActiveWindow
`func (o *InviteScheduleSnapshot) UnsetActiveWindow()`

UnsetActiveWindow ensures that no value is present for ActiveWindow, not even an explicit nil
### GetIsRecurring

`func (o *InviteScheduleSnapshot) GetIsRecurring() bool`

GetIsRecurring returns the IsRecurring field if non-nil, zero value otherwise.

### GetIsRecurringOk

`func (o *InviteScheduleSnapshot) GetIsRecurringOk() (*bool, bool)`

GetIsRecurringOk returns a tuple with the IsRecurring field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsRecurring

`func (o *InviteScheduleSnapshot) SetIsRecurring(v bool)`

SetIsRecurring sets IsRecurring field to given value.


### GetNextWindow

`func (o *InviteScheduleSnapshot) GetNextWindow() InviteWindow`

GetNextWindow returns the NextWindow field if non-nil, zero value otherwise.

### GetNextWindowOk

`func (o *InviteScheduleSnapshot) GetNextWindowOk() (*InviteWindow, bool)`

GetNextWindowOk returns a tuple with the NextWindow field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextWindow

`func (o *InviteScheduleSnapshot) SetNextWindow(v InviteWindow)`

SetNextWindow sets NextWindow field to given value.

### HasNextWindow

`func (o *InviteScheduleSnapshot) HasNextWindow() bool`

HasNextWindow returns a boolean if a field has been set.

### SetNextWindowNil

`func (o *InviteScheduleSnapshot) SetNextWindowNil(b bool)`

 SetNextWindowNil sets the value for NextWindow to be an explicit nil

### UnsetNextWindow
`func (o *InviteScheduleSnapshot) UnsetNextWindow()`

UnsetNextWindow ensures that no value is present for NextWindow, not even an explicit nil
### GetSeriesEnd

`func (o *InviteScheduleSnapshot) GetSeriesEnd() InviteRecurrenceSeriesEnd`

GetSeriesEnd returns the SeriesEnd field if non-nil, zero value otherwise.

### GetSeriesEndOk

`func (o *InviteScheduleSnapshot) GetSeriesEndOk() (*InviteRecurrenceSeriesEnd, bool)`

GetSeriesEndOk returns a tuple with the SeriesEnd field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSeriesEnd

`func (o *InviteScheduleSnapshot) SetSeriesEnd(v InviteRecurrenceSeriesEnd)`

SetSeriesEnd sets SeriesEnd field to given value.

### HasSeriesEnd

`func (o *InviteScheduleSnapshot) HasSeriesEnd() bool`

HasSeriesEnd returns a boolean if a field has been set.

### SetSeriesEndNil

`func (o *InviteScheduleSnapshot) SetSeriesEndNil(b bool)`

 SetSeriesEndNil sets the value for SeriesEnd to be an explicit nil

### UnsetSeriesEnd
`func (o *InviteScheduleSnapshot) UnsetSeriesEnd()`

UnsetSeriesEnd ensures that no value is present for SeriesEnd, not even an explicit nil
### GetSlotDurationSeconds

`func (o *InviteScheduleSnapshot) GetSlotDurationSeconds() int64`

GetSlotDurationSeconds returns the SlotDurationSeconds field if non-nil, zero value otherwise.

### GetSlotDurationSecondsOk

`func (o *InviteScheduleSnapshot) GetSlotDurationSecondsOk() (*int64, bool)`

GetSlotDurationSecondsOk returns a tuple with the SlotDurationSeconds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSlotDurationSeconds

`func (o *InviteScheduleSnapshot) SetSlotDurationSeconds(v int64)`

SetSlotDurationSeconds sets SlotDurationSeconds field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
