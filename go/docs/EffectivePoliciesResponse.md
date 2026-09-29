# EffectivePoliciesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AllowedCreatorRoles** | Pointer to **[]string** |  | [optional]
**AllowedEntryKinds** | Pointer to **[]string** |  | [optional]
**AllowedPortalIds** | Pointer to **[]string** | Present on integration effective-policies when listing-restrict binds this requester. | [optional]
**AllowedWeekdays** | Pointer to **[]int32** |  | [optional]
**BlackoutDates** | Pointer to **[]string** |  | [optional]
**CurfewWindows** | [**[]CurfewWindowResponse**](CurfewWindowResponse.md) |  |
**MaxActivePerUser** | Pointer to **NullableInt32** |  | [optional]
**MaxDoors** | Pointer to **NullableInt32** |  | [optional]
**MaxDurationSeconds** | Pointer to **NullableInt64** |  | [optional]
**MaxUses** | Pointer to **NullableInt32** |  | [optional]
**NoReshare** | Pointer to **bool** |  | [optional]
**NoTransitiveDelegation** | Pointer to **bool** |  | [optional]
**ProhibitedMasterDoorIds** | Pointer to **[]string** |  | [optional]
**RequireExpiry** | Pointer to **bool** |  | [optional]
**RequireJustification** | Pointer to **bool** |  | [optional]
**RequirePhoto** | Pointer to **bool** |  | [optional]
**RequirePin** | Pointer to **bool** |  | [optional]
**RequireVerifiedPhone** | Pointer to **bool** |  | [optional]
**RestrictToOwnListingDoors** | Pointer to **bool** |  | [optional]
**ShareAllowUnlimited** | Pointer to **bool** |  | [optional]
**ShareDefaultMaxUses** | Pointer to **NullableInt32** |  | [optional]
**ShareMaxDurationSeconds** | Pointer to **NullableInt64** | Share-specific TTL cap in seconds. &#x60;86400&#x60; (1 day) when &#x60;share_max_duration&#x60; is unset. Org policy may raise this up to &#x60;31536000&#x60; (365d). Authoring uses the min of this and &#x60;max_duration_seconds&#x60;. | [optional]
**ShareMaxUses** | Pointer to **NullableInt32** |  | [optional]
**Timezone** | **string** | IANA timezone used to evaluate curfews, or &#x60;\&quot;UTC\&quot;&#x60; when unset. |
**UserSharingMode** | **string** | Effective &#x60;user_sharing&#x60; mode: &#x60;none&#x60; | &#x60;admin_only&#x60; | &#x60;approval_required&#x60; | &#x60;all&#x60;. |

## Methods

### NewEffectivePoliciesResponse

`func NewEffectivePoliciesResponse(curfewWindows []CurfewWindowResponse, timezone string, userSharingMode string, ) *EffectivePoliciesResponse`

NewEffectivePoliciesResponse instantiates a new EffectivePoliciesResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewEffectivePoliciesResponseWithDefaults

`func NewEffectivePoliciesResponseWithDefaults() *EffectivePoliciesResponse`

NewEffectivePoliciesResponseWithDefaults instantiates a new EffectivePoliciesResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAllowedCreatorRoles

`func (o *EffectivePoliciesResponse) GetAllowedCreatorRoles() []string`

GetAllowedCreatorRoles returns the AllowedCreatorRoles field if non-nil, zero value otherwise.

### GetAllowedCreatorRolesOk

`func (o *EffectivePoliciesResponse) GetAllowedCreatorRolesOk() (*[]string, bool)`

GetAllowedCreatorRolesOk returns a tuple with the AllowedCreatorRoles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedCreatorRoles

`func (o *EffectivePoliciesResponse) SetAllowedCreatorRoles(v []string)`

SetAllowedCreatorRoles sets AllowedCreatorRoles field to given value.

### HasAllowedCreatorRoles

`func (o *EffectivePoliciesResponse) HasAllowedCreatorRoles() bool`

HasAllowedCreatorRoles returns a boolean if a field has been set.

### SetAllowedCreatorRolesNil

