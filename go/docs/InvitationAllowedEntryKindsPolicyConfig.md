# InvitationAllowedEntryKindsPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AppliesTo** | Pointer to [**[]PolicyPrincipalKind**](PolicyPrincipalKind.md) |  | [optional]
**Kinds** | [**[]InvitationEntryKind**](InvitationEntryKind.md) |  |
**Output** | Pointer to [**NullableInvitationAllowedEntryKindsPolicyConfigOutput**](InvitationAllowedEntryKindsPolicyConfigOutput.md) |  | [optional]

## Methods

### NewInvitationAllowedEntryKindsPolicyConfig

`func NewInvitationAllowedEntryKindsPolicyConfig(kinds []InvitationEntryKind, ) *InvitationAllowedEntryKindsPolicyConfig`

NewInvitationAllowedEntryKindsPolicyConfig instantiates a new InvitationAllowedEntryKindsPolicyConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInvitationAllowedEntryKindsPolicyConfigWithDefaults

`func NewInvitationAllowedEntryKindsPolicyConfigWithDefaults() *InvitationAllowedEntryKindsPolicyConfig`

NewInvitationAllowedEntryKindsPolicyConfigWithDefaults instantiates a new InvitationAllowedEntryKindsPolicyConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAppliesTo

`func (o *InvitationAllowedEntryKindsPolicyConfig) GetAppliesTo() []PolicyPrincipalKind`

GetAppliesTo returns the AppliesTo field if non-nil, zero value otherwise.

### GetAppliesToOk

`func (o *InvitationAllowedEntryKindsPolicyConfig) GetAppliesToOk() (*[]PolicyPrincipalKind, bool)`

GetAppliesToOk returns a tuple with the AppliesTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAppliesTo

`func (o *InvitationAllowedEntryKindsPolicyConfig) SetAppliesTo(v []PolicyPrincipalKind)`

SetAppliesTo sets AppliesTo field to given value.

### HasAppliesTo

`func (o *InvitationAllowedEntryKindsPolicyConfig) HasAppliesTo() bool`

HasAppliesTo returns a boolean if a field has been set.

### SetAppliesToNil

`func (o *InvitationAllowedEntryKindsPolicyConfig) SetAppliesToNil(b bool)`

 SetAppliesToNil sets the value for AppliesTo to be an explicit nil

### UnsetAppliesTo
`func (o *InvitationAllowedEntryKindsPolicyConfig) UnsetAppliesTo()`

UnsetAppliesTo ensures that no value is present for AppliesTo, not even an explicit nil
### GetKinds

`func (o *InvitationAllowedEntryKindsPolicyConfig) GetKinds() []InvitationEntryKind`

GetKinds returns the Kinds field if non-nil, zero value otherwise.

### GetKindsOk

`func (o *InvitationAllowedEntryKindsPolicyConfig) GetKindsOk() (*[]InvitationEntryKind, bool)`

GetKindsOk returns a tuple with the Kinds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKinds

`func (o *InvitationAllowedEntryKindsPolicyConfig) SetKinds(v []InvitationEntryKind)`

SetKinds sets Kinds field to given value.


### GetOutput

`func (o *InvitationAllowedEntryKindsPolicyConfig) GetOutput() InvitationAllowedEntryKindsPolicyConfigOutput`

GetOutput returns the Output field if non-nil, zero value otherwise.

### GetOutputOk

`func (o *InvitationAllowedEntryKindsPolicyConfig) GetOutputOk() (*InvitationAllowedEntryKindsPolicyConfigOutput, bool)`

GetOutputOk returns a tuple with the Output field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutput

`func (o *InvitationAllowedEntryKindsPolicyConfig) SetOutput(v InvitationAllowedEntryKindsPolicyConfigOutput)`

SetOutput sets Output field to given value.

### HasOutput

`func (o *InvitationAllowedEntryKindsPolicyConfig) HasOutput() bool`

HasOutput returns a boolean if a field has been set.

### SetOutputNil

`func (o *InvitationAllowedEntryKindsPolicyConfig) SetOutputNil(b bool)`

 SetOutputNil sets the value for Output to be an explicit nil

### UnsetOutput
`func (o *InvitationAllowedEntryKindsPolicyConfig) UnsetOutput()`

UnsetOutput ensures that no value is present for Output, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
