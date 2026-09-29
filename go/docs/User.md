# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **time.Time** |  | [optional]
**DeletedAt** | Pointer to **time.Time** |  | [optional]
**HardDeleteAt** | Pointer to **time.Time** |  | [optional]
**PurgeAt** | Pointer to **time.Time** |  | [optional]
**UpdatedAt** | Pointer to **time.Time** |  | [optional]
**Email** | **string** | Email address (used for login and provisioning). May be an internal placeholder when no verified email is available. |
**Id** | **string** | Unique identifier (ULID). |
**MergedIntoUserId** | Pointer to **NullableString** | When set, this row was absorbed into another user. Follow for live identity. | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) | Display name. |
**Phone** | Pointer to **NullableString** | Verified E.164 phone when set. | [optional]
**PhoneVerifiedAt** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUser

`func NewUser(email string, id string, name LocalizedString, ) *User`

NewUser instantiates a new User object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUserWithDefaults

`func NewUserWithDefaults() *User`

NewUserWithDefaults instantiates a new User object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *User) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *User) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *User) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *User) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetDeletedAt

`func (o *User) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *User) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *User) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *User) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetHardDeleteAt

`func (o *User) GetHardDeleteAt() time.Time`

GetHardDeleteAt returns the HardDeleteAt field if non-nil, zero value otherwise.

### GetHardDeleteAtOk

`func (o *User) GetHardDeleteAtOk() (*time.Time, bool)`

GetHardDeleteAtOk returns a tuple with the HardDeleteAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHardDeleteAt

`func (o *User) SetHardDeleteAt(v time.Time)`

SetHardDeleteAt sets HardDeleteAt field to given value.

### HasHardDeleteAt

`func (o *User) HasHardDeleteAt() bool`

HasHardDeleteAt returns a boolean if a field has been set.

### GetPurgeAt

`func (o *User) GetPurgeAt() time.Time`

GetPurgeAt returns the PurgeAt field if non-nil, zero value otherwise.

### GetPurgeAtOk

`func (o *User) GetPurgeAtOk() (*time.Time, bool)`

GetPurgeAtOk returns a tuple with the PurgeAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurgeAt

`func (o *User) SetPurgeAt(v time.Time)`

SetPurgeAt sets PurgeAt field to given value.

### HasPurgeAt

`func (o *User) HasPurgeAt() bool`

HasPurgeAt returns a boolean if a field has been set.

### GetUpdatedAt

`func (o *User) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *User) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *User) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *User) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### GetEmail

`func (o *User) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *User) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *User) SetEmail(v string)`

SetEmail sets Email field to given value.


### GetId

`func (o *User) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *User) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *User) SetId(v string)`

SetId sets Id field to given value.


### GetMergedIntoUserId

`func (o *User) GetMergedIntoUserId() string`

GetMergedIntoUserId returns the MergedIntoUserId field if non-nil, zero value otherwise.

### GetMergedIntoUserIdOk

`func (o *User) GetMergedIntoUserIdOk() (*string, bool)`

GetMergedIntoUserIdOk returns a tuple with the MergedIntoUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMergedIntoUserId

`func (o *User) SetMergedIntoUserId(v string)`

SetMergedIntoUserId sets MergedIntoUserId field to given value.

### HasMergedIntoUserId

`func (o *User) HasMergedIntoUserId() bool`

HasMergedIntoUserId returns a boolean if a field has been set.

### SetMergedIntoUserIdNil

`func (o *User) SetMergedIntoUserIdNil(b bool)`

 SetMergedIntoUserIdNil sets the value for MergedIntoUserId to be an explicit nil

### UnsetMergedIntoUserId
`func (o *User) UnsetMergedIntoUserId()`

UnsetMergedIntoUserId ensures that no value is present for MergedIntoUserId, not even an explicit nil
### GetName

`func (o *User) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *User) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *User) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetPhone

`func (o *User) GetPhone() string`

GetPhone returns the Phone field if non-nil, zero value otherwise.

### GetPhoneOk

`func (o *User) GetPhoneOk() (*string, bool)`

GetPhoneOk returns a tuple with the Phone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhone

`func (o *User) SetPhone(v string)`

SetPhone sets Phone field to given value.

### HasPhone

`func (o *User) HasPhone() bool`

HasPhone returns a boolean if a field has been set.

### SetPhoneNil

`func (o *User) SetPhoneNil(b bool)`

 SetPhoneNil sets the value for Phone to be an explicit nil

### UnsetPhone
`func (o *User) UnsetPhone()`

UnsetPhone ensures that no value is present for Phone, not even an explicit nil
### GetPhoneVerifiedAt

`func (o *User) GetPhoneVerifiedAt() string`

GetPhoneVerifiedAt returns the PhoneVerifiedAt field if non-nil, zero value otherwise.

### GetPhoneVerifiedAtOk

`func (o *User) GetPhoneVerifiedAtOk() (*string, bool)`

GetPhoneVerifiedAtOk returns a tuple with the PhoneVerifiedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhoneVerifiedAt

`func (o *User) SetPhoneVerifiedAt(v string)`

SetPhoneVerifiedAt sets PhoneVerifiedAt field to given value.

### HasPhoneVerifiedAt

`func (o *User) HasPhoneVerifiedAt() bool`

HasPhoneVerifiedAt returns a boolean if a field has been set.

### SetPhoneVerifiedAtNil

`func (o *User) SetPhoneVerifiedAtNil(b bool)`

 SetPhoneVerifiedAtNil sets the value for PhoneVerifiedAt to be an explicit nil

### UnsetPhoneVerifiedAt
`func (o *User) UnsetPhoneVerifiedAt()`

UnsetPhoneVerifiedAt ensures that no value is present for PhoneVerifiedAt, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
