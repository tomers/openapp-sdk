# UpdateAccessInviteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DisabledJustification** | Pointer to **NullableString** |  | [optional]
**ExpiresIn** | Pointer to **NullableString** |  | [optional]
**InviteRecurrence** | Pointer to **interface{}** |  | [optional]
**InviteeMessage** | Pointer to **interface{}** |  | [optional]
**IsEnabled** | Pointer to **NullableBool** |  | [optional]
**MaxUses** | Pointer to **NullableInt32** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**PortalIds** | Pointer to **[]string** |  | [optional]
**Schedules** | Pointer to [**[]InviteScheduleEntryInput**](InviteScheduleEntryInput.md) | When set, replaces all schedule entries. Required when changing times/recurrence on an invite that already has multiple schedule entries. | [optional]
**ValidFrom** | Pointer to **NullableString** |  | [optional]
**ValidTo** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUpdateAccessInviteRequest

`func NewUpdateAccessInviteRequest() *UpdateAccessInviteRequest`

NewUpdateAccessInviteRequest instantiates a new UpdateAccessInviteRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateAccessInviteRequestWithDefaults

`func NewUpdateAccessInviteRequestWithDefaults() *UpdateAccessInviteRequest`

NewUpdateAccessInviteRequestWithDefaults instantiates a new UpdateAccessInviteRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDisabledJustification

`func (o *UpdateAccessInviteRequest) GetDisabledJustification() string`

GetDisabledJustification returns the DisabledJustification field if non-nil, zero value otherwise.

### GetDisabledJustificationOk

`func (o *UpdateAccessInviteRequest) GetDisabledJustificationOk() (*string, bool)`

GetDisabledJustificationOk returns a tuple with the DisabledJustification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisabledJustification

`func (o *UpdateAccessInviteRequest) SetDisabledJustification(v string)`

SetDisabledJustification sets DisabledJustification field to given value.

### HasDisabledJustification

`func (o *UpdateAccessInviteRequest) HasDisabledJustification() bool`

HasDisabledJustification returns a boolean if a field has been set.

### SetDisabledJustificationNil

`func (o *UpdateAccessInviteRequest) SetDisabledJustificationNil(b bool)`

 SetDisabledJustificationNil sets the value for DisabledJustification to be an explicit nil

### UnsetDisabledJustification
`func (o *UpdateAccessInviteRequest) UnsetDisabledJustification()`

UnsetDisabledJustification ensures that no value is present for DisabledJustification, not even an explicit nil
### GetExpiresIn

`func (o *UpdateAccessInviteRequest) GetExpiresIn() string`

GetExpiresIn returns the ExpiresIn field if non-nil, zero value otherwise.

### GetExpiresInOk

`func (o *UpdateAccessInviteRequest) GetExpiresInOk() (*string, bool)`

GetExpiresInOk returns a tuple with the ExpiresIn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresIn

`func (o *UpdateAccessInviteRequest) SetExpiresIn(v string)`

SetExpiresIn sets ExpiresIn field to given value.

### HasExpiresIn

`func (o *UpdateAccessInviteRequest) HasExpiresIn() bool`

HasExpiresIn returns a boolean if a field has been set.

### SetExpiresInNil

`func (o *UpdateAccessInviteRequest) SetExpiresInNil(b bool)`

 SetExpiresInNil sets the value for ExpiresIn to be an explicit nil

### UnsetExpiresIn
`func (o *UpdateAccessInviteRequest) UnsetExpiresIn()`

UnsetExpiresIn ensures that no value is present for ExpiresIn, not even an explicit nil
### GetInviteRecurrence

`func (o *UpdateAccessInviteRequest) GetInviteRecurrence() interface{}`

GetInviteRecurrence returns the InviteRecurrence field if non-nil, zero value otherwise.

### GetInviteRecurrenceOk

`func (o *UpdateAccessInviteRequest) GetInviteRecurrenceOk() (*interface{}, bool)`

GetInviteRecurrenceOk returns a tuple with the InviteRecurrence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteRecurrence

`func (o *UpdateAccessInviteRequest) SetInviteRecurrence(v interface{})`

SetInviteRecurrence sets InviteRecurrence field to given value.

