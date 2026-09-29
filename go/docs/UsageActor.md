# UsageActor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**GuestId** | Pointer to **NullableString** | Per-device guest fingerprint. Present only when &#x60;kind &#x3D;&#x3D; \&quot;guest\&quot;&#x60;. | [optional]
**Kind** | **string** | &#x60;user&#x60;, &#x60;guest&#x60;, &#x60;system&#x60;, &#x60;api_key&#x60;, … (mirrors &#x60;audit_events.actor_kind&#x60;). |
**UserId** | Pointer to **NullableString** | Trusted registered-user id. Present only when &#x60;kind &#x3D;&#x3D; \&quot;user\&quot;&#x60;. | [optional]

## Methods

### NewUsageActor

`func NewUsageActor(kind string, ) *UsageActor`

NewUsageActor instantiates a new UsageActor object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUsageActorWithDefaults

`func NewUsageActorWithDefaults() *UsageActor`

NewUsageActorWithDefaults instantiates a new UsageActor object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetGuestId

`func (o *UsageActor) GetGuestId() string`

GetGuestId returns the GuestId field if non-nil, zero value otherwise.

### GetGuestIdOk

`func (o *UsageActor) GetGuestIdOk() (*string, bool)`

GetGuestIdOk returns a tuple with the GuestId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGuestId

`func (o *UsageActor) SetGuestId(v string)`

SetGuestId sets GuestId field to given value.

### HasGuestId

`func (o *UsageActor) HasGuestId() bool`

HasGuestId returns a boolean if a field has been set.

### SetGuestIdNil

`func (o *UsageActor) SetGuestIdNil(b bool)`

 SetGuestIdNil sets the value for GuestId to be an explicit nil

### UnsetGuestId
`func (o *UsageActor) UnsetGuestId()`

UnsetGuestId ensures that no value is present for GuestId, not even an explicit nil
### GetKind

`func (o *UsageActor) GetKind() string`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *UsageActor) GetKindOk() (*string, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *UsageActor) SetKind(v string)`

SetKind sets Kind field to given value.


### GetUserId

`func (o *UsageActor) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *UsageActor) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *UsageActor) SetUserId(v string)`

SetUserId sets UserId field to given value.

### HasUserId

`func (o *UsageActor) HasUserId() bool`

HasUserId returns a boolean if a field has been set.

### SetUserIdNil

`func (o *UsageActor) SetUserIdNil(b bool)`

 SetUserIdNil sets the value for UserId to be an explicit nil

### UnsetUserId
`func (o *UsageActor) UnsetUserId()`

UnsetUserId ensures that no value is present for UserId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
