# RequireStepUpPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AppliesTo** | Pointer to [**[]PolicyPrincipalKind**](PolicyPrincipalKind.md) | Principal kinds the row binds; unknown and empty values are rejected. Omitted binds the type&#39;s default principals. | [optional]
**ResourceSelector** | Pointer to **map[string]interface{}** | Entities requiring confirmation: &#x60;\&quot;*\&quot;&#x60;, one entity ULID, or a non-empty array of entity ULIDs. Omitted applies to every entity. | [optional]

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
### GetResourceSelector

`func (o *RequireStepUpPolicyConfig) GetResourceSelector() map[string]interface{}`

GetResourceSelector returns the ResourceSelector field if non-nil, zero value otherwise.

### GetResourceSelectorOk

`func (o *RequireStepUpPolicyConfig) GetResourceSelectorOk() (*map[string]interface{}, bool)`

GetResourceSelectorOk returns a tuple with the ResourceSelector field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceSelector

`func (o *RequireStepUpPolicyConfig) SetResourceSelector(v map[string]interface{})`

SetResourceSelector sets ResourceSelector field to given value.

### HasResourceSelector

`func (o *RequireStepUpPolicyConfig) HasResourceSelector() bool`

HasResourceSelector returns a boolean if a field has been set.

### SetResourceSelectorNil

`func (o *RequireStepUpPolicyConfig) SetResourceSelectorNil(b bool)`

 SetResourceSelectorNil sets the value for ResourceSelector to be an explicit nil

### UnsetResourceSelector
`func (o *RequireStepUpPolicyConfig) UnsetResourceSelector()`

UnsetResourceSelector ensures that no value is present for ResourceSelector, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
