# ReconcileAccessInviteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExternalRef** | [**ExternalReferenceInput**](ExternalReferenceInput.md) | The external record this reconcile is about. It identifies the invitation, so the caller does not need to track OpenApp invitation ids. |
**Invitation** | Pointer to [**NullableCreateAccessInviteRequest**](CreateAccessInviteRequest.md) | Desired invitation. Required when &#x60;state&#x60; is &#x60;active&#x60;, ignored when &#x60;revoked&#x60;. It must not carry its own &#x60;external_ref&#x60;: the top-level reference owns the binding. | [optional]
**State** | Pointer to [**ReconcileInviteState**](ReconcileInviteState.md) | Desired state for that record. Defaults to &#x60;active&#x60;. | [optional]

## Methods

### NewReconcileAccessInviteRequest

`func NewReconcileAccessInviteRequest(externalRef ExternalReferenceInput, ) *ReconcileAccessInviteRequest`

NewReconcileAccessInviteRequest instantiates a new ReconcileAccessInviteRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewReconcileAccessInviteRequestWithDefaults

`func NewReconcileAccessInviteRequestWithDefaults() *ReconcileAccessInviteRequest`

NewReconcileAccessInviteRequestWithDefaults instantiates a new ReconcileAccessInviteRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExternalRef

`func (o *ReconcileAccessInviteRequest) GetExternalRef() ExternalReferenceInput`

GetExternalRef returns the ExternalRef field if non-nil, zero value otherwise.

### GetExternalRefOk

`func (o *ReconcileAccessInviteRequest) GetExternalRefOk() (*ExternalReferenceInput, bool)`

GetExternalRefOk returns a tuple with the ExternalRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalRef

`func (o *ReconcileAccessInviteRequest) SetExternalRef(v ExternalReferenceInput)`

SetExternalRef sets ExternalRef field to given value.


### GetInvitation

`func (o *ReconcileAccessInviteRequest) GetInvitation() CreateAccessInviteRequest`

GetInvitation returns the Invitation field if non-nil, zero value otherwise.

### GetInvitationOk

`func (o *ReconcileAccessInviteRequest) GetInvitationOk() (*CreateAccessInviteRequest, bool)`

GetInvitationOk returns a tuple with the Invitation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvitation

`func (o *ReconcileAccessInviteRequest) SetInvitation(v CreateAccessInviteRequest)`

SetInvitation sets Invitation field to given value.

### HasInvitation

`func (o *ReconcileAccessInviteRequest) HasInvitation() bool`

HasInvitation returns a boolean if a field has been set.

### SetInvitationNil

`func (o *ReconcileAccessInviteRequest) SetInvitationNil(b bool)`

 SetInvitationNil sets the value for Invitation to be an explicit nil

### UnsetInvitation
`func (o *ReconcileAccessInviteRequest) UnsetInvitation()`

UnsetInvitation ensures that no value is present for Invitation, not even an explicit nil
### GetState

`func (o *ReconcileAccessInviteRequest) GetState() ReconcileInviteState`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *ReconcileAccessInviteRequest) GetStateOk() (*ReconcileInviteState, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *ReconcileAccessInviteRequest) SetState(v ReconcileInviteState)`

SetState sets State field to given value.

### HasState

`func (o *ReconcileAccessInviteRequest) HasState() bool`

HasState returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
