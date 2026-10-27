# PendingInvitationDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Channel** | **string** |  |
**DefaultRoles** | **[]string** | Org roles granted on accept. Reflects the invitation as it is now, not as it was when a message was sent. |
**ExpiresAt** | **string** |  |
**InvitedByName** | Pointer to **interface{}** |  | [optional]
**OrgId** | **string** |  |
**OrgName** | Pointer to **interface{}** |  | [optional]
**SiteAccess** | Pointer to [**InvitationPreviewSiteAccess**](InvitationPreviewSiteAccess.md) | Site (and listing) access granted on accept; absent when the invitation grants none. | [optional]
**Status** | **string** |  |
**TargetMasked** | **string** | Masked destination the code is sent to (for example &#x60;+9725…78&#x60;). |
**Id** | **string** | Pass to &#x60;POST /me/org-invitations/{id}/accept&#x60;. |

## Methods

### NewPendingInvitationDto

`func NewPendingInvitationDto(channel string, defaultRoles []string, expiresAt string, orgId string, status string, targetMasked string, id string, ) *PendingInvitationDto`

NewPendingInvitationDto instantiates a new PendingInvitationDto object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPendingInvitationDtoWithDefaults

`func NewPendingInvitationDtoWithDefaults() *PendingInvitationDto`

NewPendingInvitationDtoWithDefaults instantiates a new PendingInvitationDto object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetChannel

`func (o *PendingInvitationDto) GetChannel() string`

GetChannel returns the Channel field if non-nil, zero value otherwise.

### GetChannelOk

`func (o *PendingInvitationDto) GetChannelOk() (*string, bool)`

GetChannelOk returns a tuple with the Channel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChannel

`func (o *PendingInvitationDto) SetChannel(v string)`

SetChannel sets Channel field to given value.


### GetDefaultRoles

`func (o *PendingInvitationDto) GetDefaultRoles() []string`

GetDefaultRoles returns the DefaultRoles field if non-nil, zero value otherwise.

### GetDefaultRolesOk

`func (o *PendingInvitationDto) GetDefaultRolesOk() (*[]string, bool)`

GetDefaultRolesOk returns a tuple with the DefaultRoles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultRoles

`func (o *PendingInvitationDto) SetDefaultRoles(v []string)`

SetDefaultRoles sets DefaultRoles field to given value.


### GetExpiresAt

`func (o *PendingInvitationDto) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *PendingInvitationDto) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *PendingInvitationDto) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.


### GetInvitedByName

`func (o *PendingInvitationDto) GetInvitedByName() interface{}`

GetInvitedByName returns the InvitedByName field if non-nil, zero value otherwise.

### GetInvitedByNameOk

`func (o *PendingInvitationDto) GetInvitedByNameOk() (*interface{}, bool)`

GetInvitedByNameOk returns a tuple with the InvitedByName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvitedByName

`func (o *PendingInvitationDto) SetInvitedByName(v interface{})`

SetInvitedByName sets InvitedByName field to given value.

### HasInvitedByName

`func (o *PendingInvitationDto) HasInvitedByName() bool`

HasInvitedByName returns a boolean if a field has been set.

### SetInvitedByNameNil

`func (o *PendingInvitationDto) SetInvitedByNameNil(b bool)`

 SetInvitedByNameNil sets the value for InvitedByName to be an explicit nil

### UnsetInvitedByName
`func (o *PendingInvitationDto) UnsetInvitedByName()`

UnsetInvitedByName ensures that no value is present for InvitedByName, not even an explicit nil
### GetOrgId

`func (o *PendingInvitationDto) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *PendingInvitationDto) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *PendingInvitationDto) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetOrgName

`func (o *PendingInvitationDto) GetOrgName() interface{}`

GetOrgName returns the OrgName field if non-nil, zero value otherwise.

### GetOrgNameOk

`func (o *PendingInvitationDto) GetOrgNameOk() (*interface{}, bool)`

GetOrgNameOk returns a tuple with the OrgName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgName

`func (o *PendingInvitationDto) SetOrgName(v interface{})`

SetOrgName sets OrgName field to given value.

### HasOrgName

`func (o *PendingInvitationDto) HasOrgName() bool`

HasOrgName returns a boolean if a field has been set.

### SetOrgNameNil

`func (o *PendingInvitationDto) SetOrgNameNil(b bool)`

 SetOrgNameNil sets the value for OrgName to be an explicit nil

### UnsetOrgName
`func (o *PendingInvitationDto) UnsetOrgName()`

UnsetOrgName ensures that no value is present for OrgName, not even an explicit nil
### GetSiteAccess

`func (o *PendingInvitationDto) GetSiteAccess() InvitationPreviewSiteAccess`

GetSiteAccess returns the SiteAccess field if non-nil, zero value otherwise.

### GetSiteAccessOk

`func (o *PendingInvitationDto) GetSiteAccessOk() (*InvitationPreviewSiteAccess, bool)`

GetSiteAccessOk returns a tuple with the SiteAccess field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSiteAccess

`func (o *PendingInvitationDto) SetSiteAccess(v InvitationPreviewSiteAccess)`

SetSiteAccess sets SiteAccess field to given value.

### HasSiteAccess

`func (o *PendingInvitationDto) HasSiteAccess() bool`

HasSiteAccess returns a boolean if a field has been set.

### GetStatus

`func (o *PendingInvitationDto) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *PendingInvitationDto) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *PendingInvitationDto) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetTargetMasked

`func (o *PendingInvitationDto) GetTargetMasked() string`

GetTargetMasked returns the TargetMasked field if non-nil, zero value otherwise.

### GetTargetMaskedOk

`func (o *PendingInvitationDto) GetTargetMaskedOk() (*string, bool)`

GetTargetMaskedOk returns a tuple with the TargetMasked field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTargetMasked

`func (o *PendingInvitationDto) SetTargetMasked(v string)`

SetTargetMasked sets TargetMasked field to given value.


### GetId

`func (o *PendingInvitationDto) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PendingInvitationDto) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PendingInvitationDto) SetId(v string)`

SetId sets Id field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
