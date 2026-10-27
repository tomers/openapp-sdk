# PublicPortalTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AllowedActions** | **[]string** |  |
**CallAvailable** | Pointer to **NullableBool** | True when at least one resident (receives_calls) exists. When false, voice/video are excluded from allowed_actions. | [optional]
**DenialReasons** | Pointer to **[]string** | Why this entry cannot be called; empty when nothing blocks a call. &#x60;no_callees&#x60;: nobody at this entry receives calls. &#x60;dnd&#x60;: Do Not Disturb is on for this entry, or no callee can currently be notified. &#x60;guest_disabled&#x60;: visitor access to this entry is turned off by policy. | [optional]
**DisplayName** | **interface{}** |  |
**EffectiveDisplayName** | **string** | Server-resolved visitor name. Clients render this and do not recompute it. |
**Floor** | **interface{}** |  |
**FloorNumber** | Pointer to **NullableInt64** |  | [optional]
**Image** | Pointer to **NullableString** |  | [optional]
**ListingLabel** | Pointer to **interface{}** |  | [optional]
**ListingNumber** | Pointer to **NullableInt64** |  | [optional]
**LocationDetail** | Pointer to **interface{}** |  | [optional]
**RequireVideo** | Pointer to **NullableBool** |  | [optional]
**TargetId** | **string** |  |

## Methods

### NewPublicPortalTarget

`func NewPublicPortalTarget(allowedActions []string, displayName interface{}, effectiveDisplayName string, floor interface{}, targetId string, ) *PublicPortalTarget`

NewPublicPortalTarget instantiates a new PublicPortalTarget object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicPortalTargetWithDefaults

`func NewPublicPortalTargetWithDefaults() *PublicPortalTarget`

NewPublicPortalTargetWithDefaults instantiates a new PublicPortalTarget object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAllowedActions

`func (o *PublicPortalTarget) GetAllowedActions() []string`

GetAllowedActions returns the AllowedActions field if non-nil, zero value otherwise.

### GetAllowedActionsOk

`func (o *PublicPortalTarget) GetAllowedActionsOk() (*[]string, bool)`

GetAllowedActionsOk returns a tuple with the AllowedActions field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedActions

`func (o *PublicPortalTarget) SetAllowedActions(v []string)`

SetAllowedActions sets AllowedActions field to given value.


### GetCallAvailable

`func (o *PublicPortalTarget) GetCallAvailable() bool`

GetCallAvailable returns the CallAvailable field if non-nil, zero value otherwise.

### GetCallAvailableOk

`func (o *PublicPortalTarget) GetCallAvailableOk() (*bool, bool)`

GetCallAvailableOk returns a tuple with the CallAvailable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCallAvailable

`func (o *PublicPortalTarget) SetCallAvailable(v bool)`

SetCallAvailable sets CallAvailable field to given value.

### HasCallAvailable

`func (o *PublicPortalTarget) HasCallAvailable() bool`

HasCallAvailable returns a boolean if a field has been set.

### SetCallAvailableNil

`func (o *PublicPortalTarget) SetCallAvailableNil(b bool)`

 SetCallAvailableNil sets the value for CallAvailable to be an explicit nil

### UnsetCallAvailable
`func (o *PublicPortalTarget) UnsetCallAvailable()`

UnsetCallAvailable ensures that no value is present for CallAvailable, not even an explicit nil
### GetDenialReasons

`func (o *PublicPortalTarget) GetDenialReasons() []string`

GetDenialReasons returns the DenialReasons field if non-nil, zero value otherwise.

### GetDenialReasonsOk

`func (o *PublicPortalTarget) GetDenialReasonsOk() (*[]string, bool)`

GetDenialReasonsOk returns a tuple with the DenialReasons field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDenialReasons

`func (o *PublicPortalTarget) SetDenialReasons(v []string)`

SetDenialReasons sets DenialReasons field to given value.

### HasDenialReasons

`func (o *PublicPortalTarget) HasDenialReasons() bool`

HasDenialReasons returns a boolean if a field has been set.

### GetDisplayName

`func (o *PublicPortalTarget) GetDisplayName() interface{}`

GetDisplayName returns the DisplayName field if non-nil, zero value otherwise.

### GetDisplayNameOk

