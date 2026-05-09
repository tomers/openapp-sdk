# AccessInviteListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**DisabledJustification** | Pointer to **NullableString** |  | [optional]
**GrantedPortals** | [**[]AccessInviteGrantedPortal**](AccessInviteGrantedPortal.md) |  |
**Id** | **string** |  |
**InviteRecurrence** | Pointer to **interface{}** |  | [optional]
**InviteeMessage** | Pointer to **interface{}** |  | [optional]
**IsEnabled** | **bool** |  |
**LastUsedAt** | Pointer to **NullableString** |  | [optional]
**MaxUses** | Pointer to **NullableInt32** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**RevokedAt** | Pointer to **NullableString** |  | [optional]
**Schedule** | [**InviteScheduleSnapshot**](InviteScheduleSnapshot.md) |  |
**ScheduleCombined** | [**InviteScheduleCombined**](InviteScheduleCombined.md) |  |
**ScheduleEntries** | [**[]InviteScheduleEntrySnapshot**](InviteScheduleEntrySnapshot.md) |  |
**ScheduleKind** | [**InviteScheduleKind**](InviteScheduleKind.md) |  |
**State** | **string** |  |
**UpdatedAt** | Pointer to **NullableString** |  | [optional]
**Uses** | **int32** |  |
**ValidFrom** | Pointer to **NullableString** |  | [optional]
**ValidTo** | Pointer to **NullableString** |  | [optional]

## Methods

### NewAccessInviteListItem

`func NewAccessInviteListItem(grantedPortals []AccessInviteGrantedPortal, id string, isEnabled bool, schedule InviteScheduleSnapshot, scheduleCombined InviteScheduleCombined, scheduleEntries []InviteScheduleEntrySnapshot, scheduleKind InviteScheduleKind, state string, uses int32, ) *AccessInviteListItem`

NewAccessInviteListItem instantiates a new AccessInviteListItem object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAccessInviteListItemWithDefaults

`func NewAccessInviteListItemWithDefaults() *AccessInviteListItem`

NewAccessInviteListItemWithDefaults instantiates a new AccessInviteListItem object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *AccessInviteListItem) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *AccessInviteListItem) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *AccessInviteListItem) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *AccessInviteListItem) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *AccessInviteListItem) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *AccessInviteListItem) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetDisabledJustification

`func (o *AccessInviteListItem) GetDisabledJustification() string`

GetDisabledJustification returns the DisabledJustification field if non-nil, zero value otherwise.

### GetDisabledJustificationOk

`func (o *AccessInviteListItem) GetDisabledJustificationOk() (*string, bool)`

GetDisabledJustificationOk returns a tuple with the DisabledJustification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisabledJustification

`func (o *AccessInviteListItem) SetDisabledJustification(v string)`

SetDisabledJustification sets DisabledJustification field to given value.

### HasDisabledJustification

`func (o *AccessInviteListItem) HasDisabledJustification() bool`

HasDisabledJustification returns a boolean if a field has been set.

### SetDisabledJustificationNil

`func (o *AccessInviteListItem) SetDisabledJustificationNil(b bool)`

 SetDisabledJustificationNil sets the value for DisabledJustification to be an explicit nil

### UnsetDisabledJustification
`func (o *AccessInviteListItem) UnsetDisabledJustification()`

UnsetDisabledJustification ensures that no value is present for DisabledJustification, not even an explicit nil
### GetGrantedPortals

`func (o *AccessInviteListItem) GetGrantedPortals() []AccessInviteGrantedPortal`

GetGrantedPortals returns the GrantedPortals field if non-nil, zero value otherwise.

### GetGrantedPortalsOk

`func (o *AccessInviteListItem) GetGrantedPortalsOk() (*[]AccessInviteGrantedPortal, bool)`

GetGrantedPortalsOk returns a tuple with the GrantedPortals field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGrantedPortals

`func (o *AccessInviteListItem) SetGrantedPortals(v []AccessInviteGrantedPortal)`

SetGrantedPortals sets GrantedPortals field to given value.


### GetId

`func (o *AccessInviteListItem) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *AccessInviteListItem) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *AccessInviteListItem) SetId(v string)`

SetId sets Id field to given value.


### GetInviteRecurrence

`func (o *AccessInviteListItem) GetInviteRecurrence() interface{}`

GetInviteRecurrence returns the InviteRecurrence field if non-nil, zero value otherwise.

### GetInviteRecurrenceOk

`func (o *AccessInviteListItem) GetInviteRecurrenceOk() (*interface{}, bool)`

GetInviteRecurrenceOk returns a tuple with the InviteRecurrence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteRecurrence

`func (o *AccessInviteListItem) SetInviteRecurrence(v interface{})`

SetInviteRecurrence sets InviteRecurrence field to given value.

### HasInviteRecurrence

`func (o *AccessInviteListItem) HasInviteRecurrence() bool`

HasInviteRecurrence returns a boolean if a field has been set.

### SetInviteRecurrenceNil

`func (o *AccessInviteListItem) SetInviteRecurrenceNil(b bool)`

 SetInviteRecurrenceNil sets the value for InviteRecurrence to be an explicit nil

