# CreateApiKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExpiresAt** | Pointer to **NullableString** | RFC3339 absolute expiration timestamp. Mutually exclusive with expires_in. | [optional]
**ExpiresIn** | Pointer to **NullableString** | Duration from now, e.g. \&quot;1d\&quot;, \&quot;2w\&quot;, \&quot;90d\&quot;, \&quot;P30D\&quot;. Mutually exclusive with expires_at. | [optional]
**Name** | **string** |  |
**ScopedEntityIds** | Pointer to **[]string** |  | [optional]
**ScopedRoles** | Pointer to **[]string** |  | [optional]

## Methods

### NewCreateApiKeyRequest

`func NewCreateApiKeyRequest(name string, ) *CreateApiKeyRequest`

NewCreateApiKeyRequest instantiates a new CreateApiKeyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateApiKeyRequestWithDefaults

`func NewCreateApiKeyRequestWithDefaults() *CreateApiKeyRequest`

NewCreateApiKeyRequestWithDefaults instantiates a new CreateApiKeyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExpiresAt

`func (o *CreateApiKeyRequest) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *CreateApiKeyRequest) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *CreateApiKeyRequest) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *CreateApiKeyRequest) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *CreateApiKeyRequest) SetExpiresAtNil(b bool)`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *CreateApiKeyRequest) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil
### GetExpiresIn

`func (o *CreateApiKeyRequest) GetExpiresIn() string`

GetExpiresIn returns the ExpiresIn field if non-nil, zero value otherwise.

### GetExpiresInOk

`func (o *CreateApiKeyRequest) GetExpiresInOk() (*string, bool)`

GetExpiresInOk returns a tuple with the ExpiresIn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresIn

`func (o *CreateApiKeyRequest) SetExpiresIn(v string)`

SetExpiresIn sets ExpiresIn field to given value.

### HasExpiresIn

`func (o *CreateApiKeyRequest) HasExpiresIn() bool`

HasExpiresIn returns a boolean if a field has been set.

### SetExpiresInNil

`func (o *CreateApiKeyRequest) SetExpiresInNil(b bool)`

 SetExpiresInNil sets the value for ExpiresIn to be an explicit nil

### UnsetExpiresIn
`func (o *CreateApiKeyRequest) UnsetExpiresIn()`

UnsetExpiresIn ensures that no value is present for ExpiresIn, not even an explicit nil
### GetName

`func (o *CreateApiKeyRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateApiKeyRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateApiKeyRequest) SetName(v string)`

SetName sets Name field to given value.


### GetScopedEntityIds

`func (o *CreateApiKeyRequest) GetScopedEntityIds() []string`

GetScopedEntityIds returns the ScopedEntityIds field if non-nil, zero value otherwise.

### GetScopedEntityIdsOk

`func (o *CreateApiKeyRequest) GetScopedEntityIdsOk() (*[]string, bool)`

GetScopedEntityIdsOk returns a tuple with the ScopedEntityIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopedEntityIds

`func (o *CreateApiKeyRequest) SetScopedEntityIds(v []string)`

SetScopedEntityIds sets ScopedEntityIds field to given value.

### HasScopedEntityIds

`func (o *CreateApiKeyRequest) HasScopedEntityIds() bool`

HasScopedEntityIds returns a boolean if a field has been set.

### SetScopedEntityIdsNil

`func (o *CreateApiKeyRequest) SetScopedEntityIdsNil(b bool)`

 SetScopedEntityIdsNil sets the value for ScopedEntityIds to be an explicit nil

### UnsetScopedEntityIds
`func (o *CreateApiKeyRequest) UnsetScopedEntityIds()`

UnsetScopedEntityIds ensures that no value is present for ScopedEntityIds, not even an explicit nil
### GetScopedRoles

`func (o *CreateApiKeyRequest) GetScopedRoles() []string`

GetScopedRoles returns the ScopedRoles field if non-nil, zero value otherwise.

### GetScopedRolesOk

`func (o *CreateApiKeyRequest) GetScopedRolesOk() (*[]string, bool)`

GetScopedRolesOk returns a tuple with the ScopedRoles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopedRoles

`func (o *CreateApiKeyRequest) SetScopedRoles(v []string)`

SetScopedRoles sets ScopedRoles field to given value.

### HasScopedRoles

`func (o *CreateApiKeyRequest) HasScopedRoles() bool`

HasScopedRoles returns a boolean if a field has been set.

### SetScopedRolesNil

`func (o *CreateApiKeyRequest) SetScopedRolesNil(b bool)`

 SetScopedRolesNil sets the value for ScopedRoles to be an explicit nil

### UnsetScopedRoles
`func (o *CreateApiKeyRequest) UnsetScopedRoles()`

UnsetScopedRoles ensures that no value is present for ScopedRoles, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
