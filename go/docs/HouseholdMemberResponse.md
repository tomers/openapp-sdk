# HouseholdMemberResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ManagerEligible** | **bool** | Missing age data is treated as adult; only an explicit under-16 classification is ineligible. |
**Role** | **string** |  |
**UserId** | **string** |  |
**UserName** | Pointer to **interface{}** |  | [optional]

## Methods

### NewHouseholdMemberResponse

`func NewHouseholdMemberResponse(managerEligible bool, role string, userId string, ) *HouseholdMemberResponse`

NewHouseholdMemberResponse instantiates a new HouseholdMemberResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewHouseholdMemberResponseWithDefaults

`func NewHouseholdMemberResponseWithDefaults() *HouseholdMemberResponse`

NewHouseholdMemberResponseWithDefaults instantiates a new HouseholdMemberResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetManagerEligible

`func (o *HouseholdMemberResponse) GetManagerEligible() bool`

GetManagerEligible returns the ManagerEligible field if non-nil, zero value otherwise.

### GetManagerEligibleOk

`func (o *HouseholdMemberResponse) GetManagerEligibleOk() (*bool, bool)`

GetManagerEligibleOk returns a tuple with the ManagerEligible field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetManagerEligible

`func (o *HouseholdMemberResponse) SetManagerEligible(v bool)`

SetManagerEligible sets ManagerEligible field to given value.


### GetRole

`func (o *HouseholdMemberResponse) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *HouseholdMemberResponse) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *HouseholdMemberResponse) SetRole(v string)`

SetRole sets Role field to given value.


### GetUserId

`func (o *HouseholdMemberResponse) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *HouseholdMemberResponse) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *HouseholdMemberResponse) SetUserId(v string)`

SetUserId sets UserId field to given value.


### GetUserName

`func (o *HouseholdMemberResponse) GetUserName() interface{}`

GetUserName returns the UserName field if non-nil, zero value otherwise.

### GetUserNameOk

`func (o *HouseholdMemberResponse) GetUserNameOk() (*interface{}, bool)`

GetUserNameOk returns a tuple with the UserName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserName

`func (o *HouseholdMemberResponse) SetUserName(v interface{})`

SetUserName sets UserName field to given value.

### HasUserName

`func (o *HouseholdMemberResponse) HasUserName() bool`

HasUserName returns a boolean if a field has been set.

### SetUserNameNil

`func (o *HouseholdMemberResponse) SetUserNameNil(b bool)`

 SetUserNameNil sets the value for UserName to be an explicit nil

### UnsetUserName
`func (o *HouseholdMemberResponse) UnsetUserName()`

UnsetUserName ensures that no value is present for UserName, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