### UnsetInviteRecurrence
`func (o *AccessInviteListItem) UnsetInviteRecurrence()`

UnsetInviteRecurrence ensures that no value is present for InviteRecurrence, not even an explicit nil
### GetInviteeMessage

`func (o *AccessInviteListItem) GetInviteeMessage() interface{}`

GetInviteeMessage returns the InviteeMessage field if non-nil, zero value otherwise.

### GetInviteeMessageOk

`func (o *AccessInviteListItem) GetInviteeMessageOk() (*interface{}, bool)`

GetInviteeMessageOk returns a tuple with the InviteeMessage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteeMessage

`func (o *AccessInviteListItem) SetInviteeMessage(v interface{})`

SetInviteeMessage sets InviteeMessage field to given value.

### HasInviteeMessage

`func (o *AccessInviteListItem) HasInviteeMessage() bool`

HasInviteeMessage returns a boolean if a field has been set.

### SetInviteeMessageNil

`func (o *AccessInviteListItem) SetInviteeMessageNil(b bool)`

 SetInviteeMessageNil sets the value for InviteeMessage to be an explicit nil

### UnsetInviteeMessage
`func (o *AccessInviteListItem) UnsetInviteeMessage()`

UnsetInviteeMessage ensures that no value is present for InviteeMessage, not even an explicit nil
### GetIsEnabled

`func (o *AccessInviteListItem) GetIsEnabled() bool`

GetIsEnabled returns the IsEnabled field if non-nil, zero value otherwise.

### GetIsEnabledOk

`func (o *AccessInviteListItem) GetIsEnabledOk() (*bool, bool)`

GetIsEnabledOk returns a tuple with the IsEnabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsEnabled

`func (o *AccessInviteListItem) SetIsEnabled(v bool)`

SetIsEnabled sets IsEnabled field to given value.


### GetLastUsedAt

`func (o *AccessInviteListItem) GetLastUsedAt() string`

GetLastUsedAt returns the LastUsedAt field if non-nil, zero value otherwise.

### GetLastUsedAtOk

`func (o *AccessInviteListItem) GetLastUsedAtOk() (*string, bool)`

GetLastUsedAtOk returns a tuple with the LastUsedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastUsedAt

`func (o *AccessInviteListItem) SetLastUsedAt(v string)`

SetLastUsedAt sets LastUsedAt field to given value.

### HasLastUsedAt

`func (o *AccessInviteListItem) HasLastUsedAt() bool`

HasLastUsedAt returns a boolean if a field has been set.

### SetLastUsedAtNil

`func (o *AccessInviteListItem) SetLastUsedAtNil(b bool)`

 SetLastUsedAtNil sets the value for LastUsedAt to be an explicit nil

### UnsetLastUsedAt
`func (o *AccessInviteListItem) UnsetLastUsedAt()`

UnsetLastUsedAt ensures that no value is present for LastUsedAt, not even an explicit nil
### GetMaxUses

`func (o *AccessInviteListItem) GetMaxUses() int32`

GetMaxUses returns the MaxUses field if non-nil, zero value otherwise.

### GetMaxUsesOk

`func (o *AccessInviteListItem) GetMaxUsesOk() (*int32, bool)`

GetMaxUsesOk returns a tuple with the MaxUses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxUses

`func (o *AccessInviteListItem) SetMaxUses(v int32)`

SetMaxUses sets MaxUses field to given value.

### HasMaxUses

`func (o *AccessInviteListItem) HasMaxUses() bool`

HasMaxUses returns a boolean if a field has been set.

### SetMaxUsesNil

`func (o *AccessInviteListItem) SetMaxUsesNil(b bool)`

 SetMaxUsesNil sets the value for MaxUses to be an explicit nil

### UnsetMaxUses
`func (o *AccessInviteListItem) UnsetMaxUses()`

UnsetMaxUses ensures that no value is present for MaxUses, not even an explicit nil
### GetName

