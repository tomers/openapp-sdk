# PublicInviteGrantOneOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DoorImageThumbUrl** | Pointer to **NullableString** | Presigned URL for the &#x60;thumb&#x60; rendition of the door image, for avatar-sized renders. Absent when the source is already thumb-sized; fall back to &#x60;door_image_url&#x60;. | [optional]
**DoorImageUrl** | Pointer to **NullableString** | Presigned URL for door image (loaded asynchronously as card background). | [optional]
**EntryKind** | **string** | Entry type from linked portal device (&#x60;virtual_access.entry_kind&#x60;). Default &#x60;door&#x60;. |
**HasLights** | **bool** | Whether the portal has light devices configured (controls light button visibility). |
**Hold** | Pointer to [**NullablePublicHoldView**](PublicHoldView.md) | Effective door hold when any opener is held (guest-visible). | [optional]
**Id** | **string** |  |
**Kind** | **string** |  |
**Label** | Pointer to **interface{}** |  | [optional]
**OpenRateLimit** | Pointer to [**NullablePublicPortalOpenRateLimit**](PublicPortalOpenRateLimit.md) | Enforced integration-scoped minimum interval between &#x60;switchable.open&#x60; calls. A second open inside the interval returns 429 &#x60;provider_open_rate_limited&#x60;. | [optional]
**Openable** | **bool** | Whether the portal is linked to a live door device (i.e. can actually be opened). When false, the invite UI disables the open action (a misconfigured/unlinked portal). |
**PublicPortalId** | **string** |  |

## Methods

### NewPublicInviteGrantOneOf

`func NewPublicInviteGrantOneOf(entryKind string, hasLights bool, id string, kind string, openable bool, publicPortalId string, ) *PublicInviteGrantOneOf`

NewPublicInviteGrantOneOf instantiates a new PublicInviteGrantOneOf object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicInviteGrantOneOfWithDefaults

`func NewPublicInviteGrantOneOfWithDefaults() *PublicInviteGrantOneOf`

NewPublicInviteGrantOneOfWithDefaults instantiates a new PublicInviteGrantOneOf object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDoorImageThumbUrl

`func (o *PublicInviteGrantOneOf) GetDoorImageThumbUrl() string`

GetDoorImageThumbUrl returns the DoorImageThumbUrl field if non-nil, zero value otherwise.

### GetDoorImageThumbUrlOk

`func (o *PublicInviteGrantOneOf) GetDoorImageThumbUrlOk() (*string, bool)`

GetDoorImageThumbUrlOk returns a tuple with the DoorImageThumbUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorImageThumbUrl

`func (o *PublicInviteGrantOneOf) SetDoorImageThumbUrl(v string)`

SetDoorImageThumbUrl sets DoorImageThumbUrl field to given value.

### HasDoorImageThumbUrl

`func (o *PublicInviteGrantOneOf) HasDoorImageThumbUrl() bool`

HasDoorImageThumbUrl returns a boolean if a field has been set.

### SetDoorImageThumbUrlNil

`func (o *PublicInviteGrantOneOf) SetDoorImageThumbUrlNil(b bool)`

 SetDoorImageThumbUrlNil sets the value for DoorImageThumbUrl to be an explicit nil

### UnsetDoorImageThumbUrl
`func (o *PublicInviteGrantOneOf) UnsetDoorImageThumbUrl()`

UnsetDoorImageThumbUrl ensures that no value is present for DoorImageThumbUrl, not even an explicit nil
### GetDoorImageUrl

`func (o *PublicInviteGrantOneOf) GetDoorImageUrl() string`

GetDoorImageUrl returns the DoorImageUrl field if non-nil, zero value otherwise.

### GetDoorImageUrlOk

`func (o *PublicInviteGrantOneOf) GetDoorImageUrlOk() (*string, bool)`

GetDoorImageUrlOk returns a tuple with the DoorImageUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorImageUrl

`func (o *PublicInviteGrantOneOf) SetDoorImageUrl(v string)`

SetDoorImageUrl sets DoorImageUrl field to given value.

### HasDoorImageUrl

`func (o *PublicInviteGrantOneOf) HasDoorImageUrl() bool`

HasDoorImageUrl returns a boolean if a field has been set.

### SetDoorImageUrlNil

`func (o *PublicInviteGrantOneOf) SetDoorImageUrlNil(b bool)`

 SetDoorImageUrlNil sets the value for DoorImageUrl to be an explicit nil

### UnsetDoorImageUrl
`func (o *PublicInviteGrantOneOf) UnsetDoorImageUrl()`

UnsetDoorImageUrl ensures that no value is present for DoorImageUrl, not even an explicit nil
### GetEntryKind

`func (o *PublicInviteGrantOneOf) GetEntryKind() string`

GetEntryKind returns the EntryKind field if non-nil, zero value otherwise.

### GetEntryKindOk

`func (o *PublicInviteGrantOneOf) GetEntryKindOk() (*string, bool)`

GetEntryKindOk returns a tuple with the EntryKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntryKind

`func (o *PublicInviteGrantOneOf) SetEntryKind(v string)`

SetEntryKind sets EntryKind field to given value.


### GetHasLights

`func (o *PublicInviteGrantOneOf) GetHasLights() bool`

GetHasLights returns the HasLights field if non-nil, zero value otherwise.

### GetHasLightsOk

`func (o *PublicInviteGrantOneOf) GetHasLightsOk() (*bool, bool)`

GetHasLightsOk returns a tuple with the HasLights field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasLights

`func (o *PublicInviteGrantOneOf) SetHasLights(v bool)`

SetHasLights sets HasLights field to given value.


### GetHold

`func (o *PublicInviteGrantOneOf) GetHold() PublicHoldView`

GetHold returns the Hold field if non-nil, zero value otherwise.

### GetHoldOk

`func (o *PublicInviteGrantOneOf) GetHoldOk() (*PublicHoldView, bool)`

GetHoldOk returns a tuple with the Hold field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHold

`func (o *PublicInviteGrantOneOf) SetHold(v PublicHoldView)`

SetHold sets Hold field to given value.

### HasHold

`func (o *PublicInviteGrantOneOf) HasHold() bool`

HasHold returns a boolean if a field has been set.

### SetHoldNil

`func (o *PublicInviteGrantOneOf) SetHoldNil(b bool)`

 SetHoldNil sets the value for Hold to be an explicit nil

### UnsetHold
`func (o *PublicInviteGrantOneOf) UnsetHold()`

UnsetHold ensures that no value is present for Hold, not even an explicit nil
### GetId

`func (o *PublicInviteGrantOneOf) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PublicInviteGrantOneOf) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PublicInviteGrantOneOf) SetId(v string)`

SetId sets Id field to given value.


### GetKind

`func (o *PublicInviteGrantOneOf) GetKind() string`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *PublicInviteGrantOneOf) GetKindOk() (*string, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *PublicInviteGrantOneOf) SetKind(v string)`

SetKind sets Kind field to given value.


### GetLabel

`func (o *PublicInviteGrantOneOf) GetLabel() interface{}`

GetLabel returns the Label field if non-nil, zero value otherwise.

### GetLabelOk

`func (o *PublicInviteGrantOneOf) GetLabelOk() (*interface{}, bool)`

GetLabelOk returns a tuple with the Label field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLabel

`func (o *PublicInviteGrantOneOf) SetLabel(v interface{})`

SetLabel sets Label field to given value.

### HasLabel

`func (o *PublicInviteGrantOneOf) HasLabel() bool`

HasLabel returns a boolean if a field has been set.

### SetLabelNil

`func (o *PublicInviteGrantOneOf) SetLabelNil(b bool)`

 SetLabelNil sets the value for Label to be an explicit nil

### UnsetLabel
`func (o *PublicInviteGrantOneOf) UnsetLabel()`

UnsetLabel ensures that no value is present for Label, not even an explicit nil
### GetOpenRateLimit

`func (o *PublicInviteGrantOneOf) GetOpenRateLimit() PublicPortalOpenRateLimit`

GetOpenRateLimit returns the OpenRateLimit field if non-nil, zero value otherwise.

### GetOpenRateLimitOk

`func (o *PublicInviteGrantOneOf) GetOpenRateLimitOk() (*PublicPortalOpenRateLimit, bool)`

GetOpenRateLimitOk returns a tuple with the OpenRateLimit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpenRateLimit

`func (o *PublicInviteGrantOneOf) SetOpenRateLimit(v PublicPortalOpenRateLimit)`

SetOpenRateLimit sets OpenRateLimit field to given value.

### HasOpenRateLimit

`func (o *PublicInviteGrantOneOf) HasOpenRateLimit() bool`

HasOpenRateLimit returns a boolean if a field has been set.

### SetOpenRateLimitNil

`func (o *PublicInviteGrantOneOf) SetOpenRateLimitNil(b bool)`

 SetOpenRateLimitNil sets the value for OpenRateLimit to be an explicit nil

### UnsetOpenRateLimit
`func (o *PublicInviteGrantOneOf) UnsetOpenRateLimit()`

UnsetOpenRateLimit ensures that no value is present for OpenRateLimit, not even an explicit nil
### GetOpenable

`func (o *PublicInviteGrantOneOf) GetOpenable() bool`

GetOpenable returns the Openable field if non-nil, zero value otherwise.

### GetOpenableOk

`func (o *PublicInviteGrantOneOf) GetOpenableOk() (*bool, bool)`

GetOpenableOk returns a tuple with the Openable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpenable

`func (o *PublicInviteGrantOneOf) SetOpenable(v bool)`

SetOpenable sets Openable field to given value.


### GetPublicPortalId

`func (o *PublicInviteGrantOneOf) GetPublicPortalId() string`

GetPublicPortalId returns the PublicPortalId field if non-nil, zero value otherwise.

### GetPublicPortalIdOk

`func (o *PublicInviteGrantOneOf) GetPublicPortalIdOk() (*string, bool)`

GetPublicPortalIdOk returns a tuple with the PublicPortalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicPortalId

`func (o *PublicInviteGrantOneOf) SetPublicPortalId(v string)`

SetPublicPortalId sets PublicPortalId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