`func (o *EffectivePoliciesResponse) SetAllowedCreatorRolesNil(b bool)`

 SetAllowedCreatorRolesNil sets the value for AllowedCreatorRoles to be an explicit nil

### UnsetAllowedCreatorRoles
`func (o *EffectivePoliciesResponse) UnsetAllowedCreatorRoles()`

UnsetAllowedCreatorRoles ensures that no value is present for AllowedCreatorRoles, not even an explicit nil
### GetAllowedEntryKinds

`func (o *EffectivePoliciesResponse) GetAllowedEntryKinds() []string`

GetAllowedEntryKinds returns the AllowedEntryKinds field if non-nil, zero value otherwise.

### GetAllowedEntryKindsOk

`func (o *EffectivePoliciesResponse) GetAllowedEntryKindsOk() (*[]string, bool)`

GetAllowedEntryKindsOk returns a tuple with the AllowedEntryKinds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedEntryKinds

`func (o *EffectivePoliciesResponse) SetAllowedEntryKinds(v []string)`

SetAllowedEntryKinds sets AllowedEntryKinds field to given value.

### HasAllowedEntryKinds

`func (o *EffectivePoliciesResponse) HasAllowedEntryKinds() bool`

HasAllowedEntryKinds returns a boolean if a field has been set.

### SetAllowedEntryKindsNil

`func (o *EffectivePoliciesResponse) SetAllowedEntryKindsNil(b bool)`

 SetAllowedEntryKindsNil sets the value for AllowedEntryKinds to be an explicit nil

### UnsetAllowedEntryKinds
`func (o *EffectivePoliciesResponse) UnsetAllowedEntryKinds()`

UnsetAllowedEntryKinds ensures that no value is present for AllowedEntryKinds, not even an explicit nil
### GetAllowedPortalIds

`func (o *EffectivePoliciesResponse) GetAllowedPortalIds() []string`

GetAllowedPortalIds returns the AllowedPortalIds field if non-nil, zero value otherwise.

### GetAllowedPortalIdsOk

`func (o *EffectivePoliciesResponse) GetAllowedPortalIdsOk() (*[]string, bool)`

GetAllowedPortalIdsOk returns a tuple with the AllowedPortalIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedPortalIds

`func (o *EffectivePoliciesResponse) SetAllowedPortalIds(v []string)`

SetAllowedPortalIds sets AllowedPortalIds field to given value.

### HasAllowedPortalIds

`func (o *EffectivePoliciesResponse) HasAllowedPortalIds() bool`

HasAllowedPortalIds returns a boolean if a field has been set.

### SetAllowedPortalIdsNil

`func (o *EffectivePoliciesResponse) SetAllowedPortalIdsNil(b bool)`

 SetAllowedPortalIdsNil sets the value for AllowedPortalIds to be an explicit nil

### UnsetAllowedPortalIds
`func (o *EffectivePoliciesResponse) UnsetAllowedPortalIds()`

UnsetAllowedPortalIds ensures that no value is present for AllowedPortalIds, not even an explicit nil
### GetAllowedWeekdays

`func (o *EffectivePoliciesResponse) GetAllowedWeekdays() []int32`

GetAllowedWeekdays returns the AllowedWeekdays field if non-nil, zero value otherwise.

### GetAllowedWeekdaysOk

`func (o *EffectivePoliciesResponse) GetAllowedWeekdaysOk() (*[]int32, bool)`

GetAllowedWeekdaysOk returns a tuple with the AllowedWeekdays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedWeekdays

`func (o *EffectivePoliciesResponse) SetAllowedWeekdays(v []int32)`

SetAllowedWeekdays sets AllowedWeekdays field to given value.

### HasAllowedWeekdays

`func (o *EffectivePoliciesResponse) HasAllowedWeekdays() bool`

HasAllowedWeekdays returns a boolean if a field has been set.

### SetAllowedWeekdaysNil

`func (o *EffectivePoliciesResponse) SetAllowedWeekdaysNil(b bool)`

 SetAllowedWeekdaysNil sets the value for AllowedWeekdays to be an explicit nil

