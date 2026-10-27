# UserInvitationDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AcceptedUserId** | Pointer to **NullableString** |  | [optional]
**Channel** | **string** | &#x60;sms&#x60;, &#x60;email&#x60;, or &#x60;both&#x60;. |
**CreatedAt** | **string** |  |
**DateOfBirth** | Pointer to **NullableString** | Parent-declared date of birth (&#x60;YYYY-MM-DD&#x60;). Present only while the invitation is pending. Applied to the account at redeem when the account has none. | [optional]
**DefaultGroupIds** | Pointer to **[]string** | Groups joined when the invitation is redeemed, in the same transaction as the org role. | [optional]
**DefaultRoles** | **[]string** |  |
**Email** | Pointer to **NullableString** |  | [optional]
**ExpiresAt** | **string** |  |
**Id** | **string** |  |
**InvitedByUserId** | **string** |  |
**InviteeName** | Pointer to **NullableString** | Name the inviter gave for the invitee. Present only while the invitation is pending. | [optional]
**LastSentAt** | Pointer to **NullableString** |  | [optional]
**Locale** | Pointer to **NullableString** | Message language the sender chose, or &#x60;null&#x60; when none was chosen. | [optional]
**MessageLocale** | Pointer to **NullableString** | Shipped locale code of the language the invitation messages are written in: the invitee&#39;s own account language, else &#x60;locale&#x60;, else the org default, else English. Present only while the invitation is pending. | [optional]
**OrgId** | **string** |  |
**Phone** | Pointer to **NullableString** | SMS recipient when the invite includes a phone (E.164). | [optional]
**PhoneTransport** | Pointer to **NullableString** | &#x60;sms&#x60; or &#x60;whatsapp&#x60; when a phone PIN was sent. | [optional]
**ResendAvailableAt** | Pointer to **NullableString** | When a resend is next accepted (RFC 3339). Present only while the invitation is pending; a resend before this time is refused with 429. | [optional]
**SentCount** | **int32** |  |
**SiteAccess** | Pointer to [**NullableInvitationSiteAccessDto**](InvitationSiteAccessDto.md) |  | [optional]
**Status** | **string** |  |

## Methods

### NewUserInvitationDto

`func NewUserInvitationDto(channel string, createdAt string, defaultRoles []string, expiresAt string, id string, invitedByUserId string, orgId string, sentCount int32, status string, ) *UserInvitationDto`

NewUserInvitationDto instantiates a new UserInvitationDto object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUserInvitationDtoWithDefaults

`func NewUserInvitationDtoWithDefaults() *UserInvitationDto`

NewUserInvitationDtoWithDefaults instantiates a new UserInvitationDto object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAcceptedUserId

`func (o *UserInvitationDto) GetAcceptedUserId() string`

GetAcceptedUserId returns the AcceptedUserId field if non-nil, zero value otherwise.

### GetAcceptedUserIdOk

`func (o *UserInvitationDto) GetAcceptedUserIdOk() (*string, bool)`

GetAcceptedUserIdOk returns a tuple with the AcceptedUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAcceptedUserId

`func (o *UserInvitationDto) SetAcceptedUserId(v string)`

SetAcceptedUserId sets AcceptedUserId field to given value.

### HasAcceptedUserId

`func (o *UserInvitationDto) HasAcceptedUserId() bool`

HasAcceptedUserId returns a boolean if a field has been set.

### SetAcceptedUserIdNil

`func (o *UserInvitationDto) SetAcceptedUserIdNil(b bool)`

 SetAcceptedUserIdNil sets the value for AcceptedUserId to be an explicit nil

### UnsetAcceptedUserId
`func (o *UserInvitationDto) UnsetAcceptedUserId()`

UnsetAcceptedUserId ensures that no value is present for AcceptedUserId, not even an explicit nil
### GetChannel

`func (o *UserInvitationDto) GetChannel() string`

GetChannel returns the Channel field if non-nil, zero value otherwise.

### GetChannelOk

`func (o *UserInvitationDto) GetChannelOk() (*string, bool)`

GetChannelOk returns a tuple with the Channel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChannel

`func (o *UserInvitationDto) SetChannel(v string)`

SetChannel sets Channel field to given value.


### GetCreatedAt

`func (o *UserInvitationDto) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *UserInvitationDto) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *UserInvitationDto) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.


### GetDateOfBirth

`func (o *UserInvitationDto) GetDateOfBirth() string`

GetDateOfBirth returns the DateOfBirth field if non-nil, zero value otherwise.

### GetDateOfBirthOk

`func (o *UserInvitationDto) GetDateOfBirthOk() (*string, bool)`

GetDateOfBirthOk returns a tuple with the DateOfBirth field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDateOfBirth

`func (o *UserInvitationDto) SetDateOfBirth(v string)`

SetDateOfBirth sets DateOfBirth field to given value.

### HasDateOfBirth

`func (o *UserInvitationDto) HasDateOfBirth() bool`

HasDateOfBirth returns a boolean if a field has been set.

### SetDateOfBirthNil

`func (o *UserInvitationDto) SetDateOfBirthNil(b bool)`

 SetDateOfBirthNil sets the value for DateOfBirth to be an explicit nil

### UnsetDateOfBirth
`func (o *UserInvitationDto) UnsetDateOfBirth()`

UnsetDateOfBirth ensures that no value is present for DateOfBirth, not even an explicit nil
### GetDefaultGroupIds

`func (o *UserInvitationDto) GetDefaultGroupIds() []string`

GetDefaultGroupIds returns the DefaultGroupIds field if non-nil, zero value otherwise.

### GetDefaultGroupIdsOk

`func (o *UserInvitationDto) GetDefaultGroupIdsOk() (*[]string, bool)`

GetDefaultGroupIdsOk returns a tuple with the DefaultGroupIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultGroupIds

`func (o *UserInvitationDto) SetDefaultGroupIds(v []string)`

SetDefaultGroupIds sets DefaultGroupIds field to given value.

### HasDefaultGroupIds

`func (o *UserInvitationDto) HasDefaultGroupIds() bool`

HasDefaultGroupIds returns a boolean if a field has been set.

### GetDefaultRoles

`func (o *UserInvitationDto) GetDefaultRoles() []string`

GetDefaultRoles returns the DefaultRoles field if non-nil, zero value otherwise.

### GetDefaultRolesOk

`func (o *UserInvitationDto) GetDefaultRolesOk() (*[]string, bool)`

GetDefaultRolesOk returns a tuple with the DefaultRoles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultRoles

`func (o *UserInvitationDto) SetDefaultRoles(v []string)`

SetDefaultRoles sets DefaultRoles field to given value.


### GetEmail

`func (o *UserInvitationDto) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *UserInvitationDto) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *UserInvitationDto) SetEmail(v string)`

SetEmail sets Email field to given value.

### HasEmail

`func (o *UserInvitationDto) HasEmail() bool`

HasEmail returns a boolean if a field has been set.

### SetEmailNil

`func (o *UserInvitationDto) SetEmailNil(b bool)`

 SetEmailNil sets the value for Email to be an explicit nil

### UnsetEmail
`func (o *UserInvitationDto) UnsetEmail()`

UnsetEmail ensures that no value is present for Email, not even an explicit nil
### GetExpiresAt

`func (o *UserInvitationDto) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *UserInvitationDto) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *UserInvitationDto) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.


