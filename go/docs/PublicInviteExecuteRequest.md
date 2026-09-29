# PublicInviteExecuteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**GrantId** | **string** |  |
**OtpChallengeId** | Pointer to **NullableString** |  | [optional]
**OtpCode** | Pointer to **NullableString** |  | [optional]
**Pin** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPublicInviteExecuteRequest

`func NewPublicInviteExecuteRequest(grantId string, ) *PublicInviteExecuteRequest`

NewPublicInviteExecuteRequest instantiates a new PublicInviteExecuteRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicInviteExecuteRequestWithDefaults

`func NewPublicInviteExecuteRequestWithDefaults() *PublicInviteExecuteRequest`

NewPublicInviteExecuteRequestWithDefaults instantiates a new PublicInviteExecuteRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetGrantId

`func (o *PublicInviteExecuteRequest) GetGrantId() string`

GetGrantId returns the GrantId field if non-nil, zero value otherwise.

### GetGrantIdOk

`func (o *PublicInviteExecuteRequest) GetGrantIdOk() (*string, bool)`

GetGrantIdOk returns a tuple with the GrantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGrantId

`func (o *PublicInviteExecuteRequest) SetGrantId(v string)`

SetGrantId sets GrantId field to given value.


### GetOtpChallengeId

`func (o *PublicInviteExecuteRequest) GetOtpChallengeId() string`

GetOtpChallengeId returns the OtpChallengeId field if non-nil, zero value otherwise.

### GetOtpChallengeIdOk

`func (o *PublicInviteExecuteRequest) GetOtpChallengeIdOk() (*string, bool)`

GetOtpChallengeIdOk returns a tuple with the OtpChallengeId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOtpChallengeId

`func (o *PublicInviteExecuteRequest) SetOtpChallengeId(v string)`

SetOtpChallengeId sets OtpChallengeId field to given value.

### HasOtpChallengeId

`func (o *PublicInviteExecuteRequest) HasOtpChallengeId() bool`

HasOtpChallengeId returns a boolean if a field has been set.

### SetOtpChallengeIdNil

`func (o *PublicInviteExecuteRequest) SetOtpChallengeIdNil(b bool)`

 SetOtpChallengeIdNil sets the value for OtpChallengeId to be an explicit nil

### UnsetOtpChallengeId
`func (o *PublicInviteExecuteRequest) UnsetOtpChallengeId()`

UnsetOtpChallengeId ensures that no value is present for OtpChallengeId, not even an explicit nil
### GetOtpCode

`func (o *PublicInviteExecuteRequest) GetOtpCode() string`

GetOtpCode returns the OtpCode field if non-nil, zero value otherwise.

### GetOtpCodeOk

`func (o *PublicInviteExecuteRequest) GetOtpCodeOk() (*string, bool)`

GetOtpCodeOk returns a tuple with the OtpCode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOtpCode

`func (o *PublicInviteExecuteRequest) SetOtpCode(v string)`

SetOtpCode sets OtpCode field to given value.

### HasOtpCode

`func (o *PublicInviteExecuteRequest) HasOtpCode() bool`

HasOtpCode returns a boolean if a field has been set.

### SetOtpCodeNil

`func (o *PublicInviteExecuteRequest) SetOtpCodeNil(b bool)`

 SetOtpCodeNil sets the value for OtpCode to be an explicit nil

### UnsetOtpCode
`func (o *PublicInviteExecuteRequest) UnsetOtpCode()`

UnsetOtpCode ensures that no value is present for OtpCode, not even an explicit nil
### GetPin

`func (o *PublicInviteExecuteRequest) GetPin() string`

GetPin returns the Pin field if non-nil, zero value otherwise.

### GetPinOk

`func (o *PublicInviteExecuteRequest) GetPinOk() (*string, bool)`

GetPinOk returns a tuple with the Pin field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPin

`func (o *PublicInviteExecuteRequest) SetPin(v string)`

SetPin sets Pin field to given value.

### HasPin

`func (o *PublicInviteExecuteRequest) HasPin() bool`

HasPin returns a boolean if a field has been set.

### SetPinNil

`func (o *PublicInviteExecuteRequest) SetPinNil(b bool)`

 SetPinNil sets the value for Pin to be an explicit nil

### UnsetPin
`func (o *PublicInviteExecuteRequest) UnsetPin()`

UnsetPin ensures that no value is present for Pin, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