### UnsetAllowedWeekdays
`func (o *EffectivePoliciesResponse) UnsetAllowedWeekdays()`

UnsetAllowedWeekdays ensures that no value is present for AllowedWeekdays, not even an explicit nil
### GetBlackoutDates

`func (o *EffectivePoliciesResponse) GetBlackoutDates() []string`

GetBlackoutDates returns the BlackoutDates field if non-nil, zero value otherwise.

### GetBlackoutDatesOk

`func (o *EffectivePoliciesResponse) GetBlackoutDatesOk() (*[]string, bool)`

GetBlackoutDatesOk returns a tuple with the BlackoutDates field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBlackoutDates

`func (o *EffectivePoliciesResponse) SetBlackoutDates(v []string)`

SetBlackoutDates sets BlackoutDates field to given value.

### HasBlackoutDates

`func (o *EffectivePoliciesResponse) HasBlackoutDates() bool`

HasBlackoutDates returns a boolean if a field has been set.

### SetBlackoutDatesNil

`func (o *EffectivePoliciesResponse) SetBlackoutDatesNil(b bool)`

 SetBlackoutDatesNil sets the value for BlackoutDates to be an explicit nil

### UnsetBlackoutDates
`func (o *EffectivePoliciesResponse) UnsetBlackoutDates()`

UnsetBlackoutDates ensures that no value is present for BlackoutDates, not even an explicit nil
### GetCurfewWindows

`func (o *EffectivePoliciesResponse) GetCurfewWindows() []CurfewWindowResponse`

GetCurfewWindows returns the CurfewWindows field if non-nil, zero value otherwise.

### GetCurfewWindowsOk

`func (o *EffectivePoliciesResponse) GetCurfewWindowsOk() (*[]CurfewWindowResponse, bool)`

GetCurfewWindowsOk returns a tuple with the CurfewWindows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurfewWindows

`func (o *EffectivePoliciesResponse) SetCurfewWindows(v []CurfewWindowResponse)`

SetCurfewWindows sets CurfewWindows field to given value.


### GetMaxActivePerUser

`func (o *EffectivePoliciesResponse) GetMaxActivePerUser() int32`

GetMaxActivePerUser returns the MaxActivePerUser field if non-nil, zero value otherwise.

### GetMaxActivePerUserOk

`func (o *EffectivePoliciesResponse) GetMaxActivePerUserOk() (*int32, bool)`

GetMaxActivePerUserOk returns a tuple with the MaxActivePerUser field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxActivePerUser

`func (o *EffectivePoliciesResponse) SetMaxActivePerUser(v int32)`

SetMaxActivePerUser sets MaxActivePerUser field to given value.

### HasMaxActivePerUser

`func (o *EffectivePoliciesResponse) HasMaxActivePerUser() bool`

HasMaxActivePerUser returns a boolean if a field has been set.

### SetMaxActivePerUserNil

`func (o *EffectivePoliciesResponse) SetMaxActivePerUserNil(b bool)`

 SetMaxActivePerUserNil sets the value for MaxActivePerUser to be an explicit nil

### UnsetMaxActivePerUser
`func (o *EffectivePoliciesResponse) UnsetMaxActivePerUser()`

UnsetMaxActivePerUser ensures that no value is present for MaxActivePerUser, not even an explicit nil
### GetMaxDoors

`func (o *EffectivePoliciesResponse) GetMaxDoors() int32`

GetMaxDoors returns the MaxDoors field if non-nil, zero value otherwise.

### GetMaxDoorsOk

`func (o *EffectivePoliciesResponse) GetMaxDoorsOk() (*int32, bool)`

GetMaxDoorsOk returns a tuple with the MaxDoors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxDoors

`func (o *EffectivePoliciesResponse) SetMaxDoors(v int32)`

SetMaxDoors sets MaxDoors field to given value.

### HasMaxDoors

`func (o *EffectivePoliciesResponse) HasMaxDoors() bool`

HasMaxDoors returns a boolean if a field has been set.

### SetMaxDoorsNil