### GetId

`func (o *UserInvitationDto) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *UserInvitationDto) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *UserInvitationDto) SetId(v string)`

SetId sets Id field to given value.


### GetInvitedByUserId

`func (o *UserInvitationDto) GetInvitedByUserId() string`

GetInvitedByUserId returns the InvitedByUserId field if non-nil, zero value otherwise.

### GetInvitedByUserIdOk

`func (o *UserInvitationDto) GetInvitedByUserIdOk() (*string, bool)`

GetInvitedByUserIdOk returns a tuple with the InvitedByUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvitedByUserId

`func (o *UserInvitationDto) SetInvitedByUserId(v string)`

SetInvitedByUserId sets InvitedByUserId field to given value.


### GetInviteeName

`func (o *UserInvitationDto) GetInviteeName() string`

GetInviteeName returns the InviteeName field if non-nil, zero value otherwise.

### GetInviteeNameOk

`func (o *UserInvitationDto) GetInviteeNameOk() (*string, bool)`

GetInviteeNameOk returns a tuple with the InviteeName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteeName

`func (o *UserInvitationDto) SetInviteeName(v string)`

SetInviteeName sets InviteeName field to given value.

### HasInviteeName

`func (o *UserInvitationDto) HasInviteeName() bool`

HasInviteeName returns a boolean if a field has been set.

