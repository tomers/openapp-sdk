# PublicInviteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AlreadyClaimed** | Pointer to **NullableBool** |  | [optional]
**Branding** | Pointer to **interface{}** |  | [optional]
**Building** | Pointer to **interface{}** |  | [optional]
**Grants** | [**[]PublicInviteGrant**](PublicInviteGrant.md) |  |
**InviteToken** | **string** |  |
**InviteeMessage** | Pointer to **interface{}** |  | [optional]
**Name** | Pointer to **NullableString** | Optional admin-defined label (management UI \&quot;name\&quot;); exposed for link previews and guests who already know the invite by name. | [optional]
**Schedule** | Pointer to [**NullableInviteScheduleSnapshot**](InviteScheduleSnapshot.md) |  | [optional]
**ScheduleCombined** | Pointer to [**NullableInviteScheduleCombined**](InviteScheduleCombined.md) |  | [optional]
**ScheduleEntries** | Pointer to [**[]InviteScheduleEntrySnapshot**](InviteScheduleEntrySnapshot.md) |  | [optional]
**ScheduleKind** | Pointer to [**NullableInviteScheduleKind**](InviteScheduleKind.md) |  | [optional]
**State** | [**PublicInviteState**](PublicInviteState.md) |  |
**ValidFrom** | Pointer to **NullableString** |  | [optional]
**ValidTo** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPublicInviteResponse

`func NewPublicInviteResponse(grants []PublicInviteGrant, inviteToken string, state PublicInviteState, ) *PublicInviteResponse`

NewPublicInviteResponse instantiates a new PublicInviteResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicInviteResponseWithDefaults

`func NewPublicInviteResponseWithDefaults() *PublicInviteResponse`

NewPublicInviteResponseWithDefaults instantiates a new PublicInviteResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAlreadyClaimed

`func (o *PublicInviteResponse) GetAlreadyClaimed() bool`

GetAlreadyClaimed returns the AlreadyClaimed field if non-nil, zero value otherwise.

### GetAlreadyClaimedOk

`func (o *PublicInviteResponse) GetAlreadyClaimedOk() (*bool, bool)`

GetAlreadyClaimedOk returns a tuple with the AlreadyClaimed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAlreadyClaimed

`func (o *PublicInviteResponse) SetAlreadyClaimed(v bool)`

SetAlreadyClaimed sets AlreadyClaimed field to given value.

### HasAlreadyClaimed

`func (o *PublicInviteResponse) HasAlreadyClaimed() bool`

HasAlreadyClaimed returns a boolean if a field has been set.

### SetAlreadyClaimedNil

`func (o *PublicInviteResponse) SetAlreadyClaimedNil(b bool)`

 SetAlreadyClaimedNil sets the value for AlreadyClaimed to be an explicit nil

### UnsetAlreadyClaimed
`func (o *PublicInviteResponse) UnsetAlreadyClaimed()`

UnsetAlreadyClaimed ensures that no value is present for AlreadyClaimed, not even an explicit nil
### GetBranding

`func (o *PublicInviteResponse) GetBranding() interface{}`

GetBranding returns the Branding field if non-nil, zero value otherwise.

### GetBrandingOk

`func (o *PublicInviteResponse) GetBrandingOk() (*interface{}, bool)`

GetBrandingOk returns a tuple with the Branding field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBranding

`func (o *PublicInviteResponse) SetBranding(v interface{})`

SetBranding sets Branding field to given value.

### HasBranding

`func (o *PublicInviteResponse) HasBranding() bool`

HasBranding returns a boolean if a field has been set.

### SetBrandingNil

`func (o *PublicInviteResponse) SetBrandingNil(b bool)`

 SetBrandingNil sets the value for Branding to be an explicit nil

### UnsetBranding
`func (o *PublicInviteResponse) UnsetBranding()`

UnsetBranding ensures that no value is present for Branding, not even an explicit nil
### GetBuilding

`func (o *PublicInviteResponse) GetBuilding() interface{}`

GetBuilding returns the Building field if non-nil, zero value otherwise.

### GetBuildingOk

`func (o *PublicInviteResponse) GetBuildingOk() (*interface{}, bool)`

GetBuildingOk returns a tuple with the Building field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBuilding

`func (o *PublicInviteResponse) SetBuilding(v interface{})`

SetBuilding sets Building field to given value.

### HasBuilding

`func (o *PublicInviteResponse) HasBuilding() bool`

HasBuilding returns a boolean if a field has been set.

### SetBuildingNil

