# InviteScheduleEntryInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | Pointer to **NullableString** |  | [optional]
**InviteRecurrence** | Pointer to **interface{}** |  | [optional]
**IsEnabled** | Pointer to **bool** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**ValidFrom** | **string** |  |
**ValidTo** | **string** |  |

## Methods

### NewInviteScheduleEntryInput

`func NewInviteScheduleEntryInput(validFrom string, validTo string, ) *InviteScheduleEntryInput`

NewInviteScheduleEntryInput instantiates a new InviteScheduleEntryInput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInviteScheduleEntryInputWithDefaults

`func NewInviteScheduleEntryInputWithDefaults() *InviteScheduleEntryInput`

NewInviteScheduleEntryInputWithDefaults instantiates a new InviteScheduleEntryInput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *InviteScheduleEntryInput) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *InviteScheduleEntryInput) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *InviteScheduleEntryInput) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *InviteScheduleEntryInput) HasId() bool`

HasId returns a boolean if a field has been set.

### SetIdNil

`func (o *InviteScheduleEntryInput) SetIdNil(b bool)`

 SetIdNil sets the value for Id to be an explicit nil

### UnsetId
`func (o *InviteScheduleEntryInput) UnsetId()`

UnsetId ensures that no value is present for Id, not even an explicit nil
### GetInviteRecurrence

`func (o *InviteScheduleEntryInput) GetInviteRecurrence() interface{}`

GetInviteRecurrence returns the InviteRecurrence field if non-nil, zero value otherwise.

### GetInviteRecurrenceOk

`func (o *InviteScheduleEntryInput) GetInviteRecurrenceOk() (*interface{}, bool)`

GetInviteRecurrenceOk returns a tuple with the InviteRecurrence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteRecurrence

`func (o *InviteScheduleEntryInput) SetInviteRecurrence(v interface{})`

SetInviteRecurrence sets InviteRecurrence field to given value.

### HasInviteRecurrence

`func (o *InviteScheduleEntryInput) HasInviteRecurrence() bool`

HasInviteRecurrence returns a boolean if a field has been set.

### SetInviteRecurrenceNil

`func (o *InviteScheduleEntryInput) SetInviteRecurrenceNil(b bool)`

 SetInviteRecurrenceNil sets the value for InviteRecurrence to be an explicit nil

### UnsetInviteRecurrence
`func (o *InviteScheduleEntryInput) UnsetInviteRecurrence()`

UnsetInviteRecurrence ensures that no value is present for InviteRecurrence, not even an explicit nil
### GetIsEnabled

`func (o *InviteScheduleEntryInput) GetIsEnabled() bool`

GetIsEnabled returns the IsEnabled field if non-nil, zero value otherwise.

### GetIsEnabledOk

`func (o *InviteScheduleEntryInput) GetIsEnabledOk() (*bool, bool)`

GetIsEnabledOk returns a tuple with the IsEnabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsEnabled

`func (o *InviteScheduleEntryInput) SetIsEnabled(v bool)`

SetIsEnabled sets IsEnabled field to given value.

### HasIsEnabled

`func (o *InviteScheduleEntryInput) HasIsEnabled() bool`

HasIsEnabled returns a boolean if a field has been set.

### GetName

`func (o *InviteScheduleEntryInput) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *InviteScheduleEntryInput) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *InviteScheduleEntryInput) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *InviteScheduleEntryInput) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *InviteScheduleEntryInput) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *InviteScheduleEntryInput) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetValidFrom

`func (o *InviteScheduleEntryInput) GetValidFrom() string`

GetValidFrom returns the ValidFrom field if non-nil, zero value otherwise.

### GetValidFromOk

`func (o *InviteScheduleEntryInput) GetValidFromOk() (*string, bool)`

GetValidFromOk returns a tuple with the ValidFrom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidFrom

`func (o *InviteScheduleEntryInput) SetValidFrom(v string)`

SetValidFrom sets ValidFrom field to given value.


### GetValidTo

`func (o *InviteScheduleEntryInput) GetValidTo() string`

GetValidTo returns the ValidTo field if non-nil, zero value otherwise.

### GetValidToOk

`func (o *InviteScheduleEntryInput) GetValidToOk() (*string, bool)`

GetValidToOk returns a tuple with the ValidTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidTo

`func (o *InviteScheduleEntryInput) SetValidTo(v string)`

SetValidTo sets ValidTo field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