### SetInviteeNameNil

`func (o *UserInvitationDto) SetInviteeNameNil(b bool)`

 SetInviteeNameNil sets the value for InviteeName to be an explicit nil

### UnsetInviteeName
`func (o *UserInvitationDto) UnsetInviteeName()`

UnsetInviteeName ensures that no value is present for InviteeName, not even an explicit nil
### GetLastSentAt

`func (o *UserInvitationDto) GetLastSentAt() string`

GetLastSentAt returns the LastSentAt field if non-nil, zero value otherwise.

### GetLastSentAtOk

`func (o *UserInvitationDto) GetLastSentAtOk() (*string, bool)`

GetLastSentAtOk returns a tuple with the LastSentAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastSentAt

`func (o *UserInvitationDto) SetLastSentAt(v string)`

SetLastSentAt sets LastSentAt field to given value.

### HasLastSentAt

`func (o *UserInvitationDto) HasLastSentAt() bool`

HasLastSentAt returns a boolean if a field has been set.

### SetLastSentAtNil

`func (o *UserInvitationDto) SetLastSentAtNil(b bool)`

 SetLastSentAtNil sets the value for LastSentAt to be an explicit nil

### UnsetLastSentAt
`func (o *UserInvitationDto) UnsetLastSentAt()`

UnsetLastSentAt ensures that no value is present for LastSentAt, not even an explicit nil
### GetLocale

`func (o *UserInvitationDto) GetLocale() string`

GetLocale returns the Locale field if non-nil, zero value otherwise.

### GetLocaleOk

`func (o *UserInvitationDto) GetLocaleOk() (*string, bool)`

GetLocaleOk returns a tuple with the Locale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocale

`func (o *UserInvitationDto) SetLocale(v string)`

SetLocale sets Locale field to given value.

### HasLocale

`func (o *UserInvitationDto) HasLocale() bool`

HasLocale returns a boolean if a field has been set.

### SetLocaleNil

`func (o *UserInvitationDto) SetLocaleNil(b bool)`

 SetLocaleNil sets the value for Locale to be an explicit nil

### UnsetLocale
`func (o *UserInvitationDto) UnsetLocale()`

UnsetLocale ensures that no value is present for Locale, not even an explicit nil
### GetMessageLocale

`func (o *UserInvitationDto) GetMessageLocale() string`

GetMessageLocale returns the MessageLocale field if non-nil, zero value otherwise.

### GetMessageLocaleOk

`func (o *UserInvitationDto) GetMessageLocaleOk() (*string, bool)`

GetMessageLocaleOk returns a tuple with the MessageLocale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessageLocale

`func (o *UserInvitationDto) SetMessageLocale(v string)`

SetMessageLocale sets MessageLocale field to given value.

### HasMessageLocale

`func (o *UserInvitationDto) HasMessageLocale() bool`

HasMessageLocale returns a boolean if a field has been set.

### SetMessageLocaleNil

`func (o *UserInvitationDto) SetMessageLocaleNil(b bool)`

 SetMessageLocaleNil sets the value for MessageLocale to be an explicit nil

### UnsetMessageLocale
`func (o *UserInvitationDto) UnsetMessageLocale()`

UnsetMessageLocale ensures that no value is present for MessageLocale, not even an explicit nil
### GetOrgId