### HasInviteRecurrence

`func (o *UpdateAccessInviteRequest) HasInviteRecurrence() bool`

HasInviteRecurrence returns a boolean if a field has been set.

### SetInviteRecurrenceNil

`func (o *UpdateAccessInviteRequest) SetInviteRecurrenceNil(b bool)`

 SetInviteRecurrenceNil sets the value for InviteRecurrence to be an explicit nil

### UnsetInviteRecurrence
`func (o *UpdateAccessInviteRequest) UnsetInviteRecurrence()`

UnsetInviteRecurrence ensures that no value is present for InviteRecurrence, not even an explicit nil
### GetInviteeMessage

`func (o *UpdateAccessInviteRequest) GetInviteeMessage() interface{}`

GetInviteeMessage returns the InviteeMessage field if non-nil, zero value otherwise.

### GetInviteeMessageOk

`func (o *UpdateAccessInviteRequest) GetInviteeMessageOk() (*interface{}, bool)`

GetInviteeMessageOk returns a tuple with the InviteeMessage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteeMessage

`func (o *UpdateAccessInviteRequest) SetInviteeMessage(v interface{})`

SetInviteeMessage sets InviteeMessage field to given value.

### HasInviteeMessage

`func (o *UpdateAccessInviteRequest) HasInviteeMessage() bool`

HasInviteeMessage returns a boolean if a field has been set.

### SetInviteeMessageNil

`func (o *UpdateAccessInviteRequest) SetInviteeMessageNil(b bool)`

 SetInviteeMessageNil sets the value for InviteeMessage to be an explicit nil

### UnsetInviteeMessage
`func (o *UpdateAccessInviteRequest) UnsetInviteeMessage()`

UnsetInviteeMessage ensures that no value is present for InviteeMessage, not even an explicit nil
### GetIsEnabled

`func (o *UpdateAccessInviteRequest) GetIsEnabled() bool`

GetIsEnabled returns the IsEnabled field if non-nil, zero value otherwise.

### GetIsEnabledOk

`func (o *UpdateAccessInviteRequest) GetIsEnabledOk() (*bool, bool)`

GetIsEnabledOk returns a tuple with the IsEnabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsEnabled

`func (o *UpdateAccessInviteRequest) SetIsEnabled(v bool)`

SetIsEnabled sets IsEnabled field to given value.

### HasIsEnabled

`func (o *UpdateAccessInviteRequest) HasIsEnabled() bool`

HasIsEnabled returns a boolean if a field has been set.

### SetIsEnabledNil

`func (o *UpdateAccessInviteRequest) SetIsEnabledNil(b bool)`

 SetIsEnabledNil sets the value for IsEnabled to be an explicit nil

### UnsetIsEnabled
`func (o *UpdateAccessInviteRequest) UnsetIsEnabled()`

UnsetIsEnabled ensures that no value is present for IsEnabled, not even an explicit nil
### GetMaxUses

`func (o *UpdateAccessInviteRequest) GetMaxUses() int32`

GetMaxUses returns the MaxUses field if non-nil, zero value otherwise.

### GetMaxUsesOk

`func (o *UpdateAccessInviteRequest) GetMaxUsesOk() (*int32, bool)`

GetMaxUsesOk returns a tuple with the MaxUses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxUses

`func (o *UpdateAccessInviteRequest) SetMaxUses(v int32)`

SetMaxUses sets MaxUses field to given value.

### HasMaxUses

`func (o *UpdateAccessInviteRequest) HasMaxUses() bool`

HasMaxUses returns a boolean if a field has been set.

### SetMaxUsesNil

`func (o *UpdateAccessInviteRequest) SetMaxUsesNil(b bool)`

 SetMaxUsesNil sets the value for MaxUses to be an explicit nil

### UnsetMaxUses
`func (o *UpdateAccessInviteRequest) UnsetMaxUses()`

UnsetMaxUses ensures that no value is present for MaxUses, not even an explicit nil
### GetName

