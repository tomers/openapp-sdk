# CreateIntegrationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Config** | Pointer to **interface{}** |  | [optional]
**Enabled** | Pointer to **bool** |  | [optional]
**Name** | Pointer to [**NullableLocalizedString**](LocalizedString.md) | Human-friendly name for this integration (optional; defaults server-side). Accepts string or LocalizedString map, e.g. { \&quot;en\&quot;: \&quot;Name\&quot;, \&quot;he\&quot;: \&quot;שם\&quot; }. | [optional]
**OrgId** | **string** |  |
**ProviderType** | **string** |  |
**Secrets** | Pointer to **interface{}** |  | [optional]

## Methods

### NewCreateIntegrationRequest

`func NewCreateIntegrationRequest(orgId string, providerType string, ) *CreateIntegrationRequest`

NewCreateIntegrationRequest instantiates a new CreateIntegrationRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateIntegrationRequestWithDefaults

`func NewCreateIntegrationRequestWithDefaults() *CreateIntegrationRequest`

NewCreateIntegrationRequestWithDefaults instantiates a new CreateIntegrationRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetConfig

`func (o *CreateIntegrationRequest) GetConfig() interface{}`

GetConfig returns the Config field if non-nil, zero value otherwise.

### GetConfigOk

`func (o *CreateIntegrationRequest) GetConfigOk() (*interface{}, bool)`

GetConfigOk returns a tuple with the Config field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfig

`func (o *CreateIntegrationRequest) SetConfig(v interface{})`

SetConfig sets Config field to given value.

### HasConfig

`func (o *CreateIntegrationRequest) HasConfig() bool`

HasConfig returns a boolean if a field has been set.

### SetConfigNil

`func (o *CreateIntegrationRequest) SetConfigNil(b bool)`

 SetConfigNil sets the value for Config to be an explicit nil

### UnsetConfig
`func (o *CreateIntegrationRequest) UnsetConfig()`

UnsetConfig ensures that no value is present for Config, not even an explicit nil
### GetEnabled

`func (o *CreateIntegrationRequest) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *CreateIntegrationRequest) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *CreateIntegrationRequest) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.

### HasEnabled

`func (o *CreateIntegrationRequest) HasEnabled() bool`

HasEnabled returns a boolean if a field has been set.

### GetName

`func (o *CreateIntegrationRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateIntegrationRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateIntegrationRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.

### HasName

`func (o *CreateIntegrationRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *CreateIntegrationRequest) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *CreateIntegrationRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetOrgId

`func (o *CreateIntegrationRequest) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *CreateIntegrationRequest) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *CreateIntegrationRequest) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetProviderType

`func (o *CreateIntegrationRequest) GetProviderType() string`

GetProviderType returns the ProviderType field if non-nil, zero value otherwise.

### GetProviderTypeOk

`func (o *CreateIntegrationRequest) GetProviderTypeOk() (*string, bool)`

GetProviderTypeOk returns a tuple with the ProviderType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderType

`func (o *CreateIntegrationRequest) SetProviderType(v string)`

SetProviderType sets ProviderType field to given value.


### GetSecrets

`func (o *CreateIntegrationRequest) GetSecrets() interface{}`

GetSecrets returns the Secrets field if non-nil, zero value otherwise.

### GetSecretsOk

`func (o *CreateIntegrationRequest) GetSecretsOk() (*interface{}, bool)`

GetSecretsOk returns a tuple with the Secrets field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSecrets

`func (o *CreateIntegrationRequest) SetSecrets(v interface{})`

SetSecrets sets Secrets field to given value.

### HasSecrets

`func (o *CreateIntegrationRequest) HasSecrets() bool`

HasSecrets returns a boolean if a field has been set.

### SetSecretsNil

`func (o *CreateIntegrationRequest) SetSecretsNil(b bool)`

 SetSecretsNil sets the value for Secrets to be an explicit nil

### UnsetSecrets
`func (o *CreateIntegrationRequest) UnsetSecrets()`

UnsetSecrets ensures that no value is present for Secrets, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
