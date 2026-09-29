# PublicInviteOtpStartRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Channel** | Pointer to **NullableString** | &#x60;sms&#x60; (default) or &#x60;email&#x60;. The destination is the matching contact on the invitation. | [optional]

## Methods

### NewPublicInviteOtpStartRequest

`func NewPublicInviteOtpStartRequest() *PublicInviteOtpStartRequest`

NewPublicInviteOtpStartRequest instantiates a new PublicInviteOtpStartRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicInviteOtpStartRequestWithDefaults

`func NewPublicInviteOtpStartRequestWithDefaults() *PublicInviteOtpStartRequest`

NewPublicInviteOtpStartRequestWithDefaults instantiates a new PublicInviteOtpStartRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetChannel

`func (o *PublicInviteOtpStartRequest) GetChannel() string`

GetChannel returns the Channel field if non-nil, zero value otherwise.

### GetChannelOk

`func (o *PublicInviteOtpStartRequest) GetChannelOk() (*string, bool)`

GetChannelOk returns a tuple with the Channel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChannel

`func (o *PublicInviteOtpStartRequest) SetChannel(v string)`

SetChannel sets Channel field to given value.

### HasChannel

`func (o *PublicInviteOtpStartRequest) HasChannel() bool`

HasChannel returns a boolean if a field has been set.

### SetChannelNil

`func (o *PublicInviteOtpStartRequest) SetChannelNil(b bool)`

 SetChannelNil sets the value for Channel to be an explicit nil

### UnsetChannel
`func (o *PublicInviteOtpStartRequest) UnsetChannel()`

UnsetChannel ensures that no value is present for Channel, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
