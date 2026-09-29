# AccessInviteListItem

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

### NewAccessInviteListItem

`func NewAccessInviteListItem(grantedPortals []AccessInviteGrantedPortal, id string, isEnabled bool, locationMode LocationBindingMode, schedule InviteScheduleSnapshot, scheduleCombined InviteScheduleCombined, scheduleEntries []InviteScheduleEntrySnapshot, scheduleKind InviteScheduleKind, state string, uses int32, ) *AccessInviteListItem`

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
### GetCreatedByUserId

`func (o *AccessInviteListItem) GetCreatedByUserId() string`

GetCreatedByUserId returns the CreatedByUserId field if non-nil, zero value otherwise.

### GetCreatedByUserIdOk

`func (o *AccessInviteListItem) GetCreatedByUserIdOk() (*string, bool)`

GetCreatedByUserIdOk returns a tuple with the CreatedByUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedByUserId

`func (o *AccessInviteListItem) SetCreatedByUserId(v string)`

SetCreatedByUserId sets CreatedByUserId field to given value.

### HasCreatedByUserId

`func (o *AccessInviteListItem) HasCreatedByUserId() bool`

HasCreatedByUserId returns a boolean if a field has been set.

### SetCreatedByUserIdNil

`func (o *AccessInviteListItem) SetCreatedByUserIdNil(b bool)`

 SetCreatedByUserIdNil sets the value for CreatedByUserId to be an explicit nil

### UnsetCreatedByUserId
`func (o *AccessInviteListItem) UnsetCreatedByUserId()`

UnsetCreatedByUserId ensures that no value is present for CreatedByUserId, not even an explicit nil
### GetCreationJustification

`func (o *AccessInviteListItem) GetCreationJustification() string`

GetCreationJustification returns the CreationJustification field if non-nil, zero value otherwise.

### GetCreationJustificationOk

`func (o *AccessInviteListItem) GetCreationJustificationOk() (*string, bool)`

GetCreationJustificationOk returns a tuple with the CreationJustification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreationJustification

`func (o *AccessInviteListItem) SetCreationJustification(v string)`

SetCreationJustification sets CreationJustification field to given value.

### HasCreationJustification

`func (o *AccessInviteListItem) HasCreationJustification() bool`

HasCreationJustification returns a boolean if a field has been set.

### SetCreationJustificationNil

`func (o *AccessInviteListItem) SetCreationJustificationNil(b bool)`

 SetCreationJustificationNil sets the value for CreationJustification to be an explicit nil

### UnsetCreationJustification
`func (o *AccessInviteListItem) UnsetCreationJustification()`

UnsetCreationJustification ensures that no value is present for CreationJustification, not even an explicit nil
### GetCurfewExempt

`func (o *AccessInviteListItem) GetCurfewExempt() bool`

GetCurfewExempt returns the CurfewExempt field if non-nil, zero value otherwise.

### GetCurfewExemptOk

`func (o *AccessInviteListItem) GetCurfewExemptOk() (*bool, bool)`

GetCurfewExemptOk returns a tuple with the CurfewExempt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurfewExempt

`func (o *AccessInviteListItem) SetCurfewExempt(v bool)`

SetCurfewExempt sets CurfewExempt field to given value.

### HasCurfewExempt

`func (o *AccessInviteListItem) HasCurfewExempt() bool`

HasCurfewExempt returns a boolean if a field has been set.

### GetDevicesCount

`func (o *AccessInviteListItem) GetDevicesCount() int32`

GetDevicesCount returns the DevicesCount field if non-nil, zero value otherwise.

### GetDevicesCountOk

`func (o *AccessInviteListItem) GetDevicesCountOk() (*int32, bool)`

GetDevicesCountOk returns a tuple with the DevicesCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDevicesCount

`func (o *AccessInviteListItem) SetDevicesCount(v int32)`

SetDevicesCount sets DevicesCount field to given value.

### HasDevicesCount

`func (o *AccessInviteListItem) HasDevicesCount() bool`

HasDevicesCount returns a boolean if a field has been set.

### SetDevicesCountNil

`func (o *AccessInviteListItem) SetDevicesCountNil(b bool)`

 SetDevicesCountNil sets the value for DevicesCount to be an explicit nil

### UnsetDevicesCount
`func (o *AccessInviteListItem) UnsetDevicesCount()`

