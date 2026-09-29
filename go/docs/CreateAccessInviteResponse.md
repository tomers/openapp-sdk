# CreateAccessInviteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreationJustification** | Pointer to **NullableString** |  | [optional]
**CurfewExempt** | Pointer to **bool** | Admin-only emergency exception: skip invitation curfew at redemption. | [optional]
**CurfewWindows** | Pointer to [**[]CurfewWindowResponse**](CurfewWindowResponse.md) | Effective forbidden hour windows (informational; invites are never mutated). | [optional]
**DevicesCount** | Pointer to **NullableInt32** |  | [optional]
**ExternalRef** | Pointer to [**NullableExternalReferenceResponse**](ExternalReferenceResponse.md) | External system-of-record correlation stored with the invitation. | [optional]
**GrantedPortals** | [**[]AccessInviteGrantedPortal**](AccessInviteGrantedPortal.md) |  |
**IdempotentReplay** | Pointer to **bool** | True when this body is a replay of an earlier create that carried the same &#x60;Idempotency-Key&#x60;. Nothing was created by this request and the one-time token fields are omitted. | [optional]
**InviteLinkId** | **string** |  |
**InviteRecurrence** | Pointer to **interface{}** |  | [optional]
**InviteToken** | Pointer to **NullableString** | Guest bearer token, returned **once** by the original create call.  It is stored only as a hash, never logged, and never cached for idempotent replay, so a replayed create response omits it. Persist it at delivery time or rotate it through &#x60;POST /access-invites/{id}/regenerate-token&#x60;. | [optional]
**InviteUrl** | Pointer to **NullableString** | Guest-facing invite URL (shareable link for SMS/email). Same one-time visibility as &#x60;invite_token&#x60;, and subject to the same logging restrictions. | [optional]
**InviteeMessage** | Pointer to **interface{}** |  | [optional]
**IsEnabled** | **bool** |  |
**Kind** | Pointer to [**AccessInviteKind**](AccessInviteKind.md) |  | [optional]
**Location** | Pointer to [**NullableLocationResponse**](LocationResponse.md) | Resolved event location for this invite (see &#x60;location_mode&#x60;). | [optional]
**LocationMode** | [**LocationBindingMode**](LocationBindingMode.md) | How the event location is resolved: &#x60;none&#x60;, &#x60;explicit&#x60;, or &#x60;inherit&#x60; (building). |
**MaxDevices** | Pointer to **NullableInt32** |  | [optional]
**MaxUses** | Pointer to **NullableInt32** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**PendingApproval** | Pointer to **bool** | True when a &#x60;user_sharing&#x60; policy required admin approval: the invite was created disabled and an approval request is pending. The invite activates once an admin approves it. | [optional]
**Schedule** | [**InviteScheduleSnapshot**](InviteScheduleSnapshot.md) |  |
**ScheduleCombined** | [**InviteScheduleCombined**](InviteScheduleCombined.md) |  |
**ScheduleEntries** | [**[]InviteScheduleEntrySnapshot**](InviteScheduleEntrySnapshot.md) |  |
**ScheduleKind** | [**InviteScheduleKind**](InviteScheduleKind.md) |  |
**Timezone** | Pointer to **NullableString** | Org IANA timezone used to evaluate invitation curfews. | [optional]
**Uses** | **int32** |  |
**ValidFrom** | **string** |  |
**ValidTo** | **string** |  |

## Methods

### NewCreateAccessInviteResponse

`func NewCreateAccessInviteResponse(grantedPortals []AccessInviteGrantedPortal, inviteLinkId string, isEnabled bool, locationMode LocationBindingMode, schedule InviteScheduleSnapshot, scheduleCombined InviteScheduleCombined, scheduleEntries []InviteScheduleEntrySnapshot, scheduleKind InviteScheduleKind, uses int32, validFrom string, validTo string, ) *CreateAccessInviteResponse`

NewCreateAccessInviteResponse instantiates a new CreateAccessInviteResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateAccessInviteResponseWithDefaults

`func NewCreateAccessInviteResponseWithDefaults() *CreateAccessInviteResponse`

NewCreateAccessInviteResponseWithDefaults instantiates a new CreateAccessInviteResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreationJustification

`func (o *CreateAccessInviteResponse) GetCreationJustification() string`

GetCreationJustification returns the CreationJustification field if non-nil, zero value otherwise.

### GetCreationJustificationOk

`func (o *CreateAccessInviteResponse) GetCreationJustificationOk() (*string, bool)`

GetCreationJustificationOk returns a tuple with the CreationJustification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreationJustification

`func (o *CreateAccessInviteResponse) SetCreationJustification(v string)`

SetCreationJustification sets CreationJustification field to given value.

### HasCreationJustification

`func (o *CreateAccessInviteResponse) HasCreationJustification() bool`

HasCreationJustification returns a boolean if a field has been set.

### SetCreationJustificationNil

`func (o *CreateAccessInviteResponse) SetCreationJustificationNil(b bool)`

 SetCreationJustificationNil sets the value for CreationJustification to be an explicit nil

### UnsetCreationJustification
`func (o *CreateAccessInviteResponse) UnsetCreationJustification()`

UnsetCreationJustification ensures that no value is present for CreationJustification, not even an explicit nil
### GetCurfewExempt

`func (o *CreateAccessInviteResponse) GetCurfewExempt() bool`

GetCurfewExempt returns the CurfewExempt field if non-nil, zero value otherwise.

### GetCurfewExemptOk

`func (o *CreateAccessInviteResponse) GetCurfewExemptOk() (*bool, bool)`

GetCurfewExemptOk returns a tuple with the CurfewExempt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurfewExempt

`func (o *CreateAccessInviteResponse) SetCurfewExempt(v bool)`

SetCurfewExempt sets CurfewExempt field to given value.

### HasCurfewExempt

`func (o *CreateAccessInviteResponse) HasCurfewExempt() bool`

HasCurfewExempt returns a boolean if a field has been set.

### GetCurfewWindows

`func (o *CreateAccessInviteResponse) GetCurfewWindows() []CurfewWindowResponse`

GetCurfewWindows returns the CurfewWindows field if non-nil, zero value otherwise.

### GetCurfewWindowsOk

`func (o *CreateAccessInviteResponse) GetCurfewWindowsOk() (*[]CurfewWindowResponse, bool)`

GetCurfewWindowsOk returns a tuple with the CurfewWindows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurfewWindows

`func (o *CreateAccessInviteResponse) SetCurfewWindows(v []CurfewWindowResponse)`

SetCurfewWindows sets CurfewWindows field to given value.

### HasCurfewWindows

`func (o *CreateAccessInviteResponse) HasCurfewWindows() bool`

HasCurfewWindows returns a boolean if a field has been set.

### SetCurfewWindowsNil

`func (o *CreateAccessInviteResponse) SetCurfewWindowsNil(b bool)`

 SetCurfewWindowsNil sets the value for CurfewWindows to be an explicit nil

### UnsetCurfewWindows
`func (o *CreateAccessInviteResponse) UnsetCurfewWindows()`

UnsetCurfewWindows ensures that no value is present for CurfewWindows, not even an explicit nil
### GetDevicesCount

`func (o *CreateAccessInviteResponse) GetDevicesCount() int32`

GetDevicesCount returns the DevicesCount field if non-nil, zero value otherwise.

### GetDevicesCountOk

`func (o *CreateAccessInviteResponse) GetDevicesCountOk() (*int32, bool)`

GetDevicesCountOk returns a tuple with the DevicesCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDevicesCount

`func (o *CreateAccessInviteResponse) SetDevicesCount(v int32)`

SetDevicesCount sets DevicesCount field to given value.

### HasDevicesCount

`func (o *CreateAccessInviteResponse) HasDevicesCount() bool`

HasDevicesCount returns a boolean if a field has been set.

### SetDevicesCountNil

`func (o *CreateAccessInviteResponse) SetDevicesCountNil(b bool)`

 SetDevicesCountNil sets the value for DevicesCount to be an explicit nil

### UnsetDevicesCount
`func (o *CreateAccessInviteResponse) UnsetDevicesCount()`

UnsetDevicesCount ensures that no value is present for DevicesCount, not even an explicit nil
### GetExternalRef

`func (o *CreateAccessInviteResponse) GetExternalRef() ExternalReferenceResponse`

GetExternalRef returns the ExternalRef field if non-nil, zero value otherwise.

### GetExternalRefOk

`func (o *CreateAccessInviteResponse) GetExternalRefOk() (*ExternalReferenceResponse, bool)`

GetExternalRefOk returns a tuple with the ExternalRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalRef

`func (o *CreateAccessInviteResponse) SetExternalRef(v ExternalReferenceResponse)`

SetExternalRef sets ExternalRef field to given value.

### HasExternalRef

`func (o *CreateAccessInviteResponse) HasExternalRef() bool`

HasExternalRef returns a boolean if a field has been set.

### SetExternalRefNil

`func (o *CreateAccessInviteResponse) SetExternalRefNil(b bool)`

 SetExternalRefNil sets the value for ExternalRef to be an explicit nil

### UnsetExternalRef
`func (o *CreateAccessInviteResponse) UnsetExternalRef()`

UnsetExternalRef ensures that no value is present for ExternalRef, not even an explicit nil
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


### GetIdempotentReplay

`func (o *CreateAccessInviteResponse) GetIdempotentReplay() bool`

GetIdempotentReplay returns the IdempotentReplay field if non-nil, zero value otherwise.

### GetIdempotentReplayOk

`func (o *CreateAccessInviteResponse) GetIdempotentReplayOk() (*bool, bool)`

GetIdempotentReplayOk returns a tuple with the IdempotentReplay field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIdempotentReplay

`func (o *CreateAccessInviteResponse) SetIdempotentReplay(v bool)`

SetIdempotentReplay sets IdempotentReplay field to given value.

### HasIdempotentReplay

`func (o *CreateAccessInviteResponse) HasIdempotentReplay() bool`

HasIdempotentReplay returns a boolean if a field has been set.

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

### HasInviteToken

`func (o *CreateAccessInviteResponse) HasInviteToken() bool`

HasInviteToken returns a boolean if a field has been set.

### SetInviteTokenNil

`func (o *CreateAccessInviteResponse) SetInviteTokenNil(b bool)`

 SetInviteTokenNil sets the value for InviteToken to be an explicit nil

### UnsetInviteToken
`func (o *CreateAccessInviteResponse) UnsetInviteToken()`

UnsetInviteToken ensures that no value is present for InviteToken, not even an explicit nil
### GetInviteUrl

`func (o *CreateAccessInviteResponse) GetInviteUrl() string`

GetInviteUrl returns the InviteUrl field if non-nil, zero value otherwise.

### GetInviteUrlOk

`func (o *CreateAccessInviteResponse) GetInviteUrlOk() (*string, bool)`

GetInviteUrlOk returns a tuple with the InviteUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteUrl

`func (o *CreateAccessInviteResponse) SetInviteUrl(v string)`

SetInviteUrl sets InviteUrl field to given value.

### HasInviteUrl

`func (o *CreateAccessInviteResponse) HasInviteUrl() bool`

HasInviteUrl returns a boolean if a field has been set.

### SetInviteUrlNil

`func (o *CreateAccessInviteResponse) SetInviteUrlNil(b bool)`

 SetInviteUrlNil sets the value for InviteUrl to be an explicit nil

### UnsetInviteUrl
`func (o *CreateAccessInviteResponse) UnsetInviteUrl()`

UnsetInviteUrl ensures that no value is present for InviteUrl, not even an explicit nil
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


### GetKind

`func (o *CreateAccessInviteResponse) GetKind() AccessInviteKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *CreateAccessInviteResponse) GetKindOk() (*AccessInviteKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *CreateAccessInviteResponse) SetKind(v AccessInviteKind)`

SetKind sets Kind field to given value.

### HasKind

`func (o *CreateAccessInviteResponse) HasKind() bool`

HasKind returns a boolean if a field has been set.

### GetLocation

`func (o *CreateAccessInviteResponse) GetLocation() LocationResponse`

GetLocation returns the Location field if non-nil, zero value otherwise.

### GetLocationOk

`func (o *CreateAccessInviteResponse) GetLocationOk() (*LocationResponse, bool)`

GetLocationOk returns a tuple with the Location field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocation

`func (o *CreateAccessInviteResponse) SetLocation(v LocationResponse)`

SetLocation sets Location field to given value.

### HasLocation

`func (o *CreateAccessInviteResponse) HasLocation() bool`

HasLocation returns a boolean if a field has been set.

### SetLocationNil

`func (o *CreateAccessInviteResponse) SetLocationNil(b bool)`

 SetLocationNil sets the value for Location to be an explicit nil

### UnsetLocation
`func (o *CreateAccessInviteResponse) UnsetLocation()`

UnsetLocation ensures that no value is present for Location, not even an explicit nil
### GetLocationMode

`func (o *CreateAccessInviteResponse) GetLocationMode() LocationBindingMode`

GetLocationMode returns the LocationMode field if non-nil, zero value otherwise.

### GetLocationModeOk

`func (o *CreateAccessInviteResponse) GetLocationModeOk() (*LocationBindingMode, bool)`

GetLocationModeOk returns a tuple with the LocationMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocationMode

`func (o *CreateAccessInviteResponse) SetLocationMode(v LocationBindingMode)`

SetLocationMode sets LocationMode field to given value.


### GetMaxDevices

`func (o *CreateAccessInviteResponse) GetMaxDevices() int32`

GetMaxDevices returns the MaxDevices field if non-nil, zero value otherwise.

### GetMaxDevicesOk

`func (o *CreateAccessInviteResponse) GetMaxDevicesOk() (*int32, bool)`

GetMaxDevicesOk returns a tuple with the MaxDevices field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxDevices

`func (o *CreateAccessInviteResponse) SetMaxDevices(v int32)`

SetMaxDevices sets MaxDevices field to given value.

### HasMaxDevices

`func (o *CreateAccessInviteResponse) HasMaxDevices() bool`

HasMaxDevices returns a boolean if a field has been set.

### SetMaxDevicesNil

`func (o *CreateAccessInviteResponse) SetMaxDevicesNil(b bool)`

 SetMaxDevicesNil sets the value for MaxDevices to be an explicit nil

### UnsetMaxDevices
`func (o *CreateAccessInviteResponse) UnsetMaxDevices()`

UnsetMaxDevices ensures that no value is present for MaxDevices, not even an explicit nil
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
### GetPendingApproval

`func (o *CreateAccessInviteResponse) GetPendingApproval() bool`

GetPendingApproval returns the PendingApproval field if non-nil, zero value otherwise.

### GetPendingApprovalOk

`func (o *CreateAccessInviteResponse) GetPendingApprovalOk() (*bool, bool)`

GetPendingApprovalOk returns a tuple with the PendingApproval field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPendingApproval

`func (o *CreateAccessInviteResponse) SetPendingApproval(v bool)`

SetPendingApproval sets PendingApproval field to given value.

### HasPendingApproval

`func (o *CreateAccessInviteResponse) HasPendingApproval() bool`

HasPendingApproval returns a boolean if a field has been set.

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


### GetTimezone

`func (o *CreateAccessInviteResponse) GetTimezone() string`

GetTimezone returns the Timezone field if non-nil, zero value otherwise.

### GetTimezoneOk

`func (o *CreateAccessInviteResponse) GetTimezoneOk() (*string, bool)`

GetTimezoneOk returns a tuple with the Timezone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimezone

`func (o *CreateAccessInviteResponse) SetTimezone(v string)`

SetTimezone sets Timezone field to given value.

### HasTimezone

`func (o *CreateAccessInviteResponse) HasTimezone() bool`

HasTimezone returns a boolean if a field has been set.

### SetTimezoneNil

`func (o *CreateAccessInviteResponse) SetTimezoneNil(b bool)`

 SetTimezoneNil sets the value for Timezone to be an explicit nil

### UnsetTimezone
`func (o *CreateAccessInviteResponse) UnsetTimezone()`

UnsetTimezone ensures that no value is present for Timezone, not even an explicit nil
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
