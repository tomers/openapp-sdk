# InviteScheduleEntrySnapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **string** |  |
**InviteRecurrence** | Pointer to **interface{}** |  | [optional]
**IsEnabled** | **bool** |  |
**Name** | Pointer to **NullableString** |  | [optional]
**Schedule** | [**InviteScheduleSnapshot**](InviteScheduleSnapshot.md) |  |
**ValidFrom** | **string** |  |
**ValidTo** | **string** |  |

## Methods

### NewInviteScheduleEntrySnapshot

`func NewInviteScheduleEntrySnapshot(id string, isEnabled bool, schedule InviteScheduleSnapshot, validFrom string, validTo string, ) *InviteScheduleEntrySnapshot`

NewInviteScheduleEntrySnapshot instantiates a new InviteScheduleEntrySnapshot object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInviteScheduleEntrySnapshotWithDefaults

`func NewInviteScheduleEntrySnapshotWithDefaults() *InviteScheduleEntrySnapshot`

NewInviteScheduleEntrySnapshotWithDefaults instantiates a new InviteScheduleEntrySnapshot object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *InviteScheduleEntrySnapshot) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *InviteScheduleEntrySnapshot) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *InviteScheduleEntrySnapshot) SetId(v string)`

SetId sets Id field to given value.


### GetInviteRecurrence

`func (o *InviteScheduleEntrySnapshot) GetInviteRecurrence() interface{}`

GetInviteRecurrence returns the InviteRecurrence field if non-nil, zero value otherwise.

### GetInviteRecurrenceOk

`func (o *InviteScheduleEntrySnapshot) GetInviteRecurrenceOk() (*interface{}, bool)`

GetInviteRecurrenceOk returns a tuple with the InviteRecurrence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteRecurrence

`func (o *InviteScheduleEntrySnapshot) SetInviteRecurrence(v interface{})`

SetInviteRecurrence sets InviteRecurrence field to given value.

### HasInviteRecurrence

`func (o *InviteScheduleEntrySnapshot) HasInviteRecurrence() bool`

HasInviteRecurrence returns a boolean if a field has been set.

### SetInviteRecurrenceNil

`func (o *InviteScheduleEntrySnapshot) SetInviteRecurrenceNil(b bool)`

 SetInviteRecurrenceNil sets the value for InviteRecurrence to be an explicit nil

### UnsetInviteRecurrence
`func (o *InviteScheduleEntrySnapshot) UnsetInviteRecurrence()`

UnsetInviteRecurrence ensures that no value is present for InviteRecurrence, not even an explicit nil
### GetIsEnabled

`func (o *InviteScheduleEntrySnapshot) GetIsEnabled() bool`

GetIsEnabled returns the IsEnabled field if non-nil, zero value otherwise.

### GetIsEnabledOk

`func (o *InviteScheduleEntrySnapshot) GetIsEnabledOk() (*bool, bool)`

GetIsEnabledOk returns a tuple with the IsEnabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsEnabled

`func (o *InviteScheduleEntrySnapshot) SetIsEnabled(v bool)`

SetIsEnabled sets IsEnabled field to given value.


### GetName

`func (o *InviteScheduleEntrySnapshot) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *InviteScheduleEntrySnapshot) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *InviteScheduleEntrySnapshot) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *InviteScheduleEntrySnapshot) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *InviteScheduleEntrySnapshot) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *InviteScheduleEntrySnapshot) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetSchedule

`func (o *InviteScheduleEntrySnapshot) GetSchedule() InviteScheduleSnapshot`

GetSchedule returns the Schedule field if non-nil, zero value otherwise.

### GetScheduleOk

`func (o *InviteScheduleEntrySnapshot) GetScheduleOk() (*InviteScheduleSnapshot, bool)`

GetScheduleOk returns a tuple with the Schedule field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchedule

`func (o *InviteScheduleEntrySnapshot) SetSchedule(v InviteScheduleSnapshot)`

SetSchedule sets Schedule field to given value.


### GetValidFrom

`func (o *InviteScheduleEntrySnapshot) GetValidFrom() string`

GetValidFrom returns the ValidFrom field if non-nil, zero value otherwise.

### GetValidFromOk

`func (o *InviteScheduleEntrySnapshot) GetValidFromOk() (*string, bool)`

GetValidFromOk returns a tuple with the ValidFrom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidFrom

`func (o *InviteScheduleEntrySnapshot) SetValidFrom(v string)`

SetValidFrom sets ValidFrom field to given value.


### GetValidTo

`func (o *InviteScheduleEntrySnapshot) GetValidTo() string`

GetValidTo returns the ValidTo field if non-nil, zero value otherwise.

### GetValidToOk

`func (o *InviteScheduleEntrySnapshot) GetValidToOk() (*string, bool)`

GetValidToOk returns a tuple with the ValidTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidTo

`func (o *InviteScheduleEntrySnapshot) SetValidTo(v string)`

SetValidTo sets ValidTo field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