UnsetDevicesCount ensures that no value is present for DevicesCount, not even an explicit nil
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
### GetExternalRef

`func (o *AccessInviteListItem) GetExternalRef() ExternalReferenceResponse`

GetExternalRef returns the ExternalRef field if non-nil, zero value otherwise.

### GetExternalRefOk

`func (o *AccessInviteListItem) GetExternalRefOk() (*ExternalReferenceResponse, bool)`

GetExternalRefOk returns a tuple with the ExternalRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalRef

`func (o *AccessInviteListItem) SetExternalRef(v ExternalReferenceResponse)`

SetExternalRef sets ExternalRef field to given value.

### HasExternalRef

`func (o *AccessInviteListItem) HasExternalRef() bool`

HasExternalRef returns a boolean if a field has been set.

### SetExternalRefNil

`func (o *AccessInviteListItem) SetExternalRefNil(b bool)`

 SetExternalRefNil sets the value for ExternalRef to be an explicit nil

### UnsetExternalRef
`func (o *AccessInviteListItem) UnsetExternalRef()`

UnsetExternalRef ensures that no value is present for ExternalRef, not even an explicit nil
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


### GetKind

`func (o *AccessInviteListItem) GetKind() AccessInviteKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *AccessInviteListItem) GetKindOk() (*AccessInviteKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *AccessInviteListItem) SetKind(v AccessInviteKind)`

SetKind sets Kind field to given value.

### HasKind

`func (o *AccessInviteListItem) HasKind() bool`

HasKind returns a boolean if a field has been set.

### GetLastSharedAt

`func (o *AccessInviteListItem) GetLastSharedAt() string`

GetLastSharedAt returns the LastSharedAt field if non-nil, zero value otherwise.

### GetLastSharedAtOk

`func (o *AccessInviteListItem) GetLastSharedAtOk() (*string, bool)`

GetLastSharedAtOk returns a tuple with the LastSharedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastSharedAt

`func (o *AccessInviteListItem) SetLastSharedAt(v string)`

SetLastSharedAt sets LastSharedAt field to given value.

### HasLastSharedAt

`func (o *AccessInviteListItem) HasLastSharedAt() bool`

HasLastSharedAt returns a boolean if a field has been set.

### SetLastSharedAtNil

`func (o *AccessInviteListItem) SetLastSharedAtNil(b bool)`

 SetLastSharedAtNil sets the value for LastSharedAt to be an explicit nil

### UnsetLastSharedAt
`func (o *AccessInviteListItem) UnsetLastSharedAt()`

UnsetLastSharedAt ensures that no value is present for LastSharedAt, not even an explicit nil
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
### GetLocation

`func (o *AccessInviteListItem) GetLocation() LocationResponse`

GetLocation returns the Location field if non-nil, zero value otherwise.

### GetLocationOk

`func (o *AccessInviteListItem) GetLocationOk() (*LocationResponse, bool)`

GetLocationOk returns a tuple with the Location field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocation

`func (o *AccessInviteListItem) SetLocation(v LocationResponse)`

SetLocation sets Location field to given value.

### HasLocation

`func (o *AccessInviteListItem) HasLocation() bool`

HasLocation returns a boolean if a field has been set.

### SetLocationNil

`func (o *AccessInviteListItem) SetLocationNil(b bool)`

 SetLocationNil sets the value for Location to be an explicit nil

### UnsetLocation
`func (o *AccessInviteListItem) UnsetLocation()`

UnsetLocation ensures that no value is present for Location, not even an explicit nil
### GetLocationMode

`func (o *AccessInviteListItem) GetLocationMode() LocationBindingMode`

GetLocationMode returns the LocationMode field if non-nil, zero value otherwise.

### GetLocationModeOk

`func (o *AccessInviteListItem) GetLocationModeOk() (*LocationBindingMode, bool)`

GetLocationModeOk returns a tuple with the LocationMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocationMode

`func (o *AccessInviteListItem) SetLocationMode(v LocationBindingMode)`

SetLocationMode sets LocationMode field to given value.


### GetMaxDevices

`func (o *AccessInviteListItem) GetMaxDevices() int32`

GetMaxDevices returns the MaxDevices field if non-nil, zero value otherwise.

### GetMaxDevicesOk

`func (o *AccessInviteListItem) GetMaxDevicesOk() (*int32, bool)`

