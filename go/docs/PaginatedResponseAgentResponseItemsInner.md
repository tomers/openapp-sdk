# PaginatedResponseAgentResponseItemsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AllowedEntityIds** | Pointer to **[]string** |  | [optional]
**AllowedPortalIds** | Pointer to **[]string** |  | [optional]
**AllowedToolNames** | Pointer to **[]string** |  | [optional]
**ConfirmationMode** | **string** |  |
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**ExpiresAt** | Pointer to **NullableString** |  | [optional]
**Id** | **string** |  |
**LastUsedAt** | Pointer to **NullableString** |  | [optional]
**Name** | **string** |  |
**OrgId** | **string** |  |
**OwnerUserId** | **string** |  |
**Purpose** | Pointer to **NullableString** |  | [optional]
**RevokedAt** | Pointer to **NullableString** |  | [optional]
**UpdatedAt** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPaginatedResponseAgentResponseItemsInner

`func NewPaginatedResponseAgentResponseItemsInner(confirmationMode string, id string, name string, orgId string, ownerUserId string, ) *PaginatedResponseAgentResponseItemsInner`

NewPaginatedResponseAgentResponseItemsInner instantiates a new PaginatedResponseAgentResponseItemsInner object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseAgentResponseItemsInnerWithDefaults

`func NewPaginatedResponseAgentResponseItemsInnerWithDefaults() *PaginatedResponseAgentResponseItemsInner`

NewPaginatedResponseAgentResponseItemsInnerWithDefaults instantiates a new PaginatedResponseAgentResponseItemsInner object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAllowedEntityIds

`func (o *PaginatedResponseAgentResponseItemsInner) GetAllowedEntityIds() []string`

GetAllowedEntityIds returns the AllowedEntityIds field if non-nil, zero value otherwise.

### GetAllowedEntityIdsOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetAllowedEntityIdsOk() (*[]string, bool)`

GetAllowedEntityIdsOk returns a tuple with the AllowedEntityIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedEntityIds

`func (o *PaginatedResponseAgentResponseItemsInner) SetAllowedEntityIds(v []string)`

SetAllowedEntityIds sets AllowedEntityIds field to given value.

### HasAllowedEntityIds

`func (o *PaginatedResponseAgentResponseItemsInner) HasAllowedEntityIds() bool`

HasAllowedEntityIds returns a boolean if a field has been set.

### SetAllowedEntityIdsNil

`func (o *PaginatedResponseAgentResponseItemsInner) SetAllowedEntityIdsNil(b bool)`

 SetAllowedEntityIdsNil sets the value for AllowedEntityIds to be an explicit nil

### UnsetAllowedEntityIds
`func (o *PaginatedResponseAgentResponseItemsInner) UnsetAllowedEntityIds()`

UnsetAllowedEntityIds ensures that no value is present for AllowedEntityIds, not even an explicit nil
### GetAllowedPortalIds

`func (o *PaginatedResponseAgentResponseItemsInner) GetAllowedPortalIds() []string`

GetAllowedPortalIds returns the AllowedPortalIds field if non-nil, zero value otherwise.

### GetAllowedPortalIdsOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetAllowedPortalIdsOk() (*[]string, bool)`

GetAllowedPortalIdsOk returns a tuple with the AllowedPortalIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedPortalIds

`func (o *PaginatedResponseAgentResponseItemsInner) SetAllowedPortalIds(v []string)`

SetAllowedPortalIds sets AllowedPortalIds field to given value.

### HasAllowedPortalIds

`func (o *PaginatedResponseAgentResponseItemsInner) HasAllowedPortalIds() bool`

HasAllowedPortalIds returns a boolean if a field has been set.

### SetAllowedPortalIdsNil

`func (o *PaginatedResponseAgentResponseItemsInner) SetAllowedPortalIdsNil(b bool)`

 SetAllowedPortalIdsNil sets the value for AllowedPortalIds to be an explicit nil

### UnsetAllowedPortalIds
`func (o *PaginatedResponseAgentResponseItemsInner) UnsetAllowedPortalIds()`

UnsetAllowedPortalIds ensures that no value is present for AllowedPortalIds, not even an explicit nil
### GetAllowedToolNames

`func (o *PaginatedResponseAgentResponseItemsInner) GetAllowedToolNames() []string`

GetAllowedToolNames returns the AllowedToolNames field if non-nil, zero value otherwise.

### GetAllowedToolNamesOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetAllowedToolNamesOk() (*[]string, bool)`

GetAllowedToolNamesOk returns a tuple with the AllowedToolNames field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedToolNames

`func (o *PaginatedResponseAgentResponseItemsInner) SetAllowedToolNames(v []string)`

SetAllowedToolNames sets AllowedToolNames field to given value.

### HasAllowedToolNames

`func (o *PaginatedResponseAgentResponseItemsInner) HasAllowedToolNames() bool`

HasAllowedToolNames returns a boolean if a field has been set.

### SetAllowedToolNamesNil

`func (o *PaginatedResponseAgentResponseItemsInner) SetAllowedToolNamesNil(b bool)`

 SetAllowedToolNamesNil sets the value for AllowedToolNames to be an explicit nil

### UnsetAllowedToolNames
`func (o *PaginatedResponseAgentResponseItemsInner) UnsetAllowedToolNames()`

UnsetAllowedToolNames ensures that no value is present for AllowedToolNames, not even an explicit nil
### GetConfirmationMode

`func (o *PaginatedResponseAgentResponseItemsInner) GetConfirmationMode() string`

GetConfirmationMode returns the ConfirmationMode field if non-nil, zero value otherwise.

### GetConfirmationModeOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetConfirmationModeOk() (*string, bool)`

GetConfirmationModeOk returns a tuple with the ConfirmationMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfirmationMode

`func (o *PaginatedResponseAgentResponseItemsInner) SetConfirmationMode(v string)`

SetConfirmationMode sets ConfirmationMode field to given value.


### GetCreatedAt

`func (o *PaginatedResponseAgentResponseItemsInner) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *PaginatedResponseAgentResponseItemsInner) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *PaginatedResponseAgentResponseItemsInner) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *PaginatedResponseAgentResponseItemsInner) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *PaginatedResponseAgentResponseItemsInner) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetExpiresAt

`func (o *PaginatedResponseAgentResponseItemsInner) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *PaginatedResponseAgentResponseItemsInner) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *PaginatedResponseAgentResponseItemsInner) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *PaginatedResponseAgentResponseItemsInner) SetExpiresAtNil(b bool)`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *PaginatedResponseAgentResponseItemsInner) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil
### GetId

`func (o *PaginatedResponseAgentResponseItemsInner) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PaginatedResponseAgentResponseItemsInner) SetId(v string)`

SetId sets Id field to given value.


### GetLastUsedAt

`func (o *PaginatedResponseAgentResponseItemsInner) GetLastUsedAt() string`

GetLastUsedAt returns the LastUsedAt field if non-nil, zero value otherwise.

### GetLastUsedAtOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetLastUsedAtOk() (*string, bool)`

GetLastUsedAtOk returns a tuple with the LastUsedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastUsedAt

`func (o *PaginatedResponseAgentResponseItemsInner) SetLastUsedAt(v string)`

SetLastUsedAt sets LastUsedAt field to given value.

### HasLastUsedAt

`func (o *PaginatedResponseAgentResponseItemsInner) HasLastUsedAt() bool`

HasLastUsedAt returns a boolean if a field has been set.

### SetLastUsedAtNil

`func (o *PaginatedResponseAgentResponseItemsInner) SetLastUsedAtNil(b bool)`

 SetLastUsedAtNil sets the value for LastUsedAt to be an explicit nil

### UnsetLastUsedAt
`func (o *PaginatedResponseAgentResponseItemsInner) UnsetLastUsedAt()`

UnsetLastUsedAt ensures that no value is present for LastUsedAt, not even an explicit nil
### GetName

`func (o *PaginatedResponseAgentResponseItemsInner) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PaginatedResponseAgentResponseItemsInner) SetName(v string)`

SetName sets Name field to given value.


### GetOrgId

`func (o *PaginatedResponseAgentResponseItemsInner) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *PaginatedResponseAgentResponseItemsInner) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetOwnerUserId

`func (o *PaginatedResponseAgentResponseItemsInner) GetOwnerUserId() string`

GetOwnerUserId returns the OwnerUserId field if non-nil, zero value otherwise.

### GetOwnerUserIdOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetOwnerUserIdOk() (*string, bool)`

GetOwnerUserIdOk returns a tuple with the OwnerUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOwnerUserId

`func (o *PaginatedResponseAgentResponseItemsInner) SetOwnerUserId(v string)`

SetOwnerUserId sets OwnerUserId field to given value.


### GetPurpose

`func (o *PaginatedResponseAgentResponseItemsInner) GetPurpose() string`

GetPurpose returns the Purpose field if non-nil, zero value otherwise.

### GetPurposeOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetPurposeOk() (*string, bool)`

GetPurposeOk returns a tuple with the Purpose field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurpose

`func (o *PaginatedResponseAgentResponseItemsInner) SetPurpose(v string)`

SetPurpose sets Purpose field to given value.

### HasPurpose

`func (o *PaginatedResponseAgentResponseItemsInner) HasPurpose() bool`

HasPurpose returns a boolean if a field has been set.

### SetPurposeNil

`func (o *PaginatedResponseAgentResponseItemsInner) SetPurposeNil(b bool)`

 SetPurposeNil sets the value for Purpose to be an explicit nil

### UnsetPurpose
`func (o *PaginatedResponseAgentResponseItemsInner) UnsetPurpose()`

UnsetPurpose ensures that no value is present for Purpose, not even an explicit nil
### GetRevokedAt

`func (o *PaginatedResponseAgentResponseItemsInner) GetRevokedAt() string`

GetRevokedAt returns the RevokedAt field if non-nil, zero value otherwise.

### GetRevokedAtOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetRevokedAtOk() (*string, bool)`

GetRevokedAtOk returns a tuple with the RevokedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRevokedAt

`func (o *PaginatedResponseAgentResponseItemsInner) SetRevokedAt(v string)`

SetRevokedAt sets RevokedAt field to given value.

### HasRevokedAt

`func (o *PaginatedResponseAgentResponseItemsInner) HasRevokedAt() bool`

HasRevokedAt returns a boolean if a field has been set.

### SetRevokedAtNil

`func (o *PaginatedResponseAgentResponseItemsInner) SetRevokedAtNil(b bool)`

 SetRevokedAtNil sets the value for RevokedAt to be an explicit nil

### UnsetRevokedAt
`func (o *PaginatedResponseAgentResponseItemsInner) UnsetRevokedAt()`

UnsetRevokedAt ensures that no value is present for RevokedAt, not even an explicit nil
### GetUpdatedAt

`func (o *PaginatedResponseAgentResponseItemsInner) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *PaginatedResponseAgentResponseItemsInner) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *PaginatedResponseAgentResponseItemsInner) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *PaginatedResponseAgentResponseItemsInner) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *PaginatedResponseAgentResponseItemsInner) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *PaginatedResponseAgentResponseItemsInner) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
