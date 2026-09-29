# PaginatedResponseAccessInviteListItemItemsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**CreatedByUserId** | Pointer to **NullableString** | User who created this invite or share. Omitted when the row has no creator. | [optional]
**CreationJustification** | Pointer to **NullableString** |  | [optional]
**CurfewExempt** | Pointer to **bool** | Admin-only emergency exception: skip invitation curfew at redemption. | [optional]
**DevicesCount** | Pointer to **NullableInt32** |  | [optional]
**DisabledJustification** | Pointer to **NullableString** |  | [optional]
**ExternalRef** | Pointer to [**NullableExternalReferenceResponse**](ExternalReferenceResponse.md) | External system-of-record correlation, when this invitation is driven by an adapter. | [optional]
**GrantedPortals** | [**[]AccessInviteGrantedPortal**](AccessInviteGrantedPortal.md) |  |
**Id** | **string** |  |
**InviteRecurrence** | Pointer to **interface{}** |  | [optional]
**InviteeMessage** | Pointer to **interface{}** |  | [optional]
**IsEnabled** | **bool** |  |
**Kind** | Pointer to [**AccessInviteKind**](AccessInviteKind.md) |  | [optional]
**LastSharedAt** | Pointer to **NullableString** |  | [optional]
**LastUsedAt** | Pointer to **NullableString** |  | [optional]
**Location** | Pointer to [**NullableLocationResponse**](LocationResponse.md) | Resolved event location for this invite (see &#x60;location_mode&#x60;): the invite&#39;s own location when &#x60;explicit&#x60;, the building location when &#x60;inherit&#x60;, or absent when &#x60;none&#x60;. | [optional]
**LocationMode** | [**LocationBindingMode**](LocationBindingMode.md) | How the event location is resolved: &#x60;none&#x60;, &#x60;explicit&#x60;, or &#x60;inherit&#x60; (building). |
**MaxDevices** | Pointer to **NullableInt32** |  | [optional]
**MaxUses** | Pointer to **NullableInt32** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**PendingRenewalCount** | Pointer to **NullableInt32** |  | [optional]
**PhotoThumbUrl** | Pointer to **NullableString** | Presigned URL for the &#x60;thumb&#x60; rendition of the invite photo, for avatar-sized renders. Absent when the source is already thumb-sized; fall back to &#x60;photo_url&#x60;. | [optional]
**PhotoUrl** | Pointer to **NullableString** | Presigned URL for the invite&#39;s main photo (best-effort). | [optional]
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

### NewPaginatedResponseAccessInviteListItemItemsInner

`func NewPaginatedResponseAccessInviteListItemItemsInner(grantedPortals []AccessInviteGrantedPortal, id string, isEnabled bool, locationMode LocationBindingMode, schedule InviteScheduleSnapshot, scheduleCombined InviteScheduleCombined, scheduleEntries []InviteScheduleEntrySnapshot, scheduleKind InviteScheduleKind, state string, uses int32, ) *PaginatedResponseAccessInviteListItemItemsInner`

NewPaginatedResponseAccessInviteListItemItemsInner instantiates a new PaginatedResponseAccessInviteListItemItemsInner object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseAccessInviteListItemItemsInnerWithDefaults

`func NewPaginatedResponseAccessInviteListItemItemsInnerWithDefaults() *PaginatedResponseAccessInviteListItemItemsInner`

NewPaginatedResponseAccessInviteListItemItemsInnerWithDefaults instantiates a new PaginatedResponseAccessInviteListItemItemsInner object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetCreatedByUserId

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetCreatedByUserId() string`

GetCreatedByUserId returns the CreatedByUserId field if non-nil, zero value otherwise.

### GetCreatedByUserIdOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetCreatedByUserIdOk() (*string, bool)`

GetCreatedByUserIdOk returns a tuple with the CreatedByUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedByUserId

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetCreatedByUserId(v string)`

SetCreatedByUserId sets CreatedByUserId field to given value.

### HasCreatedByUserId

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasCreatedByUserId() bool`

HasCreatedByUserId returns a boolean if a field has been set.

### SetCreatedByUserIdNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetCreatedByUserIdNil(b bool)`

 SetCreatedByUserIdNil sets the value for CreatedByUserId to be an explicit nil

### UnsetCreatedByUserId
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetCreatedByUserId()`

UnsetCreatedByUserId ensures that no value is present for CreatedByUserId, not even an explicit nil
### GetCreationJustification

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetCreationJustification() string`

GetCreationJustification returns the CreationJustification field if non-nil, zero value otherwise.

### GetCreationJustificationOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetCreationJustificationOk() (*string, bool)`

GetCreationJustificationOk returns a tuple with the CreationJustification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreationJustification

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetCreationJustification(v string)`

SetCreationJustification sets CreationJustification field to given value.

### HasCreationJustification

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasCreationJustification() bool`

HasCreationJustification returns a boolean if a field has been set.

### SetCreationJustificationNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetCreationJustificationNil(b bool)`

 SetCreationJustificationNil sets the value for CreationJustification to be an explicit nil

### UnsetCreationJustification
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetCreationJustification()`

UnsetCreationJustification ensures that no value is present for CreationJustification, not even an explicit nil
### GetCurfewExempt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetCurfewExempt() bool`

GetCurfewExempt returns the CurfewExempt field if non-nil, zero value otherwise.

### GetCurfewExemptOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetCurfewExemptOk() (*bool, bool)`

GetCurfewExemptOk returns a tuple with the CurfewExempt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurfewExempt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetCurfewExempt(v bool)`

SetCurfewExempt sets CurfewExempt field to given value.

### HasCurfewExempt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasCurfewExempt() bool`

HasCurfewExempt returns a boolean if a field has been set.

### GetDevicesCount

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetDevicesCount() int32`

GetDevicesCount returns the DevicesCount field if non-nil, zero value otherwise.

### GetDevicesCountOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetDevicesCountOk() (*int32, bool)`

GetDevicesCountOk returns a tuple with the DevicesCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDevicesCount

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetDevicesCount(v int32)`

SetDevicesCount sets DevicesCount field to given value.

### HasDevicesCount

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasDevicesCount() bool`

HasDevicesCount returns a boolean if a field has been set.

### SetDevicesCountNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetDevicesCountNil(b bool)`

 SetDevicesCountNil sets the value for DevicesCount to be an explicit nil

### UnsetDevicesCount
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetDevicesCount()`

UnsetDevicesCount ensures that no value is present for DevicesCount, not even an explicit nil
### GetDisabledJustification

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetDisabledJustification() string`

GetDisabledJustification returns the DisabledJustification field if non-nil, zero value otherwise.

### GetDisabledJustificationOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetDisabledJustificationOk() (*string, bool)`

GetDisabledJustificationOk returns a tuple with the DisabledJustification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisabledJustification

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetDisabledJustification(v string)`

SetDisabledJustification sets DisabledJustification field to given value.

### HasDisabledJustification

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasDisabledJustification() bool`

HasDisabledJustification returns a boolean if a field has been set.

### SetDisabledJustificationNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetDisabledJustificationNil(b bool)`

 SetDisabledJustificationNil sets the value for DisabledJustification to be an explicit nil

### UnsetDisabledJustification
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetDisabledJustification()`

UnsetDisabledJustification ensures that no value is present for DisabledJustification, not even an explicit nil
### GetExternalRef

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetExternalRef() ExternalReferenceResponse`

GetExternalRef returns the ExternalRef field if non-nil, zero value otherwise.

### GetExternalRefOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetExternalRefOk() (*ExternalReferenceResponse, bool)`

GetExternalRefOk returns a tuple with the ExternalRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalRef

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetExternalRef(v ExternalReferenceResponse)`

SetExternalRef sets ExternalRef field to given value.

### HasExternalRef

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasExternalRef() bool`

HasExternalRef returns a boolean if a field has been set.

### SetExternalRefNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetExternalRefNil(b bool)`

 SetExternalRefNil sets the value for ExternalRef to be an explicit nil

### UnsetExternalRef
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetExternalRef()`

UnsetExternalRef ensures that no value is present for ExternalRef, not even an explicit nil
### GetGrantedPortals

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetGrantedPortals() []AccessInviteGrantedPortal`

GetGrantedPortals returns the GrantedPortals field if non-nil, zero value otherwise.

### GetGrantedPortalsOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetGrantedPortalsOk() (*[]AccessInviteGrantedPortal, bool)`

GetGrantedPortalsOk returns a tuple with the GrantedPortals field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGrantedPortals

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetGrantedPortals(v []AccessInviteGrantedPortal)`

SetGrantedPortals sets GrantedPortals field to given value.


### GetId

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetId(v string)`

SetId sets Id field to given value.


### GetInviteRecurrence

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetInviteRecurrence() interface{}`

GetInviteRecurrence returns the InviteRecurrence field if non-nil, zero value otherwise.

### GetInviteRecurrenceOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetInviteRecurrenceOk() (*interface{}, bool)`

GetInviteRecurrenceOk returns a tuple with the InviteRecurrence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteRecurrence

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetInviteRecurrence(v interface{})`

SetInviteRecurrence sets InviteRecurrence field to given value.

### HasInviteRecurrence

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasInviteRecurrence() bool`

HasInviteRecurrence returns a boolean if a field has been set.

### SetInviteRecurrenceNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetInviteRecurrenceNil(b bool)`

 SetInviteRecurrenceNil sets the value for InviteRecurrence to be an explicit nil

### UnsetInviteRecurrence
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetInviteRecurrence()`

UnsetInviteRecurrence ensures that no value is present for InviteRecurrence, not even an explicit nil
### GetInviteeMessage

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetInviteeMessage() interface{}`

GetInviteeMessage returns the InviteeMessage field if non-nil, zero value otherwise.

### GetInviteeMessageOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetInviteeMessageOk() (*interface{}, bool)`

GetInviteeMessageOk returns a tuple with the InviteeMessage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteeMessage

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetInviteeMessage(v interface{})`

SetInviteeMessage sets InviteeMessage field to given value.

### HasInviteeMessage

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasInviteeMessage() bool`

HasInviteeMessage returns a boolean if a field has been set.

### SetInviteeMessageNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetInviteeMessageNil(b bool)`

 SetInviteeMessageNil sets the value for InviteeMessage to be an explicit nil

### UnsetInviteeMessage
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetInviteeMessage()`

UnsetInviteeMessage ensures that no value is present for InviteeMessage, not even an explicit nil
### GetIsEnabled

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetIsEnabled() bool`

GetIsEnabled returns the IsEnabled field if non-nil, zero value otherwise.

### GetIsEnabledOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetIsEnabledOk() (*bool, bool)`

GetIsEnabledOk returns a tuple with the IsEnabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsEnabled

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetIsEnabled(v bool)`

SetIsEnabled sets IsEnabled field to given value.


### GetKind

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetKind() AccessInviteKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetKindOk() (*AccessInviteKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetKind(v AccessInviteKind)`

SetKind sets Kind field to given value.

### HasKind

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasKind() bool`

HasKind returns a boolean if a field has been set.

### GetLastSharedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetLastSharedAt() string`

GetLastSharedAt returns the LastSharedAt field if non-nil, zero value otherwise.

### GetLastSharedAtOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetLastSharedAtOk() (*string, bool)`

GetLastSharedAtOk returns a tuple with the LastSharedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastSharedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetLastSharedAt(v string)`

SetLastSharedAt sets LastSharedAt field to given value.

### HasLastSharedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasLastSharedAt() bool`

HasLastSharedAt returns a boolean if a field has been set.

### SetLastSharedAtNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetLastSharedAtNil(b bool)`

 SetLastSharedAtNil sets the value for LastSharedAt to be an explicit nil

### UnsetLastSharedAt
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetLastSharedAt()`

UnsetLastSharedAt ensures that no value is present for LastSharedAt, not even an explicit nil
### GetLastUsedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetLastUsedAt() string`

GetLastUsedAt returns the LastUsedAt field if non-nil, zero value otherwise.

### GetLastUsedAtOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetLastUsedAtOk() (*string, bool)`

GetLastUsedAtOk returns a tuple with the LastUsedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastUsedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetLastUsedAt(v string)`

SetLastUsedAt sets LastUsedAt field to given value.

### HasLastUsedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasLastUsedAt() bool`

HasLastUsedAt returns a boolean if a field has been set.

### SetLastUsedAtNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetLastUsedAtNil(b bool)`

 SetLastUsedAtNil sets the value for LastUsedAt to be an explicit nil

### UnsetLastUsedAt
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetLastUsedAt()`

UnsetLastUsedAt ensures that no value is present for LastUsedAt, not even an explicit nil
### GetLocation

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetLocation() LocationResponse`

GetLocation returns the Location field if non-nil, zero value otherwise.

### GetLocationOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetLocationOk() (*LocationResponse, bool)`

GetLocationOk returns a tuple with the Location field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocation

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetLocation(v LocationResponse)`

SetLocation sets Location field to given value.

### HasLocation

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasLocation() bool`

HasLocation returns a boolean if a field has been set.

### SetLocationNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetLocationNil(b bool)`

 SetLocationNil sets the value for Location to be an explicit nil

### UnsetLocation
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetLocation()`

UnsetLocation ensures that no value is present for Location, not even an explicit nil
### GetLocationMode

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetLocationMode() LocationBindingMode`

GetLocationMode returns the LocationMode field if non-nil, zero value otherwise.

### GetLocationModeOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetLocationModeOk() (*LocationBindingMode, bool)`

GetLocationModeOk returns a tuple with the LocationMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocationMode

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetLocationMode(v LocationBindingMode)`

SetLocationMode sets LocationMode field to given value.


### GetMaxDevices

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetMaxDevices() int32`

GetMaxDevices returns the MaxDevices field if non-nil, zero value otherwise.

### GetMaxDevicesOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetMaxDevicesOk() (*int32, bool)`

GetMaxDevicesOk returns a tuple with the MaxDevices field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxDevices

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetMaxDevices(v int32)`

SetMaxDevices sets MaxDevices field to given value.

### HasMaxDevices

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasMaxDevices() bool`

HasMaxDevices returns a boolean if a field has been set.

### SetMaxDevicesNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetMaxDevicesNil(b bool)`

 SetMaxDevicesNil sets the value for MaxDevices to be an explicit nil

### UnsetMaxDevices
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetMaxDevices()`

UnsetMaxDevices ensures that no value is present for MaxDevices, not even an explicit nil
### GetMaxUses

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetMaxUses() int32`

GetMaxUses returns the MaxUses field if non-nil, zero value otherwise.

### GetMaxUsesOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetMaxUsesOk() (*int32, bool)`

GetMaxUsesOk returns a tuple with the MaxUses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxUses

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetMaxUses(v int32)`

SetMaxUses sets MaxUses field to given value.

### HasMaxUses

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasMaxUses() bool`

HasMaxUses returns a boolean if a field has been set.

### SetMaxUsesNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetMaxUsesNil(b bool)`

 SetMaxUsesNil sets the value for MaxUses to be an explicit nil

### UnsetMaxUses
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetMaxUses()`

UnsetMaxUses ensures that no value is present for MaxUses, not even an explicit nil
### GetName

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetPendingRenewalCount

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetPendingRenewalCount() int32`

GetPendingRenewalCount returns the PendingRenewalCount field if non-nil, zero value otherwise.

### GetPendingRenewalCountOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetPendingRenewalCountOk() (*int32, bool)`

GetPendingRenewalCountOk returns a tuple with the PendingRenewalCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPendingRenewalCount

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetPendingRenewalCount(v int32)`

SetPendingRenewalCount sets PendingRenewalCount field to given value.

### HasPendingRenewalCount

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasPendingRenewalCount() bool`

HasPendingRenewalCount returns a boolean if a field has been set.

### SetPendingRenewalCountNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetPendingRenewalCountNil(b bool)`

 SetPendingRenewalCountNil sets the value for PendingRenewalCount to be an explicit nil

### UnsetPendingRenewalCount
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetPendingRenewalCount()`

UnsetPendingRenewalCount ensures that no value is present for PendingRenewalCount, not even an explicit nil
### GetPhotoThumbUrl

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetPhotoThumbUrl() string`

GetPhotoThumbUrl returns the PhotoThumbUrl field if non-nil, zero value otherwise.

### GetPhotoThumbUrlOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetPhotoThumbUrlOk() (*string, bool)`

GetPhotoThumbUrlOk returns a tuple with the PhotoThumbUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhotoThumbUrl

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetPhotoThumbUrl(v string)`

SetPhotoThumbUrl sets PhotoThumbUrl field to given value.

### HasPhotoThumbUrl

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasPhotoThumbUrl() bool`

HasPhotoThumbUrl returns a boolean if a field has been set.

### SetPhotoThumbUrlNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetPhotoThumbUrlNil(b bool)`

 SetPhotoThumbUrlNil sets the value for PhotoThumbUrl to be an explicit nil

### UnsetPhotoThumbUrl
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetPhotoThumbUrl()`

UnsetPhotoThumbUrl ensures that no value is present for PhotoThumbUrl, not even an explicit nil
### GetPhotoUrl

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetPhotoUrl() string`

GetPhotoUrl returns the PhotoUrl field if non-nil, zero value otherwise.

### GetPhotoUrlOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetPhotoUrlOk() (*string, bool)`

GetPhotoUrlOk returns a tuple with the PhotoUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhotoUrl

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetPhotoUrl(v string)`

SetPhotoUrl sets PhotoUrl field to given value.

### HasPhotoUrl

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasPhotoUrl() bool`

HasPhotoUrl returns a boolean if a field has been set.

### SetPhotoUrlNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetPhotoUrlNil(b bool)`

 SetPhotoUrlNil sets the value for PhotoUrl to be an explicit nil

### UnsetPhotoUrl
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetPhotoUrl()`

UnsetPhotoUrl ensures that no value is present for PhotoUrl, not even an explicit nil
### GetRevokedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetRevokedAt() string`

GetRevokedAt returns the RevokedAt field if non-nil, zero value otherwise.

### GetRevokedAtOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetRevokedAtOk() (*string, bool)`

GetRevokedAtOk returns a tuple with the RevokedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRevokedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetRevokedAt(v string)`

SetRevokedAt sets RevokedAt field to given value.

### HasRevokedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasRevokedAt() bool`

HasRevokedAt returns a boolean if a field has been set.

### SetRevokedAtNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetRevokedAtNil(b bool)`

 SetRevokedAtNil sets the value for RevokedAt to be an explicit nil

### UnsetRevokedAt
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetRevokedAt()`

UnsetRevokedAt ensures that no value is present for RevokedAt, not even an explicit nil
### GetSchedule

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetSchedule() InviteScheduleSnapshot`

GetSchedule returns the Schedule field if non-nil, zero value otherwise.

### GetScheduleOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetScheduleOk() (*InviteScheduleSnapshot, bool)`

GetScheduleOk returns a tuple with the Schedule field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchedule

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetSchedule(v InviteScheduleSnapshot)`

SetSchedule sets Schedule field to given value.


### GetScheduleCombined

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetScheduleCombined() InviteScheduleCombined`

GetScheduleCombined returns the ScheduleCombined field if non-nil, zero value otherwise.

### GetScheduleCombinedOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetScheduleCombinedOk() (*InviteScheduleCombined, bool)`

GetScheduleCombinedOk returns a tuple with the ScheduleCombined field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleCombined

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetScheduleCombined(v InviteScheduleCombined)`

SetScheduleCombined sets ScheduleCombined field to given value.


### GetScheduleEntries

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetScheduleEntries() []InviteScheduleEntrySnapshot`

GetScheduleEntries returns the ScheduleEntries field if non-nil, zero value otherwise.

### GetScheduleEntriesOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetScheduleEntriesOk() (*[]InviteScheduleEntrySnapshot, bool)`

GetScheduleEntriesOk returns a tuple with the ScheduleEntries field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleEntries

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetScheduleEntries(v []InviteScheduleEntrySnapshot)`

SetScheduleEntries sets ScheduleEntries field to given value.


### GetScheduleKind

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetScheduleKind() InviteScheduleKind`

GetScheduleKind returns the ScheduleKind field if non-nil, zero value otherwise.

### GetScheduleKindOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetScheduleKindOk() (*InviteScheduleKind, bool)`

GetScheduleKindOk returns a tuple with the ScheduleKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduleKind

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetScheduleKind(v InviteScheduleKind)`

SetScheduleKind sets ScheduleKind field to given value.


### GetState

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetState() string`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetStateOk() (*string, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetState(v string)`

SetState sets State field to given value.


### GetUpdatedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil
### GetUses

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetUses() int32`

GetUses returns the Uses field if non-nil, zero value otherwise.

### GetUsesOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetUsesOk() (*int32, bool)`

GetUsesOk returns a tuple with the Uses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUses

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetUses(v int32)`

SetUses sets Uses field to given value.


### GetValidFrom

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetValidFrom() string`

GetValidFrom returns the ValidFrom field if non-nil, zero value otherwise.

### GetValidFromOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetValidFromOk() (*string, bool)`

GetValidFromOk returns a tuple with the ValidFrom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidFrom

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetValidFrom(v string)`

SetValidFrom sets ValidFrom field to given value.

### HasValidFrom

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasValidFrom() bool`

HasValidFrom returns a boolean if a field has been set.

### SetValidFromNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetValidFromNil(b bool)`

 SetValidFromNil sets the value for ValidFrom to be an explicit nil

### UnsetValidFrom
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetValidFrom()`

UnsetValidFrom ensures that no value is present for ValidFrom, not even an explicit nil
### GetValidTo

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetValidTo() string`

GetValidTo returns the ValidTo field if non-nil, zero value otherwise.

### GetValidToOk

`func (o *PaginatedResponseAccessInviteListItemItemsInner) GetValidToOk() (*string, bool)`

GetValidToOk returns a tuple with the ValidTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidTo

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetValidTo(v string)`

SetValidTo sets ValidTo field to given value.

### HasValidTo

`func (o *PaginatedResponseAccessInviteListItemItemsInner) HasValidTo() bool`

HasValidTo returns a boolean if a field has been set.

### SetValidToNil

`func (o *PaginatedResponseAccessInviteListItemItemsInner) SetValidToNil(b bool)`

 SetValidToNil sets the value for ValidTo to be an explicit nil

### UnsetValidTo
`func (o *PaginatedResponseAccessInviteListItemItemsInner) UnsetValidTo()`

UnsetValidTo ensures that no value is present for ValidTo, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
