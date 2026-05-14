# CreateAccessInviteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExpiresIn** | Pointer to **NullableString** |  | [optional]
**InviteRecurrence** | Pointer to **interface{}** |  | [optional]
**InviteeMessage** | Pointer to **interface{}** |  | [optional]
**IsEnabled** | Pointer to **NullableBool** |  | [optional]
**MaxUses** | Pointer to **NullableInt32** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**PortalIds** | **[]string** |  |
**Schedules** | Pointer to [**[]InviteScheduleEntryInput**](InviteScheduleEntryInput.md) | When set (non-empty), defines one or more schedule entries; legacy &#x60;valid_from&#x60; / &#x60;valid_to&#x60; / &#x60;invite_recurrence&#x60; are ignored for scheduling. | [optional]
**ValidFrom** | Pointer to **NullableString** |  | [optional]
**ValidTo** | Pointer to **NullableString** |  | [optional]

## Methods

### NewCreateAccessInviteRequest

`func NewCreateAccessInviteRequest(portalIds []string, ) *CreateAccessInviteRequest`

NewCreateAccessInviteRequest instantiates a new CreateAccessInviteRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateAccessInviteRequestWithDefaults

`func NewCreateAccessInviteRequestWithDefaults() *CreateAccessInviteRequest`

NewCreateAccessInviteRequestWithDefaults instantiates a new CreateAccessInviteRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExpiresIn

`func (o *CreateAccessInviteRequest) GetExpiresIn() string`

GetExpiresIn returns the ExpiresIn field if non-nil, zero value otherwise.

### GetExpiresInOk

`func (o *CreateAccessInviteRequest) GetExpiresInOk() (*string, bool)`

GetExpiresInOk returns a tuple with the ExpiresIn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresIn

`func (o *CreateAccessInviteRequest) SetExpiresIn(v string)`

SetExpiresIn sets ExpiresIn field to given value.

### HasExpiresIn

`func (o *CreateAccessInviteRequest) HasExpiresIn() bool`

HasExpiresIn returns a boolean if a field has been set.

### SetExpiresInNil

`func (o *CreateAccessInviteRequest) SetExpiresInNil(b bool)`

 SetExpiresInNil sets the value for ExpiresIn to be an explicit nil

### UnsetExpiresIn
`func (o *CreateAccessInviteRequest) UnsetExpiresIn()`

UnsetExpiresIn ensures that no value is present for ExpiresIn, not even an explicit nil
### GetInviteRecurrence

`func (o *CreateAccessInviteRequest) GetInviteRecurrence() interface{}`

GetInviteRecurrence returns the InviteRecurrence field if non-nil, zero value otherwise.

### GetInviteRecurrenceOk

`func (o *CreateAccessInviteRequest) GetInviteRecurrenceOk() (*interface{}, bool)`

GetInviteRecurrenceOk returns a tuple with the InviteRecurrence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteRecurrence

`func (o *CreateAccessInviteRequest) SetInviteRecurrence(v interface{})`

SetInviteRecurrence sets InviteRecurrence field to given value.

### HasInviteRecurrence

`func (o *CreateAccessInviteRequest) HasInviteRecurrence() bool`

HasInviteRecurrence returns a boolean if a field has been set.

### SetInviteRecurrenceNil

`func (o *CreateAccessInviteRequest) SetInviteRecurrenceNil(b bool)`

 SetInviteRecurrenceNil sets the value for InviteRecurrence to be an explicit nil

### UnsetInviteRecurrence
`func (o *CreateAccessInviteRequest) UnsetInviteRecurrence()`

UnsetInviteRecurrence ensures that no value is present for InviteRecurrence, not even an explicit nil
### GetInviteeMessage

`func (o *CreateAccessInviteRequest) GetInviteeMessage() interface{}`

GetInviteeMessage returns the InviteeMessage field if non-nil, zero value otherwise.

### GetInviteeMessageOk

`func (o *CreateAccessInviteRequest) GetInviteeMessageOk() (*interface{}, bool)`

GetInviteeMessageOk returns a tuple with the InviteeMessage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteeMessage

`func (o *CreateAccessInviteRequest) SetInviteeMessage(v interface{})`

SetInviteeMessage sets InviteeMessage field to given value.

### HasInviteeMessage

`func (o *CreateAccessInviteRequest) HasInviteeMessage() bool`

HasInviteeMessage returns a boolean if a field has been set.

### SetInviteeMessageNil

`func (o *CreateAccessInviteRequest) SetInviteeMessageNil(b bool)`

 SetInviteeMessageNil sets the value for InviteeMessage to be an explicit nil

### UnsetInviteeMessage
`func (o *CreateAccessInviteRequest) UnsetInviteeMessage()`

UnsetInviteeMessage ensures that no value is present for InviteeMessage, not even an explicit nil
### GetIsEnabled

`func (o *CreateAccessInviteRequest) GetIsEnabled() bool`

GetIsEnabled returns the IsEnabled field if non-nil, zero value otherwise.

### GetIsEnabledOk

`func (o *CreateAccessInviteRequest) GetIsEnabledOk() (*bool, bool)`

GetIsEnabledOk returns a tuple with the IsEnabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsEnabled

`func (o *CreateAccessInviteRequest) SetIsEnabled(v bool)`

SetIsEnabled sets IsEnabled field to given value.

