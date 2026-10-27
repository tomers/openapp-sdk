# CreateAgentCredentialRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExpiresAt** | Pointer to **NullableString** |  | [optional]
**ExpiresIn** | Pointer to **NullableString** |  | [optional]
**GroupRoles** | Pointer to **[]string** | Group roles to add to the credential&#39;s default roles: any of &#x60;groups:create&#x60;, &#x60;groups:read&#x60;, &#x60;groups:update&#x60;, &#x60;groups:delete&#x60;, &#x60;groups:list&#x60;, &#x60;groups:share&#x60;, &#x60;groups:link&#x60;. Refused (403) unless the agent&#39;s organization has an enforcing &#x60;agent_group_write&#x60; policy and the caller holds each role there. The credential can use them only in the agent&#39;s organization, and only while that policy stays enforced. | [optional]
**Name** | Pointer to **NullableString** |  | [optional]

## Methods

### NewCreateAgentCredentialRequest

`func NewCreateAgentCredentialRequest() *CreateAgentCredentialRequest`

NewCreateAgentCredentialRequest instantiates a new CreateAgentCredentialRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateAgentCredentialRequestWithDefaults

`func NewCreateAgentCredentialRequestWithDefaults() *CreateAgentCredentialRequest`

NewCreateAgentCredentialRequestWithDefaults instantiates a new CreateAgentCredentialRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExpiresAt

`func (o *CreateAgentCredentialRequest) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *CreateAgentCredentialRequest) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *CreateAgentCredentialRequest) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *CreateAgentCredentialRequest) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *CreateAgentCredentialRequest) SetExpiresAtNil(b bool)`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *CreateAgentCredentialRequest) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil
### GetExpiresIn

`func (o *CreateAgentCredentialRequest) GetExpiresIn() string`

GetExpiresIn returns the ExpiresIn field if non-nil, zero value otherwise.

### GetExpiresInOk

`func (o *CreateAgentCredentialRequest) GetExpiresInOk() (*string, bool)`

GetExpiresInOk returns a tuple with the ExpiresIn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresIn

`func (o *CreateAgentCredentialRequest) SetExpiresIn(v string)`

SetExpiresIn sets ExpiresIn field to given value.

### HasExpiresIn

`func (o *CreateAgentCredentialRequest) HasExpiresIn() bool`

HasExpiresIn returns a boolean if a field has been set.

### SetExpiresInNil

`func (o *CreateAgentCredentialRequest) SetExpiresInNil(b bool)`

 SetExpiresInNil sets the value for ExpiresIn to be an explicit nil

### UnsetExpiresIn
`func (o *CreateAgentCredentialRequest) UnsetExpiresIn()`

UnsetExpiresIn ensures that no value is present for ExpiresIn, not even an explicit nil
### GetGroupRoles

`func (o *CreateAgentCredentialRequest) GetGroupRoles() []string`

GetGroupRoles returns the GroupRoles field if non-nil, zero value otherwise.

### GetGroupRolesOk

`func (o *CreateAgentCredentialRequest) GetGroupRolesOk() (*[]string, bool)`

GetGroupRolesOk returns a tuple with the GroupRoles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGroupRoles

`func (o *CreateAgentCredentialRequest) SetGroupRoles(v []string)`

SetGroupRoles sets GroupRoles field to given value.

### HasGroupRoles

`func (o *CreateAgentCredentialRequest) HasGroupRoles() bool`

HasGroupRoles returns a boolean if a field has been set.

### SetGroupRolesNil

`func (o *CreateAgentCredentialRequest) SetGroupRolesNil(b bool)`

 SetGroupRolesNil sets the value for GroupRoles to be an explicit nil

### UnsetGroupRoles
`func (o *CreateAgentCredentialRequest) UnsetGroupRoles()`

UnsetGroupRoles ensures that no value is present for GroupRoles, not even an explicit nil
### GetName

`func (o *CreateAgentCredentialRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateAgentCredentialRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateAgentCredentialRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *CreateAgentCredentialRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *CreateAgentCredentialRequest) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *CreateAgentCredentialRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
