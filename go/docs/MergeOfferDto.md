# MergeOfferDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CandidateHasAvatar** | **bool** |  |
**CandidateUserId** | **string** |  |
**ClaimEmail** | Pointer to **NullableString** |  | [optional]
**ClaimPhone** | Pointer to **NullableString** |  | [optional]
**ExpiresAt** | **string** |  |
**Id** | **string** |  |
**ProofComplete** | **bool** | False for an org-admin request until the resident proves the other identifier. |
**RequestingHasAvatar** | **bool** |  |
**RequestingUserId** | **string** |  |
**SessionWillRemap** | **bool** |  |
**SurvivorUserId** | **string** |  |
**TriggerKind** | **string** |  |
**TriggerMasked** | **string** |  |

## Methods

### NewMergeOfferDto

`func NewMergeOfferDto(candidateHasAvatar bool, candidateUserId string, expiresAt string, id string, proofComplete bool, requestingHasAvatar bool, requestingUserId string, sessionWillRemap bool, survivorUserId string, triggerKind string, triggerMasked string, ) *MergeOfferDto`

NewMergeOfferDto instantiates a new MergeOfferDto object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMergeOfferDtoWithDefaults

`func NewMergeOfferDtoWithDefaults() *MergeOfferDto`

NewMergeOfferDtoWithDefaults instantiates a new MergeOfferDto object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCandidateHasAvatar

`func (o *MergeOfferDto) GetCandidateHasAvatar() bool`

GetCandidateHasAvatar returns the CandidateHasAvatar field if non-nil, zero value otherwise.

### GetCandidateHasAvatarOk

`func (o *MergeOfferDto) GetCandidateHasAvatarOk() (*bool, bool)`

GetCandidateHasAvatarOk returns a tuple with the CandidateHasAvatar field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCandidateHasAvatar

`func (o *MergeOfferDto) SetCandidateHasAvatar(v bool)`

SetCandidateHasAvatar sets CandidateHasAvatar field to given value.


### GetCandidateUserId

`func (o *MergeOfferDto) GetCandidateUserId() string`

GetCandidateUserId returns the CandidateUserId field if non-nil, zero value otherwise.

### GetCandidateUserIdOk

`func (o *MergeOfferDto) GetCandidateUserIdOk() (*string, bool)`

GetCandidateUserIdOk returns a tuple with the CandidateUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCandidateUserId

`func (o *MergeOfferDto) SetCandidateUserId(v string)`

SetCandidateUserId sets CandidateUserId field to given value.


### GetClaimEmail

`func (o *MergeOfferDto) GetClaimEmail() string`

GetClaimEmail returns the ClaimEmail field if non-nil, zero value otherwise.

### GetClaimEmailOk

`func (o *MergeOfferDto) GetClaimEmailOk() (*string, bool)`

GetClaimEmailOk returns a tuple with the ClaimEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetClaimEmail

`func (o *MergeOfferDto) SetClaimEmail(v string)`

SetClaimEmail sets ClaimEmail field to given value.

### HasClaimEmail

`func (o *MergeOfferDto) HasClaimEmail() bool`

HasClaimEmail returns a boolean if a field has been set.

### SetClaimEmailNil

`func (o *MergeOfferDto) SetClaimEmailNil(b bool)`

 SetClaimEmailNil sets the value for ClaimEmail to be an explicit nil

### UnsetClaimEmail
`func (o *MergeOfferDto) UnsetClaimEmail()`

UnsetClaimEmail ensures that no value is present for ClaimEmail, not even an explicit nil
### GetClaimPhone

`func (o *MergeOfferDto) GetClaimPhone() string`

GetClaimPhone returns the ClaimPhone field if non-nil, zero value otherwise.

### GetClaimPhoneOk

`func (o *MergeOfferDto) GetClaimPhoneOk() (*string, bool)`

GetClaimPhoneOk returns a tuple with the ClaimPhone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetClaimPhone

`func (o *MergeOfferDto) SetClaimPhone(v string)`

SetClaimPhone sets ClaimPhone field to given value.

### HasClaimPhone

`func (o *MergeOfferDto) HasClaimPhone() bool`

HasClaimPhone returns a boolean if a field has been set.

### SetClaimPhoneNil

`func (o *MergeOfferDto) SetClaimPhoneNil(b bool)`

 SetClaimPhoneNil sets the value for ClaimPhone to be an explicit nil

