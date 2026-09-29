# AddMemberRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DisplayName** | Pointer to [**NullableLocalizedString**](LocalizedString.md) | Localized display name used to label the member&#39;s auto-created personal billing account. | [optional]
**Role** | Pointer to **NullableString** |  | [optional]
**UserId** | **string** |  |

## Methods

### NewAddMemberRequest

`func NewAddMemberRequest(userId string, ) *AddMemberRequest`

NewAddMemberRequest instantiates a new AddMemberRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAddMemberRequestWithDefaults

`func NewAddMemberRequestWithDefaults() *AddMemberRequest`

NewAddMemberRequestWithDefaults instantiates a new AddMemberRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDisplayName

`func (o *AddMemberRequest) GetDisplayName() LocalizedString`

GetDisplayName returns the DisplayName field if non-nil, zero value otherwise.

### GetDisplayNameOk

`func (o *AddMemberRequest) GetDisplayNameOk() (*LocalizedString, bool)`

GetDisplayNameOk returns a tuple with the DisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisplayName

`func (o *AddMemberRequest) SetDisplayName(v LocalizedString)`

SetDisplayName sets DisplayName field to given value.

### HasDisplayName

`func (o *AddMemberRequest) HasDisplayName() bool`

HasDisplayName returns a boolean if a field has been set.

### SetDisplayNameNil

`func (o *AddMemberRequest) SetDisplayNameNil(b bool)`

 SetDisplayNameNil sets the value for DisplayName to be an explicit nil

### UnsetDisplayName
`func (o *AddMemberRequest) UnsetDisplayName()`

UnsetDisplayName ensures that no value is present for DisplayName, not even an explicit nil
### GetRole

`func (o *AddMemberRequest) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *AddMemberRequest) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *AddMemberRequest) SetRole(v string)`

SetRole sets Role field to given value.

### HasRole

`func (o *AddMemberRequest) HasRole() bool`

HasRole returns a boolean if a field has been set.

### SetRoleNil

`func (o *AddMemberRequest) SetRoleNil(b bool)`

 SetRoleNil sets the value for Role to be an explicit nil

### UnsetRole
`func (o *AddMemberRequest) UnsetRole()`

UnsetRole ensures that no value is present for Role, not even an explicit nil
### GetUserId

`func (o *AddMemberRequest) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *AddMemberRequest) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *AddMemberRequest) SetUserId(v string)`

SetUserId sets UserId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
