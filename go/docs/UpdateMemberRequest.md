# UpdateMemberRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BillingAccountId** | Pointer to **NullableString** |  | [optional]
**Role** | Pointer to **NullableString** |  | [optional]
**Status** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUpdateMemberRequest

`func NewUpdateMemberRequest() *UpdateMemberRequest`

NewUpdateMemberRequest instantiates a new UpdateMemberRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateMemberRequestWithDefaults

`func NewUpdateMemberRequestWithDefaults() *UpdateMemberRequest`

NewUpdateMemberRequestWithDefaults instantiates a new UpdateMemberRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBillingAccountId

`func (o *UpdateMemberRequest) GetBillingAccountId() string`

GetBillingAccountId returns the BillingAccountId field if non-nil, zero value otherwise.

### GetBillingAccountIdOk

`func (o *UpdateMemberRequest) GetBillingAccountIdOk() (*string, bool)`

GetBillingAccountIdOk returns a tuple with the BillingAccountId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBillingAccountId

`func (o *UpdateMemberRequest) SetBillingAccountId(v string)`

SetBillingAccountId sets BillingAccountId field to given value.

### HasBillingAccountId

`func (o *UpdateMemberRequest) HasBillingAccountId() bool`

HasBillingAccountId returns a boolean if a field has been set.

### SetBillingAccountIdNil

`func (o *UpdateMemberRequest) SetBillingAccountIdNil(b bool)`

 SetBillingAccountIdNil sets the value for BillingAccountId to be an explicit nil

### UnsetBillingAccountId
`func (o *UpdateMemberRequest) UnsetBillingAccountId()`

UnsetBillingAccountId ensures that no value is present for BillingAccountId, not even an explicit nil
### GetRole

`func (o *UpdateMemberRequest) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *UpdateMemberRequest) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *UpdateMemberRequest) SetRole(v string)`

SetRole sets Role field to given value.

### HasRole

`func (o *UpdateMemberRequest) HasRole() bool`

HasRole returns a boolean if a field has been set.

### SetRoleNil

`func (o *UpdateMemberRequest) SetRoleNil(b bool)`

 SetRoleNil sets the value for Role to be an explicit nil

### UnsetRole
`func (o *UpdateMemberRequest) UnsetRole()`

UnsetRole ensures that no value is present for Role, not even an explicit nil
### GetStatus

`func (o *UpdateMemberRequest) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *UpdateMemberRequest) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *UpdateMemberRequest) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *UpdateMemberRequest) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### SetStatusNil

`func (o *UpdateMemberRequest) SetStatusNil(b bool)`

 SetStatusNil sets the value for Status to be an explicit nil

### UnsetStatus
`func (o *UpdateMemberRequest) UnsetStatus()`

UnsetStatus ensures that no value is present for Status, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
