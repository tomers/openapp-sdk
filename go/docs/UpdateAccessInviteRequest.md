# UpdateAccessInviteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Audience** | Pointer to [**NullableInviteAudience**](InviteAudience.md) | When set, changes whether anyone holding the link may use the invitation. | [optional]
**Contacts** | Pointer to [**[]AccessInviteContactInput**](AccessInviteContactInput.md) | When set, replaces the invitation&#39;s contacts. Omitted leaves them unchanged. | [optional]
**CreationJustification** | Pointer to **NullableString** | Why this invitation was created (required when &#x60;invitation_require_justification&#x60; applies). | [optional]
**CurfewExempt** | Pointer to **NullableBool** | Admin-only emergency exception: skip invitation curfew at redemption. | [optional]
**DisabledJustification** | Pointer to **NullableString** |  | [optional]
**ExternalRef** | Pointer to [**NullableExternalReferenceInput**](ExternalReferenceInput.md) | Omitted leaves the stored phone unchanged; JSON &#x60;null&#x60; clears it. External-record precondition and correlation. When present, &#x60;source&#x60;/&#x60;record_id&#x60; must match the invitation&#39;s stored reference (or bind it for the first time), and a &#x60;revision&#x60; older than the stored one is rejected with &#x60;external_revision_stale&#x60; so a late event cannot restore an earlier room, window, or portal set. | [optional]
**InviteeMessage** | Pointer to **interface{}** |  | [optional]
**IsEnabled** | Pointer to **NullableBool** |  | [optional]
**LocationId** | Pointer to **NullableString** | Only consumed when &#x60;location_mode&#x60; is &#x60;explicit&#x60;: the reusable org location id to bind. When the mode is &#x60;explicit&#x60; and this is absent, the existing bound location is kept. | [optional]
**LocationMode** | Pointer to [**NullableLocationBindingMode**](LocationBindingMode.md) | When set, replaces the event-location mode (&#x60;none&#x60;, &#x60;explicit&#x60;, or &#x60;inherit&#x60;). Absent leaves the mode and location unchanged. | [optional]
**MaxDevices** | Pointer to **NullableInt32** | Virtual keycard limit: max unique devices allowed. JSON &#x60;null&#x60; means unlimited. | [optional]
**MaxUses** | Pointer to **NullableInt32** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**Pin** | Pointer to **NullableString** | Omitted leaves the stored PIN unchanged; JSON &#x60;null&#x60; clears it. | [optional]
**PortalIds** | Pointer to **[]string** | When set, replaces portal grants. Array order is the display order on the public invite page. | [optional]
**Schedules** | Pointer to [**[]InviteScheduleEntryInput**](InviteScheduleEntryInput.md) | When set, replaces all schedule entries. This is the only way to change validity or recurrence. | [optional]

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

### GetAudience

`func (o *UpdateAccessInviteRequest) GetAudience() InviteAudience`

GetAudience returns the Audience field if non-nil, zero value otherwise.

### GetAudienceOk

`func (o *UpdateAccessInviteRequest) GetAudienceOk() (*InviteAudience, bool)`

GetAudienceOk returns a tuple with the Audience field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAudience

`func (o *UpdateAccessInviteRequest) SetAudience(v InviteAudience)`

SetAudience sets Audience field to given value.

### HasAudience

`func (o *UpdateAccessInviteRequest) HasAudience() bool`

HasAudience returns a boolean if a field has been set.

### SetAudienceNil

`func (o *UpdateAccessInviteRequest) SetAudienceNil(b bool)`

 SetAudienceNil sets the value for Audience to be an explicit nil

### UnsetAudience
`func (o *UpdateAccessInviteRequest) UnsetAudience()`

UnsetAudience ensures that no value is present for Audience, not even an explicit nil
### GetContacts

`func (o *UpdateAccessInviteRequest) GetContacts() []AccessInviteContactInput`

GetContacts returns the Contacts field if non-nil, zero value otherwise.

### GetContactsOk

`func (o *UpdateAccessInviteRequest) GetContactsOk() (*[]AccessInviteContactInput, bool)`

GetContactsOk returns a tuple with the Contacts field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetContacts

`func (o *UpdateAccessInviteRequest) SetContacts(v []AccessInviteContactInput)`

SetContacts sets Contacts field to given value.

### HasContacts

`func (o *UpdateAccessInviteRequest) HasContacts() bool`

HasContacts returns a boolean if a field has been set.

### SetContactsNil

`func (o *UpdateAccessInviteRequest) SetContactsNil(b bool)`

 SetContactsNil sets the value for Contacts to be an explicit nil

### UnsetContacts
`func (o *UpdateAccessInviteRequest) UnsetContacts()`

UnsetContacts ensures that no value is present for Contacts, not even an explicit nil
### GetCreationJustification

`func (o *UpdateAccessInviteRequest) GetCreationJustification() string`

GetCreationJustification returns the CreationJustification field if non-nil, zero value otherwise.

### GetCreationJustificationOk

`func (o *UpdateAccessInviteRequest) GetCreationJustificationOk() (*string, bool)`

GetCreationJustificationOk returns a tuple with the CreationJustification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreationJustification

`func (o *UpdateAccessInviteRequest) SetCreationJustification(v string)`

SetCreationJustification sets CreationJustification field to given value.

### HasCreationJustification

`func (o *UpdateAccessInviteRequest) HasCreationJustification() bool`

HasCreationJustification returns a boolean if a field has been set.

### SetCreationJustificationNil

`func (o *UpdateAccessInviteRequest) SetCreationJustificationNil(b bool)`

 SetCreationJustificationNil sets the value for CreationJustification to be an explicit nil

### UnsetCreationJustification
`func (o *UpdateAccessInviteRequest) UnsetCreationJustification()`

UnsetCreationJustification ensures that no value is present for CreationJustification, not even an explicit nil
### GetCurfewExempt

`func (o *UpdateAccessInviteRequest) GetCurfewExempt() bool`

GetCurfewExempt returns the CurfewExempt field if non-nil, zero value otherwise.

### GetCurfewExemptOk

`func (o *UpdateAccessInviteRequest) GetCurfewExemptOk() (*bool, bool)`

GetCurfewExemptOk returns a tuple with the CurfewExempt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurfewExempt

`func (o *UpdateAccessInviteRequest) SetCurfewExempt(v bool)`

SetCurfewExempt sets CurfewExempt field to given value.

### HasCurfewExempt

`func (o *UpdateAccessInviteRequest) HasCurfewExempt() bool`

HasCurfewExempt returns a boolean if a field has been set.

### SetCurfewExemptNil

`func (o *UpdateAccessInviteRequest) SetCurfewExemptNil(b bool)`

 SetCurfewExemptNil sets the value for CurfewExempt to be an explicit nil

### UnsetCurfewExempt
`func (o *UpdateAccessInviteRequest) UnsetCurfewExempt()`

UnsetCurfewExempt ensures that no value is present for CurfewExempt, not even an explicit nil
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
### GetExternalRef

`func (o *UpdateAccessInviteRequest) GetExternalRef() ExternalReferenceInput`

GetExternalRef returns the ExternalRef field if non-nil, zero value otherwise.

### GetExternalRefOk

`func (o *UpdateAccessInviteRequest) GetExternalRefOk() (*ExternalReferenceInput, bool)`

GetExternalRefOk returns a tuple with the ExternalRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalRef

`func (o *UpdateAccessInviteRequest) SetExternalRef(v ExternalReferenceInput)`

SetExternalRef sets ExternalRef field to given value.

### HasExternalRef

`func (o *UpdateAccessInviteRequest) HasExternalRef() bool`

HasExternalRef returns a boolean if a field has been set.

### SetExternalRefNil

`func (o *UpdateAccessInviteRequest) SetExternalRefNil(b bool)`

 SetExternalRefNil sets the value for ExternalRef to be an explicit nil

### UnsetExternalRef
`func (o *UpdateAccessInviteRequest) UnsetExternalRef()`

UnsetExternalRef ensures that no value is present for ExternalRef, not even an explicit nil
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
### GetLocationId

`func (o *UpdateAccessInviteRequest) GetLocationId() string`

GetLocationId returns the LocationId field if non-nil, zero value otherwise.

### GetLocationIdOk

`func (o *UpdateAccessInviteRequest) GetLocationIdOk() (*string, bool)`

GetLocationIdOk returns a tuple with the LocationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocationId

`func (o *UpdateAccessInviteRequest) SetLocationId(v string)`

SetLocationId sets LocationId field to given value.

### HasLocationId

`func (o *UpdateAccessInviteRequest) HasLocationId() bool`

HasLocationId returns a boolean if a field has been set.

### SetLocationIdNil

`func (o *UpdateAccessInviteRequest) SetLocationIdNil(b bool)`

 SetLocationIdNil sets the value for LocationId to be an explicit nil

### UnsetLocationId
`func (o *UpdateAccessInviteRequest) UnsetLocationId()`

UnsetLocationId ensures that no value is present for LocationId, not even an explicit nil
### GetLocationMode

`func (o *UpdateAccessInviteRequest) GetLocationMode() LocationBindingMode`

GetLocationMode returns the LocationMode field if non-nil, zero value otherwise.

### GetLocationModeOk

`func (o *UpdateAccessInviteRequest) GetLocationModeOk() (*LocationBindingMode, bool)`

GetLocationModeOk returns a tuple with the LocationMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocationMode

`func (o *UpdateAccessInviteRequest) SetLocationMode(v LocationBindingMode)`

SetLocationMode sets LocationMode field to given value.

### HasLocationMode

`func (o *UpdateAccessInviteRequest) HasLocationMode() bool`

HasLocationMode returns a boolean if a field has been set.

### SetLocationModeNil

`func (o *UpdateAccessInviteRequest) SetLocationModeNil(b bool)`

 SetLocationModeNil sets the value for LocationMode to be an explicit nil

### UnsetLocationMode
`func (o *UpdateAccessInviteRequest) UnsetLocationMode()`

UnsetLocationMode ensures that no value is present for LocationMode, not even an explicit nil
### GetMaxDevices

`func (o *UpdateAccessInviteRequest) GetMaxDevices() int32`

GetMaxDevices returns the MaxDevices field if non-nil, zero value otherwise.

### GetMaxDevicesOk

`func (o *UpdateAccessInviteRequest) GetMaxDevicesOk() (*int32, bool)`

GetMaxDevicesOk returns a tuple with the MaxDevices field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxDevices

`func (o *UpdateAccessInviteRequest) SetMaxDevices(v int32)`

SetMaxDevices sets MaxDevices field to given value.

### HasMaxDevices

`func (o *UpdateAccessInviteRequest) HasMaxDevices() bool`

HasMaxDevices returns a boolean if a field has been set.

### SetMaxDevicesNil

`func (o *UpdateAccessInviteRequest) SetMaxDevicesNil(b bool)`

 SetMaxDevicesNil sets the value for MaxDevices to be an explicit nil

### UnsetMaxDevices
`func (o *UpdateAccessInviteRequest) UnsetMaxDevices()`

UnsetMaxDevices ensures that no value is present for MaxDevices, not even an explicit nil
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
### GetPin

`func (o *UpdateAccessInviteRequest) GetPin() string`

GetPin returns the Pin field if non-nil, zero value otherwise.

### GetPinOk

`func (o *UpdateAccessInviteRequest) GetPinOk() (*string, bool)`

GetPinOk returns a tuple with the Pin field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPin

`func (o *UpdateAccessInviteRequest) SetPin(v string)`

SetPin sets Pin field to given value.

### HasPin

`func (o *UpdateAccessInviteRequest) HasPin() bool`

HasPin returns a boolean if a field has been set.

### SetPinNil

`func (o *UpdateAccessInviteRequest) SetPinNil(b bool)`

 SetPinNil sets the value for Pin to be an explicit nil

### UnsetPin
`func (o *UpdateAccessInviteRequest) UnsetPin()`

UnsetPin ensures that no value is present for Pin, not even an explicit nil
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

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