`func (o *PublicInviteResponse) SetBuildingNil(b bool)`

 SetBuildingNil sets the value for Building to be an explicit nil

### UnsetBuilding
`func (o *PublicInviteResponse) UnsetBuilding()`

UnsetBuilding ensures that no value is present for Building, not even an explicit nil
### GetGrants

`func (o *PublicInviteResponse) GetGrants() []PublicInviteGrant`

GetGrants returns the Grants field if non-nil, zero value otherwise.

### GetGrantsOk

`func (o *PublicInviteResponse) GetGrantsOk() (*[]PublicInviteGrant, bool)`

GetGrantsOk returns a tuple with the Grants field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGrants

`func (o *PublicInviteResponse) SetGrants(v []PublicInviteGrant)`

SetGrants sets Grants field to given value.


### GetInviteToken

`func (o *PublicInviteResponse) GetInviteToken() string`

GetInviteToken returns the InviteToken field if non-nil, zero value otherwise.

### GetInviteTokenOk

`func (o *PublicInviteResponse) GetInviteTokenOk() (*string, bool)`

GetInviteTokenOk returns a tuple with the InviteToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteToken

`func (o *PublicInviteResponse) SetInviteToken(v string)`

SetInviteToken sets InviteToken field to given value.


### GetInviteeMessage

`func (o *PublicInviteResponse) GetInviteeMessage() interface{}`

GetInviteeMessage returns the InviteeMessage field if non-nil, zero value otherwise.

### GetInviteeMessageOk

`func (o *PublicInviteResponse) GetInviteeMessageOk() (*interface{}, bool)`

GetInviteeMessageOk returns a tuple with the InviteeMessage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteeMessage

`func (o *PublicInviteResponse) SetInviteeMessage(v interface{})`

SetInviteeMessage sets InviteeMessage field to given value.

### HasInviteeMessage

`func (o *PublicInviteResponse) HasInviteeMessage() bool`

HasInviteeMessage returns a boolean if a field has been set.

### SetInviteeMessageNil

`func (o *PublicInviteResponse) SetInviteeMessageNil(b bool)`

 SetInviteeMessageNil sets the value for InviteeMessage to be an explicit nil

### UnsetInviteeMessage
`func (o *PublicInviteResponse) UnsetInviteeMessage()`

UnsetInviteeMessage ensures that no value is present for InviteeMessage, not even an explicit nil
### GetName

