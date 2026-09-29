# PreviewAccessInviteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CurfewExempt** | Pointer to **bool** |  | [optional]
**CurfewWindows** | Pointer to [**[]CurfewWindowResponse**](CurfewWindowResponse.md) |  | [optional]
**ExternalRef** | Pointer to [**NullableExternalReferenceResponse**](ExternalReferenceResponse.md) |  | [optional]
**GrantedPortals** | [**[]AccessInviteGrantedPortal**](AccessInviteGrantedPortal.md) |  |
**IsEnabled** | **bool** |  |
**Kind** | [**AccessInviteKind**](AccessInviteKind.md) |  |
**MaxDevices** | Pointer to **NullableInt32** |  | [optional]
**MaxUses** | Pointer to **NullableInt32** |  | [optional]
**PendingApproval** | **bool** | True when creating this invitation would land disabled awaiting admin approval. |
**RequiredIdentityFields** | **[]string** | Identity fields organization policy requires for this invitation: &#x60;pin&#x60;, &#x60;invitee_phone&#x60;, and/or &#x60;photo&#x60;. |
**RequiresJustification** | **bool** | Whether policy requires &#x60;creation_justification&#x60;. |
**Schedule** | [**InviteScheduleSnapshot**](InviteScheduleSnapshot.md) |  |
**ScheduleCombined** | [**InviteScheduleCombined**](InviteScheduleCombined.md) |  |
**ScheduleEntries** | [**[]InviteScheduleEntrySnapshot**](InviteScheduleEntrySnapshot.md) |  |
**ScheduleKind** | [**InviteScheduleKind**](InviteScheduleKind.md) |  |
**Timezone** | Pointer to **NullableString** | Org IANA timezone used to evaluate invitation curfews. | [optional]
**UnlinkedPortalIds** | **[]string** | Portals that would be granted but do not currently resolve to a live door device, so the guest link would not open them. |
**Valid** | **bool** | Always true on a 200. A request that would be rejected returns the same status, error code, and message that create would return. |
**ValidFrom** | **string** |  |
**ValidTo** | **string** |  |

## Methods

### NewPreviewAccessInviteResponse

`func NewPreviewAccessInviteResponse(grantedPortals []AccessInviteGrantedPortal, isEnabled bool, kind AccessInviteKind, pendingApproval bool, requiredIdentityFields []string, requiresJustification bool, schedule InviteScheduleSnapshot, scheduleCombined InviteScheduleCombined, scheduleEntries []InviteScheduleEntrySnapshot, scheduleKind InviteScheduleKind, unlinkedPortalIds []string, valid bool, validFrom string, validTo string, ) *PreviewAccessInviteResponse`

NewPreviewAccessInviteResponse instantiates a new PreviewAccessInviteResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPreviewAccessInviteResponseWithDefaults

`func NewPreviewAccessInviteResponseWithDefaults() *PreviewAccessInviteResponse`

NewPreviewAccessInviteResponseWithDefaults instantiates a new PreviewAccessInviteResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCurfewExempt

`func (o *PreviewAccessInviteResponse) GetCurfewExempt() bool`

GetCurfewExempt returns the CurfewExempt field if non-nil, zero value otherwise.

### GetCurfewExemptOk

`func (o *PreviewAccessInviteResponse) GetCurfewExemptOk() (*bool, bool)`

GetCurfewExemptOk returns a tuple with the CurfewExempt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurfewExempt

`func (o *PreviewAccessInviteResponse) SetCurfewExempt(v bool)`

SetCurfewExempt sets CurfewExempt field to given value.

### HasCurfewExempt

`func (o *PreviewAccessInviteResponse) HasCurfewExempt() bool`

HasCurfewExempt returns a boolean if a field has been set.

### GetCurfewWindows

`func (o *PreviewAccessInviteResponse) GetCurfewWindows() []CurfewWindowResponse`

GetCurfewWindows returns the CurfewWindows field if non-nil, zero value otherwise.

### GetCurfewWindowsOk

`func (o *PreviewAccessInviteResponse) GetCurfewWindowsOk() (*[]CurfewWindowResponse, bool)`

GetCurfewWindowsOk returns a tuple with the CurfewWindows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurfewWindows

`func (o *PreviewAccessInviteResponse) SetCurfewWindows(v []CurfewWindowResponse)`

