# ApprovalThresholdPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Count** | **int32** | Number of distinct organization-admin approve votes required. |
**Output** | Pointer to [**NullablePolicyOutputSelector**](PolicyOutputSelector.md) | Optional provider-derived output/channel selector. | [optional]
**Scope** | [**ApprovalThresholdScope**](ApprovalThresholdScope.md) | Only &#x60;org_admins&#x60; is currently supported. |

## Methods

### NewApprovalThresholdPolicyConfig

`func NewApprovalThresholdPolicyConfig(count int32, scope ApprovalThresholdScope, ) *ApprovalThresholdPolicyConfig`

NewApprovalThresholdPolicyConfig instantiates a new ApprovalThresholdPolicyConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApprovalThresholdPolicyConfigWithDefaults

`func NewApprovalThresholdPolicyConfigWithDefaults() *ApprovalThresholdPolicyConfig`

NewApprovalThresholdPolicyConfigWithDefaults instantiates a new ApprovalThresholdPolicyConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCount

`func (o *ApprovalThresholdPolicyConfig) GetCount() int32`

GetCount returns the Count field if non-nil, zero value otherwise.

### GetCountOk

`func (o *ApprovalThresholdPolicyConfig) GetCountOk() (*int32, bool)`

GetCountOk returns a tuple with the Count field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCount

`func (o *ApprovalThresholdPolicyConfig) SetCount(v int32)`

SetCount sets Count field to given value.


### GetOutput

`func (o *ApprovalThresholdPolicyConfig) GetOutput() PolicyOutputSelector`

GetOutput returns the Output field if non-nil, zero value otherwise.

### GetOutputOk

`func (o *ApprovalThresholdPolicyConfig) GetOutputOk() (*PolicyOutputSelector, bool)`

GetOutputOk returns a tuple with the Output field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutput

`func (o *ApprovalThresholdPolicyConfig) SetOutput(v PolicyOutputSelector)`

SetOutput sets Output field to given value.

### HasOutput

`func (o *ApprovalThresholdPolicyConfig) HasOutput() bool`

HasOutput returns a boolean if a field has been set.

### SetOutputNil

`func (o *ApprovalThresholdPolicyConfig) SetOutputNil(b bool)`

 SetOutputNil sets the value for Output to be an explicit nil

### UnsetOutput
`func (o *ApprovalThresholdPolicyConfig) UnsetOutput()`

UnsetOutput ensures that no value is present for Output, not even an explicit nil
### GetScope

`func (o *ApprovalThresholdPolicyConfig) GetScope() ApprovalThresholdScope`

GetScope returns the Scope field if non-nil, zero value otherwise.

### GetScopeOk

`func (o *ApprovalThresholdPolicyConfig) GetScopeOk() (*ApprovalThresholdScope, bool)`

GetScopeOk returns a tuple with the Scope field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScope

`func (o *ApprovalThresholdPolicyConfig) SetScope(v ApprovalThresholdScope)`

SetScope sets Scope field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
