# PublicInviteGrantOneOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DoorImageUrl** | Pointer to **NullableString** | Presigned URL for door image (loaded asynchronously as card background). | [optional]
**HasLights** | **bool** | Whether the portal has light devices configured (controls light button visibility). |
**Id** | **string** |  |
**Kind** | **string** |  |
**Label** | Pointer to **interface{}** |  | [optional]
**PublicPortalId** | **string** |  |

## Methods

### NewPublicInviteGrantOneOf

`func NewPublicInviteGrantOneOf(hasLights bool, id string, kind string, publicPortalId string, ) *PublicInviteGrantOneOf`

NewPublicInviteGrantOneOf instantiates a new PublicInviteGrantOneOf object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicInviteGrantOneOfWithDefaults

`func NewPublicInviteGrantOneOfWithDefaults() *PublicInviteGrantOneOf`

NewPublicInviteGrantOneOfWithDefaults instantiates a new PublicInviteGrantOneOf object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

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