`func (o *PublicPortalTarget) GetDisplayNameOk() (*interface{}, bool)`

GetDisplayNameOk returns a tuple with the DisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisplayName

`func (o *PublicPortalTarget) SetDisplayName(v interface{})`

SetDisplayName sets DisplayName field to given value.


### SetDisplayNameNil

`func (o *PublicPortalTarget) SetDisplayNameNil(b bool)`

 SetDisplayNameNil sets the value for DisplayName to be an explicit nil

### UnsetDisplayName
`func (o *PublicPortalTarget) UnsetDisplayName()`

UnsetDisplayName ensures that no value is present for DisplayName, not even an explicit nil
### GetEffectiveDisplayName

`func (o *PublicPortalTarget) GetEffectiveDisplayName() string`

GetEffectiveDisplayName returns the EffectiveDisplayName field if non-nil, zero value otherwise.

### GetEffectiveDisplayNameOk

`func (o *PublicPortalTarget) GetEffectiveDisplayNameOk() (*string, bool)`

GetEffectiveDisplayNameOk returns a tuple with the EffectiveDisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEffectiveDisplayName

`func (o *PublicPortalTarget) SetEffectiveDisplayName(v string)`

SetEffectiveDisplayName sets EffectiveDisplayName field to given value.


### GetFloor

`func (o *PublicPortalTarget) GetFloor() interface{}`

GetFloor returns the Floor field if non-nil, zero value otherwise.

### GetFloorOk

`func (o *PublicPortalTarget) GetFloorOk() (*interface{}, bool)`

GetFloorOk returns a tuple with the Floor field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFloor

`func (o *PublicPortalTarget) SetFloor(v interface{})`

SetFloor sets Floor field to given value.


### SetFloorNil

`func (o *PublicPortalTarget) SetFloorNil(b bool)`

 SetFloorNil sets the value for Floor to be an explicit nil

### UnsetFloor
`func (o *PublicPortalTarget) UnsetFloor()`

UnsetFloor ensures that no value is present for Floor, not even an explicit nil
### GetFloorNumber

`func (o *PublicPortalTarget) GetFloorNumber() int64`

GetFloorNumber returns the FloorNumber field if non-nil, zero value otherwise.

### GetFloorNumberOk

`func (o *PublicPortalTarget) GetFloorNumberOk() (*int64, bool)`

GetFloorNumberOk returns a tuple with the FloorNumber field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFloorNumber

`func (o *PublicPortalTarget) SetFloorNumber(v int64)`

SetFloorNumber sets FloorNumber field to given value.

### HasFloorNumber

`func (o *PublicPortalTarget) HasFloorNumber() bool`

HasFloorNumber returns a boolean if a field has been set.

### SetFloorNumberNil

`func (o *PublicPortalTarget) SetFloorNumberNil(b bool)`

 SetFloorNumberNil sets the value for FloorNumber to be an explicit nil

### UnsetFloorNumber
`func (o *PublicPortalTarget) UnsetFloorNumber()`

UnsetFloorNumber ensures that no value is present for FloorNumber, not even an explicit nil
### GetImage

`func (o *PublicPortalTarget) GetImage() string`

GetImage returns the Image field if non-nil, zero value otherwise.

### GetImageOk

`func (o *PublicPortalTarget) GetImageOk() (*string, bool)`

GetImageOk returns a tuple with the Image field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImage

`func (o *PublicPortalTarget) SetImage(v string)`

SetImage sets Image field to given value.

### HasImage

`func (o *PublicPortalTarget) HasImage() bool`

HasImage returns a boolean if a field has been set.

### SetImageNil

`func (o *PublicPortalTarget) SetImageNil(b bool)`

 SetImageNil sets the value for Image to be an explicit nil

### UnsetImage
`func (o *PublicPortalTarget) UnsetImage()`

UnsetImage ensures that no value is present for Image, not even an explicit nil
### GetListingLabel

`func (o *PublicPortalTarget) GetListingLabel() interface{}`

GetListingLabel returns the ListingLabel field if non-nil, zero value otherwise.

### GetListingLabelOk

`func (o *PublicPortalTarget) GetListingLabelOk() (*interface{}, bool)`

