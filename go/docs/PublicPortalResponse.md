# PublicPortalResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Branding** | Pointer to **interface{}** |  | [optional]
**DoorEffectiveAutoOffDuration** | Pointer to **NullableInt64** |  | [optional]
**DoorImageUrl** | Pointer to **NullableString** | Image URL for the door (presigned S3 URL from media service). | [optional]
**Lights** | Pointer to [**[]PublicPortalLight**](PublicPortalLight.md) |  | [optional]
**Mode** | [**PublicPortalMode**](PublicPortalMode.md) |  |
**Name** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**OpenRateLimit** | Pointer to [**NullablePublicPortalOpenRateLimit**](PublicPortalOpenRateLimit.md) |  | [optional]
**PublicPortalId** | **string** |  |

## Methods

### NewPublicPortalResponse

`func NewPublicPortalResponse(mode PublicPortalMode, publicPortalId string, ) *PublicPortalResponse`

NewPublicPortalResponse instantiates a new PublicPortalResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicPortalResponseWithDefaults

`func NewPublicPortalResponseWithDefaults() *PublicPortalResponse`

NewPublicPortalResponseWithDefaults instantiates a new PublicPortalResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

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