`func (o *EffectivePoliciesResponse) SetMaxDoorsNil(b bool)`

 SetMaxDoorsNil sets the value for MaxDoors to be an explicit nil

### UnsetMaxDoors
`func (o *EffectivePoliciesResponse) UnsetMaxDoors()`

UnsetMaxDoors ensures that no value is present for MaxDoors, not even an explicit nil
### GetMaxDurationSeconds

`func (o *EffectivePoliciesResponse) GetMaxDurationSeconds() int64`

GetMaxDurationSeconds returns the MaxDurationSeconds field if non-nil, zero value otherwise.

### GetMaxDurationSecondsOk

`func (o *EffectivePoliciesResponse) GetMaxDurationSecondsOk() (*int64, bool)`

GetMaxDurationSecondsOk returns a tuple with the MaxDurationSeconds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxDurationSeconds

`func (o *EffectivePoliciesResponse) SetMaxDurationSeconds(v int64)`

SetMaxDurationSeconds sets MaxDurationSeconds field to given value.

### HasMaxDurationSeconds

`func (o *EffectivePoliciesResponse) HasMaxDurationSeconds() bool`

HasMaxDurationSeconds returns a boolean if a field has been set.

### SetMaxDurationSecondsNil

`func (o *EffectivePoliciesResponse) SetMaxDurationSecondsNil(b bool)`

 SetMaxDurationSecondsNil sets the value for MaxDurationSeconds to be an explicit nil

### UnsetMaxDurationSeconds
`func (o *EffectivePoliciesResponse) UnsetMaxDurationSeconds()`

UnsetMaxDurationSeconds ensures that no value is present for MaxDurationSeconds, not even an explicit nil
### GetMaxUses

`func (o *EffectivePoliciesResponse) GetMaxUses() int32`

GetMaxUses returns the MaxUses field if non-nil, zero value otherwise.

### GetMaxUsesOk

`func (o *EffectivePoliciesResponse) GetMaxUsesOk() (*int32, bool)`

GetMaxUsesOk returns a tuple with the MaxUses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxUses

`func (o *EffectivePoliciesResponse) SetMaxUses(v int32)`

SetMaxUses sets MaxUses field to given value.

### HasMaxUses

`func (o *EffectivePoliciesResponse) HasMaxUses() bool`

HasMaxUses returns a boolean if a field has been set.

### SetMaxUsesNil

`func (o *EffectivePoliciesResponse) SetMaxUsesNil(b bool)`

 SetMaxUsesNil sets the value for MaxUses to be an explicit nil

### UnsetMaxUses
`func (o *EffectivePoliciesResponse) UnsetMaxUses()`

UnsetMaxUses ensures that no value is present for MaxUses, not even an explicit nil
### GetNoReshare

`func (o *EffectivePoliciesResponse) GetNoReshare() bool`

GetNoReshare returns the NoReshare field if non-nil, zero value otherwise.

### GetNoReshareOk

`func (o *EffectivePoliciesResponse) GetNoReshareOk() (*bool, bool)`

GetNoReshareOk returns a tuple with the NoReshare field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNoReshare

`func (o *EffectivePoliciesResponse) SetNoReshare(v bool)`

SetNoReshare sets NoReshare field to given value.

### HasNoReshare

`func (o *EffectivePoliciesResponse) HasNoReshare() bool`

HasNoReshare returns a boolean if a field has been set.

### GetNoTransitiveDelegation

`func (o *EffectivePoliciesResponse) GetNoTransitiveDelegation() bool`

GetNoTransitiveDelegation returns the NoTransitiveDelegation field if non-nil, zero value otherwise.

### GetNoTransitiveDelegationOk

`func (o *EffectivePoliciesResponse) GetNoTransitiveDelegationOk() (*bool, bool)`

GetNoTransitiveDelegationOk returns a tuple with the NoTransitiveDelegation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNoTransitiveDelegation

`func (o *EffectivePoliciesResponse) SetNoTransitiveDelegation(v bool)`

SetNoTransitiveDelegation sets NoTransitiveDelegation field to given value.

### HasNoTransitiveDelegation

