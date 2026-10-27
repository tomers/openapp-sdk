# RedeemInvitationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AcceptOnSignedInAccount** | Pointer to **bool** | Attach the invitation to the signed-in account when its mailbox is not the invited email. The invited address is not stored. Required only in that case; omitted otherwise. | [optional]
**MergeAccounts** | Pointer to **bool** | Start a combine-accounts offer when the invited email or phone already belongs to someone else. The invitation stays pending until that offer is confirmed and redeem is called again. | [optional]
**PhoneAction** | Pointer to **NullableString** | &#x60;keep&#x60; leaves the account&#39;s phone unchanged. &#x60;replace&#x60; saves the invited number over it. Required when the account already has a different phone and the invited number is free. | [optional]
**Pin** | **string** |  |
**Token** | **string** |  |

## Methods

### NewRedeemInvitationRequest

`func NewRedeemInvitationRequest(pin string, token string, ) *RedeemInvitationRequest`

NewRedeemInvitationRequest instantiates a new RedeemInvitationRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewRedeemInvitationRequestWithDefaults

`func NewRedeemInvitationRequestWithDefaults() *RedeemInvitationRequest`

NewRedeemInvitationRequestWithDefaults instantiates a new RedeemInvitationRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAcceptOnSignedInAccount

`func (o *RedeemInvitationRequest) GetAcceptOnSignedInAccount() bool`

GetAcceptOnSignedInAccount returns the AcceptOnSignedInAccount field if non-nil, zero value otherwise.

### GetAcceptOnSignedInAccountOk

`func (o *RedeemInvitationRequest) GetAcceptOnSignedInAccountOk() (*bool, bool)`

GetAcceptOnSignedInAccountOk returns a tuple with the AcceptOnSignedInAccount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAcceptOnSignedInAccount

`func (o *RedeemInvitationRequest) SetAcceptOnSignedInAccount(v bool)`

SetAcceptOnSignedInAccount sets AcceptOnSignedInAccount field to given value.

### HasAcceptOnSignedInAccount

`func (o *RedeemInvitationRequest) HasAcceptOnSignedInAccount() bool`

HasAcceptOnSignedInAccount returns a boolean if a field has been set.

### GetMergeAccounts

`func (o *RedeemInvitationRequest) GetMergeAccounts() bool`

GetMergeAccounts returns the MergeAccounts field if non-nil, zero value otherwise.

### GetMergeAccountsOk

`func (o *RedeemInvitationRequest) GetMergeAccountsOk() (*bool, bool)`

GetMergeAccountsOk returns a tuple with the MergeAccounts field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMergeAccounts

`func (o *RedeemInvitationRequest) SetMergeAccounts(v bool)`

SetMergeAccounts sets MergeAccounts field to given value.

### HasMergeAccounts

`func (o *RedeemInvitationRequest) HasMergeAccounts() bool`

HasMergeAccounts returns a boolean if a field has been set.

### GetPhoneAction

`func (o *RedeemInvitationRequest) GetPhoneAction() string`

GetPhoneAction returns the PhoneAction field if non-nil, zero value otherwise.

### GetPhoneActionOk

`func (o *RedeemInvitationRequest) GetPhoneActionOk() (*string, bool)`

GetPhoneActionOk returns a tuple with the PhoneAction field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhoneAction

`func (o *RedeemInvitationRequest) SetPhoneAction(v string)`

SetPhoneAction sets PhoneAction field to given value.

### HasPhoneAction

`func (o *RedeemInvitationRequest) HasPhoneAction() bool`

HasPhoneAction returns a boolean if a field has been set.

### SetPhoneActionNil

`func (o *RedeemInvitationRequest) SetPhoneActionNil(b bool)`

 SetPhoneActionNil sets the value for PhoneAction to be an explicit nil

### UnsetPhoneAction
`func (o *RedeemInvitationRequest) UnsetPhoneAction()`

UnsetPhoneAction ensures that no value is present for PhoneAction, not even an explicit nil
### GetPin

`func (o *RedeemInvitationRequest) GetPin() string`

GetPin returns the Pin field if non-nil, zero value otherwise.

### GetPinOk

`func (o *RedeemInvitationRequest) GetPinOk() (*string, bool)`

GetPinOk returns a tuple with the Pin field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPin

`func (o *RedeemInvitationRequest) SetPin(v string)`

SetPin sets Pin field to given value.


### GetToken

`func (o *RedeemInvitationRequest) GetToken() string`

GetToken returns the Token field if non-nil, zero value otherwise.

### GetTokenOk

`func (o *RedeemInvitationRequest) GetTokenOk() (*string, bool)`

GetTokenOk returns a tuple with the Token field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetToken

`func (o *RedeemInvitationRequest) SetToken(v string)`

SetToken sets Token field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
