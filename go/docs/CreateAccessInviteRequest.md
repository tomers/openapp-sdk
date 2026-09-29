# CreateAccessInviteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Audience** | Pointer to [**InviteAudience**](InviteAudience.md) | Whether anyone holding the link may use the invitation, or only the named contacts. | [optional]
**Contacts** | Pointer to [**[]AccessInviteContactInput**](AccessInviteContactInput.md) | Who the invitation is for. Each contact is either free-form (an email address or a phone number) or drawn from the organization (a person or a group). Rejected for shares, which carry no invitee identity. Empty when the link is open to anyone. | [optional]
**CreationJustification** | Pointer to **NullableString** | Why this invitation is being created (required when &#x60;invitation_require_justification&#x60; applies). | [optional]
**CurfewExempt** | Pointer to **NullableBool** | Admin-only emergency exception: skip invitation curfew at redemption. | [optional]
**ExternalRef** | Pointer to [**NullableExternalReferenceInput**](ExternalReferenceInput.md) | Correlation to the record in the external system of record that owns this invitation (PMS reservation, work order, enrolment). One live invitation may own a given &#x60;(source, record_id)&#x60; pair per integration, which is what makes retries and duplicate webhook deliveries safe. Omit for invitations authored inside OpenApp. | [optional]
**HasPhoto** | Pointer to **NullableBool** | True when the client will upload a photo immediately after create (media is a follow-up POST). | [optional]
**InviteeMessage** | Pointer to **interface{}** |  | [optional]
**IsEnabled** | Pointer to **NullableBool** |  | [optional]
**Kind** | Pointer to [**AccessInviteKind**](AccessInviteKind.md) |  | [optional]
**LocationId** | Pointer to **NullableString** | Reusable org-scoped location id to bind, only consumed when &#x60;location_mode&#x60; is &#x60;explicit&#x60;. Create the location first via the locations API (which can geocode it), then reference it here. | [optional]
**LocationMode** | Pointer to [**NullableLocationBindingMode**](LocationBindingMode.md) | How the event location is resolved. Defaults to &#x60;inherit&#x60; (use the building location) when omitted. &#x60;explicit&#x60; binds to the reusable org location referenced by &#x60;location_id&#x60;; &#x60;none&#x60; shows no event location. | [optional]
**MaxDevices** | Pointer to **NullableInt32** | Virtual keycard limit: max unique devices allowed to redeem/use this invite. JSON &#x60;null&#x60; or omitted means unlimited. | [optional]
**MaxUses** | Pointer to **NullableInt32** | Omit to apply the share default (or leave invitations unlimited). JSON &#x60;null&#x60; means unlimited. | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**Pin** | Pointer to **NullableString** | Host-chosen PIN (stored hashed). Required when &#x60;invitation_require_identity.require_pin&#x60;. JSON &#x60;null&#x60; means no PIN; omission leaves the create default unchanged. | [optional]
**PortalIds** | **[]string** | Portal ULIDs to grant. Array order is the display order on the public invite page. |
**Schedules** | [**[]InviteScheduleEntryInput**](InviteScheduleEntryInput.md) | One or more validity windows. A single non-recurring entry is the ordinary case; a share is exactly one entry with no explicit start (it opens at request time). |

## Methods

### NewCreateAccessInviteRequest

`func NewCreateAccessInviteRequest(portalIds []string, schedules []InviteScheduleEntryInput, ) *CreateAccessInviteRequest`

NewCreateAccessInviteRequest instantiates a new CreateAccessInviteRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateAccessInviteRequestWithDefaults

`func NewCreateAccessInviteRequestWithDefaults() *CreateAccessInviteRequest`

NewCreateAccessInviteRequestWithDefaults instantiates a new CreateAccessInviteRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAudience

`func (o *CreateAccessInviteRequest) GetAudience() InviteAudience`

GetAudience returns the Audience field if non-nil, zero value otherwise.

### GetAudienceOk

`func (o *CreateAccessInviteRequest) GetAudienceOk() (*InviteAudience, bool)`

GetAudienceOk returns a tuple with the Audience field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAudience

`func (o *CreateAccessInviteRequest) SetAudience(v InviteAudience)`

SetAudience sets Audience field to given value.

### HasAudience

`func (o *CreateAccessInviteRequest) HasAudience() bool`

HasAudience returns a boolean if a field has been set.

### GetContacts

`func (o *CreateAccessInviteRequest) GetContacts() []AccessInviteContactInput`

GetContacts returns the Contacts field if non-nil, zero value otherwise.

### GetContactsOk

`func (o *CreateAccessInviteRequest) GetContactsOk() (*[]AccessInviteContactInput, bool)`

GetContactsOk returns a tuple with the Contacts field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetContacts

`func (o *CreateAccessInviteRequest) SetContacts(v []AccessInviteContactInput)`

SetContacts sets Contacts field to given value.

### HasContacts

`func (o *CreateAccessInviteRequest) HasContacts() bool`

HasContacts returns a boolean if a field has been set.

### GetCreationJustification

`func (o *CreateAccessInviteRequest) GetCreationJustification() string`

GetCreationJustification returns the CreationJustification field if non-nil, zero value otherwise.

### GetCreationJustificationOk

`func (o *CreateAccessInviteRequest) GetCreationJustificationOk() (*string, bool)`

GetCreationJustificationOk returns a tuple with the CreationJustification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreationJustification

`func (o *CreateAccessInviteRequest) SetCreationJustification(v string)`

SetCreationJustification sets CreationJustification field to given value.

### HasCreationJustification

`func (o *CreateAccessInviteRequest) HasCreationJustification() bool`

HasCreationJustification returns a boolean if a field has been set.

### SetCreationJustificationNil

`func (o *CreateAccessInviteRequest) SetCreationJustificationNil(b bool)`

 SetCreationJustificationNil sets the value for CreationJustification to be an explicit nil

### UnsetCreationJustification
`func (o *CreateAccessInviteRequest) UnsetCreationJustification()`

UnsetCreationJustification ensures that no value is present for CreationJustification, not even an explicit nil
### GetCurfewExempt

`func (o *CreateAccessInviteRequest) GetCurfewExempt() bool`

GetCurfewExempt returns the CurfewExempt field if non-nil, zero value otherwise.

### GetCurfewExemptOk

`func (o *CreateAccessInviteRequest) GetCurfewExemptOk() (*bool, bool)`

GetCurfewExemptOk returns a tuple with the CurfewExempt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurfewExempt

`func (o *CreateAccessInviteRequest) SetCurfewExempt(v bool)`

SetCurfewExempt sets CurfewExempt field to given value.

### HasCurfewExempt

`func (o *CreateAccessInviteRequest) HasCurfewExempt() bool`

HasCurfewExempt returns a boolean if a field has been set.

### SetCurfewExemptNil

`func (o *CreateAccessInviteRequest) SetCurfewExemptNil(b bool)`

 SetCurfewExemptNil sets the value for CurfewExempt to be an explicit nil

### UnsetCurfewExempt
`func (o *CreateAccessInviteRequest) UnsetCurfewExempt()`

UnsetCurfewExempt ensures that no value is present for CurfewExempt, not even an explicit nil
### GetExternalRef

`func (o *CreateAccessInviteRequest) GetExternalRef() ExternalReferenceInput`

GetExternalRef returns the ExternalRef field if non-nil, zero value otherwise.

### GetExternalRefOk

`func (o *CreateAccessInviteRequest) GetExternalRefOk() (*ExternalReferenceInput, bool)`

GetExternalRefOk returns a tuple with the ExternalRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalRef

`func (o *CreateAccessInviteRequest) SetExternalRef(v ExternalReferenceInput)`

SetExternalRef sets ExternalRef field to given value.

### HasExternalRef

`func (o *CreateAccessInviteRequest) HasExternalRef() bool`

HasExternalRef returns a boolean if a field has been set.

### SetExternalRefNil

`func (o *CreateAccessInviteRequest) SetExternalRefNil(b bool)`

 SetExternalRefNil sets the value for ExternalRef to be an explicit nil

### UnsetExternalRef
`func (o *CreateAccessInviteRequest) UnsetExternalRef()`

UnsetExternalRef ensures that no value is present for ExternalRef, not even an explicit nil
### GetHasPhoto

`func (o *CreateAccessInviteRequest) GetHasPhoto() bool`

GetHasPhoto returns the HasPhoto field if non-nil, zero value otherwise.

### GetHasPhotoOk

`func (o *CreateAccessInviteRequest) GetHasPhotoOk() (*bool, bool)`

GetHasPhotoOk returns a tuple with the HasPhoto field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasPhoto

`func (o *CreateAccessInviteRequest) SetHasPhoto(v bool)`

SetHasPhoto sets HasPhoto field to given value.

### HasHasPhoto

`func (o *CreateAccessInviteRequest) HasHasPhoto() bool`

HasHasPhoto returns a boolean if a field has been set.

### SetHasPhotoNil

`func (o *CreateAccessInviteRequest) SetHasPhotoNil(b bool)`

 SetHasPhotoNil sets the value for HasPhoto to be an explicit nil

### UnsetHasPhoto
`func (o *CreateAccessInviteRequest) UnsetHasPhoto()`

UnsetHasPhoto ensures that no value is present for HasPhoto, not even an explicit nil
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
### GetKind

`func (o *CreateAccessInviteRequest) GetKind() AccessInviteKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *CreateAccessInviteRequest) GetKindOk() (*AccessInviteKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *CreateAccessInviteRequest) SetKind(v AccessInviteKind)`

SetKind sets Kind field to given value.

### HasKind

`func (o *CreateAccessInviteRequest) HasKind() bool`

HasKind returns a boolean if a field has been set.

### GetLocationId

`func (o *CreateAccessInviteRequest) GetLocationId() string`

GetLocationId returns the LocationId field if non-nil, zero value otherwise.

### GetLocationIdOk

`func (o *CreateAccessInviteRequest) GetLocationIdOk() (*string, bool)`

GetLocationIdOk returns a tuple with the LocationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocationId

`func (o *CreateAccessInviteRequest) SetLocationId(v string)`

SetLocationId sets LocationId field to given value.

### HasLocationId

`func (o *CreateAccessInviteRequest) HasLocationId() bool`

HasLocationId returns a boolean if a field has been set.

### SetLocationIdNil

`func (o *CreateAccessInviteRequest) SetLocationIdNil(b bool)`

 SetLocationIdNil sets the value for LocationId to be an explicit nil

### UnsetLocationId
`func (o *CreateAccessInviteRequest) UnsetLocationId()`

UnsetLocationId ensures that no value is present for LocationId, not even an explicit nil
### GetLocationMode

`func (o *CreateAccessInviteRequest) GetLocationMode() LocationBindingMode`

GetLocationMode returns the LocationMode field if non-nil, zero value otherwise.

### GetLocationModeOk

`func (o *CreateAccessInviteRequest) GetLocationModeOk() (*LocationBindingMode, bool)`

GetLocationModeOk returns a tuple with the LocationMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocationMode

`func (o *CreateAccessInviteRequest) SetLocationMode(v LocationBindingMode)`

SetLocationMode sets LocationMode field to given value.

### HasLocationMode

`func (o *CreateAccessInviteRequest) HasLocationMode() bool`

HasLocationMode returns a boolean if a field has been set.

### SetLocationModeNil

`func (o *CreateAccessInviteRequest) SetLocationModeNil(b bool)`

 SetLocationModeNil sets the value for LocationMode to be an explicit nil

### UnsetLocationMode
`func (o *CreateAccessInviteRequest) UnsetLocationMode()`

UnsetLocationMode ensures that no value is present for LocationMode, not even an explicit nil
### GetMaxDevices

`func (o *CreateAccessInviteRequest) GetMaxDevices() int32`

GetMaxDevices returns the MaxDevices field if non-nil, zero value otherwise.

### GetMaxDevicesOk

`func (o *CreateAccessInviteRequest) GetMaxDevicesOk() (*int32, bool)`

GetMaxDevicesOk returns a tuple with the MaxDevices field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxDevices

`func (o *CreateAccessInviteRequest) SetMaxDevices(v int32)`

SetMaxDevices sets MaxDevices field to given value.

### HasMaxDevices

`func (o *CreateAccessInviteRequest) HasMaxDevices() bool`

HasMaxDevices returns a boolean if a field has been set.

### SetMaxDevicesNil

`func (o *CreateAccessInviteRequest) SetMaxDevicesNil(b bool)`

 SetMaxDevicesNil sets the value for MaxDevices to be an explicit nil

### UnsetMaxDevices
`func (o *CreateAccessInviteRequest) UnsetMaxDevices()`

UnsetMaxDevices ensures that no value is present for MaxDevices, not even an explicit nil
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
### GetPin

`func (o *CreateAccessInviteRequest) GetPin() string`

GetPin returns the Pin field if non-nil, zero value otherwise.

### GetPinOk

`func (o *CreateAccessInviteRequest) GetPinOk() (*string, bool)`

GetPinOk returns a tuple with the Pin field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPin

`func (o *CreateAccessInviteRequest) SetPin(v string)`

SetPin sets Pin field to given value.

### HasPin

`func (o *CreateAccessInviteRequest) HasPin() bool`

HasPin returns a boolean if a field has been set.

### SetPinNil

`func (o *CreateAccessInviteRequest) SetPinNil(b bool)`

 SetPinNil sets the value for Pin to be an explicit nil

### UnsetPin
`func (o *CreateAccessInviteRequest) UnsetPin()`

UnsetPin ensures that no value is present for Pin, not even an explicit nil
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



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