`func (o *EffectivePoliciesResponse) HasNoTransitiveDelegation() bool`

HasNoTransitiveDelegation returns a boolean if a field has been set.

### GetProhibitedMasterDoorIds

`func (o *EffectivePoliciesResponse) GetProhibitedMasterDoorIds() []string`

GetProhibitedMasterDoorIds returns the ProhibitedMasterDoorIds field if non-nil, zero value otherwise.

### GetProhibitedMasterDoorIdsOk

`func (o *EffectivePoliciesResponse) GetProhibitedMasterDoorIdsOk() (*[]string, bool)`

GetProhibitedMasterDoorIdsOk returns a tuple with the ProhibitedMasterDoorIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProhibitedMasterDoorIds

`func (o *EffectivePoliciesResponse) SetProhibitedMasterDoorIds(v []string)`

SetProhibitedMasterDoorIds sets ProhibitedMasterDoorIds field to given value.

### HasProhibitedMasterDoorIds

`func (o *EffectivePoliciesResponse) HasProhibitedMasterDoorIds() bool`

HasProhibitedMasterDoorIds returns a boolean if a field has been set.

### SetProhibitedMasterDoorIdsNil

`func (o *EffectivePoliciesResponse) SetProhibitedMasterDoorIdsNil(b bool)`

 SetProhibitedMasterDoorIdsNil sets the value for ProhibitedMasterDoorIds to be an explicit nil

### UnsetProhibitedMasterDoorIds
`func (o *EffectivePoliciesResponse) UnsetProhibitedMasterDoorIds()`

UnsetProhibitedMasterDoorIds ensures that no value is present for ProhibitedMasterDoorIds, not even an explicit nil
### GetRequireExpiry

`func (o *EffectivePoliciesResponse) GetRequireExpiry() bool`

GetRequireExpiry returns the RequireExpiry field if non-nil, zero value otherwise.

### GetRequireExpiryOk

`func (o *EffectivePoliciesResponse) GetRequireExpiryOk() (*bool, bool)`

GetRequireExpiryOk returns a tuple with the RequireExpiry field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequireExpiry

`func (o *EffectivePoliciesResponse) SetRequireExpiry(v bool)`

SetRequireExpiry sets RequireExpiry field to given value.

### HasRequireExpiry

`func (o *EffectivePoliciesResponse) HasRequireExpiry() bool`

HasRequireExpiry returns a boolean if a field has been set.

### GetRequireJustification

`func (o *EffectivePoliciesResponse) GetRequireJustification() bool`

GetRequireJustification returns the RequireJustification field if non-nil, zero value otherwise.

### GetRequireJustificationOk

`func (o *EffectivePoliciesResponse) GetRequireJustificationOk() (*bool, bool)`

GetRequireJustificationOk returns a tuple with the RequireJustification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequireJustification

`func (o *EffectivePoliciesResponse) SetRequireJustification(v bool)`

SetRequireJustification sets RequireJustification field to given value.

### HasRequireJustification

`func (o *EffectivePoliciesResponse) HasRequireJustification() bool`

HasRequireJustification returns a boolean if a field has been set.

### GetRequirePhoto

`func (o *EffectivePoliciesResponse) GetRequirePhoto() bool`

GetRequirePhoto returns the RequirePhoto field if non-nil, zero value otherwise.

### GetRequirePhotoOk

`func (o *EffectivePoliciesResponse) GetRequirePhotoOk() (*bool, bool)`

GetRequirePhotoOk returns a tuple with the RequirePhoto field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequirePhoto

`func (o *EffectivePoliciesResponse) SetRequirePhoto(v bool)`

SetRequirePhoto sets RequirePhoto field to given value.

### HasRequirePhoto

`func (o *EffectivePoliciesResponse) HasRequirePhoto() bool`

HasRequirePhoto returns a boolean if a field has been set.

### GetRequirePin

`func (o *EffectivePoliciesResponse) GetRequirePin() bool`

GetRequirePin returns the RequirePin field if non-nil, zero value otherwise.

### GetRequirePinOk

`func (o *EffectivePoliciesResponse) GetRequirePinOk() (*bool, bool)`

