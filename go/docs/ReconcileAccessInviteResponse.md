# ReconcileAccessInviteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Action** | [**ReconcileAction**](ReconcileAction.md) |  |
**ExternalRef** | Pointer to [**NullableExternalReferenceResponse**](ExternalReferenceResponse.md) |  | [optional]
**IdempotentReplay** | Pointer to **bool** | True when this body is a replay of an earlier reconcile carrying the same &#x60;Idempotency-Key&#x60;; nothing was written by this request. | [optional]
**Invite** | Pointer to [**NullableAccessInviteListItem**](AccessInviteListItem.md) | Invitation state after reconciling. | [optional]
**InviteLinkId** | Pointer to **NullableString** | The invitation serving the external record. Absent only when the desired state is &#x60;revoked&#x60; and no invitation exists for the record. | [optional]
**InviteToken** | Pointer to **NullableString** | Guest bearer token, returned **once**, only by the call that created the invitation. A replayed reconcile omits it; use &#x60;regenerate-token&#x60; to obtain a fresh link. | [optional]
**InviteUrl** | Pointer to **NullableString** | Guest-facing invite URL, with the same one-time visibility as &#x60;invite_token&#x60;. | [optional]

## Methods

### NewReconcileAccessInviteResponse

`func NewReconcileAccessInviteResponse(action ReconcileAction, ) *ReconcileAccessInviteResponse`

NewReconcileAccessInviteResponse instantiates a new ReconcileAccessInviteResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewReconcileAccessInviteResponseWithDefaults

`func NewReconcileAccessInviteResponseWithDefaults() *ReconcileAccessInviteResponse`

NewReconcileAccessInviteResponseWithDefaults instantiates a new ReconcileAccessInviteResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAction

`func (o *ReconcileAccessInviteResponse) GetAction() ReconcileAction`

GetAction returns the Action field if non-nil, zero value otherwise.

### GetActionOk

`func (o *ReconcileAccessInviteResponse) GetActionOk() (*ReconcileAction, bool)`

GetActionOk returns a tuple with the Action field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAction

`func (o *ReconcileAccessInviteResponse) SetAction(v ReconcileAction)`

SetAction sets Action field to given value.


### GetExternalRef

`func (o *ReconcileAccessInviteResponse) GetExternalRef() ExternalReferenceResponse`

GetExternalRef returns the ExternalRef field if non-nil, zero value otherwise.

### GetExternalRefOk

`func (o *ReconcileAccessInviteResponse) GetExternalRefOk() (*ExternalReferenceResponse, bool)`

GetExternalRefOk returns a tuple with the ExternalRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalRef

`func (o *ReconcileAccessInviteResponse) SetExternalRef(v ExternalReferenceResponse)`

SetExternalRef sets ExternalRef field to given value.

### HasExternalRef

`func (o *ReconcileAccessInviteResponse) HasExternalRef() bool`

HasExternalRef returns a boolean if a field has been set.

### SetExternalRefNil

`func (o *ReconcileAccessInviteResponse) SetExternalRefNil(b bool)`

 SetExternalRefNil sets the value for ExternalRef to be an explicit nil

### UnsetExternalRef
`func (o *ReconcileAccessInviteResponse) UnsetExternalRef()`

UnsetExternalRef ensures that no value is present for ExternalRef, not even an explicit nil
### GetIdempotentReplay

`func (o *ReconcileAccessInviteResponse) GetIdempotentReplay() bool`

GetIdempotentReplay returns the IdempotentReplay field if non-nil, zero value otherwise.

### GetIdempotentReplayOk

`func (o *ReconcileAccessInviteResponse) GetIdempotentReplayOk() (*bool, bool)`

GetIdempotentReplayOk returns a tuple with the IdempotentReplay field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIdempotentReplay

`func (o *ReconcileAccessInviteResponse) SetIdempotentReplay(v bool)`

SetIdempotentReplay sets IdempotentReplay field to given value.

### HasIdempotentReplay

`func (o *ReconcileAccessInviteResponse) HasIdempotentReplay() bool`

HasIdempotentReplay returns a boolean if a field has been set.

### GetInvite

`func (o *ReconcileAccessInviteResponse) GetInvite() AccessInviteListItem`

