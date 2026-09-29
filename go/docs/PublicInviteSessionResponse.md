# PublicInviteSessionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**GuestId** | **string** | Stable per-device guest fingerprint (ULID). Either the one supplied by the client (cookie or &#x60;X-Guest-Id&#x60;) or a freshly minted one. Echoed so native clients can persist it and send it back via &#x60;X-Guest-Id&#x60; on subsequent public-access calls. Used only for audit attribution — it confers no access. |
**InviteId** | **string** | ULID of the invite (&#x3D; value of the &#x60;oa_access_invite&#x60; cookie). Echoed in the body so non-browser clients can construct the cookie without relying on a platform cookie jar for &#x60;HttpOnly&#x60; &#x60;Set-Cookie&#x60; responses. |
**Ok** | **bool** |  |

## Methods

### NewPublicInviteSessionResponse

`func NewPublicInviteSessionResponse(guestId string, inviteId string, ok bool, ) *PublicInviteSessionResponse`

NewPublicInviteSessionResponse instantiates a new PublicInviteSessionResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicInviteSessionResponseWithDefaults

`func NewPublicInviteSessionResponseWithDefaults() *PublicInviteSessionResponse`

NewPublicInviteSessionResponseWithDefaults instantiates a new PublicInviteSessionResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetGuestId

`func (o *PublicInviteSessionResponse) GetGuestId() string`

GetGuestId returns the GuestId field if non-nil, zero value otherwise.

### GetGuestIdOk

`func (o *PublicInviteSessionResponse) GetGuestIdOk() (*string, bool)`

GetGuestIdOk returns a tuple with the GuestId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGuestId

`func (o *PublicInviteSessionResponse) SetGuestId(v string)`

SetGuestId sets GuestId field to given value.


### GetInviteId

`func (o *PublicInviteSessionResponse) GetInviteId() string`

GetInviteId returns the InviteId field if non-nil, zero value otherwise.

### GetInviteIdOk

`func (o *PublicInviteSessionResponse) GetInviteIdOk() (*string, bool)`

GetInviteIdOk returns a tuple with the InviteId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteId

`func (o *PublicInviteSessionResponse) SetInviteId(v string)`

SetInviteId sets InviteId field to given value.


### GetOk

`func (o *PublicInviteSessionResponse) GetOk() bool`

GetOk returns the Ok field if non-nil, zero value otherwise.

### GetOkOk

`func (o *PublicInviteSessionResponse) GetOkOk() (*bool, bool)`

GetOkOk returns a tuple with the Ok field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOk

`func (o *PublicInviteSessionResponse) SetOk(v bool)`

SetOk sets Ok field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
