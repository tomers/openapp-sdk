# CreateHouseholdInvitation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Email** | Pointer to **NullableString** |  | [optional]
**InviteeName** | Pointer to **NullableString** | Optional name for the person you invite. It greets them in the invitation and labels the pending invitation for you. When they sign up without a name it becomes their account name, and they are asked to confirm it. Blank counts as no name; at most 100 characters. | [optional]
**Locale** | Pointer to **NullableString** | Language of the invitation SMS/email: a supported locale code. Used when the contact has no account with a language of its own; resends reuse it. Omit to fall back to the household&#39;s home org default message language, then English. | [optional]
**Phone** | Pointer to **NullableString** |  | [optional]
**SmsConsentConfirmed** | Pointer to **bool** | Must be &#x60;true&#x60; when &#x60;phone&#x60; is set: confirms the person you invite agreed to receive text messages from OpenApp at that number. The confirmation is recorded with who gave it and when. Not needed for email-only invitations. | [optional]

## Methods

### NewCreateHouseholdInvitation

`func NewCreateHouseholdInvitation() *CreateHouseholdInvitation`

NewCreateHouseholdInvitation instantiates a new CreateHouseholdInvitation object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateHouseholdInvitationWithDefaults

`func NewCreateHouseholdInvitationWithDefaults() *CreateHouseholdInvitation`

NewCreateHouseholdInvitationWithDefaults instantiates a new CreateHouseholdInvitation object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEmail

`func (o *CreateHouseholdInvitation) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *CreateHouseholdInvitation) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *CreateHouseholdInvitation) SetEmail(v string)`

SetEmail sets Email field to given value.

### HasEmail

`func (o *CreateHouseholdInvitation) HasEmail() bool`

HasEmail returns a boolean if a field has been set.

### SetEmailNil

`func (o *CreateHouseholdInvitation) SetEmailNil(b bool)`

 SetEmailNil sets the value for Email to be an explicit nil

### UnsetEmail
`func (o *CreateHouseholdInvitation) UnsetEmail()`

UnsetEmail ensures that no value is present for Email, not even an explicit nil
### GetInviteeName

`func (o *CreateHouseholdInvitation) GetInviteeName() string`

GetInviteeName returns the InviteeName field if non-nil, zero value otherwise.

### GetInviteeNameOk

`func (o *CreateHouseholdInvitation) GetInviteeNameOk() (*string, bool)`

GetInviteeNameOk returns a tuple with the InviteeName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteeName

`func (o *CreateHouseholdInvitation) SetInviteeName(v string)`

SetInviteeName sets InviteeName field to given value.

### HasInviteeName

`func (o *CreateHouseholdInvitation) HasInviteeName() bool`

HasInviteeName returns a boolean if a field has been set.

### SetInviteeNameNil

`func (o *CreateHouseholdInvitation) SetInviteeNameNil(b bool)`

 SetInviteeNameNil sets the value for InviteeName to be an explicit nil

### UnsetInviteeName
`func (o *CreateHouseholdInvitation) UnsetInviteeName()`

UnsetInviteeName ensures that no value is present for InviteeName, not even an explicit nil
### GetLocale

`func (o *CreateHouseholdInvitation) GetLocale() string`

GetLocale returns the Locale field if non-nil, zero value otherwise.

### GetLocaleOk

`func (o *CreateHouseholdInvitation) GetLocaleOk() (*string, bool)`

GetLocaleOk returns a tuple with the Locale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocale

`func (o *CreateHouseholdInvitation) SetLocale(v string)`

SetLocale sets Locale field to given value.

### HasLocale

`func (o *CreateHouseholdInvitation) HasLocale() bool`

HasLocale returns a boolean if a field has been set.

### SetLocaleNil

`func (o *CreateHouseholdInvitation) SetLocaleNil(b bool)`

 SetLocaleNil sets the value for Locale to be an explicit nil

### UnsetLocale
`func (o *CreateHouseholdInvitation) UnsetLocale()`

UnsetLocale ensures that no value is present for Locale, not even an explicit nil
### GetPhone

`func (o *CreateHouseholdInvitation) GetPhone() string`

GetPhone returns the Phone field if non-nil, zero value otherwise.

### GetPhoneOk

`func (o *CreateHouseholdInvitation) GetPhoneOk() (*string, bool)`

GetPhoneOk returns a tuple with the Phone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhone

`func (o *CreateHouseholdInvitation) SetPhone(v string)`

SetPhone sets Phone field to given value.

### HasPhone

`func (o *CreateHouseholdInvitation) HasPhone() bool`

HasPhone returns a boolean if a field has been set.

### SetPhoneNil

`func (o *CreateHouseholdInvitation) SetPhoneNil(b bool)`

 SetPhoneNil sets the value for Phone to be an explicit nil

### UnsetPhone
`func (o *CreateHouseholdInvitation) UnsetPhone()`

UnsetPhone ensures that no value is present for Phone, not even an explicit nil
### GetSmsConsentConfirmed

`func (o *CreateHouseholdInvitation) GetSmsConsentConfirmed() bool`

GetSmsConsentConfirmed returns the SmsConsentConfirmed field if non-nil, zero value otherwise.

### GetSmsConsentConfirmedOk

`func (o *CreateHouseholdInvitation) GetSmsConsentConfirmedOk() (*bool, bool)`

GetSmsConsentConfirmedOk returns a tuple with the SmsConsentConfirmed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmsConsentConfirmed

`func (o *CreateHouseholdInvitation) SetSmsConsentConfirmed(v bool)`

SetSmsConsentConfirmed sets SmsConsentConfirmed field to given value.

### HasSmsConsentConfirmed

`func (o *CreateHouseholdInvitation) HasSmsConsentConfirmed() bool`

HasSmsConsentConfirmed returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
