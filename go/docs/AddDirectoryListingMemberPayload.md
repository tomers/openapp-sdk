# AddDirectoryListingMemberPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DateOfBirth** | Pointer to **NullableString** | Parent-declared date of birth (&#x60;YYYY-MM-DD&#x60;). Required when &#x60;is_child&#x60; is true. | [optional]
**IsChild** | Pointer to **NullableBool** | When true, this household member is a child; date of birth is required. | [optional]
**ReceivesCalls** | Pointer to **NullableBool** | When omitted, defaults to false for minors and true for adults. | [optional]
**Role** | Pointer to **string** |  | [optional]
**UserId** | **string** |  |

## Methods

### NewAddDirectoryListingMemberPayload

`func NewAddDirectoryListingMemberPayload(userId string, ) *AddDirectoryListingMemberPayload`

NewAddDirectoryListingMemberPayload instantiates a new AddDirectoryListingMemberPayload object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAddDirectoryListingMemberPayloadWithDefaults

`func NewAddDirectoryListingMemberPayloadWithDefaults() *AddDirectoryListingMemberPayload`

NewAddDirectoryListingMemberPayloadWithDefaults instantiates a new AddDirectoryListingMemberPayload object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDateOfBirth

`func (o *AddDirectoryListingMemberPayload) GetDateOfBirth() string`

GetDateOfBirth returns the DateOfBirth field if non-nil, zero value otherwise.

### GetDateOfBirthOk

`func (o *AddDirectoryListingMemberPayload) GetDateOfBirthOk() (*string, bool)`

GetDateOfBirthOk returns a tuple with the DateOfBirth field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDateOfBirth

`func (o *AddDirectoryListingMemberPayload) SetDateOfBirth(v string)`

SetDateOfBirth sets DateOfBirth field to given value.

### HasDateOfBirth

`func (o *AddDirectoryListingMemberPayload) HasDateOfBirth() bool`

HasDateOfBirth returns a boolean if a field has been set.

### SetDateOfBirthNil

`func (o *AddDirectoryListingMemberPayload) SetDateOfBirthNil(b bool)`

 SetDateOfBirthNil sets the value for DateOfBirth to be an explicit nil

### UnsetDateOfBirth
`func (o *AddDirectoryListingMemberPayload) UnsetDateOfBirth()`

UnsetDateOfBirth ensures that no value is present for DateOfBirth, not even an explicit nil
### GetIsChild

`func (o *AddDirectoryListingMemberPayload) GetIsChild() bool`

GetIsChild returns the IsChild field if non-nil, zero value otherwise.

### GetIsChildOk

`func (o *AddDirectoryListingMemberPayload) GetIsChildOk() (*bool, bool)`

GetIsChildOk returns a tuple with the IsChild field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsChild

`func (o *AddDirectoryListingMemberPayload) SetIsChild(v bool)`

SetIsChild sets IsChild field to given value.

### HasIsChild

`func (o *AddDirectoryListingMemberPayload) HasIsChild() bool`

HasIsChild returns a boolean if a field has been set.

### SetIsChildNil

`func (o *AddDirectoryListingMemberPayload) SetIsChildNil(b bool)`

 SetIsChildNil sets the value for IsChild to be an explicit nil

### UnsetIsChild
`func (o *AddDirectoryListingMemberPayload) UnsetIsChild()`

UnsetIsChild ensures that no value is present for IsChild, not even an explicit nil
### GetReceivesCalls

`func (o *AddDirectoryListingMemberPayload) GetReceivesCalls() bool`

GetReceivesCalls returns the ReceivesCalls field if non-nil, zero value otherwise.

### GetReceivesCallsOk

`func (o *AddDirectoryListingMemberPayload) GetReceivesCallsOk() (*bool, bool)`

GetReceivesCallsOk returns a tuple with the ReceivesCalls field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReceivesCalls

`func (o *AddDirectoryListingMemberPayload) SetReceivesCalls(v bool)`

SetReceivesCalls sets ReceivesCalls field to given value.

### HasReceivesCalls

`func (o *AddDirectoryListingMemberPayload) HasReceivesCalls() bool`

HasReceivesCalls returns a boolean if a field has been set.

### SetReceivesCallsNil

`func (o *AddDirectoryListingMemberPayload) SetReceivesCallsNil(b bool)`

 SetReceivesCallsNil sets the value for ReceivesCalls to be an explicit nil

### UnsetReceivesCalls
`func (o *AddDirectoryListingMemberPayload) UnsetReceivesCalls()`

UnsetReceivesCalls ensures that no value is present for ReceivesCalls, not even an explicit nil
### GetRole

`func (o *AddDirectoryListingMemberPayload) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *AddDirectoryListingMemberPayload) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *AddDirectoryListingMemberPayload) SetRole(v string)`

SetRole sets Role field to given value.

### HasRole

`func (o *AddDirectoryListingMemberPayload) HasRole() bool`

HasRole returns a boolean if a field has been set.

### GetUserId

`func (o *AddDirectoryListingMemberPayload) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *AddDirectoryListingMemberPayload) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *AddDirectoryListingMemberPayload) SetUserId(v string)`

SetUserId sets UserId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
