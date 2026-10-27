# CreateUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Email** | Pointer to **NullableString** | Login email. Omit for phone-only accounts. | [optional]
**FirstName** | Pointer to **NullableString** | Given name stored on the Kratos identity (&#x60;traits.name.first&#x60;) when creating the account. | [optional]
**LastName** | Pointer to **NullableString** | Family name stored on the Kratos identity (&#x60;traits.name.last&#x60;) when creating the account. | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**Phone** | Pointer to **NullableString** | E.164 phone (&#x60;+[country][digits]&#x60;). When set, must satisfy the same pattern as Kratos. | [optional]
**Roles** | Pointer to **map[string]interface{}** | Org ID (string) to list of role names. Optional. | [optional]

## Methods

### NewCreateUserRequest

`func NewCreateUserRequest(name LocalizedString, ) *CreateUserRequest`

NewCreateUserRequest instantiates a new CreateUserRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateUserRequestWithDefaults

`func NewCreateUserRequestWithDefaults() *CreateUserRequest`

NewCreateUserRequestWithDefaults instantiates a new CreateUserRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEmail

`func (o *CreateUserRequest) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *CreateUserRequest) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *CreateUserRequest) SetEmail(v string)`

SetEmail sets Email field to given value.

### HasEmail

`func (o *CreateUserRequest) HasEmail() bool`

HasEmail returns a boolean if a field has been set.

### SetEmailNil

`func (o *CreateUserRequest) SetEmailNil(b bool)`

 SetEmailNil sets the value for Email to be an explicit nil

### UnsetEmail
`func (o *CreateUserRequest) UnsetEmail()`

UnsetEmail ensures that no value is present for Email, not even an explicit nil
### GetFirstName

`func (o *CreateUserRequest) GetFirstName() string`

GetFirstName returns the FirstName field if non-nil, zero value otherwise.

### GetFirstNameOk

`func (o *CreateUserRequest) GetFirstNameOk() (*string, bool)`

GetFirstNameOk returns a tuple with the FirstName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFirstName

`func (o *CreateUserRequest) SetFirstName(v string)`

SetFirstName sets FirstName field to given value.

### HasFirstName

`func (o *CreateUserRequest) HasFirstName() bool`

HasFirstName returns a boolean if a field has been set.

### SetFirstNameNil

`func (o *CreateUserRequest) SetFirstNameNil(b bool)`

 SetFirstNameNil sets the value for FirstName to be an explicit nil

### UnsetFirstName
`func (o *CreateUserRequest) UnsetFirstName()`

UnsetFirstName ensures that no value is present for FirstName, not even an explicit nil
### GetLastName

`func (o *CreateUserRequest) GetLastName() string`

GetLastName returns the LastName field if non-nil, zero value otherwise.

### GetLastNameOk

`func (o *CreateUserRequest) GetLastNameOk() (*string, bool)`

GetLastNameOk returns a tuple with the LastName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastName

`func (o *CreateUserRequest) SetLastName(v string)`

SetLastName sets LastName field to given value.

### HasLastName

`func (o *CreateUserRequest) HasLastName() bool`

HasLastName returns a boolean if a field has been set.

### SetLastNameNil

`func (o *CreateUserRequest) SetLastNameNil(b bool)`

 SetLastNameNil sets the value for LastName to be an explicit nil

### UnsetLastName
`func (o *CreateUserRequest) UnsetLastName()`

UnsetLastName ensures that no value is present for LastName, not even an explicit nil
### GetName

`func (o *CreateUserRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateUserRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateUserRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetPhone

`func (o *CreateUserRequest) GetPhone() string`

GetPhone returns the Phone field if non-nil, zero value otherwise.

### GetPhoneOk

`func (o *CreateUserRequest) GetPhoneOk() (*string, bool)`

GetPhoneOk returns a tuple with the Phone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhone

`func (o *CreateUserRequest) SetPhone(v string)`

SetPhone sets Phone field to given value.

### HasPhone

`func (o *CreateUserRequest) HasPhone() bool`

HasPhone returns a boolean if a field has been set.

### SetPhoneNil

`func (o *CreateUserRequest) SetPhoneNil(b bool)`

 SetPhoneNil sets the value for Phone to be an explicit nil

### UnsetPhone
`func (o *CreateUserRequest) UnsetPhone()`

UnsetPhone ensures that no value is present for Phone, not even an explicit nil
### GetRoles

`func (o *CreateUserRequest) GetRoles() map[string]interface{}`

GetRoles returns the Roles field if non-nil, zero value otherwise.

### GetRolesOk

`func (o *CreateUserRequest) GetRolesOk() (*map[string]interface{}, bool)`

GetRolesOk returns a tuple with the Roles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRoles

`func (o *CreateUserRequest) SetRoles(v map[string]interface{})`

SetRoles sets Roles field to given value.

### HasRoles

`func (o *CreateUserRequest) HasRoles() bool`

HasRoles returns a boolean if a field has been set.

### SetRolesNil

`func (o *CreateUserRequest) SetRolesNil(b bool)`

 SetRolesNil sets the value for Roles to be an explicit nil

### UnsetRoles
`func (o *CreateUserRequest) UnsetRoles()`

UnsetRoles ensures that no value is present for Roles, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
