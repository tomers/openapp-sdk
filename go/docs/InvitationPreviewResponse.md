# InvitationPreviewResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Channel** | **string** |  |
**DefaultRoles** | **[]string** | Org roles granted on accept. Reflects the invitation as it is now, not as it was when a message was sent. |
**ExpiresAt** | **string** |  |
**InvitedByName** | Pointer to **interface{}** |  | [optional]
**OrgId** | **string** |  |
**OrgName** | Pointer to **interface{}** |  | [optional]
**SiteAccess** | Pointer to [**NullableInvitationPreviewSiteAccess**](InvitationPreviewSiteAccess.md) | Site (and listing) access granted on accept; absent when the invitation grants none. | [optional]
**Status** | **string** |  |
**TargetMasked** | **string** | Masked destination the code is sent to (for example &#x60;+9725…78&#x60;). |

## Methods

### NewInvitationPreviewResponse

`func NewInvitationPreviewResponse(channel string, defaultRoles []string, expiresAt string, orgId string, status string, targetMasked string, ) *InvitationPreviewResponse`

NewInvitationPreviewResponse instantiates a new InvitationPreviewResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInvitationPreviewResponseWithDefaults

`func NewInvitationPreviewResponseWithDefaults() *InvitationPreviewResponse`

NewInvitationPreviewResponseWithDefaults instantiates a new InvitationPreviewResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetChannel

`func (o *InvitationPreviewResponse) GetChannel() string`

GetChannel returns the Channel field if non-nil, zero value otherwise.

### GetChannelOk

`func (o *InvitationPreviewResponse) GetChannelOk() (*string, bool)`

GetChannelOk returns a tuple with the Channel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChannel

`func (o *InvitationPreviewResponse) SetChannel(v string)`

SetChannel sets Channel field to given value.


### GetDefaultRoles

`func (o *InvitationPreviewResponse) GetDefaultRoles() []string`

GetDefaultRoles returns the DefaultRoles field if non-nil, zero value otherwise.

### GetDefaultRolesOk

`func (o *InvitationPreviewResponse) GetDefaultRolesOk() (*[]string, bool)`

GetDefaultRolesOk returns a tuple with the DefaultRoles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultRoles

`func (o *InvitationPreviewResponse) SetDefaultRoles(v []string)`

SetDefaultRoles sets DefaultRoles field to given value.


### GetExpiresAt

`func (o *InvitationPreviewResponse) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *InvitationPreviewResponse) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *InvitationPreviewResponse) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.


### GetInvitedByName

`func (o *InvitationPreviewResponse) GetInvitedByName() interface{}`

GetInvitedByName returns the InvitedByName field if non-nil, zero value otherwise.

### GetInvitedByNameOk

`func (o *InvitationPreviewResponse) GetInvitedByNameOk() (*interface{}, bool)`

GetInvitedByNameOk returns a tuple with the InvitedByName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvitedByName

`func (o *InvitationPreviewResponse) SetInvitedByName(v interface{})`

SetInvitedByName sets InvitedByName field to given value.

### HasInvitedByName

`func (o *InvitationPreviewResponse) HasInvitedByName() bool`

HasInvitedByName returns a boolean if a field has been set.

### SetInvitedByNameNil

`func (o *InvitationPreviewResponse) SetInvitedByNameNil(b bool)`

 SetInvitedByNameNil sets the value for InvitedByName to be an explicit nil

### UnsetInvitedByName
`func (o *InvitationPreviewResponse) UnsetInvitedByName()`

UnsetInvitedByName ensures that no value is present for InvitedByName, not even an explicit nil
### GetOrgId

`func (o *InvitationPreviewResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *InvitationPreviewResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *InvitationPreviewResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetOrgName

`func (o *InvitationPreviewResponse) GetOrgName() interface{}`

GetOrgName returns the OrgName field if non-nil, zero value otherwise.

### GetOrgNameOk

`func (o *InvitationPreviewResponse) GetOrgNameOk() (*interface{}, bool)`

GetOrgNameOk returns a tuple with the OrgName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgName

`func (o *InvitationPreviewResponse) SetOrgName(v interface{})`

SetOrgName sets OrgName field to given value.

### HasOrgName

`func (o *InvitationPreviewResponse) HasOrgName() bool`

HasOrgName returns a boolean if a field has been set.

### SetOrgNameNil

`func (o *InvitationPreviewResponse) SetOrgNameNil(b bool)`

 SetOrgNameNil sets the value for OrgName to be an explicit nil

### UnsetOrgName
`func (o *InvitationPreviewResponse) UnsetOrgName()`

UnsetOrgName ensures that no value is present for OrgName, not even an explicit nil
### GetSiteAccess

`func (o *InvitationPreviewResponse) GetSiteAccess() InvitationPreviewSiteAccess`

GetSiteAccess returns the SiteAccess field if non-nil, zero value otherwise.

### GetSiteAccessOk

`func (o *InvitationPreviewResponse) GetSiteAccessOk() (*InvitationPreviewSiteAccess, bool)`

GetSiteAccessOk returns a tuple with the SiteAccess field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSiteAccess

`func (o *InvitationPreviewResponse) SetSiteAccess(v InvitationPreviewSiteAccess)`

SetSiteAccess sets SiteAccess field to given value.

### HasSiteAccess

`func (o *InvitationPreviewResponse) HasSiteAccess() bool`

HasSiteAccess returns a boolean if a field has been set.

### SetSiteAccessNil

`func (o *InvitationPreviewResponse) SetSiteAccessNil(b bool)`

 SetSiteAccessNil sets the value for SiteAccess to be an explicit nil

### UnsetSiteAccess
`func (o *InvitationPreviewResponse) UnsetSiteAccess()`

UnsetSiteAccess ensures that no value is present for SiteAccess, not even an explicit nil
### GetStatus

`func (o *InvitationPreviewResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *InvitationPreviewResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *InvitationPreviewResponse) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetTargetMasked

`func (o *InvitationPreviewResponse) GetTargetMasked() string`

GetTargetMasked returns the TargetMasked field if non-nil, zero value otherwise.

### GetTargetMaskedOk

`func (o *InvitationPreviewResponse) GetTargetMaskedOk() (*string, bool)`

GetTargetMaskedOk returns a tuple with the TargetMasked field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTargetMasked

`func (o *InvitationPreviewResponse) SetTargetMasked(v string)`

SetTargetMasked sets TargetMasked field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
