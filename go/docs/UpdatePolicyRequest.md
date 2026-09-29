# UpdatePolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Capability** | Pointer to **NullableString** |  | [optional]
**Config** | Pointer to **map[string]interface{}** | Omitted preserves the current config; explicit JSON null is rejected rather than being silently treated as an omitted field. | [optional]
**Enabled** | Pointer to **NullableBool** |  | [optional]
**Enforcement** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUpdatePolicyRequest

`func NewUpdatePolicyRequest() *UpdatePolicyRequest`

NewUpdatePolicyRequest instantiates a new UpdatePolicyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdatePolicyRequestWithDefaults

`func NewUpdatePolicyRequestWithDefaults() *UpdatePolicyRequest`

NewUpdatePolicyRequestWithDefaults instantiates a new UpdatePolicyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCapability

`func (o *UpdatePolicyRequest) GetCapability() string`

GetCapability returns the Capability field if non-nil, zero value otherwise.

### GetCapabilityOk

`func (o *UpdatePolicyRequest) GetCapabilityOk() (*string, bool)`

GetCapabilityOk returns a tuple with the Capability field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCapability

`func (o *UpdatePolicyRequest) SetCapability(v string)`

SetCapability sets Capability field to given value.

### HasCapability

`func (o *UpdatePolicyRequest) HasCapability() bool`

HasCapability returns a boolean if a field has been set.

### SetCapabilityNil

`func (o *UpdatePolicyRequest) SetCapabilityNil(b bool)`

 SetCapabilityNil sets the value for Capability to be an explicit nil

### UnsetCapability
`func (o *UpdatePolicyRequest) UnsetCapability()`

UnsetCapability ensures that no value is present for Capability, not even an explicit nil
### GetConfig

`func (o *UpdatePolicyRequest) GetConfig() map[string]interface{}`

GetConfig returns the Config field if non-nil, zero value otherwise.

### GetConfigOk

`func (o *UpdatePolicyRequest) GetConfigOk() (*map[string]interface{}, bool)`

GetConfigOk returns a tuple with the Config field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfig

`func (o *UpdatePolicyRequest) SetConfig(v map[string]interface{})`

SetConfig sets Config field to given value.

### HasConfig

`func (o *UpdatePolicyRequest) HasConfig() bool`

HasConfig returns a boolean if a field has been set.

### SetConfigNil

`func (o *UpdatePolicyRequest) SetConfigNil(b bool)`

 SetConfigNil sets the value for Config to be an explicit nil

### UnsetConfig
`func (o *UpdatePolicyRequest) UnsetConfig()`

UnsetConfig ensures that no value is present for Config, not even an explicit nil
### GetEnabled

`func (o *UpdatePolicyRequest) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *UpdatePolicyRequest) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *UpdatePolicyRequest) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.

### HasEnabled

`func (o *UpdatePolicyRequest) HasEnabled() bool`

HasEnabled returns a boolean if a field has been set.

### SetEnabledNil

`func (o *UpdatePolicyRequest) SetEnabledNil(b bool)`

 SetEnabledNil sets the value for Enabled to be an explicit nil

### UnsetEnabled
`func (o *UpdatePolicyRequest) UnsetEnabled()`

UnsetEnabled ensures that no value is present for Enabled, not even an explicit nil
### GetEnforcement

`func (o *UpdatePolicyRequest) GetEnforcement() string`

GetEnforcement returns the Enforcement field if non-nil, zero value otherwise.

### GetEnforcementOk

`func (o *UpdatePolicyRequest) GetEnforcementOk() (*string, bool)`

GetEnforcementOk returns a tuple with the Enforcement field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnforcement

`func (o *UpdatePolicyRequest) SetEnforcement(v string)`

SetEnforcement sets Enforcement field to given value.

### HasEnforcement

`func (o *UpdatePolicyRequest) HasEnforcement() bool`

HasEnforcement returns a boolean if a field has been set.

### SetEnforcementNil

`func (o *UpdatePolicyRequest) SetEnforcementNil(b bool)`

 SetEnforcementNil sets the value for Enforcement to be an explicit nil

### UnsetEnforcement
`func (o *UpdatePolicyRequest) UnsetEnforcement()`

UnsetEnforcement ensures that no value is present for Enforcement, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
