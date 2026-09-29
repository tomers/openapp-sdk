# AddGroupMemberPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Role** | Pointer to **NullableString** |  | [optional]
**UserId** | **string** |  |

## Methods

### NewAddGroupMemberPayload

`func NewAddGroupMemberPayload(userId string, ) *AddGroupMemberPayload`

NewAddGroupMemberPayload instantiates a new AddGroupMemberPayload object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAddGroupMemberPayloadWithDefaults

`func NewAddGroupMemberPayloadWithDefaults() *AddGroupMemberPayload`

NewAddGroupMemberPayloadWithDefaults instantiates a new AddGroupMemberPayload object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetRole

`func (o *AddGroupMemberPayload) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *AddGroupMemberPayload) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *AddGroupMemberPayload) SetRole(v string)`

SetRole sets Role field to given value.

### HasRole

`func (o *AddGroupMemberPayload) HasRole() bool`

HasRole returns a boolean if a field has been set.

### SetRoleNil

`func (o *AddGroupMemberPayload) SetRoleNil(b bool)`

 SetRoleNil sets the value for Role to be an explicit nil

### UnsetRole
`func (o *AddGroupMemberPayload) UnsetRole()`

UnsetRole ensures that no value is present for Role, not even an explicit nil
### GetUserId

`func (o *AddGroupMemberPayload) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *AddGroupMemberPayload) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *AddGroupMemberPayload) SetUserId(v string)`

SetUserId sets UserId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
