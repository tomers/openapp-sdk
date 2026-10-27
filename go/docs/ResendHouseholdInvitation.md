# ResendHouseholdInvitation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**SmsConsentConfirmed** | Pointer to **bool** | Must be &#x60;true&#x60; when the invitation has a phone: confirms the person you invite agreed to receive text messages from OpenApp at that number. Not needed for email-only invitations; send &#x60;{}&#x60;. | [optional]

## Methods

### NewResendHouseholdInvitation

`func NewResendHouseholdInvitation() *ResendHouseholdInvitation`

NewResendHouseholdInvitation instantiates a new ResendHouseholdInvitation object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResendHouseholdInvitationWithDefaults

`func NewResendHouseholdInvitationWithDefaults() *ResendHouseholdInvitation`

NewResendHouseholdInvitationWithDefaults instantiates a new ResendHouseholdInvitation object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetSmsConsentConfirmed

`func (o *ResendHouseholdInvitation) GetSmsConsentConfirmed() bool`

GetSmsConsentConfirmed returns the SmsConsentConfirmed field if non-nil, zero value otherwise.

### GetSmsConsentConfirmedOk

`func (o *ResendHouseholdInvitation) GetSmsConsentConfirmedOk() (*bool, bool)`

GetSmsConsentConfirmedOk returns a tuple with the SmsConsentConfirmed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmsConsentConfirmed

`func (o *ResendHouseholdInvitation) SetSmsConsentConfirmed(v bool)`

SetSmsConsentConfirmed sets SmsConsentConfirmed field to given value.

### HasSmsConsentConfirmed

`func (o *ResendHouseholdInvitation) HasSmsConsentConfirmed() bool`

HasSmsConsentConfirmed returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
