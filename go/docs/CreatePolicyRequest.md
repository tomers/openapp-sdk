# CreatePolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Capability** | Pointer to **NullableString** | Optional capability scoping (e.g. &#x60;switchable&#x60;, &#x60;users_admin&#x60;). | [optional]
**Config** | **map[string]interface{}** | Typed-per-&#x60;policy_type&#x60; configuration document. For the documented typed shapes, see &#x60;RequireStepUpPolicyConfig&#x60; and &#x60;ApprovalThresholdPolicyConfig&#x60;. |
**Enabled** | Pointer to **NullableBool** | Defaults to true. | [optional]
**Enforcement** | Pointer to **NullableString** | &#x60;enforce&#x60; (default) | &#x60;require_approval&#x60; | &#x60;audit_only&#x60;. | [optional]
**PolicyType** | **string** | e.g. &#x60;invitation_curfew&#x60;, &#x60;user_sharing&#x60;. |

## Methods

### NewCreatePolicyRequest

`func NewCreatePolicyRequest(config map[string]interface{}, policyType string, ) *CreatePolicyRequest`

NewCreatePolicyRequest instantiates a new CreatePolicyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreatePolicyRequestWithDefaults

`func NewCreatePolicyRequestWithDefaults() *CreatePolicyRequest`

NewCreatePolicyRequestWithDefaults instantiates a new CreatePolicyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCapability

`func (o *CreatePolicyRequest) GetCapability() string`

GetCapability returns the Capability field if non-nil, zero value otherwise.

### GetCapabilityOk

`func (o *CreatePolicyRequest) GetCapabilityOk() (*string, bool)`

GetCapabilityOk returns a tuple with the Capability field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCapability

`func (o *CreatePolicyRequest) SetCapability(v string)`

SetCapability sets Capability field to given value.

### HasCapability

`func (o *CreatePolicyRequest) HasCapability() bool`

HasCapability returns a boolean if a field has been set.

### SetCapabilityNil

`func (o *CreatePolicyRequest) SetCapabilityNil(b bool)`

 SetCapabilityNil sets the value for Capability to be an explicit nil

### UnsetCapability
`func (o *CreatePolicyRequest) UnsetCapability()`

UnsetCapability ensures that no value is present for Capability, not even an explicit nil
### GetConfig

`func (o *CreatePolicyRequest) GetConfig() map[string]interface{}`

GetConfig returns the Config field if non-nil, zero value otherwise.

### GetConfigOk

`func (o *CreatePolicyRequest) GetConfigOk() (*map[string]interface{}, bool)`

GetConfigOk returns a tuple with the Config field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfig

`func (o *CreatePolicyRequest) SetConfig(v map[string]interface{})`

SetConfig sets Config field to given value.


### GetEnabled

`func (o *CreatePolicyRequest) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *CreatePolicyRequest) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *CreatePolicyRequest) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.

### HasEnabled

`func (o *CreatePolicyRequest) HasEnabled() bool`

HasEnabled returns a boolean if a field has been set.

### SetEnabledNil

`func (o *CreatePolicyRequest) SetEnabledNil(b bool)`

 SetEnabledNil sets the value for Enabled to be an explicit nil

### UnsetEnabled
`func (o *CreatePolicyRequest) UnsetEnabled()`

UnsetEnabled ensures that no value is present for Enabled, not even an explicit nil
### GetEnforcement

`func (o *CreatePolicyRequest) GetEnforcement() string`

GetEnforcement returns the Enforcement field if non-nil, zero value otherwise.

### GetEnforcementOk

`func (o *CreatePolicyRequest) GetEnforcementOk() (*string, bool)`

GetEnforcementOk returns a tuple with the Enforcement field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnforcement

`func (o *CreatePolicyRequest) SetEnforcement(v string)`

SetEnforcement sets Enforcement field to given value.

### HasEnforcement

`func (o *CreatePolicyRequest) HasEnforcement() bool`

HasEnforcement returns a boolean if a field has been set.

### SetEnforcementNil

`func (o *CreatePolicyRequest) SetEnforcementNil(b bool)`

 SetEnforcementNil sets the value for Enforcement to be an explicit nil

### UnsetEnforcement
`func (o *CreatePolicyRequest) UnsetEnforcement()`

UnsetEnforcement ensures that no value is present for Enforcement, not even an explicit nil
### GetPolicyType

`func (o *CreatePolicyRequest) GetPolicyType() string`

GetPolicyType returns the PolicyType field if non-nil, zero value otherwise.

### GetPolicyTypeOk

`func (o *CreatePolicyRequest) GetPolicyTypeOk() (*string, bool)`

GetPolicyTypeOk returns a tuple with the PolicyType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyType

`func (o *CreatePolicyRequest) SetPolicyType(v string)`

SetPolicyType sets PolicyType field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
