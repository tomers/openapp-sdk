# AgentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AllowedEntityIds** | Pointer to **[]string** |  | [optional]
**AllowedPortalIds** | Pointer to **[]string** |  | [optional]
**AllowedToolNames** | Pointer to **[]string** |  | [optional]
**ConfirmationMode** | **NullableString** |  |
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**ExpiresAt** | Pointer to **NullableString** |  | [optional]
**Id** | **NullableString** |  |
**LastUsedAt** | Pointer to **NullableString** |  | [optional]
**Name** | **NullableString** |  |
**OrgId** | **NullableString** |  |
**OwnerUserId** | **NullableString** |  |
**Purpose** | Pointer to **NullableString** |  | [optional]
**RevokedAt** | Pointer to **NullableString** |  | [optional]
**UpdatedAt** | Pointer to **NullableString** |  | [optional]

## Methods

### NewAgentResponse

`func NewAgentResponse(confirmationMode NullableString, id NullableString, name NullableString, orgId NullableString, ownerUserId NullableString, ) *AgentResponse`

NewAgentResponse instantiates a new AgentResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAgentResponseWithDefaults

`func NewAgentResponseWithDefaults() *AgentResponse`

NewAgentResponseWithDefaults instantiates a new AgentResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAllowedEntityIds

`func (o *AgentResponse) GetAllowedEntityIds() []string`

GetAllowedEntityIds returns the AllowedEntityIds field if non-nil, zero value otherwise.

### GetAllowedEntityIdsOk

`func (o *AgentResponse) GetAllowedEntityIdsOk() (*[]string, bool)`

GetAllowedEntityIdsOk returns a tuple with the AllowedEntityIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedEntityIds

`func (o *AgentResponse) SetAllowedEntityIds(v []string)`

SetAllowedEntityIds sets AllowedEntityIds field to given value.

### HasAllowedEntityIds

`func (o *AgentResponse) HasAllowedEntityIds() bool`

HasAllowedEntityIds returns a boolean if a field has been set.

### SetAllowedEntityIdsNil

`func (o *AgentResponse) SetAllowedEntityIdsNil(b bool)`

 SetAllowedEntityIdsNil sets the value for AllowedEntityIds to be an explicit nil

### UnsetAllowedEntityIds
`func (o *AgentResponse) UnsetAllowedEntityIds()`

UnsetAllowedEntityIds ensures that no value is present for AllowedEntityIds, not even an explicit nil
### GetAllowedPortalIds

`func (o *AgentResponse) GetAllowedPortalIds() []string`

GetAllowedPortalIds returns the AllowedPortalIds field if non-nil, zero value otherwise.

### GetAllowedPortalIdsOk

`func (o *AgentResponse) GetAllowedPortalIdsOk() (*[]string, bool)`

GetAllowedPortalIdsOk returns a tuple with the AllowedPortalIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedPortalIds

`func (o *AgentResponse) SetAllowedPortalIds(v []string)`

SetAllowedPortalIds sets AllowedPortalIds field to given value.

### HasAllowedPortalIds

`func (o *AgentResponse) HasAllowedPortalIds() bool`

HasAllowedPortalIds returns a boolean if a field has been set.

### SetAllowedPortalIdsNil

`func (o *AgentResponse) SetAllowedPortalIdsNil(b bool)`

 SetAllowedPortalIdsNil sets the value for AllowedPortalIds to be an explicit nil

### UnsetAllowedPortalIds
`func (o *AgentResponse) UnsetAllowedPortalIds()`

UnsetAllowedPortalIds ensures that no value is present for AllowedPortalIds, not even an explicit nil
### GetAllowedToolNames

`func (o *AgentResponse) GetAllowedToolNames() []string`

GetAllowedToolNames returns the AllowedToolNames field if non-nil, zero value otherwise.

### GetAllowedToolNamesOk

`func (o *AgentResponse) GetAllowedToolNamesOk() (*[]string, bool)`

GetAllowedToolNamesOk returns a tuple with the AllowedToolNames field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedToolNames

`func (o *AgentResponse) SetAllowedToolNames(v []string)`

SetAllowedToolNames sets AllowedToolNames field to given value.

### HasAllowedToolNames

`func (o *AgentResponse) HasAllowedToolNames() bool`

HasAllowedToolNames returns a boolean if a field has been set.

### SetAllowedToolNamesNil

`func (o *AgentResponse) SetAllowedToolNamesNil(b bool)`

 SetAllowedToolNamesNil sets the value for AllowedToolNames to be an explicit nil

### UnsetAllowedToolNames
`func (o *AgentResponse) UnsetAllowedToolNames()`

UnsetAllowedToolNames ensures that no value is present for AllowedToolNames, not even an explicit nil
### GetConfirmationMode

`func (o *AgentResponse) GetConfirmationMode() string`

GetConfirmationMode returns the ConfirmationMode field if non-nil, zero value otherwise.

### GetConfirmationModeOk

`func (o *AgentResponse) GetConfirmationModeOk() (*string, bool)`

GetConfirmationModeOk returns a tuple with the ConfirmationMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfirmationMode

`func (o *AgentResponse) SetConfirmationMode(v string)`

SetConfirmationMode sets ConfirmationMode field to given value.


### SetConfirmationModeNil

`func (o *AgentResponse) SetConfirmationModeNil(b bool)`

 SetConfirmationModeNil sets the value for ConfirmationMode to be an explicit nil

### UnsetConfirmationMode
`func (o *AgentResponse) UnsetConfirmationMode()`

