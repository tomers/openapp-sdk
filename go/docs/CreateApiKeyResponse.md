# CreateApiKeyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BaseUrl** | **string** |  |
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**ExpiresAt** | **string** |  |
**Id** | **string** |  |
**Name** | **string** |  |
**ScopedEntityIds** | Pointer to **[]string** |  | [optional]
**ScopedRoles** | Pointer to **[]string** |  | [optional]
**Token** | Pointer to **NullableString** |  | [optional]
**TokenSuffix** | **string** |  |

## Methods

### NewCreateApiKeyResponse

`func NewCreateApiKeyResponse(baseUrl string, expiresAt string, id string, name string, tokenSuffix string, ) *CreateApiKeyResponse`

NewCreateApiKeyResponse instantiates a new CreateApiKeyResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateApiKeyResponseWithDefaults

`func NewCreateApiKeyResponseWithDefaults() *CreateApiKeyResponse`

NewCreateApiKeyResponseWithDefaults instantiates a new CreateApiKeyResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBaseUrl

`func (o *CreateApiKeyResponse) GetBaseUrl() string`

GetBaseUrl returns the BaseUrl field if non-nil, zero value otherwise.

### GetBaseUrlOk

`func (o *CreateApiKeyResponse) GetBaseUrlOk() (*string, bool)`

GetBaseUrlOk returns a tuple with the BaseUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBaseUrl

`func (o *CreateApiKeyResponse) SetBaseUrl(v string)`

SetBaseUrl sets BaseUrl field to given value.


### GetCreatedAt

`func (o *CreateApiKeyResponse) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *CreateApiKeyResponse) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *CreateApiKeyResponse) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *CreateApiKeyResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *CreateApiKeyResponse) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *CreateApiKeyResponse) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetExpiresAt

`func (o *CreateApiKeyResponse) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *CreateApiKeyResponse) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *CreateApiKeyResponse) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.


### GetId

`func (o *CreateApiKeyResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *CreateApiKeyResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *CreateApiKeyResponse) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *CreateApiKeyResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateApiKeyResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateApiKeyResponse) SetName(v string)`

SetName sets Name field to given value.


### GetScopedEntityIds

`func (o *CreateApiKeyResponse) GetScopedEntityIds() []string`

GetScopedEntityIds returns the ScopedEntityIds field if non-nil, zero value otherwise.

### GetScopedEntityIdsOk

`func (o *CreateApiKeyResponse) GetScopedEntityIdsOk() (*[]string, bool)`

GetScopedEntityIdsOk returns a tuple with the ScopedEntityIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopedEntityIds

`func (o *CreateApiKeyResponse) SetScopedEntityIds(v []string)`

SetScopedEntityIds sets ScopedEntityIds field to given value.

### HasScopedEntityIds

`func (o *CreateApiKeyResponse) HasScopedEntityIds() bool`

HasScopedEntityIds returns a boolean if a field has been set.

### SetScopedEntityIdsNil

`func (o *CreateApiKeyResponse) SetScopedEntityIdsNil(b bool)`

 SetScopedEntityIdsNil sets the value for ScopedEntityIds to be an explicit nil

### UnsetScopedEntityIds
`func (o *CreateApiKeyResponse) UnsetScopedEntityIds()`

UnsetScopedEntityIds ensures that no value is present for ScopedEntityIds, not even an explicit nil
### GetScopedRoles

`func (o *CreateApiKeyResponse) GetScopedRoles() []string`

GetScopedRoles returns the ScopedRoles field if non-nil, zero value otherwise.

### GetScopedRolesOk

`func (o *CreateApiKeyResponse) GetScopedRolesOk() (*[]string, bool)`

GetScopedRolesOk returns a tuple with the ScopedRoles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopedRoles

`func (o *CreateApiKeyResponse) SetScopedRoles(v []string)`

SetScopedRoles sets ScopedRoles field to given value.

### HasScopedRoles

`func (o *CreateApiKeyResponse) HasScopedRoles() bool`

HasScopedRoles returns a boolean if a field has been set.

### SetScopedRolesNil

`func (o *CreateApiKeyResponse) SetScopedRolesNil(b bool)`

 SetScopedRolesNil sets the value for ScopedRoles to be an explicit nil

### UnsetScopedRoles
`func (o *CreateApiKeyResponse) UnsetScopedRoles()`

UnsetScopedRoles ensures that no value is present for ScopedRoles, not even an explicit nil
### GetToken

`func (o *CreateApiKeyResponse) GetToken() string`

GetToken returns the Token field if non-nil, zero value otherwise.

### GetTokenOk

`func (o *CreateApiKeyResponse) GetTokenOk() (*string, bool)`

GetTokenOk returns a tuple with the Token field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetToken

`func (o *CreateApiKeyResponse) SetToken(v string)`

SetToken sets Token field to given value.

### HasToken

`func (o *CreateApiKeyResponse) HasToken() bool`

HasToken returns a boolean if a field has been set.

### SetTokenNil

`func (o *CreateApiKeyResponse) SetTokenNil(b bool)`

 SetTokenNil sets the value for Token to be an explicit nil

### UnsetToken
`func (o *CreateApiKeyResponse) UnsetToken()`

UnsetToken ensures that no value is present for Token, not even an explicit nil
### GetTokenSuffix

`func (o *CreateApiKeyResponse) GetTokenSuffix() string`

GetTokenSuffix returns the TokenSuffix field if non-nil, zero value otherwise.

### GetTokenSuffixOk

`func (o *CreateApiKeyResponse) GetTokenSuffixOk() (*string, bool)`

GetTokenSuffixOk returns a tuple with the TokenSuffix field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTokenSuffix

`func (o *CreateApiKeyResponse) SetTokenSuffix(v string)`

SetTokenSuffix sets TokenSuffix field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
