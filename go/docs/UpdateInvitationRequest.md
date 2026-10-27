# UpdateInvitationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DateOfBirth** | Pointer to **NullableString** | Replaces the parent-declared date of birth (&#x60;YYYY-MM-DD&#x60;). Must classify as under 16. Send an empty string to clear it. Omit to leave it unchanged. | [optional]
**DefaultRoles** | Pointer to **[]string** | Replaces the roles assigned on accept (within the inviting org). | [optional]
**Locale** | Pointer to **NullableString** | Replaces the message language: a supported locale code. Applies to messages sent from now on; a message already sent keeps the language it was written in. | [optional]
**SiteAccess** | Pointer to [**NullableInvitationSiteAccessDto**](InvitationSiteAccessDto.md) | Replaces the site (and listing) access applied when the invitation is redeemed. | [optional]

## Methods

### NewUpdateInvitationRequest

`func NewUpdateInvitationRequest() *UpdateInvitationRequest`

NewUpdateInvitationRequest instantiates a new UpdateInvitationRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateInvitationRequestWithDefaults

`func NewUpdateInvitationRequestWithDefaults() *UpdateInvitationRequest`

NewUpdateInvitationRequestWithDefaults instantiates a new UpdateInvitationRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDateOfBirth

`func (o *UpdateInvitationRequest) GetDateOfBirth() string`

GetDateOfBirth returns the DateOfBirth field if non-nil, zero value otherwise.

### GetDateOfBirthOk

`func (o *UpdateInvitationRequest) GetDateOfBirthOk() (*string, bool)`

GetDateOfBirthOk returns a tuple with the DateOfBirth field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDateOfBirth

`func (o *UpdateInvitationRequest) SetDateOfBirth(v string)`

SetDateOfBirth sets DateOfBirth field to given value.

### HasDateOfBirth

`func (o *UpdateInvitationRequest) HasDateOfBirth() bool`

HasDateOfBirth returns a boolean if a field has been set.

### SetDateOfBirthNil

`func (o *UpdateInvitationRequest) SetDateOfBirthNil(b bool)`

 SetDateOfBirthNil sets the value for DateOfBirth to be an explicit nil

### UnsetDateOfBirth
`func (o *UpdateInvitationRequest) UnsetDateOfBirth()`

UnsetDateOfBirth ensures that no value is present for DateOfBirth, not even an explicit nil
### GetDefaultRoles

`func (o *UpdateInvitationRequest) GetDefaultRoles() []string`

GetDefaultRoles returns the DefaultRoles field if non-nil, zero value otherwise.

### GetDefaultRolesOk

`func (o *UpdateInvitationRequest) GetDefaultRolesOk() (*[]string, bool)`

GetDefaultRolesOk returns a tuple with the DefaultRoles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultRoles

`func (o *UpdateInvitationRequest) SetDefaultRoles(v []string)`

SetDefaultRoles sets DefaultRoles field to given value.

### HasDefaultRoles

`func (o *UpdateInvitationRequest) HasDefaultRoles() bool`

HasDefaultRoles returns a boolean if a field has been set.

### SetDefaultRolesNil

`func (o *UpdateInvitationRequest) SetDefaultRolesNil(b bool)`

 SetDefaultRolesNil sets the value for DefaultRoles to be an explicit nil

### UnsetDefaultRoles
`func (o *UpdateInvitationRequest) UnsetDefaultRoles()`

UnsetDefaultRoles ensures that no value is present for DefaultRoles, not even an explicit nil
### GetLocale

`func (o *UpdateInvitationRequest) GetLocale() string`

GetLocale returns the Locale field if non-nil, zero value otherwise.

### GetLocaleOk

`func (o *UpdateInvitationRequest) GetLocaleOk() (*string, bool)`

GetLocaleOk returns a tuple with the Locale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocale

`func (o *UpdateInvitationRequest) SetLocale(v string)`

SetLocale sets Locale field to given value.

### HasLocale

`func (o *UpdateInvitationRequest) HasLocale() bool`

HasLocale returns a boolean if a field has been set.

### SetLocaleNil

`func (o *UpdateInvitationRequest) SetLocaleNil(b bool)`

 SetLocaleNil sets the value for Locale to be an explicit nil

### UnsetLocale
`func (o *UpdateInvitationRequest) UnsetLocale()`

UnsetLocale ensures that no value is present for Locale, not even an explicit nil
### GetSiteAccess

`func (o *UpdateInvitationRequest) GetSiteAccess() InvitationSiteAccessDto`

GetSiteAccess returns the SiteAccess field if non-nil, zero value otherwise.

### GetSiteAccessOk

`func (o *UpdateInvitationRequest) GetSiteAccessOk() (*InvitationSiteAccessDto, bool)`

GetSiteAccessOk returns a tuple with the SiteAccess field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSiteAccess

`func (o *UpdateInvitationRequest) SetSiteAccess(v InvitationSiteAccessDto)`

SetSiteAccess sets SiteAccess field to given value.

### HasSiteAccess

`func (o *UpdateInvitationRequest) HasSiteAccess() bool`

HasSiteAccess returns a boolean if a field has been set.

### SetSiteAccessNil

`func (o *UpdateInvitationRequest) SetSiteAccessNil(b bool)`

 SetSiteAccessNil sets the value for SiteAccess to be an explicit nil

### UnsetSiteAccess
`func (o *UpdateInvitationRequest) UnsetSiteAccess()`

UnsetSiteAccess ensures that no value is present for SiteAccess, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