GetInvite returns the Invite field if non-nil, zero value otherwise.

### GetInviteOk

`func (o *ReconcileAccessInviteResponse) GetInviteOk() (*AccessInviteListItem, bool)`

GetInviteOk returns a tuple with the Invite field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvite

`func (o *ReconcileAccessInviteResponse) SetInvite(v AccessInviteListItem)`

SetInvite sets Invite field to given value.

### HasInvite

`func (o *ReconcileAccessInviteResponse) HasInvite() bool`

HasInvite returns a boolean if a field has been set.

### SetInviteNil

`func (o *ReconcileAccessInviteResponse) SetInviteNil(b bool)`

 SetInviteNil sets the value for Invite to be an explicit nil

### UnsetInvite
`func (o *ReconcileAccessInviteResponse) UnsetInvite()`

UnsetInvite ensures that no value is present for Invite, not even an explicit nil
### GetInviteLinkId

`func (o *ReconcileAccessInviteResponse) GetInviteLinkId() string`

GetInviteLinkId returns the InviteLinkId field if non-nil, zero value otherwise.

### GetInviteLinkIdOk

`func (o *ReconcileAccessInviteResponse) GetInviteLinkIdOk() (*string, bool)`

GetInviteLinkIdOk returns a tuple with the InviteLinkId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteLinkId

`func (o *ReconcileAccessInviteResponse) SetInviteLinkId(v string)`

SetInviteLinkId sets InviteLinkId field to given value.

### HasInviteLinkId

`func (o *ReconcileAccessInviteResponse) HasInviteLinkId() bool`

HasInviteLinkId returns a boolean if a field has been set.

### SetInviteLinkIdNil

`func (o *ReconcileAccessInviteResponse) SetInviteLinkIdNil(b bool)`

 SetInviteLinkIdNil sets the value for InviteLinkId to be an explicit nil

### UnsetInviteLinkId
`func (o *ReconcileAccessInviteResponse) UnsetInviteLinkId()`

UnsetInviteLinkId ensures that no value is present for InviteLinkId, not even an explicit nil
### GetInviteToken

`func (o *ReconcileAccessInviteResponse) GetInviteToken() string`

GetInviteToken returns the InviteToken field if non-nil, zero value otherwise.

### GetInviteTokenOk

`func (o *ReconcileAccessInviteResponse) GetInviteTokenOk() (*string, bool)`

GetInviteTokenOk returns a tuple with the InviteToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteToken

`func (o *ReconcileAccessInviteResponse) SetInviteToken(v string)`

SetInviteToken sets InviteToken field to given value.

### HasInviteToken

`func (o *ReconcileAccessInviteResponse) HasInviteToken() bool`

HasInviteToken returns a boolean if a field has been set.

### SetInviteTokenNil

`func (o *ReconcileAccessInviteResponse) SetInviteTokenNil(b bool)`

 SetInviteTokenNil sets the value for InviteToken to be an explicit nil

### UnsetInviteToken
`func (o *ReconcileAccessInviteResponse) UnsetInviteToken()`

UnsetInviteToken ensures that no value is present for InviteToken, not even an explicit nil
### GetInviteUrl

`func (o *ReconcileAccessInviteResponse) GetInviteUrl() string`

GetInviteUrl returns the InviteUrl field if non-nil, zero value otherwise.

### GetInviteUrlOk

`func (o *ReconcileAccessInviteResponse) GetInviteUrlOk() (*string, bool)`

GetInviteUrlOk returns a tuple with the InviteUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteUrl

`func (o *ReconcileAccessInviteResponse) SetInviteUrl(v string)`

SetInviteUrl sets InviteUrl field to given value.

### HasInviteUrl

`func (o *ReconcileAccessInviteResponse) HasInviteUrl() bool`

HasInviteUrl returns a boolean if a field has been set.

### SetInviteUrlNil

`func (o *ReconcileAccessInviteResponse) SetInviteUrlNil(b bool)`

 SetInviteUrlNil sets the value for InviteUrl to be an explicit nil

### UnsetInviteUrl
`func (o *ReconcileAccessInviteResponse) UnsetInviteUrl()`

UnsetInviteUrl ensures that no value is present for InviteUrl, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