GetListingLabelOk returns a tuple with the ListingLabel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListingLabel

`func (o *PublicPortalTarget) SetListingLabel(v interface{})`

SetListingLabel sets ListingLabel field to given value.

### HasListingLabel

`func (o *PublicPortalTarget) HasListingLabel() bool`

HasListingLabel returns a boolean if a field has been set.

### SetListingLabelNil

`func (o *PublicPortalTarget) SetListingLabelNil(b bool)`

 SetListingLabelNil sets the value for ListingLabel to be an explicit nil

### UnsetListingLabel
`func (o *PublicPortalTarget) UnsetListingLabel()`

UnsetListingLabel ensures that no value is present for ListingLabel, not even an explicit nil
### GetListingNumber

`func (o *PublicPortalTarget) GetListingNumber() int64`

GetListingNumber returns the ListingNumber field if non-nil, zero value otherwise.

### GetListingNumberOk

`func (o *PublicPortalTarget) GetListingNumberOk() (*int64, bool)`

GetListingNumberOk returns a tuple with the ListingNumber field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListingNumber

`func (o *PublicPortalTarget) SetListingNumber(v int64)`

SetListingNumber sets ListingNumber field to given value.

### HasListingNumber

`func (o *PublicPortalTarget) HasListingNumber() bool`

HasListingNumber returns a boolean if a field has been set.

### SetListingNumberNil

`func (o *PublicPortalTarget) SetListingNumberNil(b bool)`

 SetListingNumberNil sets the value for ListingNumber to be an explicit nil

### UnsetListingNumber
`func (o *PublicPortalTarget) UnsetListingNumber()`

UnsetListingNumber ensures that no value is present for ListingNumber, not even an explicit nil
### GetLocationDetail

`func (o *PublicPortalTarget) GetLocationDetail() interface{}`

GetLocationDetail returns the LocationDetail field if non-nil, zero value otherwise.

### GetLocationDetailOk

`func (o *PublicPortalTarget) GetLocationDetailOk() (*interface{}, bool)`

GetLocationDetailOk returns a tuple with the LocationDetail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocationDetail

`func (o *PublicPortalTarget) SetLocationDetail(v interface{})`

SetLocationDetail sets LocationDetail field to given value.

### HasLocationDetail

`func (o *PublicPortalTarget) HasLocationDetail() bool`

HasLocationDetail returns a boolean if a field has been set.

### SetLocationDetailNil

`func (o *PublicPortalTarget) SetLocationDetailNil(b bool)`

 SetLocationDetailNil sets the value for LocationDetail to be an explicit nil

### UnsetLocationDetail
`func (o *PublicPortalTarget) UnsetLocationDetail()`

UnsetLocationDetail ensures that no value is present for LocationDetail, not even an explicit nil
### GetRequireVideo

`func (o *PublicPortalTarget) GetRequireVideo() bool`

GetRequireVideo returns the RequireVideo field if non-nil, zero value otherwise.

### GetRequireVideoOk

`func (o *PublicPortalTarget) GetRequireVideoOk() (*bool, bool)`

GetRequireVideoOk returns a tuple with the RequireVideo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequireVideo

`func (o *PublicPortalTarget) SetRequireVideo(v bool)`

SetRequireVideo sets RequireVideo field to given value.

### HasRequireVideo

`func (o *PublicPortalTarget) HasRequireVideo() bool`

HasRequireVideo returns a boolean if a field has been set.

### SetRequireVideoNil

`func (o *PublicPortalTarget) SetRequireVideoNil(b bool)`

 SetRequireVideoNil sets the value for RequireVideo to be an explicit nil

### UnsetRequireVideo
`func (o *PublicPortalTarget) UnsetRequireVideo()`

UnsetRequireVideo ensures that no value is present for RequireVideo, not even an explicit nil
### GetTargetId

`func (o *PublicPortalTarget) GetTargetId() string`

GetTargetId returns the TargetId field if non-nil, zero value otherwise.

### GetTargetIdOk

`func (o *PublicPortalTarget) GetTargetIdOk() (*string, bool)`

GetTargetIdOk returns a tuple with the TargetId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTargetId

`func (o *PublicPortalTarget) SetTargetId(v string)`

SetTargetId sets TargetId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
