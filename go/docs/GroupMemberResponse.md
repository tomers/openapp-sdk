# GroupMemberResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DisplayName** | Pointer to **interface{}** |  | [optional]
**MemberRef** | **string** |  |
**Role** | **string** |  |

## Methods

### NewGroupMemberResponse

`func NewGroupMemberResponse(memberRef string, role string, ) *GroupMemberResponse`

NewGroupMemberResponse instantiates a new GroupMemberResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewGroupMemberResponseWithDefaults

`func NewGroupMemberResponseWithDefaults() *GroupMemberResponse`

NewGroupMemberResponseWithDefaults instantiates a new GroupMemberResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDisplayName

`func (o *GroupMemberResponse) GetDisplayName() interface{}`

GetDisplayName returns the DisplayName field if non-nil, zero value otherwise.

### GetDisplayNameOk

`func (o *GroupMemberResponse) GetDisplayNameOk() (*interface{}, bool)`

GetDisplayNameOk returns a tuple with the DisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisplayName

`func (o *GroupMemberResponse) SetDisplayName(v interface{})`

SetDisplayName sets DisplayName field to given value.

### HasDisplayName

`func (o *GroupMemberResponse) HasDisplayName() bool`

HasDisplayName returns a boolean if a field has been set.

### SetDisplayNameNil

`func (o *GroupMemberResponse) SetDisplayNameNil(b bool)`

 SetDisplayNameNil sets the value for DisplayName to be an explicit nil

### UnsetDisplayName
`func (o *GroupMemberResponse) UnsetDisplayName()`

UnsetDisplayName ensures that no value is present for DisplayName, not even an explicit nil
### GetMemberRef

`func (o *GroupMemberResponse) GetMemberRef() string`

GetMemberRef returns the MemberRef field if non-nil, zero value otherwise.

### GetMemberRefOk

`func (o *GroupMemberResponse) GetMemberRefOk() (*string, bool)`

GetMemberRefOk returns a tuple with the MemberRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemberRef

`func (o *GroupMemberResponse) SetMemberRef(v string)`

SetMemberRef sets MemberRef field to given value.


### GetRole

`func (o *GroupMemberResponse) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *GroupMemberResponse) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *GroupMemberResponse) SetRole(v string)`

SetRole sets Role field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