SetCurfewWindows sets CurfewWindows field to given value.

### HasCurfewWindows

`func (o *PreviewAccessInviteResponse) HasCurfewWindows() bool`

HasCurfewWindows returns a boolean if a field has been set.

### SetCurfewWindowsNil

`func (o *PreviewAccessInviteResponse) SetCurfewWindowsNil(b bool)`

 SetCurfewWindowsNil sets the value for CurfewWindows to be an explicit nil

### UnsetCurfewWindows
`func (o *PreviewAccessInviteResponse) UnsetCurfewWindows()`

UnsetCurfewWindows ensures that no value is present for CurfewWindows, not even an explicit nil
### GetExternalRef

`func (o *PreviewAccessInviteResponse) GetExternalRef() ExternalReferenceResponse`

GetExternalRef returns the ExternalRef field if non-nil, zero value otherwise.

### GetExternalRefOk

`func (o *PreviewAccessInviteResponse) GetExternalRefOk() (*ExternalReferenceResponse, bool)`

GetExternalRefOk returns a tuple with the ExternalRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalRef

`func (o *PreviewAccessInviteResponse) SetExternalRef(v ExternalReferenceResponse)`

SetExternalRef sets ExternalRef field to given value.

### HasExternalRef

`func (o *PreviewAccessInviteResponse) HasExternalRef() bool`

HasExternalRef returns a boolean if a field has been set.

### SetExternalRefNil

`func (o *PreviewAccessInviteResponse) SetExternalRefNil(b bool)`

 SetExternalRefNil sets the value for ExternalRef to be an explicit nil

### UnsetExternalRef
`func (o *PreviewAccessInviteResponse) UnsetExternalRef()`

UnsetExternalRef ensures that no value is present for ExternalRef, not even an explicit nil
### GetGrantedPortals

`func (o *PreviewAccessInviteResponse) GetGrantedPortals() []AccessInviteGrantedPortal`

GetGrantedPortals returns the GrantedPortals field if non-nil, zero value otherwise.

### GetGrantedPortalsOk

`func (o *PreviewAccessInviteResponse) GetGrantedPortalsOk() (*[]AccessInviteGrantedPortal, bool)`

GetGrantedPortalsOk returns a tuple with the GrantedPortals field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGrantedPortals

`func (o *PreviewAccessInviteResponse) SetGrantedPortals(v []AccessInviteGrantedPortal)`

SetGrantedPortals sets GrantedPortals field to given value.


### GetIsEnabled

`func (o *PreviewAccessInviteResponse) GetIsEnabled() bool`

GetIsEnabled returns the IsEnabled field if non-nil, zero value otherwise.

### GetIsEnabledOk

`func (o *PreviewAccessInviteResponse) GetIsEnabledOk() (*bool, bool)`

GetIsEnabledOk returns a tuple with the IsEnabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsEnabled

`func (o *PreviewAccessInviteResponse) SetIsEnabled(v bool)`

SetIsEnabled sets IsEnabled field to given value.


### GetKind

`func (o *PreviewAccessInviteResponse) GetKind() AccessInviteKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *PreviewAccessInviteResponse) GetKindOk() (*AccessInviteKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *PreviewAccessInviteResponse) SetKind(v AccessInviteKind)`

SetKind sets Kind field to given value.


### GetMaxDevices

`func (o *PreviewAccessInviteResponse) GetMaxDevices() int32`

GetMaxDevices returns the MaxDevices field if non-nil, zero value otherwise.

### GetMaxDevicesOk

`func (o *PreviewAccessInviteResponse) GetMaxDevicesOk() (*int32, bool)`

GetMaxDevicesOk returns a tuple with the MaxDevices field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxDevices

`func (o *PreviewAccessInviteResponse) SetMaxDevices(v int32)`

SetMaxDevices sets MaxDevices field to given value.

### HasMaxDevices

`func (o *PreviewAccessInviteResponse) HasMaxDevices() bool`

HasMaxDevices returns a boolean if a field has been set.

### SetMaxDevicesNil

`func (o *PreviewAccessInviteResponse) SetMaxDevicesNil(b bool)`

 SetMaxDevicesNil sets the value for MaxDevices to be an explicit nil

### UnsetMaxDevices
`func (o *PreviewAccessInviteResponse) UnsetMaxDevices()`

