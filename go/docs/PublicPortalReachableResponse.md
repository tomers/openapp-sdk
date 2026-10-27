# PublicPortalReachableResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Reachable** | **bool** |  |
**Reason** | Pointer to **NullableString** | Why the door is not reachable, when the server knows. &#x60;door_not_connected&#x60;: the portal has no door (it was purged, or never linked), so retrying cannot help. Omitted when reachable and when the cause is a door that exists but did not answer. | [optional]

## Methods

### NewPublicPortalReachableResponse

`func NewPublicPortalReachableResponse(reachable bool, ) *PublicPortalReachableResponse`

NewPublicPortalReachableResponse instantiates a new PublicPortalReachableResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicPortalReachableResponseWithDefaults

`func NewPublicPortalReachableResponseWithDefaults() *PublicPortalReachableResponse`

NewPublicPortalReachableResponseWithDefaults instantiates a new PublicPortalReachableResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetReachable

`func (o *PublicPortalReachableResponse) GetReachable() bool`

GetReachable returns the Reachable field if non-nil, zero value otherwise.

### GetReachableOk

`func (o *PublicPortalReachableResponse) GetReachableOk() (*bool, bool)`

GetReachableOk returns a tuple with the Reachable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReachable

`func (o *PublicPortalReachableResponse) SetReachable(v bool)`

SetReachable sets Reachable field to given value.


### GetReason

`func (o *PublicPortalReachableResponse) GetReason() string`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *PublicPortalReachableResponse) GetReasonOk() (*string, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *PublicPortalReachableResponse) SetReason(v string)`

SetReason sets Reason field to given value.

### HasReason

`func (o *PublicPortalReachableResponse) HasReason() bool`

HasReason returns a boolean if a field has been set.

### SetReasonNil

`func (o *PublicPortalReachableResponse) SetReasonNil(b bool)`

 SetReasonNil sets the value for Reason to be an explicit nil

### UnsetReason
`func (o *PublicPortalReachableResponse) UnsetReason()`

UnsetReason ensures that no value is present for Reason, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
