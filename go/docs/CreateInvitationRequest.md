# CreateInvitationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DateOfBirth** | Pointer to **NullableString** | Parent-declared date of birth (&#x60;YYYY-MM-DD&#x60;). Must classify as under 16. Applied to the account at redeem when the account has none. Omit when the invitee is an adult. | [optional]
**DefaultGroupIds** | Pointer to **[]string** | Groups the person joins when they redeem, in the same transaction as the org role. | [optional]
**DefaultRoles** | Pointer to **[]string** | Roles to assign on accept (within the inviting org). | [optional]
**Email** | Pointer to **NullableString** |  | [optional]
**InviteeName** | Pointer to **NullableString** | Name of the person being invited, up to 100 characters. Greets them in the invitation and labels the pending invitation for the inviter. At sign-up it becomes the account name only when no name is known, and the person confirms it. Omit when unknown. | [optional]
**Locale** | Pointer to **NullableString** | Language of the invitation SMS/email: a supported locale code. Used when the contact has no account with a language of its own; resends reuse it. Omit to fall back to the org&#39;s default message language, then English. | [optional]
**Phone** | Pointer to **NullableString** |  | [optional]
**SiteAccess** | Pointer to [**NullableInvitationSiteAccessDto**](InvitationSiteAccessDto.md) | Optional site (and listing) access applied atomically when the invitation is redeemed. | [optional]
**SmsConsentConfirmed** | Pointer to **bool** | Must be &#x60;true&#x60; when &#x60;phone&#x60; is set: confirms the person being invited agreed to receive text messages from OpenApp at that number. The confirmation is recorded with who gave it and when. Not needed for email-only invitations. | [optional]

## Methods

### NewCreateInvitationRequest

`func NewCreateInvitationRequest() *CreateInvitationRequest`

NewCreateInvitationRequest instantiates a new CreateInvitationRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateInvitationRequestWithDefaults

`func NewCreateInvitationRequestWithDefaults() *CreateInvitationRequest`

NewCreateInvitationRequestWithDefaults instantiates a new CreateInvitationRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDateOfBirth

`func (o *CreateInvitationRequest) GetDateOfBirth() string`

GetDateOfBirth returns the DateOfBirth field if non-nil, zero value otherwise.

### GetDateOfBirthOk

`func (o *CreateInvitationRequest) GetDateOfBirthOk() (*string, bool)`

GetDateOfBirthOk returns a tuple with the DateOfBirth field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDateOfBirth

`func (o *CreateInvitationRequest) SetDateOfBirth(v string)`

SetDateOfBirth sets DateOfBirth field to given value.

### HasDateOfBirth

`func (o *CreateInvitationRequest) HasDateOfBirth() bool`

HasDateOfBirth returns a boolean if a field has been set.

### SetDateOfBirthNil

`func (o *CreateInvitationRequest) SetDateOfBirthNil(b bool)`

 SetDateOfBirthNil sets the value for DateOfBirth to be an explicit nil

### UnsetDateOfBirth
`func (o *CreateInvitationRequest) UnsetDateOfBirth()`

UnsetDateOfBirth ensures that no value is present for DateOfBirth, not even an explicit nil
### GetDefaultGroupIds

`func (o *CreateInvitationRequest) GetDefaultGroupIds() []string`

GetDefaultGroupIds returns the DefaultGroupIds field if non-nil, zero value otherwise.

### GetDefaultGroupIdsOk

`func (o *CreateInvitationRequest) GetDefaultGroupIdsOk() (*[]string, bool)`

GetDefaultGroupIdsOk returns a tuple with the DefaultGroupIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultGroupIds

`func (o *CreateInvitationRequest) SetDefaultGroupIds(v []string)`

SetDefaultGroupIds sets DefaultGroupIds field to given value.

### HasDefaultGroupIds

`func (o *CreateInvitationRequest) HasDefaultGroupIds() bool`

HasDefaultGroupIds returns a boolean if a field has been set.

### GetDefaultRoles

`func (o *CreateInvitationRequest) GetDefaultRoles() []string`

GetDefaultRoles returns the DefaultRoles field if non-nil, zero value otherwise.

### GetDefaultRolesOk

`func (o *CreateInvitationRequest) GetDefaultRolesOk() (*[]string, bool)`

GetDefaultRolesOk returns a tuple with the DefaultRoles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultRoles

`func (o *CreateInvitationRequest) SetDefaultRoles(v []string)`

SetDefaultRoles sets DefaultRoles field to given value.

### HasDefaultRoles

`func (o *CreateInvitationRequest) HasDefaultRoles() bool`

HasDefaultRoles returns a boolean if a field has been set.

### GetEmail

`func (o *CreateInvitationRequest) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *CreateInvitationRequest) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *CreateInvitationRequest) SetEmail(v string)`

SetEmail sets Email field to given value.

### HasEmail

`func (o *CreateInvitationRequest) HasEmail() bool`

HasEmail returns a boolean if a field has been set.

### SetEmailNil

