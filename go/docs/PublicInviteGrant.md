# PublicInviteGrant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DoorImageThumbUrl** | Pointer to **string** | Presigned URL for the &#x60;thumb&#x60; rendition of the door image, for avatar-sized renders. Absent when the source is already thumb-sized; fall back to &#x60;door_image_url&#x60;. | [optional]
**DoorImageUrl** | Pointer to **string** | Presigned URL for door image (loaded asynchronously as card background). | [optional]
**EntryKind** | **string** | Entry type from linked portal device (&#x60;virtual_access.entry_kind&#x60;). Default &#x60;door&#x60;. |
**HasLights** | **bool** | Whether the portal has light devices configured (controls light button visibility). |
**Hold** | Pointer to [**PublicHoldView**](PublicHoldView.md) | Effective door hold when any opener is held (guest-visible). | [optional]
**Id** | **string** |  |
**Kind** | **string** |  |
**Label** | Pointer to **interface{}** |  | [optional]
**OpenRateLimit** | Pointer to [**PublicPortalOpenRateLimit**](PublicPortalOpenRateLimit.md) | Enforced integration-scoped minimum interval between &#x60;switchable.open&#x60; calls. A second open inside the interval returns 429 &#x60;provider_open_rate_limited&#x60;. | [optional]
**Openable** | **bool** | Whether the portal is linked to a live door device (i.e. can actually be opened). When false, the invite UI disables the open action (a misconfigured/unlinked portal). |
**PublicPortalId** | **string** |  |
**ActionId** | **string** |  |
**EntityId** | **string** |  |
**Payload** | Pointer to **interface{}** |  | [optional]

## Methods

### NewPublicInviteGrant

`func NewPublicInviteGrant(entryKind string, hasLights bool, id string, kind string, openable bool, publicPortalId string, actionId string, entityId string, ) *PublicInviteGrant`

NewPublicInviteGrant instantiates a new PublicInviteGrant object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicInviteGrantWithDefaults

`func NewPublicInviteGrantWithDefaults() *PublicInviteGrant`

NewPublicInviteGrantWithDefaults instantiates a new PublicInviteGrant object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDoorImageThumbUrl

`func (o *PublicInviteGrant) GetDoorImageThumbUrl() string`

GetDoorImageThumbUrl returns the DoorImageThumbUrl field if non-nil, zero value otherwise.

### GetDoorImageThumbUrlOk

`func (o *PublicInviteGrant) GetDoorImageThumbUrlOk() (*string, bool)`

GetDoorImageThumbUrlOk returns a tuple with the DoorImageThumbUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorImageThumbUrl

`func (o *PublicInviteGrant) SetDoorImageThumbUrl(v string)`

SetDoorImageThumbUrl sets DoorImageThumbUrl field to given value.

### HasDoorImageThumbUrl

`func (o *PublicInviteGrant) HasDoorImageThumbUrl() bool`

HasDoorImageThumbUrl returns a boolean if a field has been set.

### GetDoorImageUrl

`func (o *PublicInviteGrant) GetDoorImageUrl() string`

GetDoorImageUrl returns the DoorImageUrl field if non-nil, zero value otherwise.

### GetDoorImageUrlOk

`func (o *PublicInviteGrant) GetDoorImageUrlOk() (*string, bool)`

GetDoorImageUrlOk returns a tuple with the DoorImageUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorImageUrl

`func (o *PublicInviteGrant) SetDoorImageUrl(v string)`

SetDoorImageUrl sets DoorImageUrl field to given value.

### HasDoorImageUrl

`func (o *PublicInviteGrant) HasDoorImageUrl() bool`

HasDoorImageUrl returns a boolean if a field has been set.

### GetEntryKind

`func (o *PublicInviteGrant) GetEntryKind() string`

GetEntryKind returns the EntryKind field if non-nil, zero value otherwise.

### GetEntryKindOk

`func (o *PublicInviteGrant) GetEntryKindOk() (*string, bool)`

GetEntryKindOk returns a tuple with the EntryKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntryKind

`func (o *PublicInviteGrant) SetEntryKind(v string)`

SetEntryKind sets EntryKind field to given value.


### GetHasLights

`func (o *PublicInviteGrant) GetHasLights() bool`

GetHasLights returns the HasLights field if non-nil, zero value otherwise.

### GetHasLightsOk

`func (o *PublicInviteGrant) GetHasLightsOk() (*bool, bool)`

GetHasLightsOk returns a tuple with the HasLights field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasLights

`func (o *PublicInviteGrant) SetHasLights(v bool)`

SetHasLights sets HasLights field to given value.


### GetHold

`func (o *PublicInviteGrant) GetHold() PublicHoldView`

GetHold returns the Hold field if non-nil, zero value otherwise.

### GetHoldOk

`func (o *PublicInviteGrant) GetHoldOk() (*PublicHoldView, bool)`

GetHoldOk returns a tuple with the Hold field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHold

`func (o *PublicInviteGrant) SetHold(v PublicHoldView)`

SetHold sets Hold field to given value.

### HasHold

`func (o *PublicInviteGrant) HasHold() bool`

HasHold returns a boolean if a field has been set.

### GetId

`func (o *PublicInviteGrant) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PublicInviteGrant) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PublicInviteGrant) SetId(v string)`

SetId sets Id field to given value.


### GetKind

`func (o *PublicInviteGrant) GetKind() string`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *PublicInviteGrant) GetKindOk() (*string, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *PublicInviteGrant) SetKind(v string)`

SetKind sets Kind field to given value.


### GetLabel

`func (o *PublicInviteGrant) GetLabel() interface{}`

GetLabel returns the Label field if non-nil, zero value otherwise.

### GetLabelOk

`func (o *PublicInviteGrant) GetLabelOk() (*interface{}, bool)`

GetLabelOk returns a tuple with the Label field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLabel

`func (o *PublicInviteGrant) SetLabel(v interface{})`

SetLabel sets Label field to given value.

### HasLabel

`func (o *PublicInviteGrant) HasLabel() bool`

HasLabel returns a boolean if a field has been set.

### SetLabelNil

`func (o *PublicInviteGrant) SetLabelNil(b bool)`

 SetLabelNil sets the value for Label to be an explicit nil

### UnsetLabel
`func (o *PublicInviteGrant) UnsetLabel()`

UnsetLabel ensures that no value is present for Label, not even an explicit nil
### GetOpenRateLimit

`func (o *PublicInviteGrant) GetOpenRateLimit() PublicPortalOpenRateLimit`

GetOpenRateLimit returns the OpenRateLimit field if non-nil, zero value otherwise.

### GetOpenRateLimitOk

`func (o *PublicInviteGrant) GetOpenRateLimitOk() (*PublicPortalOpenRateLimit, bool)`

GetOpenRateLimitOk returns a tuple with the OpenRateLimit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpenRateLimit

`func (o *PublicInviteGrant) SetOpenRateLimit(v PublicPortalOpenRateLimit)`

SetOpenRateLimit sets OpenRateLimit field to given value.

### HasOpenRateLimit

`func (o *PublicInviteGrant) HasOpenRateLimit() bool`

HasOpenRateLimit returns a boolean if a field has been set.

### GetOpenable

`func (o *PublicInviteGrant) GetOpenable() bool`

GetOpenable returns the Openable field if non-nil, zero value otherwise.

### GetOpenableOk

`func (o *PublicInviteGrant) GetOpenableOk() (*bool, bool)`

GetOpenableOk returns a tuple with the Openable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpenable

`func (o *PublicInviteGrant) SetOpenable(v bool)`

SetOpenable sets Openable field to given value.


### GetPublicPortalId

`func (o *PublicInviteGrant) GetPublicPortalId() string`

GetPublicPortalId returns the PublicPortalId field if non-nil, zero value otherwise.

### GetPublicPortalIdOk

`func (o *PublicInviteGrant) GetPublicPortalIdOk() (*string, bool)`

GetPublicPortalIdOk returns a tuple with the PublicPortalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicPortalId

`func (o *PublicInviteGrant) SetPublicPortalId(v string)`

SetPublicPortalId sets PublicPortalId field to given value.


### GetActionId

`func (o *PublicInviteGrant) GetActionId() string`

GetActionId returns the ActionId field if non-nil, zero value otherwise.

### GetActionIdOk

`func (o *PublicInviteGrant) GetActionIdOk() (*string, bool)`

GetActionIdOk returns a tuple with the ActionId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActionId

`func (o *PublicInviteGrant) SetActionId(v string)`

SetActionId sets ActionId field to given value.


### GetEntityId

`func (o *PublicInviteGrant) GetEntityId() string`

GetEntityId returns the EntityId field if non-nil, zero value otherwise.

### GetEntityIdOk

`func (o *PublicInviteGrant) GetEntityIdOk() (*string, bool)`

GetEntityIdOk returns a tuple with the EntityId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityId

`func (o *PublicInviteGrant) SetEntityId(v string)`

SetEntityId sets EntityId field to given value.


### GetPayload

`func (o *PublicInviteGrant) GetPayload() interface{}`

GetPayload returns the Payload field if non-nil, zero value otherwise.

### GetPayloadOk

`func (o *PublicInviteGrant) GetPayloadOk() (*interface{}, bool)`

GetPayloadOk returns a tuple with the Payload field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPayload

`func (o *PublicInviteGrant) SetPayload(v interface{})`

SetPayload sets Payload field to given value.

### HasPayload

`func (o *PublicInviteGrant) HasPayload() bool`

HasPayload returns a boolean if a field has been set.

### SetPayloadNil

`func (o *PublicInviteGrant) SetPayloadNil(b bool)`

 SetPayloadNil sets the value for Payload to be an explicit nil

### UnsetPayload
`func (o *PublicInviteGrant) UnsetPayload()`

UnsetPayload ensures that no value is present for Payload, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