### UnsetClaimPhone
`func (o *MergeOfferDto) UnsetClaimPhone()`

UnsetClaimPhone ensures that no value is present for ClaimPhone, not even an explicit nil
### GetExpiresAt

`func (o *MergeOfferDto) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *MergeOfferDto) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *MergeOfferDto) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.


### GetId

`func (o *MergeOfferDto) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *MergeOfferDto) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *MergeOfferDto) SetId(v string)`

SetId sets Id field to given value.


### GetProofComplete

`func (o *MergeOfferDto) GetProofComplete() bool`

GetProofComplete returns the ProofComplete field if non-nil, zero value otherwise.

### GetProofCompleteOk

`func (o *MergeOfferDto) GetProofCompleteOk() (*bool, bool)`

GetProofCompleteOk returns a tuple with the ProofComplete field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProofComplete

`func (o *MergeOfferDto) SetProofComplete(v bool)`

SetProofComplete sets ProofComplete field to given value.


### GetRequestingHasAvatar

`func (o *MergeOfferDto) GetRequestingHasAvatar() bool`

GetRequestingHasAvatar returns the RequestingHasAvatar field if non-nil, zero value otherwise.

### GetRequestingHasAvatarOk

`func (o *MergeOfferDto) GetRequestingHasAvatarOk() (*bool, bool)`

GetRequestingHasAvatarOk returns a tuple with the RequestingHasAvatar field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequestingHasAvatar

`func (o *MergeOfferDto) SetRequestingHasAvatar(v bool)`

SetRequestingHasAvatar sets RequestingHasAvatar field to given value.


### GetRequestingUserId

`func (o *MergeOfferDto) GetRequestingUserId() string`

GetRequestingUserId returns the RequestingUserId field if non-nil, zero value otherwise.

### GetRequestingUserIdOk

`func (o *MergeOfferDto) GetRequestingUserIdOk() (*string, bool)`

GetRequestingUserIdOk returns a tuple with the RequestingUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequestingUserId

`func (o *MergeOfferDto) SetRequestingUserId(v string)`

SetRequestingUserId sets RequestingUserId field to given value.


### GetSessionWillRemap

`func (o *MergeOfferDto) GetSessionWillRemap() bool`

GetSessionWillRemap returns the SessionWillRemap field if non-nil, zero value otherwise.

### GetSessionWillRemapOk

`func (o *MergeOfferDto) GetSessionWillRemapOk() (*bool, bool)`

GetSessionWillRemapOk returns a tuple with the SessionWillRemap field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSessionWillRemap

`func (o *MergeOfferDto) SetSessionWillRemap(v bool)`

SetSessionWillRemap sets SessionWillRemap field to given value.


### GetSurvivorUserId

`func (o *MergeOfferDto) GetSurvivorUserId() string`

GetSurvivorUserId returns the SurvivorUserId field if non-nil, zero value otherwise.

### GetSurvivorUserIdOk

`func (o *MergeOfferDto) GetSurvivorUserIdOk() (*string, bool)`

GetSurvivorUserIdOk returns a tuple with the SurvivorUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSurvivorUserId

`func (o *MergeOfferDto) SetSurvivorUserId(v string)`

SetSurvivorUserId sets SurvivorUserId field to given value.


### GetTriggerKind

`func (o *MergeOfferDto) GetTriggerKind() string`

GetTriggerKind returns the TriggerKind field if non-nil, zero value otherwise.

### GetTriggerKindOk

`func (o *MergeOfferDto) GetTriggerKindOk() (*string, bool)`

GetTriggerKindOk returns a tuple with the TriggerKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTriggerKind

`func (o *MergeOfferDto) SetTriggerKind(v string)`

SetTriggerKind sets TriggerKind field to given value.


### GetTriggerMasked

`func (o *MergeOfferDto) GetTriggerMasked() string`

GetTriggerMasked returns the TriggerMasked field if non-nil, zero value otherwise.

### GetTriggerMaskedOk

`func (o *MergeOfferDto) GetTriggerMaskedOk() (*string, bool)`

GetTriggerMaskedOk returns a tuple with the TriggerMasked field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTriggerMasked

`func (o *MergeOfferDto) SetTriggerMasked(v string)`

SetTriggerMasked sets TriggerMasked field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
