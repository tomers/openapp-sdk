# IntegrationUserItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DisplayName** | Pointer to **NullableString** |  | [optional]
**Email** | Pointer to **NullableString** |  | [optional]
**ExternalId** | **string** |  |
**IsAdmin** | Pointer to **NullableBool** |  | [optional]
**LastSeenAt** | Pointer to **NullableString** |  | [optional]
**OpenappUserId** | Pointer to **NullableString** | Present when &#x60;status &#x3D;&#x3D; linked&#x60;. ULID string of the OpenApp user. | [optional]
**PendingInvitationId** | Pointer to **NullableString** | Present when &#x60;status &#x3D;&#x3D; invited&#x60;. ULID string of the matching &#x60;user_invitations&#x60; row. | [optional]
**Phone** | Pointer to **NullableString** |  | [optional]
**Status** | **string** | &#x60;linked&#x60; | &#x60;invited&#x60; | &#x60;unlinked&#x60;. |
**Unmanaged** | **bool** | True when the vendor user is on the gate but not linked to an OpenApp user (flag only). |

## Methods

### NewIntegrationUserItem

`func NewIntegrationUserItem(externalId string, status string, unmanaged bool, ) *IntegrationUserItem`

NewIntegrationUserItem instantiates a new IntegrationUserItem object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewIntegrationUserItemWithDefaults

`func NewIntegrationUserItemWithDefaults() *IntegrationUserItem`

NewIntegrationUserItemWithDefaults instantiates a new IntegrationUserItem object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDisplayName

`func (o *IntegrationUserItem) GetDisplayName() string`

GetDisplayName returns the DisplayName field if non-nil, zero value otherwise.

### GetDisplayNameOk

`func (o *IntegrationUserItem) GetDisplayNameOk() (*string, bool)`

GetDisplayNameOk returns a tuple with the DisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisplayName

`func (o *IntegrationUserItem) SetDisplayName(v string)`

SetDisplayName sets DisplayName field to given value.

### HasDisplayName

`func (o *IntegrationUserItem) HasDisplayName() bool`

HasDisplayName returns a boolean if a field has been set.

### SetDisplayNameNil

`func (o *IntegrationUserItem) SetDisplayNameNil(b bool)`

 SetDisplayNameNil sets the value for DisplayName to be an explicit nil

### UnsetDisplayName
`func (o *IntegrationUserItem) UnsetDisplayName()`

UnsetDisplayName ensures that no value is present for DisplayName, not even an explicit nil
### GetEmail

`func (o *IntegrationUserItem) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *IntegrationUserItem) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *IntegrationUserItem) SetEmail(v string)`

SetEmail sets Email field to given value.

### HasEmail

`func (o *IntegrationUserItem) HasEmail() bool`

HasEmail returns a boolean if a field has been set.

### SetEmailNil

`func (o *IntegrationUserItem) SetEmailNil(b bool)`

 SetEmailNil sets the value for Email to be an explicit nil

### UnsetEmail
`func (o *IntegrationUserItem) UnsetEmail()`

UnsetEmail ensures that no value is present for Email, not even an explicit nil
### GetExternalId

