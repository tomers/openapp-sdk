# ApiKeyListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BaseUrl** | **string** |  |
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**ExpiresAt** | **string** |  |
**Id** | **string** |  |
**LastUsedAt** | Pointer to **NullableString** |  | [optional]
**Name** | **string** |  |
**RevokedAt** | Pointer to **NullableString** |  | [optional]
**ScopedEntityIds** | Pointer to **[]string** |  | [optional]
**ScopedRoles** | Pointer to **[]string** |  | [optional]
**TokenSuffix** | **string** |  |
**UpdatedAt** | Pointer to **NullableString** |  | [optional]

## Methods

### NewApiKeyListItem

`func NewApiKeyListItem(baseUrl string, expiresAt string, id string, name string, tokenSuffix string, ) *ApiKeyListItem`

NewApiKeyListItem instantiates a new ApiKeyListItem object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiKeyListItemWithDefaults

`func NewApiKeyListItemWithDefaults() *ApiKeyListItem`

NewApiKeyListItemWithDefaults instantiates a new ApiKeyListItem object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBaseUrl

`func (o *ApiKeyListItem) GetBaseUrl() string`

GetBaseUrl returns the BaseUrl field if non-nil, zero value otherwise.

### GetBaseUrlOk

`func (o *ApiKeyListItem) GetBaseUrlOk() (*string, bool)`

GetBaseUrlOk returns a tuple with the BaseUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBaseUrl

`func (o *ApiKeyListItem) SetBaseUrl(v string)`

SetBaseUrl sets BaseUrl field to given value.


### GetCreatedAt

`func (o *ApiKeyListItem) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ApiKeyListItem) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ApiKeyListItem) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *ApiKeyListItem) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *ApiKeyListItem) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *ApiKeyListItem) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetExpiresAt

`func (o *ApiKeyListItem) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *ApiKeyListItem) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *ApiKeyListItem) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.


### GetId

`func (o *ApiKeyListItem) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApiKeyListItem) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApiKeyListItem) SetId(v string)`

SetId sets Id field to given value.


### GetLastUsedAt

`func (o *ApiKeyListItem) GetLastUsedAt() string`

GetLastUsedAt returns the LastUsedAt field if non-nil, zero value otherwise.

### GetLastUsedAtOk

`func (o *ApiKeyListItem) GetLastUsedAtOk() (*string, bool)`

GetLastUsedAtOk returns a tuple with the LastUsedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastUsedAt

`func (o *ApiKeyListItem) SetLastUsedAt(v string)`

SetLastUsedAt sets LastUsedAt field to given value.

### HasLastUsedAt

`func (o *ApiKeyListItem) HasLastUsedAt() bool`

HasLastUsedAt returns a boolean if a field has been set.

### SetLastUsedAtNil

`func (o *ApiKeyListItem) SetLastUsedAtNil(b bool)`

 SetLastUsedAtNil sets the value for LastUsedAt to be an explicit nil

### UnsetLastUsedAt
`func (o *ApiKeyListItem) UnsetLastUsedAt()`

UnsetLastUsedAt ensures that no value is present for LastUsedAt, not even an explicit nil
### GetName

`func (o *ApiKeyListItem) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiKeyListItem) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiKeyListItem) SetName(v string)`

SetName sets Name field to given value.


### GetRevokedAt

`func (o *ApiKeyListItem) GetRevokedAt() string`

GetRevokedAt returns the RevokedAt field if non-nil, zero value otherwise.

### GetRevokedAtOk

`func (o *ApiKeyListItem) GetRevokedAtOk() (*string, bool)`

GetRevokedAtOk returns a tuple with the RevokedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRevokedAt

`func (o *ApiKeyListItem) SetRevokedAt(v string)`

SetRevokedAt sets RevokedAt field to given value.

### HasRevokedAt

`func (o *ApiKeyListItem) HasRevokedAt() bool`

HasRevokedAt returns a boolean if a field has been set.

### SetRevokedAtNil

`func (o *ApiKeyListItem) SetRevokedAtNil(b bool)`

 SetRevokedAtNil sets the value for RevokedAt to be an explicit nil

### UnsetRevokedAt
`func (o *ApiKeyListItem) UnsetRevokedAt()`

UnsetRevokedAt ensures that no value is present for RevokedAt, not even an explicit nil
### GetScopedEntityIds

`func (o *ApiKeyListItem) GetScopedEntityIds() []string`

GetScopedEntityIds returns the ScopedEntityIds field if non-nil, zero value otherwise.

### GetScopedEntityIdsOk

`func (o *ApiKeyListItem) GetScopedEntityIdsOk() (*[]string, bool)`

GetScopedEntityIdsOk returns a tuple with the ScopedEntityIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopedEntityIds

`func (o *ApiKeyListItem) SetScopedEntityIds(v []string)`

SetScopedEntityIds sets ScopedEntityIds field to given value.

### HasScopedEntityIds

`func (o *ApiKeyListItem) HasScopedEntityIds() bool`

HasScopedEntityIds returns a boolean if a field has been set.

### SetScopedEntityIdsNil

`func (o *ApiKeyListItem) SetScopedEntityIdsNil(b bool)`

 SetScopedEntityIdsNil sets the value for ScopedEntityIds to be an explicit nil

### UnsetScopedEntityIds
`func (o *ApiKeyListItem) UnsetScopedEntityIds()`

UnsetScopedEntityIds ensures that no value is present for ScopedEntityIds, not even an explicit nil
### GetScopedRoles

`func (o *ApiKeyListItem) GetScopedRoles() []string`

GetScopedRoles returns the ScopedRoles field if non-nil, zero value otherwise.

### GetScopedRolesOk

`func (o *ApiKeyListItem) GetScopedRolesOk() (*[]string, bool)`

GetScopedRolesOk returns a tuple with the ScopedRoles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopedRoles

`func (o *ApiKeyListItem) SetScopedRoles(v []string)`

SetScopedRoles sets ScopedRoles field to given value.

### HasScopedRoles

`func (o *ApiKeyListItem) HasScopedRoles() bool`

HasScopedRoles returns a boolean if a field has been set.

### SetScopedRolesNil

`func (o *ApiKeyListItem) SetScopedRolesNil(b bool)`

 SetScopedRolesNil sets the value for ScopedRoles to be an explicit nil

### UnsetScopedRoles
`func (o *ApiKeyListItem) UnsetScopedRoles()`

UnsetScopedRoles ensures that no value is present for ScopedRoles, not even an explicit nil
### GetTokenSuffix

`func (o *ApiKeyListItem) GetTokenSuffix() string`

GetTokenSuffix returns the TokenSuffix field if non-nil, zero value otherwise.

### GetTokenSuffixOk

`func (o *ApiKeyListItem) GetTokenSuffixOk() (*string, bool)`

GetTokenSuffixOk returns a tuple with the TokenSuffix field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTokenSuffix

`func (o *ApiKeyListItem) SetTokenSuffix(v string)`

SetTokenSuffix sets TokenSuffix field to given value.


### GetUpdatedAt

`func (o *ApiKeyListItem) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *ApiKeyListItem) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *ApiKeyListItem) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *ApiKeyListItem) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *ApiKeyListItem) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *ApiKeyListItem) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