`func (o *UserInvitationDto) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *UserInvitationDto) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *UserInvitationDto) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetPhone

`func (o *UserInvitationDto) GetPhone() string`

GetPhone returns the Phone field if non-nil, zero value otherwise.

### GetPhoneOk

`func (o *UserInvitationDto) GetPhoneOk() (*string, bool)`

GetPhoneOk returns a tuple with the Phone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhone

`func (o *UserInvitationDto) SetPhone(v string)`

SetPhone sets Phone field to given value.

### HasPhone

`func (o *UserInvitationDto) HasPhone() bool`

HasPhone returns a boolean if a field has been set.

### SetPhoneNil

`func (o *UserInvitationDto) SetPhoneNil(b bool)`

 SetPhoneNil sets the value for Phone to be an explicit nil

### UnsetPhone
`func (o *UserInvitationDto) UnsetPhone()`

UnsetPhone ensures that no value is present for Phone, not even an explicit nil
### GetPhoneTransport

`func (o *UserInvitationDto) GetPhoneTransport() string`

GetPhoneTransport returns the PhoneTransport field if non-nil, zero value otherwise.

### GetPhoneTransportOk

`func (o *UserInvitationDto) GetPhoneTransportOk() (*string, bool)`

GetPhoneTransportOk returns a tuple with the PhoneTransport field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhoneTransport

`func (o *UserInvitationDto) SetPhoneTransport(v string)`

SetPhoneTransport sets PhoneTransport field to given value.

### HasPhoneTransport

`func (o *UserInvitationDto) HasPhoneTransport() bool`

HasPhoneTransport returns a boolean if a field has been set.

### SetPhoneTransportNil

`func (o *UserInvitationDto) SetPhoneTransportNil(b bool)`

 SetPhoneTransportNil sets the value for PhoneTransport to be an explicit nil

### UnsetPhoneTransport
`func (o *UserInvitationDto) UnsetPhoneTransport()`

UnsetPhoneTransport ensures that no value is present for PhoneTransport, not even an explicit nil
### GetResendAvailableAt

`func (o *UserInvitationDto) GetResendAvailableAt() string`

GetResendAvailableAt returns the ResendAvailableAt field if non-nil, zero value otherwise.

### GetResendAvailableAtOk

`func (o *UserInvitationDto) GetResendAvailableAtOk() (*string, bool)`

GetResendAvailableAtOk returns a tuple with the ResendAvailableAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResendAvailableAt

`func (o *UserInvitationDto) SetResendAvailableAt(v string)`

SetResendAvailableAt sets ResendAvailableAt field to given value.

### HasResendAvailableAt

`func (o *UserInvitationDto) HasResendAvailableAt() bool`

HasResendAvailableAt returns a boolean if a field has been set.

### SetResendAvailableAtNil

`func (o *UserInvitationDto) SetResendAvailableAtNil(b bool)`

 SetResendAvailableAtNil sets the value for ResendAvailableAt to be an explicit nil

### UnsetResendAvailableAt
`func (o *UserInvitationDto) UnsetResendAvailableAt()`

UnsetResendAvailableAt ensures that no value is present for ResendAvailableAt, not even an explicit nil
### GetSentCount

`func (o *UserInvitationDto) GetSentCount() int32`

GetSentCount returns the SentCount field if non-nil, zero value otherwise.

### GetSentCountOk

`func (o *UserInvitationDto) GetSentCountOk() (*int32, bool)`

GetSentCountOk returns a tuple with the SentCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSentCount

`func (o *UserInvitationDto) SetSentCount(v int32)`

SetSentCount sets SentCount field to given value.


### GetSiteAccess

`func (o *UserInvitationDto) GetSiteAccess() InvitationSiteAccessDto`

GetSiteAccess returns the SiteAccess field if non-nil, zero value otherwise.

### GetSiteAccessOk

`func (o *UserInvitationDto) GetSiteAccessOk() (*InvitationSiteAccessDto, bool)`

GetSiteAccessOk returns a tuple with the SiteAccess field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSiteAccess

`func (o *UserInvitationDto) SetSiteAccess(v InvitationSiteAccessDto)`

SetSiteAccess sets SiteAccess field to given value.

### HasSiteAccess

`func (o *UserInvitationDto) HasSiteAccess() bool`

HasSiteAccess returns a boolean if a field has been set.

### SetSiteAccessNil

`func (o *UserInvitationDto) SetSiteAccessNil(b bool)`

 SetSiteAccessNil sets the value for SiteAccess to be an explicit nil

### UnsetSiteAccess
`func (o *UserInvitationDto) UnsetSiteAccess()`

UnsetSiteAccess ensures that no value is present for SiteAccess, not even an explicit nil
### GetStatus

`func (o *UserInvitationDto) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *UserInvitationDto) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *UserInvitationDto) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
