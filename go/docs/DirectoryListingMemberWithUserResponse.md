# DirectoryListingMemberWithUserResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AgeClass** | Pointer to [**NullableAgeClass**](AgeClass.md) | Derived age class. Only set for listing managers. | [optional]
**DateOfBirth** | Pointer to **NullableString** | Calendar date &#x60;YYYY-MM-DD&#x60;. Only set for listing managers. | [optional]
**Member** | [**DirectoryListingMemberResponse**](DirectoryListingMemberResponse.md) |  |
**UserEmail** | Pointer to **NullableString** |  | [optional]
**UserName** | Pointer to **interface{}** |  | [optional]

## Methods

### NewDirectoryListingMemberWithUserResponse

`func NewDirectoryListingMemberWithUserResponse(member DirectoryListingMemberResponse, ) *DirectoryListingMemberWithUserResponse`

NewDirectoryListingMemberWithUserResponse instantiates a new DirectoryListingMemberWithUserResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDirectoryListingMemberWithUserResponseWithDefaults

`func NewDirectoryListingMemberWithUserResponseWithDefaults() *DirectoryListingMemberWithUserResponse`

NewDirectoryListingMemberWithUserResponseWithDefaults instantiates a new DirectoryListingMemberWithUserResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAgeClass

`func (o *DirectoryListingMemberWithUserResponse) GetAgeClass() AgeClass`

GetAgeClass returns the AgeClass field if non-nil, zero value otherwise.

### GetAgeClassOk

`func (o *DirectoryListingMemberWithUserResponse) GetAgeClassOk() (*AgeClass, bool)`

GetAgeClassOk returns a tuple with the AgeClass field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAgeClass

`func (o *DirectoryListingMemberWithUserResponse) SetAgeClass(v AgeClass)`

SetAgeClass sets AgeClass field to given value.

### HasAgeClass

`func (o *DirectoryListingMemberWithUserResponse) HasAgeClass() bool`

HasAgeClass returns a boolean if a field has been set.

### SetAgeClassNil

`func (o *DirectoryListingMemberWithUserResponse) SetAgeClassNil(b bool)`

 SetAgeClassNil sets the value for AgeClass to be an explicit nil

### UnsetAgeClass
`func (o *DirectoryListingMemberWithUserResponse) UnsetAgeClass()`

UnsetAgeClass ensures that no value is present for AgeClass, not even an explicit nil
### GetDateOfBirth

`func (o *DirectoryListingMemberWithUserResponse) GetDateOfBirth() string`

GetDateOfBirth returns the DateOfBirth field if non-nil, zero value otherwise.

### GetDateOfBirthOk

`func (o *DirectoryListingMemberWithUserResponse) GetDateOfBirthOk() (*string, bool)`

GetDateOfBirthOk returns a tuple with the DateOfBirth field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDateOfBirth

`func (o *DirectoryListingMemberWithUserResponse) SetDateOfBirth(v string)`

SetDateOfBirth sets DateOfBirth field to given value.

### HasDateOfBirth

`func (o *DirectoryListingMemberWithUserResponse) HasDateOfBirth() bool`

HasDateOfBirth returns a boolean if a field has been set.

### SetDateOfBirthNil

`func (o *DirectoryListingMemberWithUserResponse) SetDateOfBirthNil(b bool)`

 SetDateOfBirthNil sets the value for DateOfBirth to be an explicit nil

### UnsetDateOfBirth
`func (o *DirectoryListingMemberWithUserResponse) UnsetDateOfBirth()`

UnsetDateOfBirth ensures that no value is present for DateOfBirth, not even an explicit nil
### GetMember

`func (o *DirectoryListingMemberWithUserResponse) GetMember() DirectoryListingMemberResponse`

GetMember returns the Member field if non-nil, zero value otherwise.

### GetMemberOk

`func (o *DirectoryListingMemberWithUserResponse) GetMemberOk() (*DirectoryListingMemberResponse, bool)`

GetMemberOk returns a tuple with the Member field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMember

`func (o *DirectoryListingMemberWithUserResponse) SetMember(v DirectoryListingMemberResponse)`

SetMember sets Member field to given value.


### GetUserEmail

`func (o *DirectoryListingMemberWithUserResponse) GetUserEmail() string`

GetUserEmail returns the UserEmail field if non-nil, zero value otherwise.

### GetUserEmailOk

`func (o *DirectoryListingMemberWithUserResponse) GetUserEmailOk() (*string, bool)`

GetUserEmailOk returns a tuple with the UserEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserEmail

`func (o *DirectoryListingMemberWithUserResponse) SetUserEmail(v string)`

SetUserEmail sets UserEmail field to given value.

### HasUserEmail

`func (o *DirectoryListingMemberWithUserResponse) HasUserEmail() bool`

HasUserEmail returns a boolean if a field has been set.

### SetUserEmailNil

`func (o *DirectoryListingMemberWithUserResponse) SetUserEmailNil(b bool)`

 SetUserEmailNil sets the value for UserEmail to be an explicit nil

### UnsetUserEmail
`func (o *DirectoryListingMemberWithUserResponse) UnsetUserEmail()`

UnsetUserEmail ensures that no value is present for UserEmail, not even an explicit nil
### GetUserName

`func (o *DirectoryListingMemberWithUserResponse) GetUserName() interface{}`

GetUserName returns the UserName field if non-nil, zero value otherwise.

### GetUserNameOk

`func (o *DirectoryListingMemberWithUserResponse) GetUserNameOk() (*interface{}, bool)`

GetUserNameOk returns a tuple with the UserName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserName

`func (o *DirectoryListingMemberWithUserResponse) SetUserName(v interface{})`

SetUserName sets UserName field to given value.

### HasUserName

`func (o *DirectoryListingMemberWithUserResponse) HasUserName() bool`

HasUserName returns a boolean if a field has been set.

### SetUserNameNil

`func (o *DirectoryListingMemberWithUserResponse) SetUserNameNil(b bool)`

 SetUserNameNil sets the value for UserName to be an explicit nil

### UnsetUserName
`func (o *DirectoryListingMemberWithUserResponse) UnsetUserName()`

UnsetUserName ensures that no value is present for UserName, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