GetMaxDevicesOk returns a tuple with the MaxDevices field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxDevices

`func (o *AccessInviteListItem) SetMaxDevices(v int32)`

SetMaxDevices sets MaxDevices field to given value.

### HasMaxDevices

`func (o *AccessInviteListItem) HasMaxDevices() bool`

HasMaxDevices returns a boolean if a field has been set.

### SetMaxDevicesNil

`func (o *AccessInviteListItem) SetMaxDevicesNil(b bool)`

 SetMaxDevicesNil sets the value for MaxDevices to be an explicit nil

### UnsetMaxDevices
`func (o *AccessInviteListItem) UnsetMaxDevices()`

UnsetMaxDevices ensures that no value is present for MaxDevices, not even an explicit nil
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
### GetPendingRenewalCount

`func (o *AccessInviteListItem) GetPendingRenewalCount() int32`

GetPendingRenewalCount returns the PendingRenewalCount field if non-nil, zero value otherwise.

### GetPendingRenewalCountOk

`func (o *AccessInviteListItem) GetPendingRenewalCountOk() (*int32, bool)`

GetPendingRenewalCountOk returns a tuple with the PendingRenewalCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPendingRenewalCount

`func (o *AccessInviteListItem) SetPendingRenewalCount(v int32)`

SetPendingRenewalCount sets PendingRenewalCount field to given value.

### HasPendingRenewalCount

`func (o *AccessInviteListItem) HasPendingRenewalCount() bool`

HasPendingRenewalCount returns a boolean if a field has been set.

### SetPendingRenewalCountNil

`func (o *AccessInviteListItem) SetPendingRenewalCountNil(b bool)`

 SetPendingRenewalCountNil sets the value for PendingRenewalCount to be an explicit nil

### UnsetPendingRenewalCount
`func (o *AccessInviteListItem) UnsetPendingRenewalCount()`

UnsetPendingRenewalCount ensures that no value is present for PendingRenewalCount, not even an explicit nil
### GetPhotoThumbUrl

`func (o *AccessInviteListItem) GetPhotoThumbUrl() string`

GetPhotoThumbUrl returns the PhotoThumbUrl field if non-nil, zero value otherwise.

### GetPhotoThumbUrlOk

`func (o *AccessInviteListItem) GetPhotoThumbUrlOk() (*string, bool)`

GetPhotoThumbUrlOk returns a tuple with the PhotoThumbUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhotoThumbUrl

`func (o *AccessInviteListItem) SetPhotoThumbUrl(v string)`

SetPhotoThumbUrl sets PhotoThumbUrl field to given value.

### HasPhotoThumbUrl

`func (o *AccessInviteListItem) HasPhotoThumbUrl() bool`

HasPhotoThumbUrl returns a boolean if a field has been set.

### SetPhotoThumbUrlNil

`func (o *AccessInviteListItem) SetPhotoThumbUrlNil(b bool)`

 SetPhotoThumbUrlNil sets the value for PhotoThumbUrl to be an explicit nil

### UnsetPhotoThumbUrl
`func (o *AccessInviteListItem) UnsetPhotoThumbUrl()`

UnsetPhotoThumbUrl ensures that no value is present for PhotoThumbUrl, not even an explicit nil
### GetPhotoUrl

`func (o *AccessInviteListItem) GetPhotoUrl() string`

GetPhotoUrl returns the PhotoUrl field if non-nil, zero value otherwise.

### GetPhotoUrlOk

`func (o *AccessInviteListItem) GetPhotoUrlOk() (*string, bool)`

GetPhotoUrlOk returns a tuple with the PhotoUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhotoUrl

`func (o *AccessInviteListItem) SetPhotoUrl(v string)`

SetPhotoUrl sets PhotoUrl field to given value.

### HasPhotoUrl

`func (o *AccessInviteListItem) HasPhotoUrl() bool`

HasPhotoUrl returns a boolean if a field has been set.

### SetPhotoUrlNil

`func (o *AccessInviteListItem) SetPhotoUrlNil(b bool)`

 SetPhotoUrlNil sets the value for PhotoUrl to be an explicit nil

### UnsetPhotoUrl
`func (o *AccessInviteListItem) UnsetPhotoUrl()`

UnsetPhotoUrl ensures that no value is present for PhotoUrl, not even an explicit nil
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