`func (o *CreateInvitationRequest) SetEmailNil(b bool)`

 SetEmailNil sets the value for Email to be an explicit nil

### UnsetEmail
`func (o *CreateInvitationRequest) UnsetEmail()`

UnsetEmail ensures that no value is present for Email, not even an explicit nil
### GetInviteeName

`func (o *CreateInvitationRequest) GetInviteeName() string`

GetInviteeName returns the InviteeName field if non-nil, zero value otherwise.

### GetInviteeNameOk

`func (o *CreateInvitationRequest) GetInviteeNameOk() (*string, bool)`

GetInviteeNameOk returns a tuple with the InviteeName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteeName

`func (o *CreateInvitationRequest) SetInviteeName(v string)`

SetInviteeName sets InviteeName field to given value.

### HasInviteeName

`func (o *CreateInvitationRequest) HasInviteeName() bool`

HasInviteeName returns a boolean if a field has been set.

### SetInviteeNameNil

`func (o *CreateInvitationRequest) SetInviteeNameNil(b bool)`

 SetInviteeNameNil sets the value for InviteeName to be an explicit nil

### UnsetInviteeName
`func (o *CreateInvitationRequest) UnsetInviteeName()`

UnsetInviteeName ensures that no value is present for InviteeName, not even an explicit nil
### GetLocale

`func (o *CreateInvitationRequest) GetLocale() string`

GetLocale returns the Locale field if non-nil, zero value otherwise.

### GetLocaleOk

`func (o *CreateInvitationRequest) GetLocaleOk() (*string, bool)`

GetLocaleOk returns a tuple with the Locale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocale

`func (o *CreateInvitationRequest) SetLocale(v string)`

SetLocale sets Locale field to given value.

### HasLocale

`func (o *CreateInvitationRequest) HasLocale() bool`

HasLocale returns a boolean if a field has been set.

### SetLocaleNil

`func (o *CreateInvitationRequest) SetLocaleNil(b bool)`

 SetLocaleNil sets the value for Locale to be an explicit nil

### UnsetLocale
`func (o *CreateInvitationRequest) UnsetLocale()`

UnsetLocale ensures that no value is present for Locale, not even an explicit nil
### GetPhone

`func (o *CreateInvitationRequest) GetPhone() string`

GetPhone returns the Phone field if non-nil, zero value otherwise.

### GetPhoneOk

`func (o *CreateInvitationRequest) GetPhoneOk() (*string, bool)`

GetPhoneOk returns a tuple with the Phone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhone

`func (o *CreateInvitationRequest) SetPhone(v string)`

SetPhone sets Phone field to given value.

### HasPhone

`func (o *CreateInvitationRequest) HasPhone() bool`

HasPhone returns a boolean if a field has been set.

### SetPhoneNil

`func (o *CreateInvitationRequest) SetPhoneNil(b bool)`

 SetPhoneNil sets the value for Phone to be an explicit nil

### UnsetPhone
`func (o *CreateInvitationRequest) UnsetPhone()`

UnsetPhone ensures that no value is present for Phone, not even an explicit nil
### GetSiteAccess

`func (o *CreateInvitationRequest) GetSiteAccess() InvitationSiteAccessDto`

GetSiteAccess returns the SiteAccess field if non-nil, zero value otherwise.

### GetSiteAccessOk

`func (o *CreateInvitationRequest) GetSiteAccessOk() (*InvitationSiteAccessDto, bool)`

GetSiteAccessOk returns a tuple with the SiteAccess field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSiteAccess

`func (o *CreateInvitationRequest) SetSiteAccess(v InvitationSiteAccessDto)`

SetSiteAccess sets SiteAccess field to given value.

### HasSiteAccess

`func (o *CreateInvitationRequest) HasSiteAccess() bool`

HasSiteAccess returns a boolean if a field has been set.

### SetSiteAccessNil

`func (o *CreateInvitationRequest) SetSiteAccessNil(b bool)`

 SetSiteAccessNil sets the value for SiteAccess to be an explicit nil

### UnsetSiteAccess
`func (o *CreateInvitationRequest) UnsetSiteAccess()`

UnsetSiteAccess ensures that no value is present for SiteAccess, not even an explicit nil
### GetSmsConsentConfirmed

`func (o *CreateInvitationRequest) GetSmsConsentConfirmed() bool`

GetSmsConsentConfirmed returns the SmsConsentConfirmed field if non-nil, zero value otherwise.

### GetSmsConsentConfirmedOk

`func (o *CreateInvitationRequest) GetSmsConsentConfirmedOk() (*bool, bool)`

GetSmsConsentConfirmedOk returns a tuple with the SmsConsentConfirmed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmsConsentConfirmed

`func (o *CreateInvitationRequest) SetSmsConsentConfirmed(v bool)`

SetSmsConsentConfirmed sets SmsConsentConfirmed field to given value.

### HasSmsConsentConfirmed

`func (o *CreateInvitationRequest) HasSmsConsentConfirmed() bool`

HasSmsConsentConfirmed returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
