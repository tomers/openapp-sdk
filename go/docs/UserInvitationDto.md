# UserInvitationDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AcceptedUserId** | Pointer to **NullableString** |  | [optional]
**Channel** | **string** | &#x60;sms&#x60;, &#x60;email&#x60;, or &#x60;both&#x60;. |
**CreatedAt** | **string** |  |
**DefaultRoles** | **[]string** |  |
**Email** | Pointer to **NullableString** |  | [optional]
**ExpiresAt** | **string** |  |
**Id** | **string** |  |
**InvitedByUserId** | **string** |  |
**LastSentAt** | Pointer to **NullableString** |  | [optional]
**OrgId** | **string** |  |
**Phone** | Pointer to **NullableString** | SMS recipient when the invite includes a phone (E.164). | [optional]
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