`func (o *AccessInviteListItem) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *AccessInviteListItem) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *AccessInviteListItem) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *AccessInviteListItem) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *AccessInviteListItem) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *AccessInviteListItem) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetRevokedAt

`func (o *AccessInviteListItem) GetRevokedAt() string`

GetRevokedAt returns the RevokedAt field if non-nil, zero value otherwise.

### GetRevokedAtOk

`func (o *AccessInviteListItem) GetRevokedAtOk() (*string, bool)`

GetRevokedAtOk returns a tuple with the RevokedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRevokedAt

`func (o *AccessInviteListItem) SetRevokedAt(v string)`

SetRevokedAt sets RevokedAt field to given value.

### HasRevokedAt

`func (o *AccessInviteListItem) HasRevokedAt() bool`

HasRevokedAt returns a boolean if a field has been set.

### SetRevokedAtNil

`func (o *AccessInviteListItem) SetRevokedAtNil(b bool)`

 SetRevokedAtNil sets the value for RevokedAt to be an explicit nil

### UnsetRevokedAt
`func (o *AccessInviteListItem) UnsetRevokedAt()`

UnsetRevokedAt ensures that no value is present for RevokedAt, not even an explicit nil
### GetSchedule

`func (o *AccessInviteListItem) GetSchedule() InviteScheduleSnapshot`

GetSchedule returns the Schedule field if non-nil, zero value otherwise.

### GetScheduleOk

`func (o *AccessInviteListItem) GetScheduleOk() (*InviteScheduleSnapshot, bool)`

GetScheduleOk returns a tuple with the Schedule field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchedule

`func (o *AccessInviteListItem) SetSchedule(v InviteScheduleSnapshot)`

SetSchedule sets Schedule field to given value.


### GetScheduleCombined

`func (o *AccessInviteListItem) GetScheduleCombined() InviteScheduleCombined`

GetScheduleCombined returns the ScheduleCombined field if non-nil, zero value otherwise.

### GetScheduleCombinedOk

`func (o *AccessInviteListItem) GetScheduleCombinedOk() (*InviteScheduleCombined, bool)`

GetScheduleCombinedOk returns a tuple with the ScheduleCombined field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleCombined

`func (o *AccessInviteListItem) SetScheduleCombined(v InviteScheduleCombined)`

SetScheduleCombined sets ScheduleCombined field to given value.


### GetScheduleEntries

`func (o *AccessInviteListItem) GetScheduleEntries() []InviteScheduleEntrySnapshot`

GetScheduleEntries returns the ScheduleEntries field if non-nil, zero value otherwise.

### GetScheduleEntriesOk

`func (o *AccessInviteListItem) GetScheduleEntriesOk() (*[]InviteScheduleEntrySnapshot, bool)`

GetScheduleEntriesOk returns a tuple with the ScheduleEntries field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleEntries

`func (o *AccessInviteListItem) SetScheduleEntries(v []InviteScheduleEntrySnapshot)`

SetScheduleEntries sets ScheduleEntries field to given value.


### GetScheduleKind

`func (o *AccessInviteListItem) GetScheduleKind() InviteScheduleKind`

GetScheduleKind returns the ScheduleKind field if non-nil, zero value otherwise.

### GetScheduleKindOk

`func (o *AccessInviteListItem) GetScheduleKindOk() (*InviteScheduleKind, bool)`

GetScheduleKindOk returns a tuple with the ScheduleKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleKind

`func (o *AccessInviteListItem) SetScheduleKind(v InviteScheduleKind)`

SetScheduleKind sets ScheduleKind field to given value.


### GetState

`func (o *AccessInviteListItem) GetState() string`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *AccessInviteListItem) GetStateOk() (*string, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *AccessInviteListItem) SetState(v string)`

SetState sets State field to given value.


### GetUpdatedAt

`func (o *AccessInviteListItem) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *AccessInviteListItem) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *AccessInviteListItem) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *AccessInviteListItem) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *AccessInviteListItem) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *AccessInviteListItem) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil
### GetUses

`func (o *AccessInviteListItem) GetUses() int32`

GetUses returns the Uses field if non-nil, zero value otherwise.

### GetUsesOk

`func (o *AccessInviteListItem) GetUsesOk() (*int32, bool)`

GetUsesOk returns a tuple with the Uses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUses

`func (o *AccessInviteListItem) SetUses(v int32)`

SetUses sets Uses field to given value.


### GetValidFrom

`func (o *AccessInviteListItem) GetValidFrom() string`

GetValidFrom returns the ValidFrom field if non-nil, zero value otherwise.

### GetValidFromOk

`func (o *AccessInviteListItem) GetValidFromOk() (*string, bool)`

GetValidFromOk returns a tuple with the ValidFrom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidFrom

`func (o *AccessInviteListItem) SetValidFrom(v string)`

SetValidFrom sets ValidFrom field to given value.

### HasValidFrom

`func (o *AccessInviteListItem) HasValidFrom() bool`

HasValidFrom returns a boolean if a field has been set.

### SetValidFromNil

`func (o *AccessInviteListItem) SetValidFromNil(b bool)`

 SetValidFromNil sets the value for ValidFrom to be an explicit nil

### UnsetValidFrom
`func (o *AccessInviteListItem) UnsetValidFrom()`

UnsetValidFrom ensures that no value is present for ValidFrom, not even an explicit nil
### GetValidTo

`func (o *AccessInviteListItem) GetValidTo() string`

GetValidTo returns the ValidTo field if non-nil, zero value otherwise.

### GetValidToOk

`func (o *AccessInviteListItem) GetValidToOk() (*string, bool)`

GetValidToOk returns a tuple with the ValidTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidTo

`func (o *AccessInviteListItem) SetValidTo(v string)`

SetValidTo sets ValidTo field to given value.

### HasValidTo

`func (o *AccessInviteListItem) HasValidTo() bool`

HasValidTo returns a boolean if a field has been set.

### SetValidToNil

`func (o *AccessInviteListItem) SetValidToNil(b bool)`

 SetValidToNil sets the value for ValidTo to be an explicit nil

### UnsetValidTo
`func (o *AccessInviteListItem) UnsetValidTo()`

UnsetValidTo ensures that no value is present for ValidTo, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
