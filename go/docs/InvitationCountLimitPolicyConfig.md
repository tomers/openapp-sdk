# InvitationCountLimitPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AppliesTo** | Pointer to [**[]PolicyPrincipalKind**](PolicyPrincipalKind.md) | Principal kinds the row binds; unknown and empty values are rejected. Omitted binds the type&#39;s default principals. | [optional]
**Max** | **int64** |  |
**Output** | Pointer to [**NullableInvitationAllowedEntryKindsPolicyConfigOutput**](InvitationAllowedEntryKindsPolicyConfigOutput.md) |  | [optional]
**ZoneId** | Pointer to **NullableString** | Zone the row is scoped to (org and integration rows only): it binds only when the acting effective zone is this zone or a descendant. Must be a live zone of the policy&#39;s org. | [optional]

## Methods

### NewInvitationCountLimitPolicyConfig

`func NewInvitationCountLimitPolicyConfig(max int64, ) *InvitationCountLimitPolicyConfig`

NewInvitationCountLimitPolicyConfig instantiates a new InvitationCountLimitPolicyConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInvitationCountLimitPolicyConfigWithDefaults

`func NewInvitationCountLimitPolicyConfigWithDefaults() *InvitationCountLimitPolicyConfig`

NewInvitationCountLimitPolicyConfigWithDefaults instantiates a new InvitationCountLimitPolicyConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAppliesTo

`func (o *InvitationCountLimitPolicyConfig) GetAppliesTo() []PolicyPrincipalKind`

GetAppliesTo returns the AppliesTo field if non-nil, zero value otherwise.

### GetAppliesToOk

`func (o *InvitationCountLimitPolicyConfig) GetAppliesToOk() (*[]PolicyPrincipalKind, bool)`

GetAppliesToOk returns a tuple with the AppliesTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAppliesTo

`func (o *InvitationCountLimitPolicyConfig) SetAppliesTo(v []PolicyPrincipalKind)`

SetAppliesTo sets AppliesTo field to given value.

### HasAppliesTo

`func (o *InvitationCountLimitPolicyConfig) HasAppliesTo() bool`

HasAppliesTo returns a boolean if a field has been set.

### SetAppliesToNil

`func (o *InvitationCountLimitPolicyConfig) SetAppliesToNil(b bool)`

 SetAppliesToNil sets the value for AppliesTo to be an explicit nil

### UnsetAppliesTo
`func (o *InvitationCountLimitPolicyConfig) UnsetAppliesTo()`

UnsetAppliesTo ensures that no value is present for AppliesTo, not even an explicit nil
### GetMax

`func (o *InvitationCountLimitPolicyConfig) GetMax() int64`

GetMax returns the Max field if non-nil, zero value otherwise.

### GetMaxOk

`func (o *InvitationCountLimitPolicyConfig) GetMaxOk() (*int64, bool)`

GetMaxOk returns a tuple with the Max field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMax

`func (o *InvitationCountLimitPolicyConfig) SetMax(v int64)`

SetMax sets Max field to given value.


### GetOutput

`func (o *InvitationCountLimitPolicyConfig) GetOutput() InvitationAllowedEntryKindsPolicyConfigOutput`

GetOutput returns the Output field if non-nil, zero value otherwise.

### GetOutputOk

`func (o *InvitationCountLimitPolicyConfig) GetOutputOk() (*InvitationAllowedEntryKindsPolicyConfigOutput, bool)`

GetOutputOk returns a tuple with the Output field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutput

`func (o *InvitationCountLimitPolicyConfig) SetOutput(v InvitationAllowedEntryKindsPolicyConfigOutput)`

SetOutput sets Output field to given value.

### HasOutput

`func (o *InvitationCountLimitPolicyConfig) HasOutput() bool`

HasOutput returns a boolean if a field has been set.

### SetOutputNil

`func (o *InvitationCountLimitPolicyConfig) SetOutputNil(b bool)`

 SetOutputNil sets the value for Output to be an explicit nil

### UnsetOutput
`func (o *InvitationCountLimitPolicyConfig) UnsetOutput()`

UnsetOutput ensures that no value is present for Output, not even an explicit nil
### GetZoneId

`func (o *InvitationCountLimitPolicyConfig) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *InvitationCountLimitPolicyConfig) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *InvitationCountLimitPolicyConfig) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.

### HasZoneId

`func (o *InvitationCountLimitPolicyConfig) HasZoneId() bool`

HasZoneId returns a boolean if a field has been set.

### SetZoneIdNil

`func (o *InvitationCountLimitPolicyConfig) SetZoneIdNil(b bool)`

 SetZoneIdNil sets the value for ZoneId to be an explicit nil

### UnsetZoneId
`func (o *InvitationCountLimitPolicyConfig) UnsetZoneId()`

UnsetZoneId ensures that no value is present for ZoneId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
