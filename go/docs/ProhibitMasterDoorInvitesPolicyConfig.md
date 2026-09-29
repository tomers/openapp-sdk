# ProhibitMasterDoorInvitesPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AppliesTo** | Pointer to [**[]PolicyPrincipalKind**](PolicyPrincipalKind.md) |  | [optional]
**MasterDoorIds** | **[]string** |  |
**Output** | Pointer to [**NullableInvitationAllowedEntryKindsPolicyConfigOutput**](InvitationAllowedEntryKindsPolicyConfigOutput.md) |  | [optional]

## Methods

### NewProhibitMasterDoorInvitesPolicyConfig

`func NewProhibitMasterDoorInvitesPolicyConfig(masterDoorIds []string, ) *ProhibitMasterDoorInvitesPolicyConfig`

NewProhibitMasterDoorInvitesPolicyConfig instantiates a new ProhibitMasterDoorInvitesPolicyConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewProhibitMasterDoorInvitesPolicyConfigWithDefaults

`func NewProhibitMasterDoorInvitesPolicyConfigWithDefaults() *ProhibitMasterDoorInvitesPolicyConfig`

NewProhibitMasterDoorInvitesPolicyConfigWithDefaults instantiates a new ProhibitMasterDoorInvitesPolicyConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAppliesTo

`func (o *ProhibitMasterDoorInvitesPolicyConfig) GetAppliesTo() []PolicyPrincipalKind`

GetAppliesTo returns the AppliesTo field if non-nil, zero value otherwise.

### GetAppliesToOk

`func (o *ProhibitMasterDoorInvitesPolicyConfig) GetAppliesToOk() (*[]PolicyPrincipalKind, bool)`

GetAppliesToOk returns a tuple with the AppliesTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAppliesTo

`func (o *ProhibitMasterDoorInvitesPolicyConfig) SetAppliesTo(v []PolicyPrincipalKind)`

SetAppliesTo sets AppliesTo field to given value.

### HasAppliesTo

`func (o *ProhibitMasterDoorInvitesPolicyConfig) HasAppliesTo() bool`

HasAppliesTo returns a boolean if a field has been set.

### SetAppliesToNil

`func (o *ProhibitMasterDoorInvitesPolicyConfig) SetAppliesToNil(b bool)`

 SetAppliesToNil sets the value for AppliesTo to be an explicit nil

### UnsetAppliesTo
`func (o *ProhibitMasterDoorInvitesPolicyConfig) UnsetAppliesTo()`

UnsetAppliesTo ensures that no value is present for AppliesTo, not even an explicit nil
### GetMasterDoorIds

`func (o *ProhibitMasterDoorInvitesPolicyConfig) GetMasterDoorIds() []string`

GetMasterDoorIds returns the MasterDoorIds field if non-nil, zero value otherwise.

### GetMasterDoorIdsOk

`func (o *ProhibitMasterDoorInvitesPolicyConfig) GetMasterDoorIdsOk() (*[]string, bool)`

GetMasterDoorIdsOk returns a tuple with the MasterDoorIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMasterDoorIds

`func (o *ProhibitMasterDoorInvitesPolicyConfig) SetMasterDoorIds(v []string)`

SetMasterDoorIds sets MasterDoorIds field to given value.


### GetOutput

`func (o *ProhibitMasterDoorInvitesPolicyConfig) GetOutput() InvitationAllowedEntryKindsPolicyConfigOutput`

GetOutput returns the Output field if non-nil, zero value otherwise.

### GetOutputOk

`func (o *ProhibitMasterDoorInvitesPolicyConfig) GetOutputOk() (*InvitationAllowedEntryKindsPolicyConfigOutput, bool)`

GetOutputOk returns a tuple with the Output field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutput

`func (o *ProhibitMasterDoorInvitesPolicyConfig) SetOutput(v InvitationAllowedEntryKindsPolicyConfigOutput)`

SetOutput sets Output field to given value.

### HasOutput

`func (o *ProhibitMasterDoorInvitesPolicyConfig) HasOutput() bool`

HasOutput returns a boolean if a field has been set.

### SetOutputNil

`func (o *ProhibitMasterDoorInvitesPolicyConfig) SetOutputNil(b bool)`

 SetOutputNil sets the value for Output to be an explicit nil

### UnsetOutput
`func (o *ProhibitMasterDoorInvitesPolicyConfig) UnsetOutput()`

UnsetOutput ensures that no value is present for Output, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
