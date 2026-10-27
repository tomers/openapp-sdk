# InvitationAuthoringPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AppliesTo** | Pointer to [**[]PolicyPrincipalKind**](PolicyPrincipalKind.md) | Principal kinds the row binds; unknown and empty values are rejected. Omitted binds the type&#39;s default principals. | [optional]
**Output** | Pointer to [**NullableInvitationAllowedEntryKindsPolicyConfigOutput**](InvitationAllowedEntryKindsPolicyConfigOutput.md) |  | [optional]
**ZoneId** | Pointer to **NullableString** | Zone the row is scoped to (org and integration rows only): it binds only when the acting effective zone is this zone or a descendant. Must be a live zone of the policy&#39;s org. | [optional]

## Methods

### NewInvitationAuthoringPolicyConfig

`func NewInvitationAuthoringPolicyConfig() *InvitationAuthoringPolicyConfig`

NewInvitationAuthoringPolicyConfig instantiates a new InvitationAuthoringPolicyConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInvitationAuthoringPolicyConfigWithDefaults

`func NewInvitationAuthoringPolicyConfigWithDefaults() *InvitationAuthoringPolicyConfig`

NewInvitationAuthoringPolicyConfigWithDefaults instantiates a new InvitationAuthoringPolicyConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAppliesTo

`func (o *InvitationAuthoringPolicyConfig) GetAppliesTo() []PolicyPrincipalKind`

GetAppliesTo returns the AppliesTo field if non-nil, zero value otherwise.

### GetAppliesToOk

`func (o *InvitationAuthoringPolicyConfig) GetAppliesToOk() (*[]PolicyPrincipalKind, bool)`

GetAppliesToOk returns a tuple with the AppliesTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAppliesTo

`func (o *InvitationAuthoringPolicyConfig) SetAppliesTo(v []PolicyPrincipalKind)`

SetAppliesTo sets AppliesTo field to given value.

### HasAppliesTo

`func (o *InvitationAuthoringPolicyConfig) HasAppliesTo() bool`

HasAppliesTo returns a boolean if a field has been set.

### SetAppliesToNil

`func (o *InvitationAuthoringPolicyConfig) SetAppliesToNil(b bool)`

 SetAppliesToNil sets the value for AppliesTo to be an explicit nil

### UnsetAppliesTo
`func (o *InvitationAuthoringPolicyConfig) UnsetAppliesTo()`

UnsetAppliesTo ensures that no value is present for AppliesTo, not even an explicit nil
### GetOutput

`func (o *InvitationAuthoringPolicyConfig) GetOutput() InvitationAllowedEntryKindsPolicyConfigOutput`

GetOutput returns the Output field if non-nil, zero value otherwise.

### GetOutputOk

`func (o *InvitationAuthoringPolicyConfig) GetOutputOk() (*InvitationAllowedEntryKindsPolicyConfigOutput, bool)`

GetOutputOk returns a tuple with the Output field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutput

`func (o *InvitationAuthoringPolicyConfig) SetOutput(v InvitationAllowedEntryKindsPolicyConfigOutput)`

SetOutput sets Output field to given value.

### HasOutput

`func (o *InvitationAuthoringPolicyConfig) HasOutput() bool`

HasOutput returns a boolean if a field has been set.

### SetOutputNil

`func (o *InvitationAuthoringPolicyConfig) SetOutputNil(b bool)`

 SetOutputNil sets the value for Output to be an explicit nil

### UnsetOutput
`func (o *InvitationAuthoringPolicyConfig) UnsetOutput()`

UnsetOutput ensures that no value is present for Output, not even an explicit nil
### GetZoneId

`func (o *InvitationAuthoringPolicyConfig) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *InvitationAuthoringPolicyConfig) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *InvitationAuthoringPolicyConfig) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.

### HasZoneId

`func (o *InvitationAuthoringPolicyConfig) HasZoneId() bool`

HasZoneId returns a boolean if a field has been set.

### SetZoneIdNil

`func (o *InvitationAuthoringPolicyConfig) SetZoneIdNil(b bool)`

 SetZoneIdNil sets the value for ZoneId to be an explicit nil

### UnsetZoneId
`func (o *InvitationAuthoringPolicyConfig) UnsetZoneId()`

UnsetZoneId ensures that no value is present for ZoneId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
