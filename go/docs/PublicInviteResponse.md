# PublicInviteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AccessControl** | Pointer to [**NullablePublicPortalAccessControl**](PublicPortalAccessControl.md) | Access-control integration summary (building name, address, photo). | [optional]
**AllowedWeekdays** | Pointer to **[]int32** |  | [optional]
**AlreadyClaimed** | Pointer to **NullableBool** |  | [optional]
**Audience** | **string** | Whether anyone holding the link may use the invitation, or only the named contacts. |
**BlackoutDates** | Pointer to **[]string** |  | [optional]
**Building** | Pointer to **interface{}** |  | [optional]
**ContactChallengeRequired** | Pointer to **bool** | True when the bearer must prove they are one of the named contacts before the invitation can be used. Which contact was matched is never echoed back, so this cannot be used to enumerate who an invitation is for. | [optional]
**ContactDisplayName** | Pointer to **NullableString** | The display name of the contact this token resolved to, when it resolved to one. | [optional]
**CurfewWindows** | Pointer to [**[]CurfewWindowResponse**](CurfewWindowResponse.md) | Effective forbidden hour windows for invitation access (informational). | [optional]
**DevicesCount** | Pointer to **NullableInt32** |  | [optional]
**EventLocation** | Pointer to [**NullableLocationResponse**](LocationResponse.md) | Event location (address / map-picked coordinates) owned by this invite, distinct from the building/portal location surfaced via &#x60;access_control&#x60;. | [optional]
**Grants** | [**[]PublicInviteGrant**](PublicInviteGrant.md) |  |
**InviteToken** | **string** |  |
**InviteUrl** | **string** | Guest-facing invite URL (shareable link for SMS/email). |
**InviteeMessage** | Pointer to **interface{}** |  | [optional]
**LocationMode** | [**LocationBindingMode**](LocationBindingMode.md) | How the invite resolves its event location: &#x60;none&#x60; (hide all location, including the building address), &#x60;explicit&#x60; (own location), or &#x60;inherit&#x60; (building location). Lets the guest UI suppress the building address when the invite opts out of location entirely. |
**MaxDevices** | Pointer to **NullableInt32** |  | [optional]
**Name** | Pointer to **NullableString** | Optional admin-defined label (management UI \&quot;name\&quot;); exposed for link previews and guests who already know the invite by name. | [optional]
**PhotoUrl** | Pointer to **NullableString** | Presigned URL for the invite&#39;s main photo (best-effort). | [optional]
**RenewalRequested** | Pointer to **NullableBool** | True when the authenticated requester already submitted a pending renewal request. | [optional]
**RequirePin** | Pointer to **bool** |  | [optional]
**RequireVerifiedPhone** | Pointer to **bool** |  | [optional]
**Schedule** | Pointer to [**NullableInviteScheduleSnapshot**](InviteScheduleSnapshot.md) |  | [optional]
**ScheduleCombined** | Pointer to [**NullableInviteScheduleCombined**](InviteScheduleCombined.md) |  | [optional]
**ScheduleEntries** | Pointer to [**[]InviteScheduleEntrySnapshot**](InviteScheduleEntrySnapshot.md) |  | [optional]
**ScheduleKind** | Pointer to [**NullableInviteScheduleKind**](InviteScheduleKind.md) |  | [optional]
**State** | [**PublicInviteState**](PublicInviteState.md) |  |
**Timezone** | Pointer to **NullableString** | Org IANA timezone used to evaluate invitation curfews (&#x60;UTC&#x60; when unset). | [optional]
**ValidFrom** | Pointer to **NullableString** |  | [optional]
**ValidTo** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPublicInviteResponse

`func NewPublicInviteResponse(audience string, grants []PublicInviteGrant, inviteToken string, inviteUrl string, locationMode LocationBindingMode, state PublicInviteState, ) *PublicInviteResponse`

NewPublicInviteResponse instantiates a new PublicInviteResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicInviteResponseWithDefaults

`func NewPublicInviteResponseWithDefaults() *PublicInviteResponse`

NewPublicInviteResponseWithDefaults instantiates a new PublicInviteResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAccessControl

`func (o *PublicInviteResponse) GetAccessControl() PublicPortalAccessControl`

GetAccessControl returns the AccessControl field if non-nil, zero value otherwise.

### GetAccessControlOk

`func (o *PublicInviteResponse) GetAccessControlOk() (*PublicPortalAccessControl, bool)`

GetAccessControlOk returns a tuple with the AccessControl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAccessControl

`func (o *PublicInviteResponse) SetAccessControl(v PublicPortalAccessControl)`

SetAccessControl sets AccessControl field to given value.

### HasAccessControl

`func (o *PublicInviteResponse) HasAccessControl() bool`

HasAccessControl returns a boolean if a field has been set.

### SetAccessControlNil

`func (o *PublicInviteResponse) SetAccessControlNil(b bool)`

 SetAccessControlNil sets the value for AccessControl to be an explicit nil

### UnsetAccessControl
`func (o *PublicInviteResponse) UnsetAccessControl()`

UnsetAccessControl ensures that no value is present for AccessControl, not even an explicit nil
### GetAllowedWeekdays

`func (o *PublicInviteResponse) GetAllowedWeekdays() []int32`

GetAllowedWeekdays returns the AllowedWeekdays field if non-nil, zero value otherwise.

### GetAllowedWeekdaysOk

`func (o *PublicInviteResponse) GetAllowedWeekdaysOk() (*[]int32, bool)`

GetAllowedWeekdaysOk returns a tuple with the AllowedWeekdays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedWeekdays

`func (o *PublicInviteResponse) SetAllowedWeekdays(v []int32)`

SetAllowedWeekdays sets AllowedWeekdays field to given value.

### HasAllowedWeekdays

`func (o *PublicInviteResponse) HasAllowedWeekdays() bool`

HasAllowedWeekdays returns a boolean if a field has been set.

### SetAllowedWeekdaysNil

`func (o *PublicInviteResponse) SetAllowedWeekdaysNil(b bool)`

 SetAllowedWeekdaysNil sets the value for AllowedWeekdays to be an explicit nil

### UnsetAllowedWeekdays
`func (o *PublicInviteResponse) UnsetAllowedWeekdays()`

UnsetAllowedWeekdays ensures that no value is present for AllowedWeekdays, not even an explicit nil
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
### GetAudience

`func (o *PublicInviteResponse) GetAudience() string`

GetAudience returns the Audience field if non-nil, zero value otherwise.

### GetAudienceOk

`func (o *PublicInviteResponse) GetAudienceOk() (*string, bool)`

GetAudienceOk returns a tuple with the Audience field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAudience

`func (o *PublicInviteResponse) SetAudience(v string)`

SetAudience sets Audience field to given value.


### GetBlackoutDates

`func (o *PublicInviteResponse) GetBlackoutDates() []string`

GetBlackoutDates returns the BlackoutDates field if non-nil, zero value otherwise.

### GetBlackoutDatesOk

`func (o *PublicInviteResponse) GetBlackoutDatesOk() (*[]string, bool)`

GetBlackoutDatesOk returns a tuple with the BlackoutDates field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBlackoutDates

`func (o *PublicInviteResponse) SetBlackoutDates(v []string)`

SetBlackoutDates sets BlackoutDates field to given value.

### HasBlackoutDates

`func (o *PublicInviteResponse) HasBlackoutDates() bool`

HasBlackoutDates returns a boolean if a field has been set.

### SetBlackoutDatesNil

`func (o *PublicInviteResponse) SetBlackoutDatesNil(b bool)`

 SetBlackoutDatesNil sets the value for BlackoutDates to be an explicit nil

### UnsetBlackoutDates
`func (o *PublicInviteResponse) UnsetBlackoutDates()`

UnsetBlackoutDates ensures that no value is present for BlackoutDates, not even an explicit nil
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
### GetContactChallengeRequired

`func (o *PublicInviteResponse) GetContactChallengeRequired() bool`

GetContactChallengeRequired returns the ContactChallengeRequired field if non-nil, zero value otherwise.

### GetContactChallengeRequiredOk

`func (o *PublicInviteResponse) GetContactChallengeRequiredOk() (*bool, bool)`

GetContactChallengeRequiredOk returns a tuple with the ContactChallengeRequired field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetContactChallengeRequired

`func (o *PublicInviteResponse) SetContactChallengeRequired(v bool)`

SetContactChallengeRequired sets ContactChallengeRequired field to given value.

### HasContactChallengeRequired

`func (o *PublicInviteResponse) HasContactChallengeRequired() bool`

HasContactChallengeRequired returns a boolean if a field has been set.

### GetContactDisplayName

`func (o *PublicInviteResponse) GetContactDisplayName() string`

GetContactDisplayName returns the ContactDisplayName field if non-nil, zero value otherwise.

### GetContactDisplayNameOk

`func (o *PublicInviteResponse) GetContactDisplayNameOk() (*string, bool)`

GetContactDisplayNameOk returns a tuple with the ContactDisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetContactDisplayName

`func (o *PublicInviteResponse) SetContactDisplayName(v string)`

SetContactDisplayName sets ContactDisplayName field to given value.

### HasContactDisplayName

`func (o *PublicInviteResponse) HasContactDisplayName() bool`

HasContactDisplayName returns a boolean if a field has been set.

### SetContactDisplayNameNil

`func (o *PublicInviteResponse) SetContactDisplayNameNil(b bool)`

 SetContactDisplayNameNil sets the value for ContactDisplayName to be an explicit nil

### UnsetContactDisplayName
`func (o *PublicInviteResponse) UnsetContactDisplayName()`

UnsetContactDisplayName ensures that no value is present for ContactDisplayName, not even an explicit nil
### GetCurfewWindows

`func (o *PublicInviteResponse) GetCurfewWindows() []CurfewWindowResponse`

GetCurfewWindows returns the CurfewWindows field if non-nil, zero value otherwise.

### GetCurfewWindowsOk

`func (o *PublicInviteResponse) GetCurfewWindowsOk() (*[]CurfewWindowResponse, bool)`

GetCurfewWindowsOk returns a tuple with the CurfewWindows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurfewWindows

`func (o *PublicInviteResponse) SetCurfewWindows(v []CurfewWindowResponse)`

SetCurfewWindows sets CurfewWindows field to given value.

### HasCurfewWindows

`func (o *PublicInviteResponse) HasCurfewWindows() bool`

HasCurfewWindows returns a boolean if a field has been set.

### SetCurfewWindowsNil

`func (o *PublicInviteResponse) SetCurfewWindowsNil(b bool)`

 SetCurfewWindowsNil sets the value for CurfewWindows to be an explicit nil

### UnsetCurfewWindows
`func (o *PublicInviteResponse) UnsetCurfewWindows()`

UnsetCurfewWindows ensures that no value is present for CurfewWindows, not even an explicit nil
### GetDevicesCount

`func (o *PublicInviteResponse) GetDevicesCount() int32`

GetDevicesCount returns the DevicesCount field if non-nil, zero value otherwise.

### GetDevicesCountOk

`func (o *PublicInviteResponse) GetDevicesCountOk() (*int32, bool)`

GetDevicesCountOk returns a tuple with the DevicesCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDevicesCount

`func (o *PublicInviteResponse) SetDevicesCount(v int32)`

SetDevicesCount sets DevicesCount field to given value.

### HasDevicesCount

`func (o *PublicInviteResponse) HasDevicesCount() bool`

HasDevicesCount returns a boolean if a field has been set.

### SetDevicesCountNil

`func (o *PublicInviteResponse) SetDevicesCountNil(b bool)`

 SetDevicesCountNil sets the value for DevicesCount to be an explicit nil

### UnsetDevicesCount
`func (o *PublicInviteResponse) UnsetDevicesCount()`

UnsetDevicesCount ensures that no value is present for DevicesCount, not even an explicit nil
### GetEventLocation

`func (o *PublicInviteResponse) GetEventLocation() LocationResponse`

GetEventLocation returns the EventLocation field if non-nil, zero value otherwise.

### GetEventLocationOk

`func (o *PublicInviteResponse) GetEventLocationOk() (*LocationResponse, bool)`

GetEventLocationOk returns a tuple with the EventLocation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEventLocation

`func (o *PublicInviteResponse) SetEventLocation(v LocationResponse)`

SetEventLocation sets EventLocation field to given value.

### HasEventLocation

`func (o *PublicInviteResponse) HasEventLocation() bool`

HasEventLocation returns a boolean if a field has been set.

### SetEventLocationNil

`func (o *PublicInviteResponse) SetEventLocationNil(b bool)`

 SetEventLocationNil sets the value for EventLocation to be an explicit nil

### UnsetEventLocation
`func (o *PublicInviteResponse) UnsetEventLocation()`

UnsetEventLocation ensures that no value is present for EventLocation, not even an explicit nil
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


### GetInviteUrl

`func (o *PublicInviteResponse) GetInviteUrl() string`

GetInviteUrl returns the InviteUrl field if non-nil, zero value otherwise.

### GetInviteUrlOk

`func (o *PublicInviteResponse) GetInviteUrlOk() (*string, bool)`

GetInviteUrlOk returns a tuple with the InviteUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteUrl

`func (o *PublicInviteResponse) SetInviteUrl(v string)`

SetInviteUrl sets InviteUrl field to given value.


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
### GetLocationMode

`func (o *PublicInviteResponse) GetLocationMode() LocationBindingMode`

GetLocationMode returns the LocationMode field if non-nil, zero value otherwise.

### GetLocationModeOk

`func (o *PublicInviteResponse) GetLocationModeOk() (*LocationBindingMode, bool)`

GetLocationModeOk returns a tuple with the LocationMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocationMode

`func (o *PublicInviteResponse) SetLocationMode(v LocationBindingMode)`

SetLocationMode sets LocationMode field to given value.


### GetMaxDevices

`func (o *PublicInviteResponse) GetMaxDevices() int32`

GetMaxDevices returns the MaxDevices field if non-nil, zero value otherwise.

### GetMaxDevicesOk

`func (o *PublicInviteResponse) GetMaxDevicesOk() (*int32, bool)`

GetMaxDevicesOk returns a tuple with the MaxDevices field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxDevices

`func (o *PublicInviteResponse) SetMaxDevices(v int32)`

SetMaxDevices sets MaxDevices field to given value.

### HasMaxDevices

`func (o *PublicInviteResponse) HasMaxDevices() bool`

HasMaxDevices returns a boolean if a field has been set.

### SetMaxDevicesNil

`func (o *PublicInviteResponse) SetMaxDevicesNil(b bool)`

 SetMaxDevicesNil sets the value for MaxDevices to be an explicit nil

### UnsetMaxDevices
`func (o *PublicInviteResponse) UnsetMaxDevices()`

UnsetMaxDevices ensures that no value is present for MaxDevices, not even an explicit nil
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
### GetPhotoUrl

`func (o *PublicInviteResponse) GetPhotoUrl() string`

GetPhotoUrl returns the PhotoUrl field if non-nil, zero value otherwise.

### GetPhotoUrlOk

`func (o *PublicInviteResponse) GetPhotoUrlOk() (*string, bool)`

GetPhotoUrlOk returns a tuple with the PhotoUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhotoUrl

`func (o *PublicInviteResponse) SetPhotoUrl(v string)`

SetPhotoUrl sets PhotoUrl field to given value.

### HasPhotoUrl

`func (o *PublicInviteResponse) HasPhotoUrl() bool`

HasPhotoUrl returns a boolean if a field has been set.

### SetPhotoUrlNil

`func (o *PublicInviteResponse) SetPhotoUrlNil(b bool)`

 SetPhotoUrlNil sets the value for PhotoUrl to be an explicit nil

### UnsetPhotoUrl
`func (o *PublicInviteResponse) UnsetPhotoUrl()`

UnsetPhotoUrl ensures that no value is present for PhotoUrl, not even an explicit nil
### GetRenewalRequested

`func (o *PublicInviteResponse) GetRenewalRequested() bool`

GetRenewalRequested returns the RenewalRequested field if non-nil, zero value otherwise.

### GetRenewalRequestedOk

`func (o *PublicInviteResponse) GetRenewalRequestedOk() (*bool, bool)`

GetRenewalRequestedOk returns a tuple with the RenewalRequested field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRenewalRequested

`func (o *PublicInviteResponse) SetRenewalRequested(v bool)`

SetRenewalRequested sets RenewalRequested field to given value.

### HasRenewalRequested

`func (o *PublicInviteResponse) HasRenewalRequested() bool`

HasRenewalRequested returns a boolean if a field has been set.

### SetRenewalRequestedNil

`func (o *PublicInviteResponse) SetRenewalRequestedNil(b bool)`

 SetRenewalRequestedNil sets the value for RenewalRequested to be an explicit nil

### UnsetRenewalRequested
`func (o *PublicInviteResponse) UnsetRenewalRequested()`

UnsetRenewalRequested ensures that no value is present for RenewalRequested, not even an explicit nil
### GetRequirePin

`func (o *PublicInviteResponse) GetRequirePin() bool`

GetRequirePin returns the RequirePin field if non-nil, zero value otherwise.

### GetRequirePinOk

`func (o *PublicInviteResponse) GetRequirePinOk() (*bool, bool)`

GetRequirePinOk returns a tuple with the RequirePin field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequirePin

`func (o *PublicInviteResponse) SetRequirePin(v bool)`

SetRequirePin sets RequirePin field to given value.

### HasRequirePin

`func (o *PublicInviteResponse) HasRequirePin() bool`

HasRequirePin returns a boolean if a field has been set.

### GetRequireVerifiedPhone

`func (o *PublicInviteResponse) GetRequireVerifiedPhone() bool`

GetRequireVerifiedPhone returns the RequireVerifiedPhone field if non-nil, zero value otherwise.

### GetRequireVerifiedPhoneOk

`func (o *PublicInviteResponse) GetRequireVerifiedPhoneOk() (*bool, bool)`

GetRequireVerifiedPhoneOk returns a tuple with the RequireVerifiedPhone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequireVerifiedPhone

`func (o *PublicInviteResponse) SetRequireVerifiedPhone(v bool)`

SetRequireVerifiedPhone sets RequireVerifiedPhone field to given value.

### HasRequireVerifiedPhone

`func (o *PublicInviteResponse) HasRequireVerifiedPhone() bool`

HasRequireVerifiedPhone returns a boolean if a field has been set.

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


### GetTimezone

`func (o *PublicInviteResponse) GetTimezone() string`

GetTimezone returns the Timezone field if non-nil, zero value otherwise.

### GetTimezoneOk

`func (o *PublicInviteResponse) GetTimezoneOk() (*string, bool)`

GetTimezoneOk returns a tuple with the Timezone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimezone

`func (o *PublicInviteResponse) SetTimezone(v string)`

SetTimezone sets Timezone field to given value.

### HasTimezone

`func (o *PublicInviteResponse) HasTimezone() bool`

HasTimezone returns a boolean if a field has been set.

### SetTimezoneNil

`func (o *PublicInviteResponse) SetTimezoneNil(b bool)`

 SetTimezoneNil sets the value for Timezone to be an explicit nil

### UnsetTimezone
`func (o *PublicInviteResponse) UnsetTimezone()`

UnsetTimezone ensures that no value is present for Timezone, not even an explicit nil
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
