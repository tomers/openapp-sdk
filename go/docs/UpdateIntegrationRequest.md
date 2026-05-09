# UpdateIntegrationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Config** | Pointer to **interface{}** |  | [optional]
**Enabled** | Pointer to **NullableBool** |  | [optional]
**Name** | Pointer to [**NullableLocalizedString**](LocalizedString.md) | Human-friendly name for this integration. Accepts string or LocalizedString map, e.g. { \&quot;en\&quot;: \&quot;Name\&quot;, \&quot;he\&quot;: \&quot;שם\&quot; }. | [optional]
**ProviderType** | Pointer to **NullableString** |  | [optional]
**Secrets** | Pointer to **interface{}** |  | [optional]

## Methods

### NewUpdateIntegrationRequest

`func NewUpdateIntegrationRequest() *UpdateIntegrationRequest`

NewUpdateIntegrationRequest instantiates a new UpdateIntegrationRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateIntegrationRequestWithDefaults

`func NewUpdateIntegrationRequestWithDefaults() *UpdateIntegrationRequest`

NewUpdateIntegrationRequestWithDefaults instantiates a new UpdateIntegrationRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetConfig

`func (o *UpdateIntegrationRequest) GetConfig() interface{}`

GetConfig returns the Config field if non-nil, zero value otherwise.

### GetConfigOk

`func (o *UpdateIntegrationRequest) GetConfigOk() (*interface{}, bool)`

GetConfigOk returns a tuple with the Config field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfig

`func (o *UpdateIntegrationRequest) SetConfig(v interface{})`

SetConfig sets Config field to given value.

### HasConfig

`func (o *UpdateIntegrationRequest) HasConfig() bool`

HasConfig returns a boolean if a field has been set.

### SetConfigNil

`func (o *UpdateIntegrationRequest) SetConfigNil(b bool)`

 SetConfigNil sets the value for Config to be an explicit nil

### UnsetConfig
`func (o *UpdateIntegrationRequest) UnsetConfig()`

UnsetConfig ensures that no value is present for Config, not even an explicit nil
### GetEnabled

`func (o *UpdateIntegrationRequest) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *UpdateIntegrationRequest) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *UpdateIntegrationRequest) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.

### HasEnabled

`func (o *UpdateIntegrationRequest) HasEnabled() bool`

HasEnabled returns a boolean if a field has been set.

### SetEnabledNil

`func (o *UpdateIntegrationRequest) SetEnabledNil(b bool)`

 SetEnabledNil sets the value for Enabled to be an explicit nil

### UnsetEnabled
`func (o *UpdateIntegrationRequest) UnsetEnabled()`

UnsetEnabled ensures that no value is present for Enabled, not even an explicit nil
### GetName

`func (o *UpdateIntegrationRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UpdateIntegrationRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UpdateIntegrationRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.

### HasName

`func (o *UpdateIntegrationRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *UpdateIntegrationRequest) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *UpdateIntegrationRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetProviderType

`func (o *UpdateIntegrationRequest) GetProviderType() string`

GetProviderType returns the ProviderType field if non-nil, zero value otherwise.

### GetProviderTypeOk

`func (o *UpdateIntegrationRequest) GetProviderTypeOk() (*string, bool)`

GetProviderTypeOk returns a tuple with the ProviderType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderType

`func (o *UpdateIntegrationRequest) SetProviderType(v string)`

SetProviderType sets ProviderType field to given value.

### HasProviderType

`func (o *UpdateIntegrationRequest) HasProviderType() bool`

HasProviderType returns a boolean if a field has been set.

### SetProviderTypeNil

`func (o *UpdateIntegrationRequest) SetProviderTypeNil(b bool)`

 SetProviderTypeNil sets the value for ProviderType to be an explicit nil

### UnsetProviderType
`func (o *UpdateIntegrationRequest) UnsetProviderType()`

UnsetProviderType ensures that no value is present for ProviderType, not even an explicit nil
### GetSecrets

`func (o *UpdateIntegrationRequest) GetSecrets() interface{}`

GetSecrets returns the Secrets field if non-nil, zero value otherwise.

### GetSecretsOk

`func (o *UpdateIntegrationRequest) GetSecretsOk() (*interface{}, bool)`

GetSecretsOk returns a tuple with the Secrets field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSecrets

`func (o *UpdateIntegrationRequest) SetSecrets(v interface{})`

SetSecrets sets Secrets field to given value.

### HasSecrets

`func (o *UpdateIntegrationRequest) HasSecrets() bool`

HasSecrets returns a boolean if a field has been set.

### SetSecretsNil

`func (o *UpdateIntegrationRequest) SetSecretsNil(b bool)`

 SetSecretsNil sets the value for Secrets to be an explicit nil

### UnsetSecrets
`func (o *UpdateIntegrationRequest) UnsetSecrets()`

UnsetSecrets ensures that no value is present for Secrets, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
