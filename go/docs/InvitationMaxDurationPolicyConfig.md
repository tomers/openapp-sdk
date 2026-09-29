# InvitationMaxDurationPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AppliesTo** | Pointer to [**[]PolicyPrincipalKind**](PolicyPrincipalKind.md) |  | [optional]
**MaxSeconds** | **int64** |  |
**Output** | Pointer to [**NullableInvitationAllowedEntryKindsPolicyConfigOutput**](InvitationAllowedEntryKindsPolicyConfigOutput.md) |  | [optional]

## Methods

### NewInvitationMaxDurationPolicyConfig

`func NewInvitationMaxDurationPolicyConfig(maxSeconds int64, ) *InvitationMaxDurationPolicyConfig`

NewInvitationMaxDurationPolicyConfig instantiates a new InvitationMaxDurationPolicyConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInvitationMaxDurationPolicyConfigWithDefaults

`func NewInvitationMaxDurationPolicyConfigWithDefaults() *InvitationMaxDurationPolicyConfig`

NewInvitationMaxDurationPolicyConfigWithDefaults instantiates a new InvitationMaxDurationPolicyConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAppliesTo

`func (o *InvitationMaxDurationPolicyConfig) GetAppliesTo() []PolicyPrincipalKind`

GetAppliesTo returns the AppliesTo field if non-nil, zero value otherwise.

### GetAppliesToOk

`func (o *InvitationMaxDurationPolicyConfig) GetAppliesToOk() (*[]PolicyPrincipalKind, bool)`

GetAppliesToOk returns a tuple with the AppliesTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAppliesTo

`func (o *InvitationMaxDurationPolicyConfig) SetAppliesTo(v []PolicyPrincipalKind)`

SetAppliesTo sets AppliesTo field to given value.

### HasAppliesTo

`func (o *InvitationMaxDurationPolicyConfig) HasAppliesTo() bool`

HasAppliesTo returns a boolean if a field has been set.

### SetAppliesToNil

`func (o *InvitationMaxDurationPolicyConfig) SetAppliesToNil(b bool)`

 SetAppliesToNil sets the value for AppliesTo to be an explicit nil

### UnsetAppliesTo
`func (o *InvitationMaxDurationPolicyConfig) UnsetAppliesTo()`

UnsetAppliesTo ensures that no value is present for AppliesTo, not even an explicit nil
### GetMaxSeconds

`func (o *InvitationMaxDurationPolicyConfig) GetMaxSeconds() int64`

GetMaxSeconds returns the MaxSeconds field if non-nil, zero value otherwise.

### GetMaxSecondsOk

`func (o *InvitationMaxDurationPolicyConfig) GetMaxSecondsOk() (*int64, bool)`

GetMaxSecondsOk returns a tuple with the MaxSeconds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxSeconds

`func (o *InvitationMaxDurationPolicyConfig) SetMaxSeconds(v int64)`

SetMaxSeconds sets MaxSeconds field to given value.


### GetOutput

`func (o *InvitationMaxDurationPolicyConfig) GetOutput() InvitationAllowedEntryKindsPolicyConfigOutput`

GetOutput returns the Output field if non-nil, zero value otherwise.

### GetOutputOk

`func (o *InvitationMaxDurationPolicyConfig) GetOutputOk() (*InvitationAllowedEntryKindsPolicyConfigOutput, bool)`

GetOutputOk returns a tuple with the Output field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutput

`func (o *InvitationMaxDurationPolicyConfig) SetOutput(v InvitationAllowedEntryKindsPolicyConfigOutput)`

SetOutput sets Output field to given value.

### HasOutput

`func (o *InvitationMaxDurationPolicyConfig) HasOutput() bool`

HasOutput returns a boolean if a field has been set.

### SetOutputNil

`func (o *InvitationMaxDurationPolicyConfig) SetOutputNil(b bool)`

 SetOutputNil sets the value for Output to be an explicit nil

### UnsetOutput
`func (o *InvitationMaxDurationPolicyConfig) UnsetOutput()`

UnsetOutput ensures that no value is present for Output, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