UnsetMaxDevices ensures that no value is present for MaxDevices, not even an explicit nil
### GetMaxUses

`func (o *PreviewAccessInviteResponse) GetMaxUses() int32`

GetMaxUses returns the MaxUses field if non-nil, zero value otherwise.

### GetMaxUsesOk

`func (o *PreviewAccessInviteResponse) GetMaxUsesOk() (*int32, bool)`

GetMaxUsesOk returns a tuple with the MaxUses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxUses

`func (o *PreviewAccessInviteResponse) SetMaxUses(v int32)`

SetMaxUses sets MaxUses field to given value.

### HasMaxUses

`func (o *PreviewAccessInviteResponse) HasMaxUses() bool`

HasMaxUses returns a boolean if a field has been set.

### SetMaxUsesNil

`func (o *PreviewAccessInviteResponse) SetMaxUsesNil(b bool)`

 SetMaxUsesNil sets the value for MaxUses to be an explicit nil

### UnsetMaxUses
`func (o *PreviewAccessInviteResponse) UnsetMaxUses()`

UnsetMaxUses ensures that no value is present for MaxUses, not even an explicit nil
### GetPendingApproval

`func (o *PreviewAccessInviteResponse) GetPendingApproval() bool`

GetPendingApproval returns the PendingApproval field if non-nil, zero value otherwise.

### GetPendingApprovalOk

`func (o *PreviewAccessInviteResponse) GetPendingApprovalOk() (*bool, bool)`

GetPendingApprovalOk returns a tuple with the PendingApproval field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPendingApproval

`func (o *PreviewAccessInviteResponse) SetPendingApproval(v bool)`

SetPendingApproval sets PendingApproval field to given value.


### GetRequiredIdentityFields

`func (o *PreviewAccessInviteResponse) GetRequiredIdentityFields() []string`

GetRequiredIdentityFields returns the RequiredIdentityFields field if non-nil, zero value otherwise.

### GetRequiredIdentityFieldsOk

`func (o *PreviewAccessInviteResponse) GetRequiredIdentityFieldsOk() (*[]string, bool)`

GetRequiredIdentityFieldsOk returns a tuple with the RequiredIdentityFields field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequiredIdentityFields

`func (o *PreviewAccessInviteResponse) SetRequiredIdentityFields(v []string)`

SetRequiredIdentityFields sets RequiredIdentityFields field to given value.


### GetRequiresJustification

`func (o *PreviewAccessInviteResponse) GetRequiresJustification() bool`

GetRequiresJustification returns the RequiresJustification field if non-nil, zero value otherwise.

### GetRequiresJustificationOk

`func (o *PreviewAccessInviteResponse) GetRequiresJustificationOk() (*bool, bool)`

GetRequiresJustificationOk returns a tuple with the RequiresJustification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequiresJustification

`func (o *PreviewAccessInviteResponse) SetRequiresJustification(v bool)`

SetRequiresJustification sets RequiresJustification field to given value.


### GetSchedule

`func (o *PreviewAccessInviteResponse) GetSchedule() InviteScheduleSnapshot`

GetSchedule returns the Schedule field if non-nil, zero value otherwise.

### GetScheduleOk

`func (o *PreviewAccessInviteResponse) GetScheduleOk() (*InviteScheduleSnapshot, bool)`

GetScheduleOk returns a tuple with the Schedule field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchedule

`func (o *PreviewAccessInviteResponse) SetSchedule(v InviteScheduleSnapshot)`

SetSchedule sets Schedule field to given value.


### GetScheduleCombined

`func (o *PreviewAccessInviteResponse) GetScheduleCombined() InviteScheduleCombined`

GetScheduleCombined returns the ScheduleCombined field if non-nil, zero value otherwise.

### GetScheduleCombinedOk

`func (o *PreviewAccessInviteResponse) GetScheduleCombinedOk() (*InviteScheduleCombined, bool)`

GetScheduleCombinedOk returns a tuple with the ScheduleCombined field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleCombined

`func (o *PreviewAccessInviteResponse) SetScheduleCombined(v InviteScheduleCombined)`

SetScheduleCombined sets ScheduleCombined field to given value.


### GetScheduleEntries

