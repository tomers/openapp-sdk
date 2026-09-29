# InvitationIdentityPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AppliesTo** | Pointer to [**[]PolicyPrincipalKind**](PolicyPrincipalKind.md) |  | [optional]
**Output** | Pointer to [**NullableInvitationAllowedEntryKindsPolicyConfigOutput**](InvitationAllowedEntryKindsPolicyConfigOutput.md) |  | [optional]
**RequirePhoto** | Pointer to **NullableBool** |  | [optional]
**RequirePin** | Pointer to **NullableBool** |  | [optional]
**RequireVerifiedPhone** | Pointer to **NullableBool** |  | [optional]

## Methods

### NewInvitationIdentityPolicyConfig

`func NewInvitationIdentityPolicyConfig() *InvitationIdentityPolicyConfig`

NewInvitationIdentityPolicyConfig instantiates a new InvitationIdentityPolicyConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInvitationIdentityPolicyConfigWithDefaults

`func NewInvitationIdentityPolicyConfigWithDefaults() *InvitationIdentityPolicyConfig`

NewInvitationIdentityPolicyConfigWithDefaults instantiates a new InvitationIdentityPolicyConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAppliesTo

`func (o *InvitationIdentityPolicyConfig) GetAppliesTo() []PolicyPrincipalKind`

GetAppliesTo returns the AppliesTo field if non-nil, zero value otherwise.

### GetAppliesToOk

`func (o *InvitationIdentityPolicyConfig) GetAppliesToOk() (*[]PolicyPrincipalKind, bool)`

GetAppliesToOk returns a tuple with the AppliesTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAppliesTo

`func (o *InvitationIdentityPolicyConfig) SetAppliesTo(v []PolicyPrincipalKind)`

SetAppliesTo sets AppliesTo field to given value.

### HasAppliesTo

`func (o *InvitationIdentityPolicyConfig) HasAppliesTo() bool`

HasAppliesTo returns a boolean if a field has been set.

### SetAppliesToNil

`func (o *InvitationIdentityPolicyConfig) SetAppliesToNil(b bool)`

 SetAppliesToNil sets the value for AppliesTo to be an explicit nil

### UnsetAppliesTo
`func (o *InvitationIdentityPolicyConfig) UnsetAppliesTo()`

UnsetAppliesTo ensures that no value is present for AppliesTo, not even an explicit nil
### GetOutput

`func (o *InvitationIdentityPolicyConfig) GetOutput() InvitationAllowedEntryKindsPolicyConfigOutput`

GetOutput returns the Output field if non-nil, zero value otherwise.

### GetOutputOk

`func (o *InvitationIdentityPolicyConfig) GetOutputOk() (*InvitationAllowedEntryKindsPolicyConfigOutput, bool)`

GetOutputOk returns a tuple with the Output field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutput

`func (o *InvitationIdentityPolicyConfig) SetOutput(v InvitationAllowedEntryKindsPolicyConfigOutput)`

SetOutput sets Output field to given value.

### HasOutput

`func (o *InvitationIdentityPolicyConfig) HasOutput() bool`

HasOutput returns a boolean if a field has been set.

### SetOutputNil

`func (o *InvitationIdentityPolicyConfig) SetOutputNil(b bool)`

 SetOutputNil sets the value for Output to be an explicit nil

### UnsetOutput
`func (o *InvitationIdentityPolicyConfig) UnsetOutput()`

UnsetOutput ensures that no value is present for Output, not even an explicit nil
### GetRequirePhoto

`func (o *InvitationIdentityPolicyConfig) GetRequirePhoto() bool`

GetRequirePhoto returns the RequirePhoto field if non-nil, zero value otherwise.

### GetRequirePhotoOk

`func (o *InvitationIdentityPolicyConfig) GetRequirePhotoOk() (*bool, bool)`

GetRequirePhotoOk returns a tuple with the RequirePhoto field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequirePhoto

`func (o *InvitationIdentityPolicyConfig) SetRequirePhoto(v bool)`

SetRequirePhoto sets RequirePhoto field to given value.

### HasRequirePhoto

`func (o *InvitationIdentityPolicyConfig) HasRequirePhoto() bool`

HasRequirePhoto returns a boolean if a field has been set.

### SetRequirePhotoNil

`func (o *InvitationIdentityPolicyConfig) SetRequirePhotoNil(b bool)`

 SetRequirePhotoNil sets the value for RequirePhoto to be an explicit nil

### UnsetRequirePhoto
`func (o *InvitationIdentityPolicyConfig) UnsetRequirePhoto()`

UnsetRequirePhoto ensures that no value is present for RequirePhoto, not even an explicit nil
### GetRequirePin

`func (o *InvitationIdentityPolicyConfig) GetRequirePin() bool`

GetRequirePin returns the RequirePin field if non-nil, zero value otherwise.

### GetRequirePinOk

`func (o *InvitationIdentityPolicyConfig) GetRequirePinOk() (*bool, bool)`

GetRequirePinOk returns a tuple with the RequirePin field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequirePin

`func (o *InvitationIdentityPolicyConfig) SetRequirePin(v bool)`

SetRequirePin sets RequirePin field to given value.

### HasRequirePin

`func (o *InvitationIdentityPolicyConfig) HasRequirePin() bool`

HasRequirePin returns a boolean if a field has been set.

### SetRequirePinNil

`func (o *InvitationIdentityPolicyConfig) SetRequirePinNil(b bool)`

 SetRequirePinNil sets the value for RequirePin to be an explicit nil

### UnsetRequirePin
`func (o *InvitationIdentityPolicyConfig) UnsetRequirePin()`

UnsetRequirePin ensures that no value is present for RequirePin, not even an explicit nil
### GetRequireVerifiedPhone

`func (o *InvitationIdentityPolicyConfig) GetRequireVerifiedPhone() bool`

GetRequireVerifiedPhone returns the RequireVerifiedPhone field if non-nil, zero value otherwise.

### GetRequireVerifiedPhoneOk

`func (o *InvitationIdentityPolicyConfig) GetRequireVerifiedPhoneOk() (*bool, bool)`

GetRequireVerifiedPhoneOk returns a tuple with the RequireVerifiedPhone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequireVerifiedPhone

`func (o *InvitationIdentityPolicyConfig) SetRequireVerifiedPhone(v bool)`

SetRequireVerifiedPhone sets RequireVerifiedPhone field to given value.

### HasRequireVerifiedPhone

`func (o *InvitationIdentityPolicyConfig) HasRequireVerifiedPhone() bool`

HasRequireVerifiedPhone returns a boolean if a field has been set.

### SetRequireVerifiedPhoneNil

`func (o *InvitationIdentityPolicyConfig) SetRequireVerifiedPhoneNil(b bool)`

 SetRequireVerifiedPhoneNil sets the value for RequireVerifiedPhone to be an explicit nil

### UnsetRequireVerifiedPhone
`func (o *InvitationIdentityPolicyConfig) UnsetRequireVerifiedPhone()`

UnsetRequireVerifiedPhone ensures that no value is present for RequireVerifiedPhone, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
