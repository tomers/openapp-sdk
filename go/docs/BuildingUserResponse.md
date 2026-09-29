# BuildingUserResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Listings** | **[]string** |  |
**Role** | **NullableString** |  |
**UserEmail** | Pointer to **NullableString** |  | [optional]
**UserId** | **NullableString** |  |
**UserName** | Pointer to **interface{}** |  | [optional]

## Methods

### NewBuildingUserResponse

`func NewBuildingUserResponse(listings []string, role NullableString, userId NullableString, ) *BuildingUserResponse`

NewBuildingUserResponse instantiates a new BuildingUserResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewBuildingUserResponseWithDefaults

`func NewBuildingUserResponseWithDefaults() *BuildingUserResponse`

NewBuildingUserResponseWithDefaults instantiates a new BuildingUserResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetListings

`func (o *BuildingUserResponse) GetListings() []string`

GetListings returns the Listings field if non-nil, zero value otherwise.

### GetListingsOk

`func (o *BuildingUserResponse) GetListingsOk() (*[]string, bool)`

GetListingsOk returns a tuple with the Listings field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListings

`func (o *BuildingUserResponse) SetListings(v []string)`

SetListings sets Listings field to given value.


### GetRole

`func (o *BuildingUserResponse) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *BuildingUserResponse) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *BuildingUserResponse) SetRole(v string)`

SetRole sets Role field to given value.


### SetRoleNil

`func (o *BuildingUserResponse) SetRoleNil(b bool)`

 SetRoleNil sets the value for Role to be an explicit nil

### UnsetRole
`func (o *BuildingUserResponse) UnsetRole()`

UnsetRole ensures that no value is present for Role, not even an explicit nil
### GetUserEmail

`func (o *BuildingUserResponse) GetUserEmail() string`

GetUserEmail returns the UserEmail field if non-nil, zero value otherwise.

### GetUserEmailOk

`func (o *BuildingUserResponse) GetUserEmailOk() (*string, bool)`

GetUserEmailOk returns a tuple with the UserEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserEmail

`func (o *BuildingUserResponse) SetUserEmail(v string)`

SetUserEmail sets UserEmail field to given value.

### HasUserEmail

`func (o *BuildingUserResponse) HasUserEmail() bool`

HasUserEmail returns a boolean if a field has been set.

### SetUserEmailNil

`func (o *BuildingUserResponse) SetUserEmailNil(b bool)`

 SetUserEmailNil sets the value for UserEmail to be an explicit nil

### UnsetUserEmail
`func (o *BuildingUserResponse) UnsetUserEmail()`

UnsetUserEmail ensures that no value is present for UserEmail, not even an explicit nil
### GetUserId

`func (o *BuildingUserResponse) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *BuildingUserResponse) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *BuildingUserResponse) SetUserId(v string)`

SetUserId sets UserId field to given value.


### SetUserIdNil

`func (o *BuildingUserResponse) SetUserIdNil(b bool)`

 SetUserIdNil sets the value for UserId to be an explicit nil

### UnsetUserId
`func (o *BuildingUserResponse) UnsetUserId()`

UnsetUserId ensures that no value is present for UserId, not even an explicit nil
### GetUserName

`func (o *BuildingUserResponse) GetUserName() interface{}`

GetUserName returns the UserName field if non-nil, zero value otherwise.

### GetUserNameOk

`func (o *BuildingUserResponse) GetUserNameOk() (*interface{}, bool)`

GetUserNameOk returns a tuple with the UserName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserName

`func (o *BuildingUserResponse) SetUserName(v interface{})`

SetUserName sets UserName field to given value.

### HasUserName

`func (o *BuildingUserResponse) HasUserName() bool`

HasUserName returns a boolean if a field has been set.

### SetUserNameNil

`func (o *BuildingUserResponse) SetUserNameNil(b bool)`

 SetUserNameNil sets the value for UserName to be an explicit nil

### UnsetUserName
`func (o *BuildingUserResponse) UnsetUserName()`

UnsetUserName ensures that no value is present for UserName, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