`func (o *PreviewAccessInviteResponse) GetScheduleEntries() []InviteScheduleEntrySnapshot`

GetScheduleEntries returns the ScheduleEntries field if non-nil, zero value otherwise.

### GetScheduleEntriesOk

`func (o *PreviewAccessInviteResponse) GetScheduleEntriesOk() (*[]InviteScheduleEntrySnapshot, bool)`

GetScheduleEntriesOk returns a tuple with the ScheduleEntries field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleEntries

`func (o *PreviewAccessInviteResponse) SetScheduleEntries(v []InviteScheduleEntrySnapshot)`

SetScheduleEntries sets ScheduleEntries field to given value.


### GetScheduleKind

`func (o *PreviewAccessInviteResponse) GetScheduleKind() InviteScheduleKind`

GetScheduleKind returns the ScheduleKind field if non-nil, zero value otherwise.

### GetScheduleKindOk

`func (o *PreviewAccessInviteResponse) GetScheduleKindOk() (*InviteScheduleKind, bool)`

GetScheduleKindOk returns a tuple with the ScheduleKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleKind

`func (o *PreviewAccessInviteResponse) SetScheduleKind(v InviteScheduleKind)`

SetScheduleKind sets ScheduleKind field to given value.


### GetTimezone

`func (o *PreviewAccessInviteResponse) GetTimezone() string`

GetTimezone returns the Timezone field if non-nil, zero value otherwise.

### GetTimezoneOk

`func (o *PreviewAccessInviteResponse) GetTimezoneOk() (*string, bool)`

GetTimezoneOk returns a tuple with the Timezone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimezone

`func (o *PreviewAccessInviteResponse) SetTimezone(v string)`

SetTimezone sets Timezone field to given value.

### HasTimezone

`func (o *PreviewAccessInviteResponse) HasTimezone() bool`

HasTimezone returns a boolean if a field has been set.

### SetTimezoneNil

`func (o *PreviewAccessInviteResponse) SetTimezoneNil(b bool)`

 SetTimezoneNil sets the value for Timezone to be an explicit nil

### UnsetTimezone
`func (o *PreviewAccessInviteResponse) UnsetTimezone()`

UnsetTimezone ensures that no value is present for Timezone, not even an explicit nil
### GetUnlinkedPortalIds

`func (o *PreviewAccessInviteResponse) GetUnlinkedPortalIds() []string`

GetUnlinkedPortalIds returns the UnlinkedPortalIds field if non-nil, zero value otherwise.

### GetUnlinkedPortalIdsOk

`func (o *PreviewAccessInviteResponse) GetUnlinkedPortalIdsOk() (*[]string, bool)`

GetUnlinkedPortalIdsOk returns a tuple with the UnlinkedPortalIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnlinkedPortalIds

`func (o *PreviewAccessInviteResponse) SetUnlinkedPortalIds(v []string)`

SetUnlinkedPortalIds sets UnlinkedPortalIds field to given value.


### GetValid

`func (o *PreviewAccessInviteResponse) GetValid() bool`

GetValid returns the Valid field if non-nil, zero value otherwise.

### GetValidOk

`func (o *PreviewAccessInviteResponse) GetValidOk() (*bool, bool)`

GetValidOk returns a tuple with the Valid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValid

`func (o *PreviewAccessInviteResponse) SetValid(v bool)`

SetValid sets Valid field to given value.


### GetValidFrom

`func (o *PreviewAccessInviteResponse) GetValidFrom() string`

GetValidFrom returns the ValidFrom field if non-nil, zero value otherwise.

### GetValidFromOk

`func (o *PreviewAccessInviteResponse) GetValidFromOk() (*string, bool)`

GetValidFromOk returns a tuple with the ValidFrom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidFrom

`func (o *PreviewAccessInviteResponse) SetValidFrom(v string)`

SetValidFrom sets ValidFrom field to given value.


### GetValidTo

`func (o *PreviewAccessInviteResponse) GetValidTo() string`

GetValidTo returns the ValidTo field if non-nil, zero value otherwise.

### GetValidToOk

`func (o *PreviewAccessInviteResponse) GetValidToOk() (*string, bool)`

GetValidToOk returns a tuple with the ValidTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidTo

`func (o *PreviewAccessInviteResponse) SetValidTo(v string)`

SetValidTo sets ValidTo field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