GetRequirePinOk returns a tuple with the RequirePin field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequirePin

`func (o *EffectivePoliciesResponse) SetRequirePin(v bool)`

SetRequirePin sets RequirePin field to given value.

### HasRequirePin

`func (o *EffectivePoliciesResponse) HasRequirePin() bool`

HasRequirePin returns a boolean if a field has been set.

### GetRequireVerifiedPhone

`func (o *EffectivePoliciesResponse) GetRequireVerifiedPhone() bool`

GetRequireVerifiedPhone returns the RequireVerifiedPhone field if non-nil, zero value otherwise.

### GetRequireVerifiedPhoneOk

`func (o *EffectivePoliciesResponse) GetRequireVerifiedPhoneOk() (*bool, bool)`

GetRequireVerifiedPhoneOk returns a tuple with the RequireVerifiedPhone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequireVerifiedPhone

`func (o *EffectivePoliciesResponse) SetRequireVerifiedPhone(v bool)`

SetRequireVerifiedPhone sets RequireVerifiedPhone field to given value.

### HasRequireVerifiedPhone

`func (o *EffectivePoliciesResponse) HasRequireVerifiedPhone() bool`

HasRequireVerifiedPhone returns a boolean if a field has been set.

### GetRestrictToOwnListingDoors

`func (o *EffectivePoliciesResponse) GetRestrictToOwnListingDoors() bool`

GetRestrictToOwnListingDoors returns the RestrictToOwnListingDoors field if non-nil, zero value otherwise.

### GetRestrictToOwnListingDoorsOk

`func (o *EffectivePoliciesResponse) GetRestrictToOwnListingDoorsOk() (*bool, bool)`

GetRestrictToOwnListingDoorsOk returns a tuple with the RestrictToOwnListingDoors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRestrictToOwnListingDoors

`func (o *EffectivePoliciesResponse) SetRestrictToOwnListingDoors(v bool)`

SetRestrictToOwnListingDoors sets RestrictToOwnListingDoors field to given value.

### HasRestrictToOwnListingDoors

`func (o *EffectivePoliciesResponse) HasRestrictToOwnListingDoors() bool`

HasRestrictToOwnListingDoors returns a boolean if a field has been set.

### GetShareAllowUnlimited

`func (o *EffectivePoliciesResponse) GetShareAllowUnlimited() bool`

GetShareAllowUnlimited returns the ShareAllowUnlimited field if non-nil, zero value otherwise.

### GetShareAllowUnlimitedOk

`func (o *EffectivePoliciesResponse) GetShareAllowUnlimitedOk() (*bool, bool)`

GetShareAllowUnlimitedOk returns a tuple with the ShareAllowUnlimited field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetShareAllowUnlimited

`func (o *EffectivePoliciesResponse) SetShareAllowUnlimited(v bool)`

SetShareAllowUnlimited sets ShareAllowUnlimited field to given value.

### HasShareAllowUnlimited

`func (o *EffectivePoliciesResponse) HasShareAllowUnlimited() bool`

HasShareAllowUnlimited returns a boolean if a field has been set.

### GetShareDefaultMaxUses

`func (o *EffectivePoliciesResponse) GetShareDefaultMaxUses() int32`

GetShareDefaultMaxUses returns the ShareDefaultMaxUses field if non-nil, zero value otherwise.

### GetShareDefaultMaxUsesOk

`func (o *EffectivePoliciesResponse) GetShareDefaultMaxUsesOk() (*int32, bool)`

GetShareDefaultMaxUsesOk returns a tuple with the ShareDefaultMaxUses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetShareDefaultMaxUses

`func (o *EffectivePoliciesResponse) SetShareDefaultMaxUses(v int32)`

SetShareDefaultMaxUses sets ShareDefaultMaxUses field to given value.

### HasShareDefaultMaxUses

`func (o *EffectivePoliciesResponse) HasShareDefaultMaxUses() bool`

HasShareDefaultMaxUses returns a boolean if a field has been set.

### SetShareDefaultMaxUsesNil

