# RequireStepUpPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AppliesTo** | Pointer to [**[]PolicyPrincipalKind**](PolicyPrincipalKind.md) | Principal kinds requiring confirmation; unknown and empty values are rejected. | [optional]
**Output** | Pointer to [**NullablePolicyOutputSelector**](PolicyOutputSelector.md) | Optional provider-derived output/channel selector. | [optional]

## Methods

### NewRequireStepUpPolicyConfig

`func NewRequireStepUpPolicyConfig() *RequireStepUpPolicyConfig`

NewRequireStepUpPolicyConfig instantiates a new RequireStepUpPolicyConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewRequireStepUpPolicyConfigWithDefaults

`func NewRequireStepUpPolicyConfigWithDefaults() *RequireStepUpPolicyConfig`

NewRequireStepUpPolicyConfigWithDefaults instantiates a new RequireStepUpPolicyConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAppliesTo

`func (o *RequireStepUpPolicyConfig) GetAppliesTo() []PolicyPrincipalKind`

GetAppliesTo returns the AppliesTo field if non-nil, zero value otherwise.

### GetAppliesToOk

`func (o *RequireStepUpPolicyConfig) GetAppliesToOk() (*[]PolicyPrincipalKind, bool)`

GetAppliesToOk returns a tuple with the AppliesTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAppliesTo

`func (o *RequireStepUpPolicyConfig) SetAppliesTo(v []PolicyPrincipalKind)`

SetAppliesTo sets AppliesTo field to given value.

### HasAppliesTo

`func (o *RequireStepUpPolicyConfig) HasAppliesTo() bool`

HasAppliesTo returns a boolean if a field has been set.

### SetAppliesToNil

`func (o *RequireStepUpPolicyConfig) SetAppliesToNil(b bool)`

 SetAppliesToNil sets the value for AppliesTo to be an explicit nil

### UnsetAppliesTo
`func (o *RequireStepUpPolicyConfig) UnsetAppliesTo()`

UnsetAppliesTo ensures that no value is present for AppliesTo, not even an explicit nil
### GetOutput

`func (o *RequireStepUpPolicyConfig) GetOutput() PolicyOutputSelector`

GetOutput returns the Output field if non-nil, zero value otherwise.

### GetOutputOk

`func (o *RequireStepUpPolicyConfig) GetOutputOk() (*PolicyOutputSelector, bool)`

GetOutputOk returns a tuple with the Output field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutput

`func (o *RequireStepUpPolicyConfig) SetOutput(v PolicyOutputSelector)`

SetOutput sets Output field to given value.

### HasOutput

`func (o *RequireStepUpPolicyConfig) HasOutput() bool`

HasOutput returns a boolean if a field has been set.

### SetOutputNil

`func (o *RequireStepUpPolicyConfig) SetOutputNil(b bool)`

 SetOutputNil sets the value for Output to be an explicit nil

### UnsetOutput
`func (o *RequireStepUpPolicyConfig) UnsetOutput()`

UnsetOutput ensures that no value is present for Output, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
