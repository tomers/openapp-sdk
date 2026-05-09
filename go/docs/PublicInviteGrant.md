# PublicInviteGrant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DoorImageUrl** | Pointer to **string** | Presigned URL for door image (loaded asynchronously as card background). | [optional]
**HasLights** | **bool** | Whether the portal has light devices configured (controls light button visibility). |
**Id** | **string** |  |
**Kind** | **string** |  |
**Label** | Pointer to **interface{}** |  | [optional]
**PublicPortalId** | **string** |  |
**ActionId** | **string** |  |
**EntityId** | **string** |  |
**Payload** | Pointer to **interface{}** |  | [optional]

## Methods

### NewPublicInviteGrant

`func NewPublicInviteGrant(hasLights bool, id string, kind string, publicPortalId string, actionId string, entityId string, ) *PublicInviteGrant`

NewPublicInviteGrant instantiates a new PublicInviteGrant object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicInviteGrantWithDefaults

`func NewPublicInviteGrantWithDefaults() *PublicInviteGrant`

NewPublicInviteGrantWithDefaults instantiates a new PublicInviteGrant object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

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