`func (o *UpdateAccessInviteRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UpdateAccessInviteRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UpdateAccessInviteRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *UpdateAccessInviteRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *UpdateAccessInviteRequest) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *UpdateAccessInviteRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetPortalIds

`func (o *UpdateAccessInviteRequest) GetPortalIds() []string`

GetPortalIds returns the PortalIds field if non-nil, zero value otherwise.

### GetPortalIdsOk

`func (o *UpdateAccessInviteRequest) GetPortalIdsOk() (*[]string, bool)`

GetPortalIdsOk returns a tuple with the PortalIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPortalIds

`func (o *UpdateAccessInviteRequest) SetPortalIds(v []string)`

SetPortalIds sets PortalIds field to given value.

### HasPortalIds

`func (o *UpdateAccessInviteRequest) HasPortalIds() bool`

HasPortalIds returns a boolean if a field has been set.

### SetPortalIdsNil

`func (o *UpdateAccessInviteRequest) SetPortalIdsNil(b bool)`

 SetPortalIdsNil sets the value for PortalIds to be an explicit nil

### UnsetPortalIds
`func (o *UpdateAccessInviteRequest) UnsetPortalIds()`

UnsetPortalIds ensures that no value is present for PortalIds, not even an explicit nil
### GetSchedules

`func (o *UpdateAccessInviteRequest) GetSchedules() []InviteScheduleEntryInput`

GetSchedules returns the Schedules field if non-nil, zero value otherwise.

### GetSchedulesOk

`func (o *UpdateAccessInviteRequest) GetSchedulesOk() (*[]InviteScheduleEntryInput, bool)`

GetSchedulesOk returns a tuple with the Schedules field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchedules

`func (o *UpdateAccessInviteRequest) SetSchedules(v []InviteScheduleEntryInput)`

SetSchedules sets Schedules field to given value.

### HasSchedules

`func (o *UpdateAccessInviteRequest) HasSchedules() bool`

HasSchedules returns a boolean if a field has been set.

### SetSchedulesNil

`func (o *UpdateAccessInviteRequest) SetSchedulesNil(b bool)`

 SetSchedulesNil sets the value for Schedules to be an explicit nil

### UnsetSchedules
`func (o *UpdateAccessInviteRequest) UnsetSchedules()`

UnsetSchedules ensures that no value is present for Schedules, not even an explicit nil
### GetValidFrom

`func (o *UpdateAccessInviteRequest) GetValidFrom() string`

GetValidFrom returns the ValidFrom field if non-nil, zero value otherwise.

### GetValidFromOk

`func (o *UpdateAccessInviteRequest) GetValidFromOk() (*string, bool)`

GetValidFromOk returns a tuple with the ValidFrom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidFrom

`func (o *UpdateAccessInviteRequest) SetValidFrom(v string)`

SetValidFrom sets ValidFrom field to given value.

### HasValidFrom

`func (o *UpdateAccessInviteRequest) HasValidFrom() bool`

HasValidFrom returns a boolean if a field has been set.

### SetValidFromNil

`func (o *UpdateAccessInviteRequest) SetValidFromNil(b bool)`

 SetValidFromNil sets the value for ValidFrom to be an explicit nil

### UnsetValidFrom
`func (o *UpdateAccessInviteRequest) UnsetValidFrom()`

UnsetValidFrom ensures that no value is present for ValidFrom, not even an explicit nil
### GetValidTo

`func (o *UpdateAccessInviteRequest) GetValidTo() string`

GetValidTo returns the ValidTo field if non-nil, zero value otherwise.

### GetValidToOk

`func (o *UpdateAccessInviteRequest) GetValidToOk() (*string, bool)`

GetValidToOk returns a tuple with the ValidTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidTo

`func (o *UpdateAccessInviteRequest) SetValidTo(v string)`

SetValidTo sets ValidTo field to given value.

### HasValidTo

`func (o *UpdateAccessInviteRequest) HasValidTo() bool`

HasValidTo returns a boolean if a field has been set.

### SetValidToNil

`func (o *UpdateAccessInviteRequest) SetValidToNil(b bool)`

 SetValidToNil sets the value for ValidTo to be an explicit nil

### UnsetValidTo
`func (o *UpdateAccessInviteRequest) UnsetValidTo()`

UnsetValidTo ensures that no value is present for ValidTo, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
