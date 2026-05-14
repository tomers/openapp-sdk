# CreateAccessInviteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**GrantedPortals** | [**[]AccessInviteGrantedPortal**](AccessInviteGrantedPortal.md) |  |
**InviteLinkId** | **string** |  |
**InviteRecurrence** | Pointer to **interface{}** |  | [optional]
**InviteToken** | **string** |  |
**InviteeMessage** | Pointer to **interface{}** |  | [optional]
**IsEnabled** | **bool** |  |
**MaxUses** | Pointer to **NullableInt32** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**Schedule** | [**InviteScheduleSnapshot**](InviteScheduleSnapshot.md) |  |
**ScheduleCombined** | [**InviteScheduleCombined**](InviteScheduleCombined.md) |  |
**ScheduleEntries** | [**[]InviteScheduleEntrySnapshot**](InviteScheduleEntrySnapshot.md) |  |
**ScheduleKind** | [**InviteScheduleKind**](InviteScheduleKind.md) |  |
**Uses** | **int32** |  |
**ValidFrom** | **string** |  |
**ValidTo** | **string** |  |

## Methods

### NewCreateAccessInviteResponse

`func NewCreateAccessInviteResponse(grantedPortals []AccessInviteGrantedPortal, inviteLinkId string, inviteToken string, isEnabled bool, schedule InviteScheduleSnapshot, scheduleCombined InviteScheduleCombined, scheduleEntries []InviteScheduleEntrySnapshot, scheduleKind InviteScheduleKind, uses int32, validFrom string, validTo string, ) *CreateAccessInviteResponse`

NewCreateAccessInviteResponse instantiates a new CreateAccessInviteResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateAccessInviteResponseWithDefaults

`func NewCreateAccessInviteResponseWithDefaults() *CreateAccessInviteResponse`

NewCreateAccessInviteResponseWithDefaults instantiates a new CreateAccessInviteResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetGrantedPortals

`func (o *CreateAccessInviteResponse) GetGrantedPortals() []AccessInviteGrantedPortal`

GetGrantedPortals returns the GrantedPortals field if non-nil, zero value otherwise.

### GetGrantedPortalsOk

`func (o *CreateAccessInviteResponse) GetGrantedPortalsOk() (*[]AccessInviteGrantedPortal, bool)`

GetGrantedPortalsOk returns a tuple with the GrantedPortals field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGrantedPortals

`func (o *CreateAccessInviteResponse) SetGrantedPortals(v []AccessInviteGrantedPortal)`

SetGrantedPortals sets GrantedPortals field to given value.


### GetInviteLinkId

`func (o *CreateAccessInviteResponse) GetInviteLinkId() string`

GetInviteLinkId returns the InviteLinkId field if non-nil, zero value otherwise.

### GetInviteLinkIdOk

`func (o *CreateAccessInviteResponse) GetInviteLinkIdOk() (*string, bool)`

GetInviteLinkIdOk returns a tuple with the InviteLinkId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteLinkId

`func (o *CreateAccessInviteResponse) SetInviteLinkId(v string)`

SetInviteLinkId sets InviteLinkId field to given value.


### GetInviteRecurrence

`func (o *CreateAccessInviteResponse) GetInviteRecurrence() interface{}`

GetInviteRecurrence returns the InviteRecurrence field if non-nil, zero value otherwise.

### GetInviteRecurrenceOk

`func (o *CreateAccessInviteResponse) GetInviteRecurrenceOk() (*interface{}, bool)`

GetInviteRecurrenceOk returns a tuple with the InviteRecurrence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteRecurrence

`func (o *CreateAccessInviteResponse) SetInviteRecurrence(v interface{})`

SetInviteRecurrence sets InviteRecurrence field to given value.

### HasInviteRecurrence

`func (o *CreateAccessInviteResponse) HasInviteRecurrence() bool`

HasInviteRecurrence returns a boolean if a field has been set.

### SetInviteRecurrenceNil

`func (o *CreateAccessInviteResponse) SetInviteRecurrenceNil(b bool)`

 SetInviteRecurrenceNil sets the value for InviteRecurrence to be an explicit nil

### UnsetInviteRecurrence
`func (o *CreateAccessInviteResponse) UnsetInviteRecurrence()`

UnsetInviteRecurrence ensures that no value is present for InviteRecurrence, not even an explicit nil
### GetInviteToken

`func (o *CreateAccessInviteResponse) GetInviteToken() string`

GetInviteToken returns the InviteToken field if non-nil, zero value otherwise.

### GetInviteTokenOk

`func (o *CreateAccessInviteResponse) GetInviteTokenOk() (*string, bool)`

GetInviteTokenOk returns a tuple with the InviteToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteToken

`func (o *CreateAccessInviteResponse) SetInviteToken(v string)`

SetInviteToken sets InviteToken field to given value.


### GetInviteeMessage

`func (o *CreateAccessInviteResponse) GetInviteeMessage() interface{}`

GetInviteeMessage returns the InviteeMessage field if non-nil, zero value otherwise.

### GetInviteeMessageOk

`func (o *CreateAccessInviteResponse) GetInviteeMessageOk() (*interface{}, bool)`

GetInviteeMessageOk returns a tuple with the InviteeMessage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteeMessage

`func (o *CreateAccessInviteResponse) SetInviteeMessage(v interface{})`

SetInviteeMessage sets InviteeMessage field to given value.

### HasInviteeMessage

`func (o *CreateAccessInviteResponse) HasInviteeMessage() bool`

HasInviteeMessage returns a boolean if a field has been set.

### SetInviteeMessageNil

`func (o *CreateAccessInviteResponse) SetInviteeMessageNil(b bool)`

 SetInviteeMessageNil sets the value for InviteeMessage to be an explicit nil