UnsetConfirmationMode ensures that no value is present for ConfirmationMode, not even an explicit nil
### GetCreatedAt

`func (o *AgentResponse) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *AgentResponse) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *AgentResponse) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *AgentResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *AgentResponse) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *AgentResponse) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetExpiresAt

`func (o *AgentResponse) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *AgentResponse) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *AgentResponse) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *AgentResponse) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *AgentResponse) SetExpiresAtNil(b bool)`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *AgentResponse) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil
### GetId

`func (o *AgentResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *AgentResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *AgentResponse) SetId(v string)`

SetId sets Id field to given value.


### SetIdNil

`func (o *AgentResponse) SetIdNil(b bool)`

 SetIdNil sets the value for Id to be an explicit nil

### UnsetId
`func (o *AgentResponse) UnsetId()`

UnsetId ensures that no value is present for Id, not even an explicit nil
### GetLastUsedAt

`func (o *AgentResponse) GetLastUsedAt() string`

GetLastUsedAt returns the LastUsedAt field if non-nil, zero value otherwise.

### GetLastUsedAtOk

`func (o *AgentResponse) GetLastUsedAtOk() (*string, bool)`

GetLastUsedAtOk returns a tuple with the LastUsedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastUsedAt

`func (o *AgentResponse) SetLastUsedAt(v string)`

SetLastUsedAt sets LastUsedAt field to given value.

### HasLastUsedAt

`func (o *AgentResponse) HasLastUsedAt() bool`

HasLastUsedAt returns a boolean if a field has been set.

### SetLastUsedAtNil

`func (o *AgentResponse) SetLastUsedAtNil(b bool)`

 SetLastUsedAtNil sets the value for LastUsedAt to be an explicit nil

### UnsetLastUsedAt
`func (o *AgentResponse) UnsetLastUsedAt()`

UnsetLastUsedAt ensures that no value is present for LastUsedAt, not even an explicit nil
### GetName

`func (o *AgentResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *AgentResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *AgentResponse) SetName(v string)`

SetName sets Name field to given value.


### SetNameNil

`func (o *AgentResponse) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *AgentResponse) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetOrgId

`func (o *AgentResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *AgentResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *AgentResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### SetOrgIdNil

`func (o *AgentResponse) SetOrgIdNil(b bool)`

 SetOrgIdNil sets the value for OrgId to be an explicit nil

### UnsetOrgId
`func (o *AgentResponse) UnsetOrgId()`

UnsetOrgId ensures that no value is present for OrgId, not even an explicit nil
### GetOwnerUserId

`func (o *AgentResponse) GetOwnerUserId() string`

GetOwnerUserId returns the OwnerUserId field if non-nil, zero value otherwise.

### GetOwnerUserIdOk

`func (o *AgentResponse) GetOwnerUserIdOk() (*string, bool)`

GetOwnerUserIdOk returns a tuple with the OwnerUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOwnerUserId

`func (o *AgentResponse) SetOwnerUserId(v string)`

SetOwnerUserId sets OwnerUserId field to given value.


### SetOwnerUserIdNil

`func (o *AgentResponse) SetOwnerUserIdNil(b bool)`

 SetOwnerUserIdNil sets the value for OwnerUserId to be an explicit nil

### UnsetOwnerUserId
`func (o *AgentResponse) UnsetOwnerUserId()`

UnsetOwnerUserId ensures that no value is present for OwnerUserId, not even an explicit nil
### GetPurpose

`func (o *AgentResponse) GetPurpose() string`

GetPurpose returns the Purpose field if non-nil, zero value otherwise.

### GetPurposeOk

`func (o *AgentResponse) GetPurposeOk() (*string, bool)`

GetPurposeOk returns a tuple with the Purpose field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurpose

`func (o *AgentResponse) SetPurpose(v string)`

SetPurpose sets Purpose field to given value.

### HasPurpose

`func (o *AgentResponse) HasPurpose() bool`

HasPurpose returns a boolean if a field has been set.

### SetPurposeNil

`func (o *AgentResponse) SetPurposeNil(b bool)`

 SetPurposeNil sets the value for Purpose to be an explicit nil

### UnsetPurpose
`func (o *AgentResponse) UnsetPurpose()`

UnsetPurpose ensures that no value is present for Purpose, not even an explicit nil
### GetRevokedAt

`func (o *AgentResponse) GetRevokedAt() string`

GetRevokedAt returns the RevokedAt field if non-nil, zero value otherwise.

### GetRevokedAtOk

`func (o *AgentResponse) GetRevokedAtOk() (*string, bool)`

GetRevokedAtOk returns a tuple with the RevokedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRevokedAt

`func (o *AgentResponse) SetRevokedAt(v string)`

SetRevokedAt sets RevokedAt field to given value.

### HasRevokedAt

`func (o *AgentResponse) HasRevokedAt() bool`

HasRevokedAt returns a boolean if a field has been set.

### SetRevokedAtNil

`func (o *AgentResponse) SetRevokedAtNil(b bool)`

 SetRevokedAtNil sets the value for RevokedAt to be an explicit nil

### UnsetRevokedAt
`func (o *AgentResponse) UnsetRevokedAt()`

UnsetRevokedAt ensures that no value is present for RevokedAt, not even an explicit nil
### GetUpdatedAt

`func (o *AgentResponse) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *AgentResponse) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *AgentResponse) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *AgentResponse) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *AgentResponse) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *AgentResponse) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