`func (o *PublicInviteResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PublicInviteResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PublicInviteResponse) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *PublicInviteResponse) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *PublicInviteResponse) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *PublicInviteResponse) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetSchedule

`func (o *PublicInviteResponse) GetSchedule() InviteScheduleSnapshot`

GetSchedule returns the Schedule field if non-nil, zero value otherwise.

### GetScheduleOk

`func (o *PublicInviteResponse) GetScheduleOk() (*InviteScheduleSnapshot, bool)`

GetScheduleOk returns a tuple with the Schedule field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchedule

`func (o *PublicInviteResponse) SetSchedule(v InviteScheduleSnapshot)`

SetSchedule sets Schedule field to given value.

### HasSchedule

`func (o *PublicInviteResponse) HasSchedule() bool`

HasSchedule returns a boolean if a field has been set.

### SetScheduleNil

`func (o *PublicInviteResponse) SetScheduleNil(b bool)`

 SetScheduleNil sets the value for Schedule to be an explicit nil

### UnsetSchedule
`func (o *PublicInviteResponse) UnsetSchedule()`

UnsetSchedule ensures that no value is present for Schedule, not even an explicit nil
### GetScheduleCombined

`func (o *PublicInviteResponse) GetScheduleCombined() InviteScheduleCombined`

GetScheduleCombined returns the ScheduleCombined field if non-nil, zero value otherwise.

### GetScheduleCombinedOk

`func (o *PublicInviteResponse) GetScheduleCombinedOk() (*InviteScheduleCombined, bool)`

GetScheduleCombinedOk returns a tuple with the ScheduleCombined field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleCombined

`func (o *PublicInviteResponse) SetScheduleCombined(v InviteScheduleCombined)`

SetScheduleCombined sets ScheduleCombined field to given value.

### HasScheduleCombined

`func (o *PublicInviteResponse) HasScheduleCombined() bool`

HasScheduleCombined returns a boolean if a field has been set.

### SetScheduleCombinedNil

`func (o *PublicInviteResponse) SetScheduleCombinedNil(b bool)`

 SetScheduleCombinedNil sets the value for ScheduleCombined to be an explicit nil

### UnsetScheduleCombined
`func (o *PublicInviteResponse) UnsetScheduleCombined()`

UnsetScheduleCombined ensures that no value is present for ScheduleCombined, not even an explicit nil
### GetScheduleEntries

`func (o *PublicInviteResponse) GetScheduleEntries() []InviteScheduleEntrySnapshot`

GetScheduleEntries returns the ScheduleEntries field if non-nil, zero value otherwise.

### GetScheduleEntriesOk

`func (o *PublicInviteResponse) GetScheduleEntriesOk() (*[]InviteScheduleEntrySnapshot, bool)`

GetScheduleEntriesOk returns a tuple with the ScheduleEntries field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleEntries

`func (o *PublicInviteResponse) SetScheduleEntries(v []InviteScheduleEntrySnapshot)`

SetScheduleEntries sets ScheduleEntries field to given value.

### HasScheduleEntries

`func (o *PublicInviteResponse) HasScheduleEntries() bool`

HasScheduleEntries returns a boolean if a field has been set.

### SetScheduleEntriesNil

`func (o *PublicInviteResponse) SetScheduleEntriesNil(b bool)`

 SetScheduleEntriesNil sets the value for ScheduleEntries to be an explicit nil

### UnsetScheduleEntries
`func (o *PublicInviteResponse) UnsetScheduleEntries()`

UnsetScheduleEntries ensures that no value is present for ScheduleEntries, not even an explicit nil
### GetScheduleKind

`func (o *PublicInviteResponse) GetScheduleKind() InviteScheduleKind`

GetScheduleKind returns the ScheduleKind field if non-nil, zero value otherwise.

### GetScheduleKindOk

`func (o *PublicInviteResponse) GetScheduleKindOk() (*InviteScheduleKind, bool)`

GetScheduleKindOk returns a tuple with the ScheduleKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleKind

`func (o *PublicInviteResponse) SetScheduleKind(v InviteScheduleKind)`

SetScheduleKind sets ScheduleKind field to given value.

### HasScheduleKind

`func (o *PublicInviteResponse) HasScheduleKind() bool`

HasScheduleKind returns a boolean if a field has been set.

### SetScheduleKindNil

`func (o *PublicInviteResponse) SetScheduleKindNil(b bool)`

 SetScheduleKindNil sets the value for ScheduleKind to be an explicit nil

### UnsetScheduleKind
`func (o *PublicInviteResponse) UnsetScheduleKind()`

UnsetScheduleKind ensures that no value is present for ScheduleKind, not even an explicit nil
### GetState

`func (o *PublicInviteResponse) GetState() PublicInviteState`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *PublicInviteResponse) GetStateOk() (*PublicInviteState, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *PublicInviteResponse) SetState(v PublicInviteState)`

SetState sets State field to given value.


### GetValidFrom

`func (o *PublicInviteResponse) GetValidFrom() string`

GetValidFrom returns the ValidFrom field if non-nil, zero value otherwise.

### GetValidFromOk

`func (o *PublicInviteResponse) GetValidFromOk() (*string, bool)`

GetValidFromOk returns a tuple with the ValidFrom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidFrom

`func (o *PublicInviteResponse) SetValidFrom(v string)`

SetValidFrom sets ValidFrom field to given value.

### HasValidFrom

`func (o *PublicInviteResponse) HasValidFrom() bool`

HasValidFrom returns a boolean if a field has been set.

### SetValidFromNil

`func (o *PublicInviteResponse) SetValidFromNil(b bool)`

 SetValidFromNil sets the value for ValidFrom to be an explicit nil

### UnsetValidFrom
`func (o *PublicInviteResponse) UnsetValidFrom()`

UnsetValidFrom ensures that no value is present for ValidFrom, not even an explicit nil
### GetValidTo

`func (o *PublicInviteResponse) GetValidTo() string`

GetValidTo returns the ValidTo field if non-nil, zero value otherwise.

### GetValidToOk

`func (o *PublicInviteResponse) GetValidToOk() (*string, bool)`

GetValidToOk returns a tuple with the ValidTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidTo

`func (o *PublicInviteResponse) SetValidTo(v string)`

SetValidTo sets ValidTo field to given value.

### HasValidTo

`func (o *PublicInviteResponse) HasValidTo() bool`

HasValidTo returns a boolean if a field has been set.

### SetValidToNil

`func (o *PublicInviteResponse) SetValidToNil(b bool)`

 SetValidToNil sets the value for ValidTo to be an explicit nil

### UnsetValidTo
`func (o *PublicInviteResponse) UnsetValidTo()`

UnsetValidTo ensures that no value is present for ValidTo, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