### UnsetInviteeMessage
`func (o *CreateAccessInviteResponse) UnsetInviteeMessage()`

UnsetInviteeMessage ensures that no value is present for InviteeMessage, not even an explicit nil
### GetIsEnabled

`func (o *CreateAccessInviteResponse) GetIsEnabled() bool`

GetIsEnabled returns the IsEnabled field if non-nil, zero value otherwise.

### GetIsEnabledOk

`func (o *CreateAccessInviteResponse) GetIsEnabledOk() (*bool, bool)`

GetIsEnabledOk returns a tuple with the IsEnabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsEnabled

`func (o *CreateAccessInviteResponse) SetIsEnabled(v bool)`

SetIsEnabled sets IsEnabled field to given value.


### GetMaxUses

`func (o *CreateAccessInviteResponse) GetMaxUses() int32`

GetMaxUses returns the MaxUses field if non-nil, zero value otherwise.

### GetMaxUsesOk

`func (o *CreateAccessInviteResponse) GetMaxUsesOk() (*int32, bool)`

GetMaxUsesOk returns a tuple with the MaxUses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxUses

`func (o *CreateAccessInviteResponse) SetMaxUses(v int32)`

SetMaxUses sets MaxUses field to given value.

### HasMaxUses

`func (o *CreateAccessInviteResponse) HasMaxUses() bool`

HasMaxUses returns a boolean if a field has been set.

### SetMaxUsesNil

`func (o *CreateAccessInviteResponse) SetMaxUsesNil(b bool)`

 SetMaxUsesNil sets the value for MaxUses to be an explicit nil

### UnsetMaxUses
`func (o *CreateAccessInviteResponse) UnsetMaxUses()`

UnsetMaxUses ensures that no value is present for MaxUses, not even an explicit nil
### GetName

`func (o *CreateAccessInviteResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateAccessInviteResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateAccessInviteResponse) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *CreateAccessInviteResponse) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *CreateAccessInviteResponse) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *CreateAccessInviteResponse) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetSchedule

`func (o *CreateAccessInviteResponse) GetSchedule() InviteScheduleSnapshot`

GetSchedule returns the Schedule field if non-nil, zero value otherwise.

### GetScheduleOk

`func (o *CreateAccessInviteResponse) GetScheduleOk() (*InviteScheduleSnapshot, bool)`

GetScheduleOk returns a tuple with the Schedule field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchedule

`func (o *CreateAccessInviteResponse) SetSchedule(v InviteScheduleSnapshot)`

SetSchedule sets Schedule field to given value.


### GetScheduleCombined

`func (o *CreateAccessInviteResponse) GetScheduleCombined() InviteScheduleCombined`

GetScheduleCombined returns the ScheduleCombined field if non-nil, zero value otherwise.

### GetScheduleCombinedOk

`func (o *CreateAccessInviteResponse) GetScheduleCombinedOk() (*InviteScheduleCombined, bool)`

GetScheduleCombinedOk returns a tuple with the ScheduleCombined field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleCombined

`func (o *CreateAccessInviteResponse) SetScheduleCombined(v InviteScheduleCombined)`

SetScheduleCombined sets ScheduleCombined field to given value.


### GetScheduleEntries

`func (o *CreateAccessInviteResponse) GetScheduleEntries() []InviteScheduleEntrySnapshot`

GetScheduleEntries returns the ScheduleEntries field if non-nil, zero value otherwise.

### GetScheduleEntriesOk

`func (o *CreateAccessInviteResponse) GetScheduleEntriesOk() (*[]InviteScheduleEntrySnapshot, bool)`

GetScheduleEntriesOk returns a tuple with the ScheduleEntries field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleEntries

`func (o *CreateAccessInviteResponse) SetScheduleEntries(v []InviteScheduleEntrySnapshot)`

SetScheduleEntries sets ScheduleEntries field to given value.


### GetScheduleKind

`func (o *CreateAccessInviteResponse) GetScheduleKind() InviteScheduleKind`

GetScheduleKind returns the ScheduleKind field if non-nil, zero value otherwise.

### GetScheduleKindOk

`func (o *CreateAccessInviteResponse) GetScheduleKindOk() (*InviteScheduleKind, bool)`

GetScheduleKindOk returns a tuple with the ScheduleKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleKind

`func (o *CreateAccessInviteResponse) SetScheduleKind(v InviteScheduleKind)`

SetScheduleKind sets ScheduleKind field to given value.


### GetUses

`func (o *CreateAccessInviteResponse) GetUses() int32`

GetUses returns the Uses field if non-nil, zero value otherwise.

### GetUsesOk

`func (o *CreateAccessInviteResponse) GetUsesOk() (*int32, bool)`

GetUsesOk returns a tuple with the Uses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUses

`func (o *CreateAccessInviteResponse) SetUses(v int32)`

SetUses sets Uses field to given value.


### GetValidFrom

`func (o *CreateAccessInviteResponse) GetValidFrom() string`

GetValidFrom returns the ValidFrom field if non-nil, zero value otherwise.

### GetValidFromOk

`func (o *CreateAccessInviteResponse) GetValidFromOk() (*string, bool)`

GetValidFromOk returns a tuple with the ValidFrom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidFrom

`func (o *CreateAccessInviteResponse) SetValidFrom(v string)`

SetValidFrom sets ValidFrom field to given value.


### GetValidTo

`func (o *CreateAccessInviteResponse) GetValidTo() string`

GetValidTo returns the ValidTo field if non-nil, zero value otherwise.

### GetValidToOk

`func (o *CreateAccessInviteResponse) GetValidToOk() (*string, bool)`

GetValidToOk returns a tuple with the ValidTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidTo

`func (o *CreateAccessInviteResponse) SetValidTo(v string)`

SetValidTo sets ValidTo field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