`func (o *EffectivePoliciesResponse) SetShareDefaultMaxUsesNil(b bool)`

 SetShareDefaultMaxUsesNil sets the value for ShareDefaultMaxUses to be an explicit nil

### UnsetShareDefaultMaxUses
`func (o *EffectivePoliciesResponse) UnsetShareDefaultMaxUses()`

UnsetShareDefaultMaxUses ensures that no value is present for ShareDefaultMaxUses, not even an explicit nil
### GetShareMaxDurationSeconds

`func (o *EffectivePoliciesResponse) GetShareMaxDurationSeconds() int64`

GetShareMaxDurationSeconds returns the ShareMaxDurationSeconds field if non-nil, zero value otherwise.

### GetShareMaxDurationSecondsOk

`func (o *EffectivePoliciesResponse) GetShareMaxDurationSecondsOk() (*int64, bool)`

GetShareMaxDurationSecondsOk returns a tuple with the ShareMaxDurationSeconds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetShareMaxDurationSeconds

`func (o *EffectivePoliciesResponse) SetShareMaxDurationSeconds(v int64)`

SetShareMaxDurationSeconds sets ShareMaxDurationSeconds field to given value.

### HasShareMaxDurationSeconds

`func (o *EffectivePoliciesResponse) HasShareMaxDurationSeconds() bool`

HasShareMaxDurationSeconds returns a boolean if a field has been set.

### SetShareMaxDurationSecondsNil

`func (o *EffectivePoliciesResponse) SetShareMaxDurationSecondsNil(b bool)`

 SetShareMaxDurationSecondsNil sets the value for ShareMaxDurationSeconds to be an explicit nil

### UnsetShareMaxDurationSeconds
`func (o *EffectivePoliciesResponse) UnsetShareMaxDurationSeconds()`

UnsetShareMaxDurationSeconds ensures that no value is present for ShareMaxDurationSeconds, not even an explicit nil
### GetShareMaxUses

`func (o *EffectivePoliciesResponse) GetShareMaxUses() int32`

GetShareMaxUses returns the ShareMaxUses field if non-nil, zero value otherwise.

### GetShareMaxUsesOk

`func (o *EffectivePoliciesResponse) GetShareMaxUsesOk() (*int32, bool)`

GetShareMaxUsesOk returns a tuple with the ShareMaxUses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetShareMaxUses

`func (o *EffectivePoliciesResponse) SetShareMaxUses(v int32)`

SetShareMaxUses sets ShareMaxUses field to given value.

### HasShareMaxUses

`func (o *EffectivePoliciesResponse) HasShareMaxUses() bool`

HasShareMaxUses returns a boolean if a field has been set.

### SetShareMaxUsesNil

`func (o *EffectivePoliciesResponse) SetShareMaxUsesNil(b bool)`

 SetShareMaxUsesNil sets the value for ShareMaxUses to be an explicit nil

### UnsetShareMaxUses
`func (o *EffectivePoliciesResponse) UnsetShareMaxUses()`

UnsetShareMaxUses ensures that no value is present for ShareMaxUses, not even an explicit nil
### GetTimezone

`func (o *EffectivePoliciesResponse) GetTimezone() string`

GetTimezone returns the Timezone field if non-nil, zero value otherwise.

### GetTimezoneOk

`func (o *EffectivePoliciesResponse) GetTimezoneOk() (*string, bool)`

GetTimezoneOk returns a tuple with the Timezone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimezone

`func (o *EffectivePoliciesResponse) SetTimezone(v string)`

SetTimezone sets Timezone field to given value.


### GetUserSharingMode

`func (o *EffectivePoliciesResponse) GetUserSharingMode() string`

GetUserSharingMode returns the UserSharingMode field if non-nil, zero value otherwise.

### GetUserSharingModeOk

`func (o *EffectivePoliciesResponse) GetUserSharingModeOk() (*string, bool)`

GetUserSharingModeOk returns a tuple with the UserSharingMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserSharingMode

`func (o *EffectivePoliciesResponse) SetUserSharingMode(v string)`

SetUserSharingMode sets UserSharingMode field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
