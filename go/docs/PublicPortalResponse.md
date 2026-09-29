# PublicPortalResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AccessControl** | Pointer to [**NullablePublicPortalAccessControl**](PublicPortalAccessControl.md) | Access-control integration summary (building name, address, photo). | [optional]
**AccessDirectory** | Pointer to [**NullablePublicPortalAccessControl**](PublicPortalAccessControl.md) | Visitor directory device summary for this portal (building directory name, photo). | [optional]
**Branding** | Pointer to **interface{}** |  | [optional]
**DoorEffectiveAutoOffDuration** | Pointer to **NullableInt64** |  | [optional]
**DoorImageUrl** | Pointer to **NullableString** | Image URL for the door (presigned S3 URL from media service). | [optional]
**EntryKind** | **string** | Entry type from linked portal device (&#x60;virtual_access.entry_kind&#x60;). Default &#x60;door&#x60;. |
**Hold** | Pointer to [**NullablePublicHoldView**](PublicHoldView.md) | Effective door hold when any opener is held (guest-visible). | [optional]
**Lights** | Pointer to [**[]PublicPortalLight**](PublicPortalLight.md) |  | [optional]
**Mode** | [**PublicPortalMode**](PublicPortalMode.md) |  |
**Name** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**OpenRateLimit** | Pointer to [**NullablePublicPortalOpenRateLimit**](PublicPortalOpenRateLimit.md) |  | [optional]
**PublicPortalId** | **string** |  |

## Methods

### NewPublicPortalResponse

`func NewPublicPortalResponse(entryKind string, mode PublicPortalMode, publicPortalId string, ) *PublicPortalResponse`

NewPublicPortalResponse instantiates a new PublicPortalResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicPortalResponseWithDefaults

`func NewPublicPortalResponseWithDefaults() *PublicPortalResponse`

NewPublicPortalResponseWithDefaults instantiates a new PublicPortalResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAccessControl

`func (o *PublicPortalResponse) GetAccessControl() PublicPortalAccessControl`

GetAccessControl returns the AccessControl field if non-nil, zero value otherwise.

### GetAccessControlOk

`func (o *PublicPortalResponse) GetAccessControlOk() (*PublicPortalAccessControl, bool)`

GetAccessControlOk returns a tuple with the AccessControl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAccessControl

`func (o *PublicPortalResponse) SetAccessControl(v PublicPortalAccessControl)`

SetAccessControl sets AccessControl field to given value.

### HasAccessControl

`func (o *PublicPortalResponse) HasAccessControl() bool`

HasAccessControl returns a boolean if a field has been set.

### SetAccessControlNil

`func (o *PublicPortalResponse) SetAccessControlNil(b bool)`

 SetAccessControlNil sets the value for AccessControl to be an explicit nil

### UnsetAccessControl
`func (o *PublicPortalResponse) UnsetAccessControl()`

UnsetAccessControl ensures that no value is present for AccessControl, not even an explicit nil
### GetAccessDirectory

`func (o *PublicPortalResponse) GetAccessDirectory() PublicPortalAccessControl`

GetAccessDirectory returns the AccessDirectory field if non-nil, zero value otherwise.

### GetAccessDirectoryOk

`func (o *PublicPortalResponse) GetAccessDirectoryOk() (*PublicPortalAccessControl, bool)`

GetAccessDirectoryOk returns a tuple with the AccessDirectory field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAccessDirectory

`func (o *PublicPortalResponse) SetAccessDirectory(v PublicPortalAccessControl)`

SetAccessDirectory sets AccessDirectory field to given value.

### HasAccessDirectory

`func (o *PublicPortalResponse) HasAccessDirectory() bool`

HasAccessDirectory returns a boolean if a field has been set.

### SetAccessDirectoryNil

`func (o *PublicPortalResponse) SetAccessDirectoryNil(b bool)`

 SetAccessDirectoryNil sets the value for AccessDirectory to be an explicit nil

### UnsetAccessDirectory
`func (o *PublicPortalResponse) UnsetAccessDirectory()`

UnsetAccessDirectory ensures that no value is present for AccessDirectory, not even an explicit nil
### GetBranding

`func (o *PublicPortalResponse) GetBranding() interface{}`

GetBranding returns the Branding field if non-nil, zero value otherwise.

### GetBrandingOk

`func (o *PublicPortalResponse) GetBrandingOk() (*interface{}, bool)`

GetBrandingOk returns a tuple with the Branding field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBranding

`func (o *PublicPortalResponse) SetBranding(v interface{})`

SetBranding sets Branding field to given value.

### HasBranding

`func (o *PublicPortalResponse) HasBranding() bool`

HasBranding returns a boolean if a field has been set.

### SetBrandingNil

`func (o *PublicPortalResponse) SetBrandingNil(b bool)`

 SetBrandingNil sets the value for Branding to be an explicit nil

### UnsetBranding
`func (o *PublicPortalResponse) UnsetBranding()`

UnsetBranding ensures that no value is present for Branding, not even an explicit nil
### GetDoorEffectiveAutoOffDuration

`func (o *PublicPortalResponse) GetDoorEffectiveAutoOffDuration() int64`

GetDoorEffectiveAutoOffDuration returns the DoorEffectiveAutoOffDuration field if non-nil, zero value otherwise.

### GetDoorEffectiveAutoOffDurationOk

`func (o *PublicPortalResponse) GetDoorEffectiveAutoOffDurationOk() (*int64, bool)`

GetDoorEffectiveAutoOffDurationOk returns a tuple with the DoorEffectiveAutoOffDuration field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorEffectiveAutoOffDuration

`func (o *PublicPortalResponse) SetDoorEffectiveAutoOffDuration(v int64)`

SetDoorEffectiveAutoOffDuration sets DoorEffectiveAutoOffDuration field to given value.

### HasDoorEffectiveAutoOffDuration

`func (o *PublicPortalResponse) HasDoorEffectiveAutoOffDuration() bool`

HasDoorEffectiveAutoOffDuration returns a boolean if a field has been set.

### SetDoorEffectiveAutoOffDurationNil

`func (o *PublicPortalResponse) SetDoorEffectiveAutoOffDurationNil(b bool)`

 SetDoorEffectiveAutoOffDurationNil sets the value for DoorEffectiveAutoOffDuration to be an explicit nil

### UnsetDoorEffectiveAutoOffDuration
`func (o *PublicPortalResponse) UnsetDoorEffectiveAutoOffDuration()`

UnsetDoorEffectiveAutoOffDuration ensures that no value is present for DoorEffectiveAutoOffDuration, not even an explicit nil
### GetDoorImageUrl

`func (o *PublicPortalResponse) GetDoorImageUrl() string`

GetDoorImageUrl returns the DoorImageUrl field if non-nil, zero value otherwise.

### GetDoorImageUrlOk

`func (o *PublicPortalResponse) GetDoorImageUrlOk() (*string, bool)`

GetDoorImageUrlOk returns a tuple with the DoorImageUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorImageUrl

`func (o *PublicPortalResponse) SetDoorImageUrl(v string)`

SetDoorImageUrl sets DoorImageUrl field to given value.

### HasDoorImageUrl

`func (o *PublicPortalResponse) HasDoorImageUrl() bool`

HasDoorImageUrl returns a boolean if a field has been set.

### SetDoorImageUrlNil

`func (o *PublicPortalResponse) SetDoorImageUrlNil(b bool)`

 SetDoorImageUrlNil sets the value for DoorImageUrl to be an explicit nil

### UnsetDoorImageUrl
`func (o *PublicPortalResponse) UnsetDoorImageUrl()`

UnsetDoorImageUrl ensures that no value is present for DoorImageUrl, not even an explicit nil
### GetEntryKind

`func (o *PublicPortalResponse) GetEntryKind() string`

GetEntryKind returns the EntryKind field if non-nil, zero value otherwise.

### GetEntryKindOk

`func (o *PublicPortalResponse) GetEntryKindOk() (*string, bool)`

GetEntryKindOk returns a tuple with the EntryKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntryKind

`func (o *PublicPortalResponse) SetEntryKind(v string)`

SetEntryKind sets EntryKind field to given value.


### GetHold

`func (o *PublicPortalResponse) GetHold() PublicHoldView`

GetHold returns the Hold field if non-nil, zero value otherwise.

### GetHoldOk

`func (o *PublicPortalResponse) GetHoldOk() (*PublicHoldView, bool)`

GetHoldOk returns a tuple with the Hold field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHold

`func (o *PublicPortalResponse) SetHold(v PublicHoldView)`

SetHold sets Hold field to given value.

### HasHold

`func (o *PublicPortalResponse) HasHold() bool`

HasHold returns a boolean if a field has been set.

### SetHoldNil

`func (o *PublicPortalResponse) SetHoldNil(b bool)`

 SetHoldNil sets the value for Hold to be an explicit nil

### UnsetHold
`func (o *PublicPortalResponse) UnsetHold()`

UnsetHold ensures that no value is present for Hold, not even an explicit nil
### GetLights

`func (o *PublicPortalResponse) GetLights() []PublicPortalLight`

GetLights returns the Lights field if non-nil, zero value otherwise.

### GetLightsOk

`func (o *PublicPortalResponse) GetLightsOk() (*[]PublicPortalLight, bool)`

GetLightsOk returns a tuple with the Lights field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLights

`func (o *PublicPortalResponse) SetLights(v []PublicPortalLight)`

SetLights sets Lights field to given value.

### HasLights

`func (o *PublicPortalResponse) HasLights() bool`

HasLights returns a boolean if a field has been set.

### SetLightsNil

`func (o *PublicPortalResponse) SetLightsNil(b bool)`

 SetLightsNil sets the value for Lights to be an explicit nil

### UnsetLights
`func (o *PublicPortalResponse) UnsetLights()`

UnsetLights ensures that no value is present for Lights, not even an explicit nil
### GetMode

`func (o *PublicPortalResponse) GetMode() PublicPortalMode`

GetMode returns the Mode field if non-nil, zero value otherwise.

### GetModeOk

`func (o *PublicPortalResponse) GetModeOk() (*PublicPortalMode, bool)`

GetModeOk returns a tuple with the Mode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMode

`func (o *PublicPortalResponse) SetMode(v PublicPortalMode)`

SetMode sets Mode field to given value.


### GetName

`func (o *PublicPortalResponse) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PublicPortalResponse) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PublicPortalResponse) SetName(v LocalizedString)`

SetName sets Name field to given value.

### HasName

`func (o *PublicPortalResponse) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *PublicPortalResponse) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *PublicPortalResponse) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetOpenRateLimit

`func (o *PublicPortalResponse) GetOpenRateLimit() PublicPortalOpenRateLimit`

GetOpenRateLimit returns the OpenRateLimit field if non-nil, zero value otherwise.

### GetOpenRateLimitOk

`func (o *PublicPortalResponse) GetOpenRateLimitOk() (*PublicPortalOpenRateLimit, bool)`

GetOpenRateLimitOk returns a tuple with the OpenRateLimit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpenRateLimit

`func (o *PublicPortalResponse) SetOpenRateLimit(v PublicPortalOpenRateLimit)`

SetOpenRateLimit sets OpenRateLimit field to given value.

### HasOpenRateLimit

`func (o *PublicPortalResponse) HasOpenRateLimit() bool`

HasOpenRateLimit returns a boolean if a field has been set.

### SetOpenRateLimitNil

`func (o *PublicPortalResponse) SetOpenRateLimitNil(b bool)`

 SetOpenRateLimitNil sets the value for OpenRateLimit to be an explicit nil

### UnsetOpenRateLimit
`func (o *PublicPortalResponse) UnsetOpenRateLimit()`

UnsetOpenRateLimit ensures that no value is present for OpenRateLimit, not even an explicit nil
### GetPublicPortalId

`func (o *PublicPortalResponse) GetPublicPortalId() string`

GetPublicPortalId returns the PublicPortalId field if non-nil, zero value otherwise.

### GetPublicPortalIdOk

`func (o *PublicPortalResponse) GetPublicPortalIdOk() (*string, bool)`

GetPublicPortalIdOk returns a tuple with the PublicPortalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicPortalId

`func (o *PublicPortalResponse) SetPublicPortalId(v string)`

SetPublicPortalId sets PublicPortalId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