### HasIsEnabled

`func (o *CreateAccessInviteRequest) HasIsEnabled() bool`

HasIsEnabled returns a boolean if a field has been set.

### SetIsEnabledNil

`func (o *CreateAccessInviteRequest) SetIsEnabledNil(b bool)`

 SetIsEnabledNil sets the value for IsEnabled to be an explicit nil

### UnsetIsEnabled
`func (o *CreateAccessInviteRequest) UnsetIsEnabled()`

UnsetIsEnabled ensures that no value is present for IsEnabled, not even an explicit nil
### GetMaxUses

`func (o *CreateAccessInviteRequest) GetMaxUses() int32`

GetMaxUses returns the MaxUses field if non-nil, zero value otherwise.

### GetMaxUsesOk

`func (o *CreateAccessInviteRequest) GetMaxUsesOk() (*int32, bool)`

GetMaxUsesOk returns a tuple with the MaxUses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxUses

`func (o *CreateAccessInviteRequest) SetMaxUses(v int32)`

SetMaxUses sets MaxUses field to given value.

### HasMaxUses

`func (o *CreateAccessInviteRequest) HasMaxUses() bool`

HasMaxUses returns a boolean if a field has been set.

### SetMaxUsesNil

`func (o *CreateAccessInviteRequest) SetMaxUsesNil(b bool)`

 SetMaxUsesNil sets the value for MaxUses to be an explicit nil

### UnsetMaxUses
`func (o *CreateAccessInviteRequest) UnsetMaxUses()`

UnsetMaxUses ensures that no value is present for MaxUses, not even an explicit nil
### GetName

`func (o *CreateAccessInviteRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateAccessInviteRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateAccessInviteRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *CreateAccessInviteRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *CreateAccessInviteRequest) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *CreateAccessInviteRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetPortalIds

`func (o *CreateAccessInviteRequest) GetPortalIds() []string`

GetPortalIds returns the PortalIds field if non-nil, zero value otherwise.

### GetPortalIdsOk

`func (o *CreateAccessInviteRequest) GetPortalIdsOk() (*[]string, bool)`

GetPortalIdsOk returns a tuple with the PortalIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPortalIds

`func (o *CreateAccessInviteRequest) SetPortalIds(v []string)`

SetPortalIds sets PortalIds field to given value.


### GetSchedules

`func (o *CreateAccessInviteRequest) GetSchedules() []InviteScheduleEntryInput`

GetSchedules returns the Schedules field if non-nil, zero value otherwise.

### GetSchedulesOk

`func (o *CreateAccessInviteRequest) GetSchedulesOk() (*[]InviteScheduleEntryInput, bool)`

GetSchedulesOk returns a tuple with the Schedules field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchedules

`func (o *CreateAccessInviteRequest) SetSchedules(v []InviteScheduleEntryInput)`

SetSchedules sets Schedules field to given value.

### HasSchedules

`func (o *CreateAccessInviteRequest) HasSchedules() bool`

HasSchedules returns a boolean if a field has been set.

### SetSchedulesNil

`func (o *CreateAccessInviteRequest) SetSchedulesNil(b bool)`

 SetSchedulesNil sets the value for Schedules to be an explicit nil

### UnsetSchedules
`func (o *CreateAccessInviteRequest) UnsetSchedules()`

UnsetSchedules ensures that no value is present for Schedules, not even an explicit nil
### GetValidFrom

`func (o *CreateAccessInviteRequest) GetValidFrom() string`

GetValidFrom returns the ValidFrom field if non-nil, zero value otherwise.

### GetValidFromOk

`func (o *CreateAccessInviteRequest) GetValidFromOk() (*string, bool)`

GetValidFromOk returns a tuple with the ValidFrom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidFrom

`func (o *CreateAccessInviteRequest) SetValidFrom(v string)`

SetValidFrom sets ValidFrom field to given value.

### HasValidFrom

`func (o *CreateAccessInviteRequest) HasValidFrom() bool`

HasValidFrom returns a boolean if a field has been set.

### SetValidFromNil

`func (o *CreateAccessInviteRequest) SetValidFromNil(b bool)`

 SetValidFromNil sets the value for ValidFrom to be an explicit nil

### UnsetValidFrom
`func (o *CreateAccessInviteRequest) UnsetValidFrom()`

UnsetValidFrom ensures that no value is present for ValidFrom, not even an explicit nil
### GetValidTo

`func (o *CreateAccessInviteRequest) GetValidTo() string`

GetValidTo returns the ValidTo field if non-nil, zero value otherwise.

### GetValidToOk

`func (o *CreateAccessInviteRequest) GetValidToOk() (*string, bool)`

GetValidToOk returns a tuple with the ValidTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidTo

`func (o *CreateAccessInviteRequest) SetValidTo(v string)`

SetValidTo sets ValidTo field to given value.

### HasValidTo

`func (o *CreateAccessInviteRequest) HasValidTo() bool`

HasValidTo returns a boolean if a field has been set.

### SetValidToNil

`func (o *CreateAccessInviteRequest) SetValidToNil(b bool)`

 SetValidToNil sets the value for ValidTo to be an explicit nil

### UnsetValidTo
`func (o *CreateAccessInviteRequest) UnsetValidTo()`

UnsetValidTo ensures that no value is present for ValidTo, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