`func (o *IntegrationUserItem) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *IntegrationUserItem) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *IntegrationUserItem) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.


### GetIsAdmin

`func (o *IntegrationUserItem) GetIsAdmin() bool`

GetIsAdmin returns the IsAdmin field if non-nil, zero value otherwise.

### GetIsAdminOk

`func (o *IntegrationUserItem) GetIsAdminOk() (*bool, bool)`

GetIsAdminOk returns a tuple with the IsAdmin field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsAdmin

`func (o *IntegrationUserItem) SetIsAdmin(v bool)`

SetIsAdmin sets IsAdmin field to given value.

### HasIsAdmin

`func (o *IntegrationUserItem) HasIsAdmin() bool`

HasIsAdmin returns a boolean if a field has been set.

### SetIsAdminNil

`func (o *IntegrationUserItem) SetIsAdminNil(b bool)`

 SetIsAdminNil sets the value for IsAdmin to be an explicit nil

### UnsetIsAdmin
`func (o *IntegrationUserItem) UnsetIsAdmin()`

UnsetIsAdmin ensures that no value is present for IsAdmin, not even an explicit nil
### GetLastSeenAt

`func (o *IntegrationUserItem) GetLastSeenAt() string`

GetLastSeenAt returns the LastSeenAt field if non-nil, zero value otherwise.

### GetLastSeenAtOk

`func (o *IntegrationUserItem) GetLastSeenAtOk() (*string, bool)`

GetLastSeenAtOk returns a tuple with the LastSeenAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastSeenAt

`func (o *IntegrationUserItem) SetLastSeenAt(v string)`

SetLastSeenAt sets LastSeenAt field to given value.

### HasLastSeenAt

`func (o *IntegrationUserItem) HasLastSeenAt() bool`

HasLastSeenAt returns a boolean if a field has been set.

### SetLastSeenAtNil

`func (o *IntegrationUserItem) SetLastSeenAtNil(b bool)`

 SetLastSeenAtNil sets the value for LastSeenAt to be an explicit nil

### UnsetLastSeenAt
`func (o *IntegrationUserItem) UnsetLastSeenAt()`

UnsetLastSeenAt ensures that no value is present for LastSeenAt, not even an explicit nil
### GetOpenappUserId

`func (o *IntegrationUserItem) GetOpenappUserId() string`

GetOpenappUserId returns the OpenappUserId field if non-nil, zero value otherwise.

### GetOpenappUserIdOk

`func (o *IntegrationUserItem) GetOpenappUserIdOk() (*string, bool)`

GetOpenappUserIdOk returns a tuple with the OpenappUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpenappUserId

`func (o *IntegrationUserItem) SetOpenappUserId(v string)`

SetOpenappUserId sets OpenappUserId field to given value.

### HasOpenappUserId

`func (o *IntegrationUserItem) HasOpenappUserId() bool`

HasOpenappUserId returns a boolean if a field has been set.

### SetOpenappUserIdNil

`func (o *IntegrationUserItem) SetOpenappUserIdNil(b bool)`

 SetOpenappUserIdNil sets the value for OpenappUserId to be an explicit nil

### UnsetOpenappUserId
`func (o *IntegrationUserItem) UnsetOpenappUserId()`

UnsetOpenappUserId ensures that no value is present for OpenappUserId, not even an explicit nil
### GetPendingInvitationId

`func (o *IntegrationUserItem) GetPendingInvitationId() string`

GetPendingInvitationId returns the PendingInvitationId field if non-nil, zero value otherwise.

### GetPendingInvitationIdOk

`func (o *IntegrationUserItem) GetPendingInvitationIdOk() (*string, bool)`

GetPendingInvitationIdOk returns a tuple with the PendingInvitationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPendingInvitationId

`func (o *IntegrationUserItem) SetPendingInvitationId(v string)`

SetPendingInvitationId sets PendingInvitationId field to given value.

### HasPendingInvitationId

`func (o *IntegrationUserItem) HasPendingInvitationId() bool`

HasPendingInvitationId returns a boolean if a field has been set.

### SetPendingInvitationIdNil

`func (o *IntegrationUserItem) SetPendingInvitationIdNil(b bool)`

 SetPendingInvitationIdNil sets the value for PendingInvitationId to be an explicit nil

### UnsetPendingInvitationId
`func (o *IntegrationUserItem) UnsetPendingInvitationId()`

UnsetPendingInvitationId ensures that no value is present for PendingInvitationId, not even an explicit nil
### GetPhone

`func (o *IntegrationUserItem) GetPhone() string`

GetPhone returns the Phone field if non-nil, zero value otherwise.

### GetPhoneOk

`func (o *IntegrationUserItem) GetPhoneOk() (*string, bool)`

GetPhoneOk returns a tuple with the Phone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhone

`func (o *IntegrationUserItem) SetPhone(v string)`

SetPhone sets Phone field to given value.

### HasPhone

`func (o *IntegrationUserItem) HasPhone() bool`

HasPhone returns a boolean if a field has been set.

### SetPhoneNil

`func (o *IntegrationUserItem) SetPhoneNil(b bool)`

 SetPhoneNil sets the value for Phone to be an explicit nil

### UnsetPhone
`func (o *IntegrationUserItem) UnsetPhone()`

UnsetPhone ensures that no value is present for Phone, not even an explicit nil
### GetStatus

`func (o *IntegrationUserItem) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *IntegrationUserItem) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *IntegrationUserItem) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetUnmanaged

`func (o *IntegrationUserItem) GetUnmanaged() bool`

GetUnmanaged returns the Unmanaged field if non-nil, zero value otherwise.

### GetUnmanagedOk

`func (o *IntegrationUserItem) GetUnmanagedOk() (*bool, bool)`

GetUnmanagedOk returns a tuple with the Unmanaged field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnmanaged

`func (o *IntegrationUserItem) SetUnmanaged(v bool)`

SetUnmanaged sets Unmanaged field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
